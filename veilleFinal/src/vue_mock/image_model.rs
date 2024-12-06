use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::vue_mock::schema::images;

#[derive(Queryable, Insertable, Selectable, serde::Serialize, Debug, Clone, QueryableByName)]
#[diesel(table_name = images)]
pub struct Image {
    pub id: i32,
    pub filepath: String,
    pub file_content: Vec<u8>,
    pub rotation: i32,
    pub brightness: i32,
    pub crop_x: Option<i32>,
    pub crop_y: Option<i32>,
    pub created_at: NaiveDateTime,
}


#[derive(Queryable, Insertable, Selectable, serde::Serialize, Debug, Clone)]
#[diesel(table_name = images)]
pub struct NewImage {
    pub filepath: String,
    pub file_content: Vec<u8>,
    pub rotation: i32,
    pub brightness: i32,
    pub crop_x: Option<i32>,
    pub crop_y: Option<i32>,
    pub created_at: NaiveDateTime,
}
