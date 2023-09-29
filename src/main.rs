mod inventory;
mod sales;
mod views;

use std::io; 
use views::TableGenerator;

use crate::{inventory::Product, sales::{SalesManager, Transaction, Customer, Sales}};

fn get_number(mut buffer: &mut String) -> i32 {
    io::stdin().read_line(&mut buffer).expect("Failed to read line");  
    let operation = buffer.trim_end().parse::<i32>().expect("Failed to parse");
    buffer.clear();
    operation
}

fn get_number_f64(mut buffer: &mut String) -> f64 {
    io::stdin().read_line(&mut buffer).expect("Failed to read line");  
    let operation = buffer.trim_end().parse::<f64>().unwrap_or_else(|_| buffer.parse::<i32>().expect("Failed to parse number") as f64);
    buffer.clear();
    operation
}


fn get_text(mut buffer: &mut String) -> String {
    io::stdin().read_line(&mut buffer).expect("Failed to read line");  
    let text = buffer.trim_end().parse::<String>().expect("Failed to parse!");
    buffer.clear();
    text
}

fn main() -> Result<(), String> {
    let mut buffer = String::new(); 
    println!("Shop Management System is starting.."); 

    let mut inv = inventory::Inventory::get_from_drive()?;
    let mut sales = sales::Sales::get_from_drive()?;

    loop {
        //TODO Add security for managers
        let default_manager = SalesManager { name: "Hakan".to_string() };

        println!(r#"Select a mode for operation (1: Add a product to inventory, 
                                                 2: See current inventory,
                                                 3: Delete a product, 
                                                 4: Edit a product, 
                                                 5: Make a sale) -> 
                                                 0: Exit
        "#);
        let operation = get_number(&mut buffer);

        match operation {
            1 => {
                inv.items.push(Product::new("Asus Sabertooth X79", "Motherboard", 1, 999.91));
                println!("Added new item!");
            },
            2 => {
                println!("Current inventory: ");
                inv.view();
            },
            5 => {
                println!("Ready to sale product!, Select a product with index from table ->");
                inv.view();
                let index = get_number(&mut buffer);
                if index >= 0 && inv.items.len() > 0 && index < inv.items.len() as i32 {
                    if let Some(mut item) = inv.items.get_mut(index as usize) {
                        println!("You selected: {} ", item.name);
                        
                        println!("Enter customer name: ");
                        let customer_name = get_text(&mut buffer);

                        println!("Enter quantity: ");
                        let quantity = get_number(&mut buffer);
                        if quantity > 0 {
                            if item.quantity >= quantity as u32 {
                                println!("Enter sale price: ");
                                let sale_price = get_number_f64(&mut buffer);

                                if sale_price < item.price {
                                    println!("Price too low to sale!");
                                } else {
                                    let transaction = Transaction {
                                        customer: Customer {
                                            name: customer_name
                                        },
                                        manager: default_manager.to_owned(),
                                        product_name: item.name.to_owned(),
                                        sale_quantity: quantity as u32,
                                        sale_price,
                                    };

                                    match transaction.make_transaction(&mut inv) {
                                        Ok(comp_tr) => {
                                            println!("Done! Here is reciepe: {:?}", comp_tr);
                                            sales.items.push(comp_tr);
                                        }, 
                                        Err(e) => {
                                            println!("Transaction aborted: {}", e);
                                        }
                                    }


                                }
                            } 
                        } else {
                            println!("Sale quantity must be higher than zero!");
                        }
                    } 
                } else {
                    println!("Item not found!");
                } 
            }
            _ => { break; }
        }
    }

    inv.save_to_disk()?;
    sales.save_to_disk()?;

    println!("Exiting.."); 
    Ok(())
}
