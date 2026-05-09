use std::any::Any;

pub trait DomainEvent: Any + Send + Sync {
    fn event_type(&self) -> &'static str;
}

pub trait EventBus {
    fn publish(&self, event: &dyn DomainEvent);
}

pub struct InMemoryEventBus;

impl EventBus for InMemoryEventBus {
    fn publish(&self, _event: &dyn DomainEvent) {
        // Simulated event publishing
    }
}
