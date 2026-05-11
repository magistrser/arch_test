use crate::domain::models::{LineItem, OrderId, OrderStatus};

#[derive(Debug, Clone)]
pub struct CreateOrderRequest {
    pub customer_name: String,
    pub items: Vec<LineItem>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OrderResponse {
    pub id: OrderId,
    pub customer_name: String,
    pub total: f64,
    pub status: OrderStatus,
}
