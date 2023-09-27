use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufWriter}; 


#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub items: Vec<Product>,
}

impl Inventory {
    const FILENAME: &'static str = "inventory.json";

    pub fn get_from_drive() -> Result<Self, String> {  
        if let Ok(file) = File::open(Self::FILENAME) {
            let rdr = BufReader::new(file);
            let inventory = serde_json::from_reader(rdr).map_err(|x| format!("Inventory file corruption error: {:?}", x))?;
            Ok(inventory)
        } else {
            let empty_inventory = Inventory { items: vec![] };
            let output = serde_json::to_string(&empty_inventory).map_err(|x| format!("Inventory conversion error: {:?}", x))?; 
            std::fs::write(Self::FILENAME, output).map_err(|x| format!("Inventory write error: {:?}", x))?; 
            Ok(empty_inventory)
        }
    }

    pub fn save_to_disk(&self) -> Result<(), String> {
        let output = serde_json::to_string(self).map_err(|x| format!("Inventory conversion error: {:?}", x))?; 
        std::fs::write(Self::FILENAME, output).map_err(|x| format!("Inventory write error: {:?}", x))?; 
        Ok(())
    }
}