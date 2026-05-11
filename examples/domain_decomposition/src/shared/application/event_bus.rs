use std::any::Any;

#[allow(dead_code)]
pub trait DomainEvent: Any + Send + Sync {
    fn event_type(&self) -> &'static str;
}

#[allow(dead_code)]
pub trait EventBus {
    fn publish(&self, event: &dyn DomainEvent);
}

#[allow(dead_code)]
pub struct InMemoryEventBus;

impl EventBus for InMemoryEventBus {
    fn publish(&self, _event: &dyn DomainEvent) {
        // Simulated event publishing
    }
}
