use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use chrono::{Datelike, NaiveDate};
use miette::IntoDiagnostic;
use polars::{
    error::PolarsError,
    frame::DataFrame,
    functions::concat_df_diagonal,
    io::SerReader,
    prelude::{
        AnyValue, Column, DataType, DateType, IntoLazy, ParquetReader, ParquetWriter, col, lit,
    },
};

use length_log_core::{
    models::{AddPersonError, Person, PersonName},
    ports::LengthLogRepository,
};
use polars::{
    io::SerWriter,
    prelude::{CsvReadOptions, CsvWriter, Field, Schema},
    series::ChunkCompareEq,
};

pub struct PolarsRepository {
    datapoints: RwLock<DataFrame>,
    data_path: Option<PathBuf>,
    persons: RwLock<DataFrame>,
    person_path: Option<PathBuf>,
}

impl Default for PolarsRepository {
    fn default() -> Self {
        let dates = Column::new_empty("date".into(), &DataType::Date);
        let datapoints = RwLock::new(DataFrame::new(vec![dates]).unwrap());
        let columns = vec![
            Column::new_empty("name".into(), &DataType::String),
            Column::new_empty("start_date".into(), &DataType::Date),
        ];
        let persons = RwLock::new(
            DataFrame::new(columns).expect("length-log-polars/persons: valid DataFrame"),
        );
        Self {
            datapoints,
            persons,
            data_path: None,
            person_path: None,
        }
    }
}

impl PolarsRepository {
    pub fn with_paths(data_path: PathBuf, person_path: PathBuf) -> Self {
        Self {
            data_path: Some(data_path),
            person_path: Some(person_path),
            ..Default::default()
        }
    }
    pub fn from_path<P: AsRef<Path>>(data_path: P, person_path: P) -> Result<Self, PolarsError> {
        let mut file = File::open(data_path.as_ref())?;
        let datapoints = ParquetReader::new(&mut file).finish()?;
        log::debug!("Reading Persons from {}", person_path.as_ref().display());
        let schema = Arc::new(Schema::from_iter(vec![
            Field::new("name".into(), DataType::String),
            Field::new("start_date".into(), DataType::Date),
        ]));
        let persons = CsvReadOptions::default()
            .with_has_header(true)
            .with_schema(Some(schema))
            .try_into_reader_with_file_path(Some(person_path.as_ref().into()))?
            .finish()?;
        Ok(Self {
            datapoints: RwLock::new(datapoints),
            persons: RwLock::new(persons),
            data_path: Some(data_path.as_ref().into()),
            person_path: Some(person_path.as_ref().into()),
        })
    }
    fn dump_datapoints(&self) -> miette::Result<()> {
        let mut file = File::create(self.data_path.as_ref().unwrap()).unwrap();
        let mut datapoints = self.datapoints.write().unwrap();
        ParquetWriter::new(&mut file)
            .finish(&mut datapoints)
            .into_diagnostic()?;
        Ok(())
    }
    fn dump_persons(&self) -> miette::Result<()> {
        let mut file = File::create(self.person_path.as_ref().unwrap()).unwrap();
        let mut persons = self.persons.write().unwrap();
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut persons)
            .into_diagnostic()?;
        Ok(())
    }
}

impl LengthLogRepository for PolarsRepository {
    fn get_person(
        &self,
        name: &length_log_core::models::PersonName,
    ) -> Result<Option<length_log_core::models::Person>, length_log_core::ports::ServiceError> {
        let persons = self.persons.read().unwrap();
        let mask = persons
            .column("name")
            .unwrap()
            .str()
            .unwrap()
            .equal(name.as_str());
        let filtered_persons = persons
            .clone()
            .lazy()
            .filter(col("name").eq(lit(name.as_str())))
            .collect()
            .unwrap();
        let person = filtered_persons.get_row(0).unwrap();
        dbg!(&person);
        dbg!(&person.0[0]);
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let epoch_days_since_ce = epoch.num_days_from_ce();
        let name = PersonName::new(&person.0[0].get_str().unwrap()).unwrap();
        let start_date = match person.0[1].cast(&DataType::Date) {
            AnyValue::Date(days) => {
                NaiveDate::from_num_days_from_ce_opt(days + epoch_days_since_ce).unwrap()
            }
            _ => todo!(),
        };
        Ok(Some(Person::new(name, start_date)))
    }
    fn save_datapoint(
        &self,
        name: &length_log_core::models::PersonName,
        date: NaiveDate,
        data: f64,
    ) -> Result<(), length_log_core::ports::ServiceError> {
        log::info!(
            "saving data for name={} data={} date={:?}",
            name,
            data,
            date
        );

        let columns = vec![
            Column::new("date".into(), &[date]),
            Column::new(name.as_str().into(), &[data]),
        ];
        // let persons = self.persons.read().unwrap();
        // let person_names = persons.column("name").unwrap();
        // for col_name in person_names {
        //     dbg!(&col_name);

        //     if col_name == name.as_str() {
        //         columns.push(Column::new(name.as_str().into(), &[data]));
        //     } else {
        //         columns.push(Column::new(col_name.into(), &[0.0f64]));
        //     }
        // }
        let datapoint = DataFrame::new(columns).unwrap();
        dbg!(&datapoint);
        {
            let mut datapoints = self.datapoints.write().unwrap();
            let tmp = concat_df_diagonal(&[datapoints.clone(), datapoint])
                // .extend(&datapoint)
                .unwrap();
            *datapoints = tmp;
        }
        println!("{:?}", self.datapoints);
        Ok(())
    }
    fn save_person(&self, person: &length_log_core::models::Person) -> Result<(), AddPersonError> {
        {
            let persons = self.persons.read().unwrap();
            let mask = persons
                .column("name")
                .unwrap()
                .str()
                .unwrap()
                .equal(person.name().as_str());
            let person_exists = persons.filter(&mask).unwrap().column("name").unwrap().len() > 0;
            if person_exists {
                return Err(AddPersonError::Duplicate {
                    name: person.name().clone(),
                });
            }
        }

        let names = Column::new("name".into(), vec![person.name().as_str()]);
        let start_dates = Column::new("start_date".into(), vec![person.start_date()]);
        let person_df = DataFrame::new(vec![names, start_dates]).unwrap();
        self.persons.write().unwrap().extend(&person_df).unwrap();
        log::debug!("persons ={:?}", self.persons);

        let name_column = Column::new::<Vec<f64>, _>(person.name().as_str().into(), vec![]);
        let name_data_df = DataFrame::new(vec![name_column]).unwrap();
        {
            let mut datapoints = self.datapoints.write().unwrap();
            *datapoints =
                polars::functions::concat_df_horizontal(&[datapoints.clone(), name_data_df], true)
                    .unwrap();
        }
        log::debug!("datapoints={:?}", self.datapoints);
        Ok(())
    }
    fn dump(&self) -> miette::Result<()> {
        self.dump_persons()?;
        self.dump_datapoints()?;
        // let mut file = File::create(self.path.as_ref().unwrap()).unwrap();
        // let mut datapoints = self.datapoints.write().unwrap();
        // ParquetWriter::new(&mut file)
        //     .finish(&mut datapoints)
        //     .into_diagnostic()?;
        Ok(())
    }
    // fn save(&self, id: &str, date: NaiveDate, data: f64) -> Result<(), services::ServiceError> {
    //     log::info!("saving data for id={:?} data={} date={:?}", id, data, date);
    //     let ids = Column::new("id", vec![id]);
    //     let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    //     let date = AnyValue::Date(date.signed_duration_since(epoch).num_days() as i32);
    //     let dates = Column::new("date", vec![date]);
    //     let datum = Column::new("data", vec![data]);
    //     let datapoint = DataFrame::new(vec![ids, dates, datum]).unwrap();
    //     self.datapoints.write().unwrap().extend(&datapoint).unwrap();
    //     println!("{:?}", self.datapoints);
    //     Ok(())
    // }
}
