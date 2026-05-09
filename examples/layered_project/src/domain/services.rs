use crate::domain::models::{Order, OrderId, OrderStatus};
use crate::domain::repository::OrderRepository;

pub struct OrderDomainService;

impl OrderDomainService {
    pub fn confirm_order(repo: &impl OrderRepository, order_id: OrderId) -> Result<Order, String> {
        let mut order = repo.find_by_id(order_id).ok_or("Order not found")?;
        match order.status {
            OrderStatus::Pending => {
                order.status = OrderStatus::Confirmed;
                repo.save(&order);
                Ok(order)
            }
            _ => Err("Order cannot be confirmed from its current state".into()),
        }
    }
}
