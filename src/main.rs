#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::{
    config::{Config, Environment, LoggingLevel},
    data::FromData,
};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

#[post("/api", format="json", data="<data>")]
fn api(mut data: Json<MyData>) -> Json<MyData> {
    data.a += 1;
    println!("{:?}", data.a);

    data
}

fn main() {
    let config = Config::build(Environment::Development)
        .log_level(LoggingLevel::Debug)
        .finalize().expect("Config failed");

    rocket::custom(config).mount("/", routes![api]).launch();
}

#[derive(Serialize,Deserialize,Debug)]
struct MyData {
    a: i32,
}
