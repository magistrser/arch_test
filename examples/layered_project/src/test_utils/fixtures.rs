use crate::domain::models::{LineItem, Order, OrderId, OrderStatus};

pub fn order_fixture() -> Order {
    Order {
        id: OrderId(42),
        customer_name: "Test Customer".into(),
        items: vec![line_item_fixture()],
        total: 100.0,
        status: OrderStatus::Pending,
    }
}

pub fn line_item_fixture() -> LineItem {
    LineItem {
        product_name: "Test Product".into(),
        quantity: 2,
        unit_price: 50.0,
    }
}
