use crate::db::model::user::UserData;
use sqlx::{PgPool, Error};

#[derive(Clone)]
pub struct UserDataRepository {
    pool: PgPool,
}

impl UserDataRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // 一覧取得
    pub async fn list(&self) -> Result<Vec<UserData>, Error> {
        sqlx::query_as!(
            UserData,
            "SELECT id, name FROM users ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await
    }

    // 1件取得
    pub async fn find(&self, id: i32) -> Result<Option<UserData>, Error> {
        sqlx::query_as!(
            UserData,
            "SELECT id, name FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    // 新規作成
    pub async fn create(&self, name: &str) -> Result<UserData, Error> {
        sqlx::query_as!(
            UserData,
            "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await
    }

    // 更新
    pub async fn update(&self, id: i32, name: &str) -> Result<Option<UserData>, Error> {
        sqlx::query_as!(
            UserData,
            "UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name",
            name, id
        )
        .fetch_optional(&self.pool)
        .await
    }

    // 削除
    pub async fn delete(&self, id: i32) -> Result<u64, Error> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn find_with_password_by_name(&self, name: &str) -> Result<Option<(UserData, String)>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, name, password FROM users WHERE name = $1",
            name
        )
        .fetch_optional(&self.pool)
        .await?;
    
        Ok(row.map(|r| (UserData { id: r.id, name: r.name }, r.password)))
        // r.password はすでに String 型
    }
}