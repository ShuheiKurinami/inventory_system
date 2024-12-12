use crate::domain::user::User;
use crate::infrastructure::user_repository::{UserRepository, UserRepositoryTrait};
use async_trait::async_trait;
use uuid::Uuid;

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let repository = UserRepository::new().await?;
        Ok(UserService { repository })
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, String> {
        self.repository
            .get_all_users()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn create_user(&self, name: &str, email: &str) -> Result<User, String> {
        // ここにビジネスロジックを追加できます（例: メールのバリデーション）
        if !self.validate_email(email) {
            return Err("Invalid email format".into());
        }

        let user = User {
            id: Uuid::new_v4(),
            name: name.to_string(),
            email: email.to_string(),
        };

        self.repository
            .create_user(&user)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    fn validate_email(&self, email: &str) -> bool {
        // 簡単なメール形式のチェック
        email.contains('@')
    }
}
