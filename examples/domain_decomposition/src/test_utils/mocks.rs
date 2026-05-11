use crate::billing::domain::payment::{PaymentMethod, PaymentProcessor};
use crate::ordering::domain::order::{Order, OrderId};
use crate::ordering::infrastructure::order_repo::OrderRepository;
use crate::shared::domain::money::Money;
use std::collections::HashMap;
use std::sync::RwLock;

#[allow(dead_code)]
pub struct MockPaymentGateway;

impl PaymentProcessor for MockPaymentGateway {
    fn charge(&self, _amount: Money, _method: PaymentMethod) -> Result<(), String> {
        Ok(())
    }
}

#[allow(dead_code)]
pub struct MockOrderRepository {
    store: RwLock<HashMap<u64, Order>>,
}

impl MockOrderRepository {
    pub fn new() -> Self {
        MockOrderRepository {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MockOrderRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderRepository for MockOrderRepository {
    fn find_by_id(&self, id: OrderId) -> Option<Order> {
        self.store.read().unwrap().get(&id.0).cloned()
    }

    fn save(&self, order: &Order) {
        self.store
            .write()
            .unwrap()
            .insert(order.id.0, order.clone());
    }
}
