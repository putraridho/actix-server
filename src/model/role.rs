use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Role {
    Admin,
    User,
}
