use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use length_log_core::{models::AddPersonError, ports::person::PersonRepository};
use miette::IntoDiagnostic;
use polars::{
    error::PolarsError,
    frame::DataFrame,
    io::{SerReader, SerWriter},
    prelude::{Column, CsvReadOptions, CsvWriter, DataType, Field, Schema},
    series::ChunkCompareEq,
};

pub struct PolarsPersonRepo {
    persons: RwLock<DataFrame>,
    path: Option<PathBuf>,
}

impl Default for PolarsPersonRepo {
    fn default() -> Self {
        let columns = vec![
            Column::new_empty("name".into(), &DataType::String),
            Column::new_empty("start_date".into(), &DataType::Date),
        ];
        let persons = RwLock::new(
            DataFrame::new(columns).expect("length-log-polars/persons: valid DataFrame"),
        );
        Self {
            persons,
            path: None,
        }
    }
}

impl PolarsPersonRepo {
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            path: Some(path),
            ..Default::default()
        }
    }
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, PolarsError> {
        log::debug!("Reading PolarsPersonRepo from {}", path.as_ref().display());
        let schema = Arc::new(Schema::from_iter(vec![
            Field::new("name".into(), DataType::String),
            Field::new("start_date".into(), DataType::Date),
        ]));
        let persons = CsvReadOptions::default()
            .with_has_header(true)
            .with_schema(Some(schema))
            .try_into_reader_with_file_path(Some(path.as_ref().into()))?
            .finish()?;
        Ok(Self {
            persons: RwLock::new(persons),
            path: Some(path.as_ref().into()),
        })
    }

    pub fn from_path_or_default<P: AsRef<Path>>(path: P) -> Self {
        match Self::from_path(path.as_ref()) {
            Ok(df) => df,
            Err(err) => {
                log::warn!(
                    "Could not open {}: {:?}. Creating default ...",
                    path.as_ref().display(),
                    err
                );
                Self::default()
            }
        }
    }
}

impl PersonRepository for PolarsPersonRepo {
    fn save(&self, person: &length_log_core::models::Person) -> Result<(), AddPersonError> {
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
        let person = DataFrame::new(vec![names, start_dates]).unwrap();
        self.persons.write().unwrap().extend(&person).unwrap();
        log::debug!("persons ={:?}", self.persons);
        Ok(())
    }
    fn dump(&self) -> miette::Result<()> {
        let mut file = File::create(self.path.as_ref().unwrap()).unwrap();
        let mut persons = self.persons.write().unwrap();
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut persons)
            .into_diagnostic()?;
        Ok(())
    }
}
