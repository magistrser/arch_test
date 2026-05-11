use crate::billing::domain::payment::{PaymentMethod, PaymentProcessor};
use crate::shared::domain::money::Money;

#[allow(dead_code)]
pub struct StripeGateway;

impl PaymentProcessor for StripeGateway {
    fn charge(&self, _amount: Money, _method: PaymentMethod) -> Result<(), String> {
        // Simulated payment processing
        Ok(())
    }
}
