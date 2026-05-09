use crate::shared::domain::money::Money;

#[derive(Debug, Clone)]
pub struct InvoiceId(pub u64);

#[derive(Debug, Clone)]
pub struct Invoice {
    pub id: InvoiceId,
    pub amount: Money,
    pub status: InvoiceStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvoiceStatus {
    Draft,
    Issued,
    Paid,
    Overdue,
}
