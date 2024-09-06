use diesel::prelude::*;

use crate::schema::{user_account, user_permission, user_profile};

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = user_account)]
pub struct UserAccount {
    pub id: i32,
    pub name: String
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = user_profile)]
pub struct UserProfile {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub desc: String,
    pub image: String,
    pub join_time: i64,
    pub update_time: i64
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = user_permission)]
pub struct UserPermission {
    pub id: i32,
    pub user_id: i32,
    pub perm_name: String,
    pub perm_action: String,
}