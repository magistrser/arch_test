// CreateInvoiceService — creates an invoice for a given amount.
// Called by ordering subdomain to generate invoices for placed orders.
use crate::billing::domain::invoice::{Invoice, InvoiceId, InvoiceStatus};
use crate::shared::domain::money::Money;

pub struct CreateInvoiceService;

impl CreateInvoiceService {
    pub fn create_invoice(amount: Money) -> Invoice {
        Invoice {
            id: InvoiceId(1),
            amount,
            status: InvoiceStatus::Issued,
        }
    }
}
