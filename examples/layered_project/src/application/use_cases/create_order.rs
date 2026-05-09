use crate::application::dto::{CreateOrderRequest, OrderResponse};
use crate::domain::models::{Order, OrderId, OrderStatus};
use crate::domain::repository::OrderRepository;

pub struct CreateOrderUseCase;

impl CreateOrderUseCase {
    pub fn execute(
        repo: &impl OrderRepository,
        request: CreateOrderRequest,
    ) -> Result<OrderResponse, String> {
        let total: f64 = request
            .items
            .iter()
            .map(|i| i.unit_price * i.quantity as f64)
            .sum();

        let order = Order {
            id: OrderId(1), // simplified
            customer_name: request.customer_name,
            items: request.items,
            total,
            status: OrderStatus::Pending,
        };

        repo.save(&order);

        Ok(OrderResponse {
            id: order.id,
            customer_name: order.customer_name,
            total: order.total,
            status: order.status,
        })
    }
}
