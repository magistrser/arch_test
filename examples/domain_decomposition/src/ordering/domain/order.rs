use crate::ordering::domain::line_item::LineItem;
use crate::shared::domain::money::Money;

#[derive(Debug, Clone)]
pub struct OrderId(pub u64);

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub customer_name: String,
    pub items: Vec<LineItem>,
    pub total: Money,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Placed,
    Confirmed,
    Shipped,
    Delivered,
    Cancelled,
}
