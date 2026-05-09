use crate::billing::domain::invoice::{Invoice, InvoiceId, InvoiceStatus};
use crate::ordering::domain::order::{Order, OrderId, OrderStatus};
use crate::shared::domain::money::{Currency, Money};

pub fn create_invoice(amount: f64) -> Invoice {
    Invoice {
        id: InvoiceId(99),
        amount: Money::new(amount, Currency::USD),
        status: InvoiceStatus::Draft,
    }
}

pub fn create_order(customer: &str, total: f64) -> Order {
    Order {
        id: OrderId(99),
        customer_name: customer.into(),
        items: vec![],
        total: Money::new(total, Currency::USD),
        status: OrderStatus::Placed,
    }
}
