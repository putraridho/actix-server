use std::{collections::HashMap, sync::Mutex};

use crate::model::user::User;

pub struct Store {
    pub users: Mutex<HashMap<String, User>>,
}
