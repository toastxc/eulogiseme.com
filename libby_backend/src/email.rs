use crate::{
    mongo::{DBError, DB},
    structs::{Access, DataClientEmail, DataUser},
};

pub async fn register(data: DataClientEmail) -> Result<(), DBError> {
    println!("[INFO] Processing email request");

    let db = DB::new();

    db.user_insert(data).await?;

    println!("[INFO] There is currently no way to create emails :/");

    Ok(())
}

pub async fn fetch(access: Option<&str>) -> Result<DataUser, DBError> {
    println!("{:#?}", access);

    let token = match access {
        None => return Err(DBError::Parity(crate::mongo::DataParity::AccessDenied)),
        Some(token) => token,
    };

    let json = String::from_utf8(std::fs::read("./conf/access.json").unwrap()).unwrap();

    let data: Access = serde_json::from_str(&json).unwrap();

    if !data.authorised(token) {
        return Err(DBError::Parity(crate::mongo::DataParity::AccessDenied));
    };

    Ok(DataUser::from_vec(DB::new().user_vec().await?))
}
