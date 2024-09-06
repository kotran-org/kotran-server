use anyhow::Context;
use async_trait::async_trait;
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::database::Database;

use super::model::{AuthRepository, AuthPassword, AuthSession, AuthToken};

#[async_trait]
impl AuthRepository for Database {
    async fn create_auth_password(&self, var_user_id: i32, var_salt: &String, var_password: &String) -> anyhow::Result<AuthPassword> {
        use crate::database::schema::auth_password::dsl::*;

        let conn = &mut self.pool.clone().get().expect("Failed to get the pooled connection.");
        let result = diesel::insert_into(auth_password)
            .values((user_id.eq(&var_user_id), salt.eq(var_salt), password.eq(var_password)))
            .get_result(conn)
            .context("Failed to insert an auth_password item.")?;
        
        Ok(result)
    }
    async fn read_auth_password(&self, user_id: i32) -> anyhow::Result<Option<AuthPassword>> {

    }
    async fn update_auth_password(&self, user_id: i32, salt: &String, password: &String) -> anyhow::Result<Option<AuthPassword>> {

    }
    async fn delete_auth_password(&self, var_user_id: i32) -> anyhow::Result<()> {
        use crate::database::schema::auth_password::dsl::*;

        let conn = &mut self.pool.clone().get().expect("Failed to get the pooled connection.");
        diesel::delete(auth_password.filter(user_id.eq(var_user_id)))
            .execute(conn)
            .expect("Failed to delete an auth password.");

        Ok(())
    }

    async fn create_auth_session(&self, user_id: i32, session_key: &String, creation_time: i64, last_access_time: i64, expiry_time: i64, max_inactive_interval: i32) -> anyhow::Result<AuthSession> {

    }
    async fn read_auth_session(&self, session_key: &String) -> anyhow::Result<Option<AuthSession>> {

    }
    async fn update_auth_session_last_access_time(&self, session_key: &String, last_access_time: i64) -> anyhow::Result<Option<AuthSession>> {

    }
    async fn update_auth_session_expiry_time(&self, session_key: &String, expiry_time: i64) -> anyhow::Result<Option<AuthSession>> {

    }
    async fn delete_auth_session(&self, session_key: &String) -> anyhow::Result<()> {

        Ok(())
    }
    async fn clean_auth_session(&self) -> anyhow::Result<()> {

        Ok(())
    }

    async fn create_auth_token(&self, user_id: i32, token: &String, quota: i32, quota_type: i8, expiry_time: i64) -> anyhow::Result<AuthToken> {

    }
    async fn read_auth_token(&self, token: &String) -> anyhow::Result<Option<AuthToken>> {

    }
    async fn read_auth_tokens_from_user_id(&self, user_id: i32) -> anyhow::Result<Vec<AuthToken>> {

    }
    async fn update_auth_token(&self, token: &String, qouta: i32, quota_type: i8, expiry_time: i64) -> anyhow::Result<Vec<AuthToken>> {

    }
    async fn delete_auth_token(&self, token: &String) -> anyhow::Result<()> {

        Ok(())
    }
}