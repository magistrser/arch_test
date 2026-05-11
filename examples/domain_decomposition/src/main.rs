// Entry point — the architecture is validated by tests/architecture_test.rs.

#[cfg(test)]
mod test_utils;

mod billing;
mod ordering;
mod shared;

use billing::application::create_invoice::CreateInvoiceService;
use ordering::application::place_order::PlaceOrderUseCase;
use ordering::infrastructure::order_repo::InMemoryOrderRepository;
use shared::domain::money::{Currency, Money};

fn main() {
    // Demo like layered_project
    let repo = InMemoryOrderRepository::new();
    let total = Money::new(100.0, Currency::USD);
    let order = PlaceOrderUseCase::execute(&repo, "test".to_string(), total).unwrap();
    let invoice = CreateInvoiceService::create_invoice(total);

    println!(
        "Created order id {} and invoice id {}",
        order.id.0, invoice.id.0
    );
    println!("Run `cargo test --test architecture_test` to validate the architecture.");
}
