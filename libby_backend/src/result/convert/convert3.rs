use rocket::http::Status;

use crate::result::error::{DataParity, Error};

pub trait Converter3<T: std::fmt::Debug> {
    fn opt_r(self) -> Result<T, Error>;
}

impl<T: std::fmt::Debug> Converter3<T> for Option<Result<T, mongodb::error::Error>> {
    fn opt_r(self) -> Result<T, Error> {
        match self {
            Some(data) => match data {
                Ok(a) => Ok(a),
                Err(error) => Err(Error::Mongo(error)),
            },
            None => Err(Error::Status(Status::NotFound)),
        }
    }
}

impl<T: std::fmt::Debug> Converter3<T> for Option<Result<T, Status>> {
    fn opt_r(self) -> Result<T, Error> {
        match self {
            Some(data) => match data {
                Ok(a) => Ok(a),
                Err(error) => Err(Error::Status(error)),
            },
            None => Err(Error::Status(Status::NotFound)),
        }
    }
}

impl<T: std::fmt::Debug> Converter3<T> for Option<Result<T, DataParity>> {
    fn opt_r(self) -> Result<T, Error> {
        match self {
            Some(data) => match data {
                Ok(a) => Ok(a),
                Err(error) => Err(Error::Parity(error)),
            },
            None => Err(Error::Status(Status::NotFound)),
        }
    }
}

impl<T: std::fmt::Debug> Converter3<T> for Option<Result<T, String>> {
    fn opt_r(self) -> Result<T, Error> {
        match self {
            Some(data) => match data {
                Ok(a) => Ok(a),
                Err(error) => Err(Error::Generic(error)),
            },
            None => Err(Error::Status(Status::NotFound)),
        }
    }
}
