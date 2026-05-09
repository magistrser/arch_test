use crate::fixation_processing::domain::entity::DomainEntity;

pub struct UseCase;

impl UseCase {
    pub fn process(&self) -> DomainEntity {
        DomainEntity
    }
}
