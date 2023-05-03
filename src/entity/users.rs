//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub displayname: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
    pub last_login: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::chapter_pages::Entity")]
    ChapterPages,
    #[sea_orm(has_many = "super::chapters::Entity")]
    Chapters,
    #[sea_orm(has_many = "super::comics::Entity")]
    Comics,
    #[sea_orm(has_many = "super::comments::Entity")]
    Comments,
    #[sea_orm(has_many = "super::profile_images::Entity")]
    ProfileImages,
    #[sea_orm(has_many = "super::user_roles::Entity")]
    UserRoles,
}

impl Related<super::chapter_pages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChapterPages.def()
    }
}

impl Related<super::chapters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chapters.def()
    }
}

impl Related<super::comics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comics.def()
    }
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::profile_images::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProfileImages.def()
    }
}

impl Related<super::user_roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
