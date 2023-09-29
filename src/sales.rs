use std::{io::{BufReader, Write, Read, BufWriter}, fs::{File, OpenOptions}, sync::Arc};

use crate::inventory::Inventory;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Customer {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompleteTransaction {
    pub manager: SalesManager,
    pub total_profit: f64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sales {
    pub items: Vec<CompleteTransaction>,
}

impl Sales {
    const FILENAME: &'static str = "sales.json";

    pub fn get_from_drive() -> Result<Self, String> {  
        if let Ok(file) = File::open(Self::FILENAME) {
            let rdr = BufReader::new(file);
            let inventory = serde_json::from_reader(rdr).map_err(|x| format!("Sales file corruption error: {:?}", x))?;
            Ok(inventory)
        } else {
            let empty_sales = Sales { items: vec![] };
            let output = serde_json::to_string(&empty_sales).map_err(|x| format!("Sales conversion error: {:?}", x))?; 
            std::fs::write(Self::FILENAME, output).map_err(|x| format!("Sales write error: {:?}", x))?; 
            Ok(empty_sales)
        }
    }

    pub fn save_to_disk(&self) -> Result<(), String> {
        let output = serde_json::to_string(self).map_err(|x| format!("Sales conversion error: {:?}", x))?; 
        std::fs::write(Self::FILENAME, output).map_err(|x| format!("Sales write error: {:?}", x))?; 
        Ok(())
    }
}

impl Transaction {
    const FILENAME: &'static str = "transactions.jsonl"; 

    fn append_to_disk(&self) -> Result<(), String> {
        let file = OpenOptions::new().create(true).append(true).open(Self::FILENAME).map_err(|x| format!("Transaction log error: {:?}", x))?;
        let output = serde_json::to_string(self).map_err(|x| format!("Transaction conversion error: {:?}", x))? + "\n";
        let mut writer = BufWriter::new(file); 
        writer.write_all(output.as_bytes()).map_err(|x| format!("Transaction write error: {:?}", x))?;
        Ok(()) 
    }

    pub fn make_transaction(&self, inventory: &mut Inventory) -> Result<CompleteTransaction, String> {
        if let Some(mut item) = inventory.items.iter_mut().find(|item| item.name == self.product_name) {
            if item.quantity >= self.sale_quantity {
                item.quantity -= self.sale_quantity;

                let profit = self.sale_price - item.price;

                if profit > 0.0 {
                    self.append_to_disk()?;
                    
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