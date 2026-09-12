use crate::domain::models::UserId;
use crate::domain::repositories::UserRepository;

pub struct PostgresUserRepository;

impl UserRepository for PostgresUserRepository {
    fn find_by_id(&self, id: UserId) -> Option<UserId> {
        Some(id)
    }
}
