use std::{io::{BufReader, Write, Read, BufWriter, BufRead}, fs::{File, OpenOptions}, sync::Arc};

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
    pub total_sale: f64,
    pub total_profit: f64
}

pub struct Sales {
    pub items: Vec<CompleteTransaction>,
}

pub struct DetailedSales {
    pub items: Vec<Transaction>,
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

                let profit = (self.sale_price - item.price) * self.sale_quantity as f64;

                if profit > 0.0 {
                    self.append_to_disk()?;

                    let comp_tr = CompleteTransaction {
                        manager: self.manager.to_owned(),
                        total_profit: profit,
                        total_sale: self.sale_price * self.sale_quantity as f64,
                    };

                    comp_tr.append_to_disk()?;
                    Ok(comp_tr)
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

impl CompleteTransaction {
    const FILENAME: &'static str = "sales.jsonl";

    fn append_to_disk(&self) -> Result<(), String> {
        let file = OpenOptions::new().create(true).append(true).open(Self::FILENAME).map_err(|x| format!("Sales log error: {:?}", x))?;
        let output = serde_json::to_string(self).map_err(|x| format!("Transaction conversion error: {:?}", x))? + "\n";
        let mut writer = BufWriter::new(file); 
        writer.write_all(output.as_bytes()).map_err(|x| format!("Transaction write error: {:?}", x))?;
        Ok(()) 
    }
}

impl Sales {
    pub fn get_from_drive() -> Result<Self, String> {
        let file = File::open(CompleteTransaction::FILENAME).map_err(|x| format!("Sales file error: {}", x))?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new(); 
        let mut items = Vec::new();

        reader.read_to_string(&mut buffer).map_err(|x| format!("Sales file read error: {}", x))?;

        for line in buffer.lines() {  
            if line.len() > 0 {
                let item: CompleteTransaction = serde_json::from_str(&line).map_err(|x| format!("Sales file error: {}", x))?;
                items.push(item); 
            }
        }
        Ok(Self {
            items
        })
    }
}