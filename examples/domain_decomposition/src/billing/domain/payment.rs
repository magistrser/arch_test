use crate::shared::domain::money::Money;

#[derive(Debug, Clone)]
pub struct PaymentId(pub u64);

#[derive(Debug, Clone)]
pub struct Payment {
    pub id: PaymentId,
    pub invoice_id: u64,
    pub amount: Money,
    pub method: PaymentMethod,
}

#[derive(Debug, Clone)]
pub enum PaymentMethod {
    CreditCard,
    BankTransfer,
    PayPal,
}
