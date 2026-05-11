use crate::shared::domain::money::Money;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LineItem {
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: Money,
}
