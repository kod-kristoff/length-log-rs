use std::{
    fs::File,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use chrono::{Days, NaiveDate};
use length_log_core::{models, services};
use polars::{
    datatypes::{AnyValue, ArrowDataType, DataType},
    frame::DataFrame,
    io::{
        SerReader, SerWriter,
    },
    prelude::{ArrowField, NamedFrom, Schema},
    series::{ Series},
};
use polars::prelude::{ChunkCompareEq,CsvReader, CsvWriter, CsvReadOptions};

use super::PolarsServiceError;

pub struct PolarsPersonService {
    persons: RwLock<DataFrame>,
    path: Option<PathBuf>,
}

impl Default for PolarsPersonService {
    fn default() -> Self {
        let ids = Series::new_empty("id".into(), &DataType::String);
        let names = Series::new_empty("name".into(), &DataType::String);
        let start_dates = Series::new_empty("start_date".into(), &DataType::Date);
        let persons = RwLock::new(DataFrame::new(vec![ids.into(), names.into(), start_dates.into()]).unwrap());
        Self {
            persons,
            path: None,
        }
    }
}
impl PolarsPersonService {
    pub fn new_shared() -> services::SharedPersonService {
        Arc::new(Self::default())
    }
    pub fn load_or_create(path: PathBuf) -> Result<Arc<Self>, PolarsServiceError> {
        match Self::try_load(&path) {
        // match CsvReader::from_path(&path) {
            Ok(arc_self) => {
                Ok(arc_self)
            }
            Err(err) => {
                log::warn!(
                    "Couldn't open {}: '{:?}'. Creating new ...",
                    path.display(),
                    err
                );
                Ok(Arc::new(Self {
                    path: Some(path),
                    ..Default::default()
                }))
            }
        }
    }

    fn try_load(path: &Path) -> Result<Arc<Self>, PolarsServiceError> {
        log::debug!("reading file '{}'", path.display());
        let schema = Arc::new(Schema::from_iter(&[
            ArrowField::new("id".into(), ArrowDataType::Utf8, false),
            ArrowField::new("name".into(), ArrowDataType::Utf8, false),
            ArrowField::new("start_date".into(), ArrowDataType::Date32, false),
        ]));
        let df = CsvReadOptions::default()
            .with_has_header(true)
            .with_schema(Some(schema))
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
    .try_into_reader_with_file_path(Some(path))?
    .finish()?;

        Ok(Arc::new(Self {
            persons: RwLock::new(
                df
            ),
            path: Some(path.into())
        }))
    }

    pub fn dump(&self) -> Result<(), PolarsServiceError> {
        let mut file = File::create(self.path.as_ref().unwrap()).unwrap();
        let mut persons = self.persons.write().unwrap();
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut persons)?;
        Ok(())
    }
}

impl services::PersonService for PolarsPersonService {
    fn get_by_name(&self, name: &str) -> Result<Option<models::Person>, services::ServiceError> {
        log::info!("searching for name={}", name);
        Ok(None)
    }
    fn get_id_by_name(&self, name: &str) -> Result<String, services::ServiceError> {
        log::info!("searching id for name={}", name);
        let persons = self.persons.read().unwrap();
        let names = persons.column("name").unwrap().str().unwrap();
        let mask = names.equal(name);
        let df = persons.filter(&mask).unwrap();
        println!("{:?}", df);
        let id = df.column("id").unwrap();
        println!("{:?}", id);
        if id.len() == 0 {
            return Err(services::ServiceError::PersonNotFound(name.to_string()));
        }
        Ok(id.get(0).unwrap().to_string())
    }
    fn save(
        &self,
        models::Person {
            id,
            name,
            start_date,
        }: models::Person,
    ) -> Result<(), services::ServiceError> {
        log::info!(
            "saving person id={:?} name={} start_date={:?}",
            id,
            name,
            start_date
        );
        let ids = Series::new("id".into(), vec![id]);
        let names = Series::new("name".into(), vec![name]);
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let start_date = AnyValue::Date(start_date.signed_duration_since(epoch).num_days() as i32);
        let start_dates = Series::from_any_values("start_date".into(), &[start_date], true).unwrap();
        let person = DataFrame::new(vec![ids.into(), names.into(), start_dates.into()]).unwrap();
        self.persons.write().unwrap().extend(&person).unwrap();
        log::debug!("persons={:?}", self.persons);
        Ok(())
    }
    fn get_all(&self) -> Result<Vec<models::Person>, services::ServiceError> {
        log::info!("listing all persons");
        println!("{:?}", self.persons);
        let read_lock = self.persons.read().unwrap();
        let persons = read_lock.columns(["id", "name", "start_date"]).unwrap();
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let mut result = Vec::new();
        for row in 0..persons[0].len() {
            let mut id = String::new();
            let mut name = String::new();
            let mut start_date = None;
            for (i, series) in persons.iter().enumerate() {
                match i {
                    0 => id = series.get(row).unwrap().to_string(),
                    1 => name = series.get(row).unwrap().to_string(),
                    2 => {
                        if let AnyValue::Date(num_days) =
                            series.get(row).unwrap().cast(&DataType::Date)
                        {
                            start_date = epoch.checked_add_days(Days::new(num_days as u64));
                        } else {
                            todo!("handle this case");
                        }
                    }
                    _ => unreachable!(),
                }
            }
            result.push(models::Person::new(id, name, start_date));
        }
        Ok(result)
    }
}
