
pub struct Product {
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub price: f64,
}

impl Product {
    pub fn new(name: &str, description: &str, quantity: u32, price: f64) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            quantity,
            price
        }
    }
}

pub struct Invertory {
    pub items: Vec<Product>,
}
