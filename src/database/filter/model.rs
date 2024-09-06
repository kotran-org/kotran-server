use diesel::prelude::*;

use crate::schema::{filter_group, filter_line};

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = filter_group)]
pub struct FilterGroup {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub priority: i8, // 0 : Pre / 1 : Post
    pub from_lang: String,
    pub to_lang: String,
    pub ignore: bool
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = filter_line)]
pub struct FilterLine {
    pub id: i32,
    pub group_id: i32,
    pub degree: i32,
    pub use_regex: bool,
    pub from_sentence: String,
    pub to_sentence: String,
    pub include: String,
    pub exclude: String,
    pub ignore: bool
}