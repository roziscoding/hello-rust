use crate::db::entities::friend::{Friend, Friends};

use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::{Error as SqlxError, SqlitePool, query_as_with, query_with, sqlite::SqliteQueryResult};
use uuid::Uuid;

use crate::models::NewFriend;

#[derive(Clone)]
pub struct FriendRepository {
    pool: SqlitePool,
}

impl FriendRepository {
    pub fn new(pool: SqlitePool) -> Self {
        return FriendRepository { pool };
    }

    async fn fetch_all(&self, query: &impl SqlxBinder) -> Result<Vec<Friend>, SqlxError> {
        let (sql, arguments) = query.build_sqlx(SqliteQueryBuilder);
        return query_as_with(&sql, arguments).fetch_all(&self.pool).await;
    }

    async fn fetch_optional(&self, query: &impl SqlxBinder) -> Result<Option<Friend>, SqlxError> {
        let (sql, arguments) = query.build_sqlx(SqliteQueryBuilder);
        return query_as_with(&sql, arguments)
            .fetch_optional(&self.pool)
            .await;
    }

    async fn fetch_one(&self, query: &impl SqlxBinder) -> Result<Friend, SqlxError> {
        let (sql, arguments) = query.build_sqlx(SqliteQueryBuilder);
        return query_as_with(&sql, arguments).fetch_one(&self.pool).await;
    }

    async fn execute(&self, query: &impl SqlxBinder) -> Result<SqliteQueryResult, SqlxError> {
        let (sql, arguments) = query.build_sqlx(SqliteQueryBuilder);
        return query_with(&sql, arguments).execute(&self.pool).await;
    }

    pub async fn list(&self) -> Result<Vec<Friend>, SqlxError> {
        let mut query = Query::select();
        query.from(Friends::Table);
        return self.fetch_all(&query).await;
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Friend>, SqlxError> {
        let mut query = Query::select();
        query
            .from(Friends::Table)
            .and_where(Expr::col(Friends::Id).eq(id))
            .limit(1);

        return self.fetch_optional(&query).await;
    }

    pub async fn delete(&self, id: Uuid) -> Result<SqliteQueryResult, SqlxError> {
        let mut query = Query::delete();
        query
            .from_table(Friends::Table)
            .and_where(Expr::column(Friends::Id).eq(id))
            .returning_all();

        return self.execute(&query).await;
    }

    pub async fn add(&self, friend_params: NewFriend) -> Result<Friend, SqlxError> {
        let (columns, values) = Friend::new(Uuid::new_v4(), friend_params).into_insert_tuple();

        let mut query = Query::insert();
        query
            .into_table(Friends::Table)
            .columns(columns)
            .values_panic(values)
            .returning_all();

        return self.fetch_one(&query).await;
    }

    pub async fn update(&self, id: Uuid, params: NewFriend) -> Result<Option<Friend>, SqlxError> {
        let values = Friend::new(id, params).into_update_tuples();

        let mut query = Query::update();
        query.table(Friends::Table).values(values).returning_all();

        return self.fetch_optional(&query).await;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn make_friend() -> (Uuid, Friend) {
//         let id = Uuid::new_v4();
//         let friend = Friend {
//             id,
//             name: "Friend".into(),
//             pronouns: "Pronouns".into(),
//             notes: None,
//         };

//         return (id, friend);
//     }

//     #[test]
//     fn it_lists_friends() {
//         let mut friends: HashMap<Uuid, Friend> = HashMap::new();
//         let (uuid, friend) = make_friend();
//         friends.insert(uuid, friend);

//         let repo = FriendRepository::seed(friends);
//         let friends = repo.list();
//         assert_eq!(friends.len(), 1);
//     }

//     #[test]
//     fn it_adds_a_friend() {
//         let repo = FriendRepository::new();

//         let new_friend = repo.add(NewFriend {
//             name: "Test friend".into(),
//             pronouns: "Test Pronouns".into(),
//             notes: None,
//         });

//         assert_eq!(new_friend.name, "Test friend");

//         let friend = repo.get(&new_friend.id);

//         assert!(friend.is_some());
//         assert_eq!(friend.unwrap().name, "Test friend");
//     }

//     #[test]
//     fn update_returns_none_on_wrong_id() {
//         let repo = FriendRepository::new();

//         let result = repo.update(
//             &Uuid::new_v4(),
//             NewFriend {
//                 name: String::new(),
//                 pronouns: String::new(),
//                 notes: None,
//             },
//         );

//         assert!(result.is_none());
//     }

//     #[test]
//     fn it_updates_when_called_correctly() {
//         let (uuid, friend) = make_friend();
//         let mut friends: HashMap<Uuid, Friend> = HashMap::new();
//         friends.insert(uuid, friend);
//         let repo = FriendRepository::seed(friends);

//         repo.update(
//             &uuid,
//             NewFriend {
//                 name: "Updated".into(),
//                 pronouns: "Updated".into(),
//                 notes: Some("Present".into()),
//             },
//         );

//         let updated_friend = repo.get(&uuid).unwrap();

//         assert!(updated_friend.notes.is_some());
//         assert_eq!(updated_friend.notes.unwrap(), "Present");
//         assert_eq!(updated_friend.name, "Updated");
//         assert_eq!(updated_friend.pronouns, "Updated");
//     }
// }
