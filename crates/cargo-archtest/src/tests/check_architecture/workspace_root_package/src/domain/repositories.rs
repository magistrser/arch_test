use crate::domain::models::UserId;

pub trait UserRepository {
    fn find_by_id(&self, id: UserId) -> Option<UserId>;
}
