use std::{sync::Arc, time::SystemTime};
use async_trait::async_trait;
use diesel::{prelude::*, sql_types::{Integer, BigInt}};
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
    pub quota_type: i8,
    pub expiry_time: i64
}

#[async_trait]
pub trait AuthRepository {
    async fn create_auth_password(&self, user_id: i32, salt: &String, password: &String) -> anyhow::Result<AuthToken>;
    async fn read_auth_password(&self, user_id: i32) -> anyhow::Result<Option<AuthToken>>;
    async fn update_auth_password(&self, user_id: i32, salt: &String, password: &String) -> anyhow::Result<Option<AuthToken>>;
    async fn delete_auth_password(&self, user_id: i32) -> anyhow::Result<()>;

    async fn create_auth_session(&self, user_id: i32, session_key: &String, creation_time: i64, last_access_time: i64, expiry_time: i64, max_inactive_interval: i32) -> anyhow::Result<AuthToken>;
    async fn read_auth_session(&self, session_key: &String) -> anyhow::Result<Option<AuthToken>>;
    async fn update_auth_session_last_access_time(&self, session_key: &String, last_access_time: i64) -> anyhow::Result<Option<AuthToken>>;
    async fn update_auth_session_expiry_time(&self, session_key: &String, expiry_time: i64) -> anyhow::Result<Option<AuthToken>>;
    async fn delete_auth_session(&self, session_key: &String) -> anyhow::Result<()>;
    async fn clean_auth_session(&self) -> anyhow::Result<()>;

    async fn create_auth_token(&self, user_id: i32, token: &String, quota: i32, quota_type: i8, expiry_time: i64) -> anyhow::Result<AuthToken>;
    async fn read_auth_token(&self, token: &String) -> anyhow::Result<Option<AuthToken>>;
    async fn read_auth_tokens_from_user_id(&self, user_id: i32) -> anyhow::Result<Vec<AuthToken>>;
    async fn update_auth_token(&self, token: &String, qouta: i32, quota_type: i8, expiry_time: i64) -> anyhow::Result<Vec<AuthToken>>;
    async fn delete_auth_token(&self, token: &String) -> anyhow::Result<()>;
}