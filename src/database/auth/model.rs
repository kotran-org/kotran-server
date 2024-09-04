use std::{sync::Arc, time::SystemTime};
use async_trait::async_trait;
use diesel::prelude::*;

use crate::schema::{auth_password, auth_session, auth_token};

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = auth_password)]
pub struct AuthPassword {
    pub id: i32,
    pub user_id: i32,
    pub salt: String,
    pub password: String,
    pub update_time: i64
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = auth_session)]
pub struct AuthSession {
    pub id: i32,
    pub user_id: i32,
    pub session_key: String,
    pub creation_time: i64,
    pub last_access_time: i64,
    pub expiry_time: i64,
    pub max_inactive_interval: i32,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = auth_token)]
pub struct AuthToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub count: i32,
    pub quota: i32,
    pub quota_type: i32,
    pub expiry_time: i64
}