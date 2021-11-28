use core::fmt;

pub trait Entity {
    fn created_at(&self) -> &str;
}

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

macro_rules! impl_Entity {
    (for $($e:ty),+) => {
        $(impl Entity for $e {
            fn created_at(&self) -> &str {
                &self.created_at
            }
        })*
    }
}

impl_Entity!(for Order, InventoryLevel);


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

#[derive(Debug,Clone)]
pub struct InventoryLevel {
    pub location: Location,
    pub item: InventoryItem,
    pub created_at: String
}

#[derive(Debug,Clone)]
pub struct InventoryItem {
    pub id: String,
    pub display_name: String,
    pub price: MoneyAmount,
    pub quantity: i32
}

#[derive(Debug,Clone)]
pub struct Location {
    pub id: String,
    pub name: String
}
