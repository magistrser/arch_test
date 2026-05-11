use crate::shared::domain::money::Money;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PaymentId(pub u64);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Payment {
    pub id: PaymentId,
    pub invoice_id: u64,
    pub amount: Money,
    pub method: PaymentMethod,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PaymentMethod {
    CreditCard,
    BankTransfer,
    PayPal,
}

#[allow(dead_code)]
pub trait PaymentProcessor {
    fn charge(&self, amount: Money, method: PaymentMethod) -> Result<(), String>;
}
