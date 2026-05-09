use crate::billing::domain::payment::PaymentMethod;
use crate::shared::domain::money::Money;

pub trait PaymentGatewayClient {
    fn charge(&self, amount: Money, method: PaymentMethod) -> Result<(), String>;
}

pub struct StripeGateway;

impl PaymentGatewayClient for StripeGateway {
    fn charge(&self, amount: Money, method: PaymentMethod) -> Result<(), String> {
        // Simulated payment processing
        Ok(())
    }
}
