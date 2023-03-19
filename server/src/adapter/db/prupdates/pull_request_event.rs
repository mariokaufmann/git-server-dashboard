use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "pull_request_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pr_id: String,
    pub event_type: String,
    pub author: String,
    pub timestamp: String,
    pub repository: String,
    pub title: String,
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
