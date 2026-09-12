use crate::domain::models::UserId;
use crate::domain::repositories::UserRepository;

pub struct CreateUserUseCase;

impl CreateUserUseCase {
    pub fn execute(&self, repo: &impl UserRepository) -> UserId {
        repo.find_by_id(UserId(1)).unwrap_or(UserId(0))
    }
}
