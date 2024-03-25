use std::{
    fs::File,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use chrono::NaiveDate;
use length_log_core::services;
use polars::{
    datatypes::{AnyValue, DataType},
    frame::DataFrame,
    io::{parquet::{ParquetReader, ParquetWriter}, SerReader},
    prelude::NamedFrom,
    series::Series,
};

use super::PolarsServiceError;

pub struct PolarsDataService {
    datapoints: RwLock<DataFrame>,
    path: Option<PathBuf>,
}

impl Default for PolarsDataService {
    fn default() -> Self {
        let ids = Series::new_empty("id", &DataType::String);
        let dates = Series::new_empty("date", &DataType::Date);
        let datum = Series::new_empty("data", &DataType::Float64);
        let datapoints = RwLock::new(DataFrame::new(vec![ids, dates, datum]).unwrap());
        Self {
            datapoints,
            path: None,
        }
    }
}

impl PolarsDataService {
    pub fn new_shared() -> services::SharedDataService {
        Arc::new(Self::default())
    }
    pub fn load_or_create(
        path: PathBuf,
    ) -> Result<Arc<Self>, PolarsServiceError> {
        match File::open(&path) {
            Ok(file) => {
                let datapoints = ParquetReader::new(file).finish()?;
                let datapoints = RwLock::new(datapoints);
                Ok(Arc::new(Self {
                    datapoints,
                    path: Some(path),
                }))
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
    pub fn dump(&self) -> Result<(), PolarsServiceError> {
        let file = File::create(self.path.as_ref().unwrap()).unwrap();
        let mut datapoints = self.datapoints.write().unwrap();
        ParquetWriter::new(file).finish(&mut datapoints)?;
        Ok(())
    }
}

impl services::DataService for PolarsDataService {
    fn save(&self, id: &str, date: NaiveDate, data: f64) -> Result<(), services::ServiceError> {
        log::info!("saving data for id={:?} data={} date={:?}", id, data, date);
        let ids = Series::new("id", vec![id]);
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let date = AnyValue::Date(date.signed_duration_since(epoch).num_days() as i32);
        let dates = Series::new("date", vec![date]);
        let datum = Series::new("data", vec![data]);
        let datapoint = DataFrame::new(vec![ids, dates, datum]).unwrap();
        self.datapoints.write().unwrap().extend(&datapoint).unwrap();
        println!("{:?}", self.datapoints);
        Ok(())
    }
}
