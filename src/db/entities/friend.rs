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
    fn value_for(&self, column: &FriendColumn) -> SimpleExpr {
        return match column {
            FriendColumn::Id => self.id.into(),
            FriendColumn::Name => self.name.clone().into(),
            FriendColumn::Pronouns => self.pronouns.clone().into(),
            FriendColumn::Notes => self.notes.clone().into(),
            FriendColumn::Table => unreachable!("Table is not a valid column"),
        };
    }

    pub fn new(id: Uuid, params: NewFriend) -> Self {
        return Friend {
            id,
            name: params.name,
            pronouns: params.pronouns,
            notes: params.notes,
        };
    }

    pub fn into_insert_tuple(self) -> (Vec<FriendColumn>, Vec<SimpleExpr>) {
        let columns = FriendColumn::columns();
        let values = columns.iter().map(|c| self.value_for(c)).collect();
        return (columns, values);
    }

    pub fn into_update_tuples(self) -> Vec<(FriendColumn, SimpleExpr)> {
        return FriendColumn::columns()
            .into_iter()
            .filter(|col| !matches!(col, FriendColumn::Id))
            .map(|c| (c, self.value_for(&c)))
            .collect();
    }
}

#[derive(Iden, Clone, Copy)]
pub enum FriendColumn {
    #[iden = "friends"]
    Table,
    Id,
    Name,
    Pronouns,
    Notes,
}

impl FriendColumn {
    pub fn columns() -> Vec<FriendColumn> {
        return vec![
            FriendColumn::Id,
            FriendColumn::Name,
            FriendColumn::Pronouns,
            FriendColumn::Notes,
        ];
    }
}
