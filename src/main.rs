use actix_web::{web::Data, App, HttpServer};
use api::user;
use model::user::User;
use std::{collections::HashMap, io::Result, sync::Mutex};
use store::Store;

mod api;
mod model;
mod store;

#[actix_web::main]
async fn main() -> Result<()> {
    let data = Data::new(Store {
        users: Mutex::new(HashMap::<String, User>::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(user::new_user)
            .service(user::get_users)
            .service(user::get_user_by_id)
            .service(user::update_user_by_id)
            .service(user::delete_user_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
