use crate::{
    result::{
        convert::{res, res_opt},
        error::{DataParity, Error},
    },
    structs::DataClientEmail,
};
use check_if_email_exists::{check_email, CheckEmailInput};
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::{bson::doc, results::InsertOneResult, Database};

pub struct DB {}

// Collections
impl DB {
    pub async fn users(&self) -> Result<Collection<DataClientEmail>, Error> {
        Ok(common().await?.collection::<DataClientEmail>("Users"))
    }
}

// utilities
impl DB {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn email_validate(input: &str) -> Result<(), Error> {
        let input = input.to_string();

        let email_other: Vec<&str> = input.split('@').collect();

        if email_other.contains(&"toastxc.xyz") {
            return Ok(());
        };

        let email_status = check_email(&CheckEmailInput::new(input.to_owned())).await;
        match email_status.is_reachable {
            check_if_email_exists::Reachable::Safe => Ok(()),
            check_if_email_exists::Reachable::Risky => {
                Err(Error::Parity(DataParity::BadEmailProvider))
            }
            check_if_email_exists::Reachable::Invalid => Err(Error::Parity(DataParity::BadEmail)),
            check_if_email_exists::Reachable::Unknown => Err(Error::Parity(DataParity::BadEmail)),
        }
    }
}

// users
impl DB {
    pub async fn user_field_exists(&self, input: &DataClientEmail) -> Result<(), Error> {
        let user = self
            .users()
            .await?
            .find_one(doc!("name": input.name.clone()), None)
            .await
            .res()?
            .is_some();

        let email = self
            .users()
            .await?
            .find_one(doc!("email": input.email.clone()), None)
            .await
            .res()?
            .is_some();

        match (user, email) {
            (true, _) => Err(Error::Parity(DataParity::DuplicatedUser)),
            (_, true) => Err(Error::Parity(DataParity::DuplicatedEmail)),
            _ => Ok(()),
        }
    }

    pub async fn user_fetch(&self, user_id: &str) -> Result<DataClientEmail, Error> {
        self.users()
            .await?
            .find_one(doc!("name": &user_id), None)
            .await
            .opt()
    }

    pub async fn user_insert(&self, input: DataClientEmail) -> Result<InsertOneResult, Error> {
        // prerun check, validating email and username
        let validate = tokio::join!(
            self.user_field_exists(&input),
            DB::email_validate(&input.email),
        );

        validate.0?;
        validate.1?;

        // if all is valid, insert

        self.users().await?.insert_one(input, None).await.res()
    }

    pub async fn user_vec(&self) -> Result<Vec<DataClientEmail>, Error> {
        self.users()
            .await?
            .find(None, None)
            .await
            .res()?
            .try_collect()
            .await
            .res()
    }
}
async fn common() -> Result<Database, Error> {
    easymongo::mongo::Mongo::new()
        .username("username")
        .password("password")
        .database("test")
        .db_generate()
        .await
        .res()
}

impl Default for DB {
    fn default() -> Self {
        Self::new()
    }
}
