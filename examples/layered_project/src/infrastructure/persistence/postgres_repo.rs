use crate::domain::models::{Order, OrderId};
use crate::domain::repository::OrderRepository;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct PostgresOrderRepository {
    store: RwLock<HashMap<u64, Order>>,
}

impl PostgresOrderRepository {
    pub fn new() -> Self {
        PostgresOrderRepository {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for PostgresOrderRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderRepository for PostgresOrderRepository {
    fn find_by_id(&self, id: OrderId) -> Option<Order> {
        self.store.read().unwrap().get(&id.0).cloned()
    }

    fn save(&self, order: &Order) {
        self.store
            .write()
            .unwrap()
            .insert(order.id.0, order.clone());
    }

    fn delete(&self, id: OrderId) {
        self.store.write().unwrap().remove(&id.0);
    }
}
