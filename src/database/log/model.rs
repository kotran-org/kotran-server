use diesel::prelude::*;

use crate::schema::{log_activity, log_api, log_auth};

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = log_auth)]
pub struct AuthLog {
    pub id: i32,
    pub user_id: i32,
    pub status_code: i32,
    pub device_id: String,
    pub ip_addr: String,
    pub fail_count: i32,
    pub fail_reason: String,
    pub time: i64
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = log_api)]
pub struct ApiLog {
    pub id: i32,
    pub token_id: i32,
    pub status_code: i32,
    pub endpoint: String,
    pub time: i64
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = log_activity)]
pub struct ActivityLog {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub desc: String,
    pub time: i64,
}