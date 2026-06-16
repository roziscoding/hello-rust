use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "friends")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub pronouns: String,
    pub notes: Option<String>,
}

impl Model {
    pub fn new(
        name: String,
        pronouns: String,
        notes: Option<String>
    ) -> ActiveModel {
        return ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            pronouns: Set(pronouns),
            notes: Set(notes),
        };
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}