use crate::domain::user::User;
use crate::service::user_service::UserService;
use async_trait::async_trait;

pub struct ListUsers;

#[async_trait]
pub trait ListUsersUseCase {
    async fn execute(&self) -> Result<Vec<User>, String>;
}

#[async_trait]
impl ListUsersUseCase for ListUsers {
    async fn execute(&self) -> Result<Vec<User>, String> {
        let service = UserService::new().await.map_err(|e| e.to_string())?;
        service.get_all_users().await
    }
}
