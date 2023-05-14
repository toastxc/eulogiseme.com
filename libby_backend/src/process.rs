use rocket::http::Status;

use crate::{
    mongo::DB,
    result::error::Error,
    structs::{Access, DataClientEmail},
};

pub async fn user_register(data: DataClientEmail) -> Result<(), Error> {
    println!("[INFO] Processing email request");

    let db = DB::new();

    db.user_insert(data).await?;

    println!("[INFO] There is currently no way to create emails :/");

    Ok(())
}

pub async fn fetch(access: Option<&str>) -> Result<Vec<DataClientEmail>, Error> {
    println!("{:#?}", access);

    let token = match access {
        None => return Err(Error::Status(Status::NotFound)),
        Some(token) => token,
    };

    let json = String::from_utf8(std::fs::read("./conf/access.json").unwrap()).unwrap();

    let data: Access = serde_json::from_str(&json).unwrap();
    if !data.authorised(token) {
        return Err(Error::Status(Status::Forbidden));
    };

    let newdata = DB::new().user_vec().await;

    println!("{:#?}", newdata);

    newdata
}
