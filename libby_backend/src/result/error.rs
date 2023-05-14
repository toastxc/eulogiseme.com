#[derive(Debug, Clone)]
pub enum Error {
    // database errrors
    Mongo(mongodb::error::Error),
    // data errors (duplicate user, reused email, etc)
    Parity(DataParity),
    // HTTP Error codes
    Status(Status),
    // Catchall for other errors
    Generic(String),
}

use std::fmt;

use rocket::http::Status;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = match self {
            Error::Mongo(_) => String::from("DatabaseError"),
            Error::Parity(ptype) => format!("{:#?}", ptype),
            Error::Status(status) => format!("{}", status),
            Error::Generic(str) => format!("{}", str),
        };

        write!(f, "{}", data)
    }
}
#[derive(Debug, Clone)]
pub enum DataParity {
    // duplicated fields
    // these fields have been used before
    DuplicatedUser,
    DuplicatedEmail,
    // bad fields
    // they use invalid characters, malformed emails, etc
    BadEmail,
    BadUserName,
    // illegal fields
    // these are not allowed
    BadEmailProvider,
    // HTTP based errors have been moved to Status
}
