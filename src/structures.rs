use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sled_json::JsonDb;

lazy_static! {
    pub static ref DB: JsonDb = JsonDb::open("db").unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Resp {
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenUrl {
    pub channel_url: String,
    pub url: String,
}
