use crate::domain::models::{Order, OrderId};

pub trait OrderRepository {
    fn find_by_id(&self, id: OrderId) -> Option<Order>;
    fn save(&self, order: &Order);
}
