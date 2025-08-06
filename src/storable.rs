use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

#[derive(Debug)]
pub enum Error {
    IoE(io::Error),
    ParTomlE(toml::ser::Error),
    DesTomlE(toml::de::Error),
}

pub trait Storable {
    fn save<P: AsRef<Path>>(&self, path: P, new_create: bool) -> Result<(), Error>
    where
        Self: Serialize + DeserializeOwned,
    {
        let s = toml::to_string_pretty(self).map_err(Error::ParTomlE)?;
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(new_create)
            .open(path)
            .unwrap();
        f.write_all(s.as_bytes()).map_err(Error::IoE)
    }
    fn load<T, P: AsRef<Path>>(path: P) -> Result<T, Error>
    where
        T: Serialize + DeserializeOwned,
    {
        let file_content = fs::read_to_string(path).map_err(Error::IoE)?;
        toml::from_str::<T>(&file_content).map_err(Error::DesTomlE)
    }
}
