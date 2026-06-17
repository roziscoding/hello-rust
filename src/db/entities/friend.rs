use sea_query::{Iden, SimpleExpr};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::NewFriend;

#[derive(Clone, Debug, PartialEq, Serialize, FromRow)]
pub struct Friend {
    pub id: Uuid,
    pub name: String,
    pub pronouns: String,
    pub notes: Option<String>,
}

impl Friend {
    pub fn new(id: Uuid, params: NewFriend) -> Self {
        return Friend {
            id,
            name: params.name,
            pronouns: params.pronouns,
            notes: params.notes,
        };
    }

    pub fn into_insert_tuple(self) -> (Vec<Friends>, Vec<SimpleExpr>) {
        return (
            vec![
                Friends::Id,
                Friends::Name,
                Friends::Pronouns,
                Friends::Notes,
            ],
            vec![
                self.id.into(),
                self.name.into(),
                self.pronouns.into(),
                self.notes.into(),
            ],
        );
    }

    pub fn into_update_tuples(self) -> Vec<(Friends, SimpleExpr)> {
        return vec![
            (Friends::Name, self.name.into()),
            (Friends::Pronouns, self.pronouns.into()),
            (Friends::Notes, self.notes.into()),
        ];
    }
}

#[derive(Iden)]
pub enum Friends {
    Table,
    Id,
    Name,
    Pronouns,
    Notes,
}
