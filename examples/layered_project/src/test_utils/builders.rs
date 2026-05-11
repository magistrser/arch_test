use crate::domain::models::{LineItem, Order, OrderId, OrderStatus};

#[allow(dead_code)]
pub struct OrderBuilder {
    id: Option<OrderId>,
    customer_name: Option<String>,
    items: Vec<LineItem>,
    total: Option<f64>,
    status: Option<OrderStatus>,
}

#[allow(dead_code)]
impl OrderBuilder {
    pub fn new() -> Self {
        OrderBuilder {
            id: None,
            customer_name: None,
            items: Vec::new(),
            total: None,
            status: None,
        }
    }

    pub fn with_id(mut self, id: OrderId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_customer(mut self, name: &str) -> Self {
        self.customer_name = Some(name.into());
        self
    }

    pub fn with_items(mut self, items: Vec<LineItem>) -> Self {
        self.items = items;
        self
    }

    pub fn with_status(mut self, status: OrderStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn build(self) -> Order {
        let total = self.total.unwrap_or_else(|| {
            self.items
                .iter()
                .map(|i| i.unit_price * i.quantity as f64)
                .sum()
        });
        Order {
            id: self.id.unwrap_or(OrderId(0)),
            customer_name: self.customer_name.unwrap_or_default(),
            items: self.items,
            total,
            status: self.status.unwrap_or(OrderStatus::Pending),
        }
    }
}

impl Default for OrderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
