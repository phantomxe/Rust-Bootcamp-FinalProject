use once_cell::sync::Lazy;

use crate::sales::SalesManager;

pub const USERS: Lazy<Vec<AuthUser>> =  Lazy::new(|| 
vec![
    AuthUser {
        username: "demo".to_string(),
        password: "demo".to_string(),
    }
]);

pub struct AuthUser {
    pub username: String,
    pub password: String,
}

impl From<AuthUser> for SalesManager {
    fn from(user: AuthUser) -> Self {
        Self {
            name: user.username,
        }
    }
}