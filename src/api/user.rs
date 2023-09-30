use crate::{
    model::{
        role::Role,
        user::{NewUser, UpdateUser, User},
    },
    store::Store,
};
use actix_web::{
    delete, error, get, post, put,
    web::{Data, Json, Path},
    Result,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostNewUserReq {
    name: String,
    email: String,
    pw: String,
    role: Option<Role>,
}

#[derive(Deserialize, Clone)]
pub struct PutUpdateUserByIdReq {
    name: Option<String>,
    email: Option<String>,
    pw: Option<String>,
    role: Option<Role>,
}

#[post("/user")]
pub async fn new_user(req: Json<PostNewUserReq>, data: Data<Store>) -> Result<Json<User>> {
    let user = User::new(NewUser {
        name: req.name.clone(),
        email: req.email.clone(),
        pw: req.pw.clone(),
        role: req.role,
    });

    let mut users = data.users.lock().unwrap();

    let cloned_user = user.clone();

    users.insert(cloned_user.uid.to_string(), cloned_user);

    Ok(Json(user))
}

#[get("/user")]
pub async fn get_users(data: Data<Store>) -> Result<Json<Vec<User>>> {
    let users = data.users.lock().unwrap();

    let vec_users = users.values().map(|v| v.to_owned()).collect();

    Ok(Json(vec_users))
}

#[get("/user/{uid}")]
pub async fn get_user_by_id(uid: Path<String>, data: Data<Store>) -> Result<Json<User>> {
    let users = data.users.lock().unwrap();

    if let Some(user) = users.get(&uid.to_owned()) {
        Ok(Json(user.to_owned()))
    } else {
        Err(error::ErrorNotFound("User Not Found: Invalid uid"))
    }
}

#[put("/user/{uid}")]
pub async fn update_user_by_id(
    uid: Path<String>,
    req: Json<PutUpdateUserByIdReq>,
    data: Data<Store>,
) -> Result<Json<User>> {
    let mut users = data.users.lock().unwrap();

    (*users).entry(uid.to_string()).and_modify(|user| {
        *user = user.clone().update(UpdateUser {
            name: req.name.clone(),
            email: req.email.clone(),
            pw: req.pw.clone(),
            role: req.role,
        });
    });

    if let Some(user) = (*users).get(&uid.to_string()) {
        Ok(Json((*user).clone()))
    } else {
        Err(error::ErrorNotFound("User Not Found: Invalid uid"))
    }
}

#[delete("/user/{uid}")]
pub async fn delete_user_by_id(uid: Path<String>, data: Data<Store>) -> Result<Json<()>> {
    let mut users = data.users.lock().unwrap();

    if let Some(_) = (*users).get(&uid.to_owned()) {
        (*users).remove(&uid.to_string());
        Ok(Json(()))
    } else {
        Err(error::ErrorNotFound("User Not Found: Invalid uid"))
    }
}
