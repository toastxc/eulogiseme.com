use check_if_email_exists::{check_email, CheckEmailInput};
use futures::TryStreamExt;
use mongodb::{bson::doc, results::InsertOneResult, Collection, Database};

use crate::structs::DataClientEmail;

pub struct DB {}

// Collections
impl DB {
    pub async fn users(&self) -> Result<Collection<DataClientEmail>, DBError> {
        Ok(common().await?.collection::<DataClientEmail>("Users"))
    }
}

// utilities
impl DB {
    pub fn mas<T>(input: Result<T, mongodb::error::Error>) -> Result<T, DBError> {
        match input {
            Ok(data) => Ok(data),
            Err(mongo_er) => Err(DBError::Mongo(mongo_er)),
        }
    }

    pub fn sumer<T>(input: Result<Option<T>, mongodb::error::Error>) -> Result<T, DBError> {
        let a = DB::mas(input);

        match a {
            Ok(a) => match a {
                Some(a) => Ok(a),
                None => Err(DBError::Parity(DataParity::NotFound)),
            },
            Err(error) => Err(error),
        }
    }
    pub fn new() -> Self {
        Self {}
    }

    pub async fn email_validate(input: &str) -> Result<(), DBError> {
        let input = input.to_string();

        let email_other: Vec<&str> = input.split('@').collect();

        if email_other.contains(&"toastxc.xyz") {
            return Ok(());
        };

        let email_status = check_email(&CheckEmailInput::new(input.to_owned())).await;
        match email_status.is_reachable {
            check_if_email_exists::Reachable::Safe => Ok(()),
            check_if_email_exists::Reachable::Risky => {
                Err(DBError::Parity(DataParity::BadEmailProvider))
            }
            check_if_email_exists::Reachable::Invalid => Err(DBError::Parity(DataParity::BadEmail)),
            check_if_email_exists::Reachable::Unknown => Err(DBError::Parity(DataParity::BadEmail)),
        }
    }
}

// users
impl DB {
    pub async fn user_field_exists(&self, input: &DataClientEmail) -> Result<(), DBError> {
        let user = DB::mas(
            self.users()
                .await?
                .find_one(doc!("name": input.name.clone()), None)
                .await,
        )?
        .is_some();
        let email = DB::mas(
            self.users()
                .await?
                .find_one(doc!("email": input.email.clone()), None)
                .await,
        )?
        .is_some();

        match (user, email) {
            (true, _) => Err(DBError::Parity(DataParity::DuplicatedUser)),
            (_, true) => Err(DBError::Parity(DataParity::DuplicatedEmail)),
            _ => Ok(()),
        }
    }

    pub async fn user_fetch(&self, user_id: &str) -> Result<DataClientEmail, DBError> {
        DB::sumer(
            self.users()
                .await?
                .find_one(doc!("name": &user_id), None)
                .await,
        )
    }

    pub async fn user_insert(&self, input: DataClientEmail) -> Result<InsertOneResult, DBError> {
        // prerun check, validating email and username
        let validate = tokio::join!(
            self.user_field_exists(&input),
            DB::email_validate(&input.email),
        );

        validate.0?;
        validate.1?;

        // if all is valid, insert
        DB::mas(self.users().await?.insert_one(input, None).await)
    }

    pub async fn user_vec(&self) -> Result<Vec<DataClientEmail>, DBError> {
        Ok(DB::mas(
            DB::mas(self.users().await?.find(None, None).await)?
                .try_collect()
                .await,
        )?)
    }
}
async fn common() -> Result<Database, DBError> {
    DB::mas(
        easymongo::mongo::Mongo::new()
            .username("username")
            .password("password")
            .database("test")
            .db_generate()
            .await,
    )
}

pub enum DBError {
    Mongo(mongodb::error::Error),
    Parity(DataParity),
}

use std::fmt;

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = match self {
            DBError::Mongo(_) => String::from("DatabaseError"),
            DBError::Parity(ptype) => format!("{:#?}", ptype),
        };

        write!(f, "{}", data)
    }
}
#[derive(Debug)]
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
    // they use invalid characters, malformed emails, etc
    NotFound,
    AccessDenied,
}
