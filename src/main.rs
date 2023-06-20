mod sled_json;

use std::{io::Result, format};
use actix_web::{HttpServer, App, get, post, HttpResponse, web::{ Json, Path}};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use sled_json::JsonDb;

lazy_static! {
    static ref DB: JsonDb = JsonDb::open("idk").unwrap();
}

#[derive(Serialize, Deserialize)]
struct Resp {
    info: String
}


#[derive(Serialize, Deserialize, Debug)]
struct GenUrl {
    channel_url: String,
    url: String
}

#[actix_web::main]
async fn main() -> Result<()>{
    HttpServer::new(|| {
        App::new().service(get_idk).service(get_id)
    }).bind(("localhost", 8090)).unwrap().run().await.unwrap();
    Ok(())
}

#[get("/{id}")]
async fn get_idk(id: Path<String>) -> HttpResponse {
    match DB.get::<GenUrl>(&id.to_string()) {
        Ok(r) => {
                    HttpResponse::Ok().json(Resp{info: format!("{} {}", r.url, r.channel_url)})
        },
        Err(_) => HttpResponse::InternalServerError().json(Resp{info: "Sorry Error is in server!".into()})
    }
}

#[post("/gen-url")]
async fn get_id(req: Json<GenUrl>) -> HttpResponse {
    let uuid_gen: String = uuid::Uuid::new_v4().to_string().split("-").collect();
    match DB.insert(&uuid_gen, &req.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(Resp{info: uuid_gen}),
        Err(_) => HttpResponse::InternalServerError().json(Resp{info: "Sorry Error is in server!".into()})
    }
}
