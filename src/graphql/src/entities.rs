use core::fmt;

#[derive(Debug,Clone)]
pub struct Order {
    pub name: String,
    pub customer: Customer,
    pub created_at: String,
    pub updated_at: String,
    pub shipping_address: Address,
    pub fully_paid: bool,
    pub can_mark_as_paid: bool,
    pub current_total_price: MoneyAmount,
    pub original_total_price: MoneyAmount,
    pub total_refund: MoneyAmount
}

#[derive(Debug,Clone)]
pub struct MoneyAmount {
    pub amount: f32,
    pub currency: CurrencyCode
}

#[derive(Debug,Clone)]
pub enum CurrencyCode {
    EGP,
    GBP,
    USD
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug,Clone)]
pub struct Customer {
    pub id: String
}

#[derive(Debug,Clone)]
pub struct Address {
    pub line_1: String,
    pub line_2: String,
    pub zip: String
}

