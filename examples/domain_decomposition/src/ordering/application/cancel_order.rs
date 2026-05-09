use crate::ordering::domain::order::{Order, OrderId, OrderStatus};
use crate::ordering::infrastructure::order_repo::OrderRepository;

pub struct CancelOrderUseCase;

impl CancelOrderUseCase {
    pub fn execute(repo: &impl OrderRepository, order_id: OrderId) -> Result<Order, String> {
        let mut order = repo.find_by_id(order_id).ok_or("Order not found")?;
        match order.status {
            OrderStatus::Placed | OrderStatus::Confirmed => {
                order.status = OrderStatus::Cancelled;
                repo.save(&order);
                Ok(order)
            }
            _ => Err("Order cannot be cancelled in its current state".into()),
        }
    }
}
