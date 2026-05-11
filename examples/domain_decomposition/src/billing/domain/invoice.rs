use crate::shared::domain::money::Money;

#[derive(Debug, Clone)]
pub struct InvoiceId(pub u64);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Invoice {
    pub id: InvoiceId,
    pub amount: Money,
    pub status: InvoiceStatus,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvoiceStatus {
    Draft,
    Issued,
    Paid,
    Overdue,
}
