use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pw: String,
}

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub role: Option<Role>,
    pub pw: String,
}

impl User {
    // Create
    pub fn new(req: NewUser) -> Self {
        use uuid::Uuid;

        User {
            uid: Uuid::new_v4().to_string(),
            name: req.name,
            email: req.email,
            pw: req.pw,
            role: if let Some(role) = req.role {
                role
            } else {
                Role::User
            },
        }
    }
}
