use crate::billing::application::create_invoice::CreateInvoiceService;
use crate::ordering::domain::order::{Order, OrderId, OrderStatus};
use crate::ordering::infrastructure::order_repo::OrderRepository;
use crate::shared::domain::money::Money;

pub struct PlaceOrderUseCase;

impl PlaceOrderUseCase {
    pub fn execute(
        repo: &impl OrderRepository,
        customer_name: String,
        total: Money,
    ) -> Result<Order, String> {
        let order = Order {
            id: OrderId(1),
            customer_name,
            items: vec![],
            total,
            status: OrderStatus::Placed,
        };

        repo.save(&order);

        // Cross-subdomain: generate invoice in billing subdomain
        let _invoice = CreateInvoiceService::create_invoice(total);

        Ok(order)
    }
}
