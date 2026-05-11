#[cfg(test)]
mod test_utils;

mod application;
mod domain;
mod infrastructure;

use application::{
    dto::CreateOrderRequest,
    use_cases::{create_order::CreateOrderUseCase, get_order::GetOrderUseCase},
};
use domain::models::OrderId;
use infrastructure::persistence::postgres_repo::PostgresOrderRepository;

fn main() {
    // --------------------------------------------------------------
    // FAKE EXAMPLE
    // --------------------------------------------------------------
    let pg = PostgresOrderRepository::new();
    let request = CreateOrderRequest {
        customer_name: "test".to_owned(),
        items: vec![],
    };
    let order_id = OrderId(123);

    let _order_response = CreateOrderUseCase::execute(&pg, request);
    let _order = GetOrderUseCase::execute(&pg, order_id);
    // --------------------------------------------------------------

    println!("Run `cargo test --test architecture_test` to validate the architecture.");
}
