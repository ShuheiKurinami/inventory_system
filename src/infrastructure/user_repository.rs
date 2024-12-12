// src/infrastructure/user_repository.rs

use crate::domain::user::User;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use dotenv::dotenv;
use std::env;
use async_trait::async_trait;
use uuid::Uuid; // 追加

#[async_trait]
pub trait UserRepositoryTrait {
    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn create_user(&self, user: &User) -> Result<(), sqlx::Error>;
}

pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        Ok(UserRepository { pool })
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email
            FROM users
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }

    async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, name, email)
            VALUES ($1, $2, $3)
            "#,
            user.id,
            user.name,
            user.email
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
