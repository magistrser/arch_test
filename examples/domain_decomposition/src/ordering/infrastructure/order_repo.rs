use crate::ordering::domain::order::{Order, OrderId};
use std::collections::HashMap;
use std::sync::RwLock;

pub trait OrderRepository {
    fn find_by_id(&self, id: OrderId) -> Option<Order>;
    fn save(&self, order: &Order);
}

pub struct InMemoryOrderRepository {
    store: RwLock<HashMap<u64, Order>>,
}

impl InMemoryOrderRepository {
    pub fn new() -> Self {
        InMemoryOrderRepository {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryOrderRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderRepository for InMemoryOrderRepository {
    fn find_by_id(&self, id: OrderId) -> Option<Order> {
        self.store.read().unwrap().get(&id.0).cloned()
    }

    fn save(&self, order: &Order) {
        self.store.write().unwrap().insert(order.id.0, order.clone());
    }
}
