use crate::billing::domain::payment::{PaymentId, PaymentMethod, PaymentProcessor};
use crate::shared::domain::money::Money;

#[allow(dead_code)]
pub struct ProcessPaymentService;

#[allow(dead_code)]
impl ProcessPaymentService {
    pub fn process(
        processor: &impl PaymentProcessor,
        _invoice_id: u64,
        amount: Money,
        method: PaymentMethod,
    ) -> Result<PaymentId, String> {
        processor.charge(amount, method)?;
        Ok(PaymentId(1))
    }
}
