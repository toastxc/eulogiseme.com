use std::collections::HashMap;

use rocket::FromForm;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Serialize, Deserialize, Clone, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct DataClientEmail {
    pub email: String,
    pub name: String,
    pub description: String,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Access {
    list: Vec<String>,
}

impl Access {
    pub fn authorised(&self, input: &str) -> bool {
        self.list.contains(&String::from(input))
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct DataUser {
    pub users: HashMap<String, String>,
}

impl DataUser {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn from_vec(input: Vec<DataClientEmail>) -> Self {
        let mut data = DataUser::new();
        for entry in input {
            data.users.insert(entry.name, entry.email);
        }

        data
    }
}
