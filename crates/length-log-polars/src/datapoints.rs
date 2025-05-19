use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use chrono::NaiveDate;
use length_log_core::ports::data::DataRepository;
use miette::IntoDiagnostic;
use polars::{
    error::PolarsError,
    frame::DataFrame,
    io::SerReader,
    prelude::{Column, DataType, ParquetReader, ParquetWriter},
};

pub struct PolarsDataRepo {
    datapoints: RwLock<DataFrame>,
    path: Option<PathBuf>,
}

impl Default for PolarsDataRepo {
    fn default() -> Self {
        let names = Column::new_empty("name".into(), &DataType::String);
        let dates = Column::new_empty("date".into(), &DataType::Date);
        let datum = Column::new_empty("data".into(), &DataType::Float64);
        let datapoints = RwLock::new(DataFrame::new(vec![dates, names, datum]).unwrap());
        Self {
            datapoints,
            path: None,
        }
    }
}

impl PolarsDataRepo {
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            path: Some(path),
            ..Default::default()
        }
    }
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, PolarsError> {
        let mut file = File::open(path.as_ref())?;
        let datapoints = ParquetReader::new(&mut file).finish()?;
        Ok(Self {
            datapoints: RwLock::new(datapoints),
            path: Some(path.as_ref().into()),
        })
    }
}

impl DataRepository for PolarsDataRepo {
    fn save(
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
        let names = Column::new("name".into(), &[name.as_str()]);
        let dates = Column::new("date".into(), &[date]);
        let datum = Column::new("data".into(), &[data]);
        let datapoint = DataFrame::new(vec![dates, names, datum]).unwrap();
        self.datapoints.write().unwrap().extend(&datapoint).unwrap();
        println!("{:?}", self.datapoints);
        Ok(())
    }
    fn dump(&self) -> miette::Result<()> {
        let mut file = File::create(self.path.as_ref().unwrap()).unwrap();
        let mut datapoints = self.datapoints.write().unwrap();
        ParquetWriter::new(&mut file)
            .finish(&mut datapoints)
            .into_diagnostic()?;
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
