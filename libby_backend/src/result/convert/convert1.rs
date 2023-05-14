use rocket::http::Status;

use crate::result::error::{DataParity, Error};

pub trait Converter<T: std::fmt::Debug> {
    fn res(self) -> Result<T, Error>;
}

impl<T: std::fmt::Debug> Converter<T> for Result<T, mongodb::error::Error> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Mongo(error)),
        }
    }
}

impl<T: std::fmt::Debug> Converter<T> for Result<T, Status> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Status(error.to_owned())),
        }
    }
}

impl<T: std::fmt::Debug> Converter<T> for Result<T, DataParity> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Parity(error)),
        }
    }
}

impl<T: std::fmt::Debug> Converter<T> for Result<T, String> {
    fn res(self) -> Result<T, Error> {
        match self {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Generic(error)),
        }
    }
}
