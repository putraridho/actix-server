use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
    pub pw: Option<String>,
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

    pub fn update(mut self, req: UpdateUser) -> Self {
        if let Some(name) = req.name {
            self.name = name
        }

        if let Some(email) = req.email {
            self.email = email
        }

        if let Some(pw) = req.pw {
            self.pw = pw
        }

        if let Some(role) = req.role {
            self.role = role
        }
        self
    }
}
