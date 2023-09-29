use actix_web::{App, HttpServer};
use api::user;
use std::io::Result;

mod api;
mod model;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(move || App::new().service(user::new_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
