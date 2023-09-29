use crate::model::{
    role::Role,
    user::{NewUser, User},
};
use actix_web::{post, web::Json, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostNewUserReq {
    name: String,
    email: String,
    pw: String,
    role: Option<Role>,
}

#[post("/user")]
pub async fn new_user(req: Json<PostNewUserReq>) -> Result<Json<User>> {
    let user = User::new(NewUser {
        name: req.name.clone(),
        email: req.email.clone(),
        pw: req.pw.clone(),
        role: req.role,
    });

    Ok(Json(user))
}
