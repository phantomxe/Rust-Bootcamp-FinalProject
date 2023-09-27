use crate::inventory::Inventory;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Customer {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SalesManager {
    pub name: String, 
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub manager: SalesManager,
    pub customer: Customer,
    pub product_name: String,
    pub sale_quantity: u32,
    pub sale_price: f64,
} 

#[derive(Serialize, Deserialize, Clone)]
pub struct CompleteTransaction {
    pub manager: SalesManager,
    pub total_profit: f64
}

impl Transaction {
    fn make_transaction(&self, inventory: &mut Inventory) -> Result<CompleteTransaction, String> {
        if let Some(mut item) = inventory.items.iter_mut().find(|item| item.name == self.product_name) {
            if item.quantity >= self.sale_quantity {
                item.quantity -= self.sale_quantity;

                let profit = self.sale_price - item.price;

                if profit > 0.0 {
                    Ok(CompleteTransaction {
                        manager: self.manager.to_owned(),
                        total_profit: profit,
                    })
                } else {
                    Err("There is loss/no profit in transaction!".to_string())
                } 
            } else {
                Err("Not enough quantity".to_string())
            }
        } else {
            Err("Product not found!".to_string())
        }
    }
}