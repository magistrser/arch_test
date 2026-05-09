use crate::billing::domain::payment::{Payment, PaymentId, PaymentMethod};
use crate::billing::infrastructure::payment_gateway::PaymentGatewayClient;
use crate::shared::domain::money::Money;

pub struct ProcessPaymentService;

impl ProcessPaymentService {
    pub fn process(
        gateway: &impl PaymentGatewayClient,
        invoice_id: u64,
        amount: Money,
        method: PaymentMethod,
    ) -> Result<PaymentId, String> {
        gateway.charge(amount, method)?;
        Ok(PaymentId(1))
    }
}
