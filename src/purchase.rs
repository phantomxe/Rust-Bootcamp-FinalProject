use std::{fs::OpenOptions, io::{BufWriter, Write}};

use crate::{sales::SalesManager, inventory::{self, Inventory}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FirmManager {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Purchase {
    pub manager: SalesManager,
    pub firm: FirmManager,
    pub product_name: String,
    pub purchase_quantity: u32,
    pub purchase_price: f64,
}

impl Purchase {
    const FILENAME: &'static str = "purchase.jsonl"; 

    fn append_to_disk(&self) -> Result<(), String> {
        let file = OpenOptions::new().create(true).append(true).open(Self::FILENAME).map_err(|x| format!("Purchase log error: {:?}", x))?;
        let output = serde_json::to_string(self).map_err(|x| format!("Purchase conversion error: {:?}", x))? + "\n";
        let mut writer = BufWriter::new(file); 
        writer.write_all(output.as_bytes()).map_err(|x| format!("Purchase write error: {:?}", x))?;
        Ok(()) 
    }

    pub fn make_purchase(&self, inventory: &mut Inventory) -> Result<(), String> {
        let product = inventory.items.iter_mut().find(|x| x.name == self.product_name).ok_or_else(|| format!("Product not found"))?;
        product.quantity += self.purchase_quantity;
        product.price = ((product.price * product.quantity as f64) + (self.purchase_price * self.purchase_quantity as f64)) / (product.quantity + self.purchase_quantity) as f64;
        inventory.save_to_disk()?;
        self.append_to_disk()?;
        Ok(())
    }
}
