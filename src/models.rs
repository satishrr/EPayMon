
use std::time::SystemTime;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub wallet_info: String,
    pub created_at: SystemTime,
    pub user_id: String
}