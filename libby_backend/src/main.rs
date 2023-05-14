use std::net::{IpAddr, Ipv4Addr};

use libby_backend::{cors::CORS, process, structs::DataClientEmail};
use rocket::{form::Form, serde::json::Json, Config};

#[macro_use]
extern crate rocket;

#[post("/submit", data = "<data>")]
async fn user_register(data: Form<DataClientEmail>) -> Result<(), String> {
    let res = process::user_register(data.into_inner()).await;
    if let Err(res) = res {
        println!("{}", res);
        return Err(res.to_string());
    };
    Ok(())
}

#[get("/admin?<access>")]
pub async fn user_fetch(access: Option<&str>) -> Result<Json<Vec<DataClientEmail>>, String> {
    let res = process::fetch(access).await;

    match res {
        Err(res) => Err(res.to_string()),
        Ok(data) => Ok(Json(data)),
    }
}

#[launch]
async fn rocket() -> _ {
    let mut config = Config::default();
    let mut ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    if cfg!(debug_assertions) {
        ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
    };

    config.address = ip;

    rocket::custom(config)
        .mount("/", routes![user_register, user_fetch])
        .attach(CORS)
}
