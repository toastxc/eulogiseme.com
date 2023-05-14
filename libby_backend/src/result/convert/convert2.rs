use rocket::http::Status;

use crate::result::error::{DataParity, Error};

pub trait Converter2<T: std::fmt::Debug> {
    fn opt(self) -> Result<T, Error>;
}
impl<T: std::fmt::Debug> Converter2<T> for Result<Option<T>, mongodb::error::Error> {
    fn opt(self) -> Result<T, Error> {
        match self {
            Ok(data) => match data {
                Some(data) => Ok(data),
                None => Err(Error::Status(Status::NotFound)),
            },
            Err(error) => Err(Error::Mongo(error)),
        }
    }
}
impl<T: std::fmt::Debug> Converter2<T> for Result<Option<T>, Status> {
    fn opt(self) -> Result<T, Error> {
        match self {
            Ok(data) => match data {
                Some(data) => Ok(data),
                None => Err(Error::Status(Status::NotFound)),
            },
            Err(error) => Err(Error::Status(error.to_owned())),
        }
    }
}
impl<T: std::fmt::Debug> Converter2<T> for Result<Option<T>, DataParity> {
    fn opt(self) -> Result<T, Error> {
        match self {
            Ok(data) => match data {
                Some(data) => Ok(data),
                None => Err(Error::Status(Status::NotFound)),
            },
            Err(error) => Err(Error::Parity(error)),
        }
    }
}
impl<T: std::fmt::Debug> Converter2<T> for Result<Option<T>, String> {
    fn opt(self) -> Result<T, Error> {
        match self {
            Ok(data) => match data {
                Some(data) => Ok(data),
                None => Err(Error::Status(Status::NotFound)),
            },
            Err(error) => Err(Error::Generic(error)),
        }
    }
}
