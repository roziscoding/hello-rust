use crate::db::entities::friend::{Friend, FriendColumn};

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
        query
            .from(FriendColumn::Table)
            .columns(FriendColumn::columns());
        return self.fetch_all(&query).await;
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Friend>, SqlxError> {
        let mut query = Query::select();
        query
            .from(FriendColumn::Table)
            .columns(FriendColumn::columns())
            .and_where(Expr::col(FriendColumn::Id).eq(id))
            .limit(1);

        return self.fetch_optional(&query).await;
    }

    pub async fn delete(&self, id: Uuid) -> Result<SqliteQueryResult, SqlxError> {
        let mut query = Query::delete();
        query
            .from_table(FriendColumn::Table)
            .and_where(Expr::column(FriendColumn::Id).eq(id));

        return self.execute(&query).await;
    }

    pub async fn add(&self, friend_params: NewFriend) -> Result<Friend, SqlxError> {
        let (columns, values) = Friend::new(Uuid::new_v4(), friend_params).into_insert_tuple();

        let mut query = Query::insert();
        query
            .into_table(FriendColumn::Table)
            .columns(columns)
            .values_panic(values)
            .returning_all();

        return self.fetch_one(&query).await;
    }

    pub async fn update(&self, id: Uuid, params: NewFriend) -> Result<Option<Friend>, SqlxError> {
        let values = Friend::new(id, params).into_update_tuples();

        let mut query = Query::update();
        query
            .table(FriendColumn::Table)
            .values(values)
            .returning_all();

        return self.fetch_optional(&query).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::{Migrator, MigratorTrait};
    use sea_orm::SqlxSqliteConnector;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup() -> FriendRepository {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("failed to open in-memory db");

        let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
        Migrator::up(&conn, None).await.expect("migration failed");

        return FriendRepository::new(pool);
    }

    fn make_friend_params() -> NewFriend {
        return NewFriend {
            name: "Friend".into(),
            pronouns: "Pronouns".into(),
            notes: None,
        };
    }

    #[tokio::test]
    async fn it_stores_uuids_correctly() {
        let repo = setup().await;

        let new_friend = make_friend_params();
        let inserted_friend = repo
            .add(new_friend.clone())
            .await
            .expect("cannot create friend");

        let retrieved_friend = repo
            .get(inserted_friend.id)
            .await
            .expect("cannot retrieve friend")
            .expect("friend not found");

        assert_eq!(retrieved_friend.name, new_friend.name);
    }

    #[tokio::test]
    async fn list_returns_all_friends() {
        let repo = setup().await;

        repo.add(make_friend_params())
            .await
            .expect("cannot create friend");
        repo.add(make_friend_params())
            .await
            .expect("cannot create friend");

        let friends = repo.list().await.expect("cannot list friends");

        assert_eq!(friends.len(), 2);
    }

    #[tokio::test]
    async fn update_changes_the_stored_fields() {
        let repo = setup().await;

        let inserted = repo
            .add(make_friend_params())
            .await
            .expect("cannot create friend");

        let updated = repo
            .update(
                inserted.id,
                NewFriend {
                    name: "Updated".into(),
                    pronouns: "they/them".into(),
                    notes: Some("Present".into()),
                },
            )
            .await
            .expect("cannot update friend")
            .expect("friend not found");

        assert_eq!(updated.id, inserted.id);
        assert_eq!(updated.name, "Updated");
        assert_eq!(updated.pronouns, "they/them");
        assert_eq!(updated.notes.as_deref(), Some("Present"));
    }

    #[tokio::test]
    async fn update_returns_none_for_unknown_id() {
        let repo = setup().await;

        let result = repo
            .update(Uuid::new_v4(), make_friend_params())
            .await
            .expect("update should not error");

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn delete_removes_the_friend() {
        let repo = setup().await;

        let inserted = repo
            .add(make_friend_params())
            .await
            .expect("cannot create friend");

        repo.delete(inserted.id)
            .await
            .expect("cannot delete friend");

        let retrieved = repo.get(inserted.id).await.expect("cannot retrieve friend");

        assert!(retrieved.is_none());
    }
}
