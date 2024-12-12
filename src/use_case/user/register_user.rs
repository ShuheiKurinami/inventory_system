use crate::domain::user::User;
use crate::service::user_service::UserService;
use async_trait::async_trait;


pub struct RegisterUser {
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait RegisterUserUseCase {
    async fn execute(&self) -> Result<User, String>;
}

#[async_trait]
impl RegisterUserUseCase for RegisterUser {
    async fn execute(&self) -> Result<User, String> {
        let service = UserService::new().await.map_err(|e| e.to_string())?;
        service.create_user(&self.name, &self.email).await
    }
}
