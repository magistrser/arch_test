use crate::application::dto::OrderResponse;
use crate::domain::models::OrderId;
use crate::domain::repository::OrderRepository;

pub struct GetOrderUseCase;

impl GetOrderUseCase {
    pub fn execute(repo: &impl OrderRepository, order_id: OrderId) -> Option<OrderResponse> {
        repo.find_by_id(order_id).map(|order| OrderResponse {
            id: order.id,
            customer_name: order.customer_name,
            total: order.total,
            status: order.status,
        })
    }
}
