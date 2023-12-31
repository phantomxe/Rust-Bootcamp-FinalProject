mod inventory;
mod sales;
mod views;
mod purchase;
mod auth;

use std::{io, default}; 
use views::TableGenerator;

use crate::{inventory::Product, sales::{SalesManager, Transaction, Customer}, purchase::{Purchase, FirmManager}, auth::USERS};

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

    println!("Please login to continue.. Enter Username:");
    let username = get_text(&mut buffer);
    println!("Enter Password:");
    let password = get_text(&mut buffer);

    let user_trying = auth::AuthUser {
        username,
        password,
    };

    if let Some(_) = USERS.iter().find(|x| x.username == user_trying.username && x.password == user_trying.password) {
        println!("Welcome {}!", user_trying.username);

        let default_manager = SalesManager::from(user_trying);

        loop { 
            println!(
r#"Select a mode for operation;
1: Add a product to inventory, 
2: See current inventory,
3: Delete a product, 
4: Edit a product, 
5: Make a sale,
6: Make a purchase,
7: See current sales,
8: See current purchases,
0: Exit"#);
            let operation = get_number(&mut buffer);

            match operation {
                1 => {
                    println!("Enter Product Name: ");
                    let name = get_text(&mut buffer);

                    println!("Enter Product Description: ");
                    let description = get_text(&mut buffer);
    
                    let product = Product {
                        name,
                        description,
                        quantity: 0,
                        price: 0.0
                    };
                    println!("Done! New product: {:?}", product);
                    inv.items.push(product);
                },
                2 => {
                    println!("Current inventory: ");
                    inv.view();
                },
                3 => {
                    println!("Ready to delete a product!, Select a product with index from table ->");
                    inv.view();
                    let index = get_number(&mut buffer);

                    if index >= 0 && inv.items.len() > 0 && index < inv.items.len() as i32 {
                        if let Some(item) = inv.items.get(index as usize) {
                            println!("This product: {} will be deleted! Type DELETE to confirm.", item.name);

                            let confirm = get_text(&mut buffer);
                            if confirm == "DELETE" {
                                inv.items.remove(index as usize);
                                println!("Deleted!");
                            } else {
                                println!("Aborted!");
                            }                        
                        }
                    }
                },
                4 => {
                    println!("Ready to edit a product!, Select a product with index from table ->");
                    inv.view();
                    let index = get_number(&mut buffer);

                    if index >= 0 && inv.items.len() > 0 && index < inv.items.len() as i32 {
                        if let Some(mut item) = inv.items.get_mut(index as usize) {
                            println!("Enter Product Name (Current: {}, Press Enter to skip)", item.name);
                            let name = get_text(&mut buffer);
                            if name.len() > 0 {
                                item.name = name;
                            }

                            println!("Enter Product Description (Current: {}, Press Enter to skip)", item.description);
                            let description = get_text(&mut buffer);     
                            if description.len() > 0 {
                                item.description = description;
                            } 

                            println!("Success!, Result: {:?}", item);
                        }
                    }
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
                },
                6 => {
                    println!("Ready to purchase a product!, Select a product with index from table ->");
                    inv.view();
                    let index = get_number(&mut buffer);
                    if index >= 0 && inv.items.len() > 0 && index < inv.items.len() as i32 {
                        if let Some(mut item) = inv.items.get_mut(index as usize) {
                            println!("You selected: {} ", item.name);
                            
                            println!("Enter firm name: ");
                            let firm_name = get_text(&mut buffer);

                            println!("Enter quantity: ");
                            let quantity = get_number(&mut buffer);
                            if quantity > 0 {
                                if item.quantity >= quantity as u32 {
                                    println!("Enter price: ");
                                    let purchase_price = get_number_f64(&mut buffer);
    
                                    if purchase_price > 0.0 {
                                        let purchase = Purchase {
                                            firm: FirmManager {
                                                name: firm_name
                                            },
                                            manager: default_manager.to_owned(),
                                            product_name: item.name.to_owned(),
                                            purchase_quantity: quantity as u32,
                                            purchase_price,  
                                        };

                                        match purchase.make_purchase(&mut inv) {
                                            Ok(comp_tr) => {
                                                println!("Done! Here is reciepe: {:?}", comp_tr); 
                                            }, 
                                            Err(e) => {
                                                println!("Purchase aborted: {}", e);
                                            }
                                        }
                                    } else {
                                        println!("Price must be higher than zero!");
                                    }       
                                } 
                            } else {
                                println!("Purchase quantity must be higher than zero!");
                            }
                        } 
                    } else {
                        println!("Item not found!");
                    } 
                },
                7 => { 
                    match sales::Sales::get_from_drive() {
                        Ok(current_sales) => {
                            println!("Current sales: ");
                            current_sales.view(); 
                        },
                        Err(e) => {
                            println!("I got an error on sales view: {}", e);
                        }
                    } 
                }, 
                8 => {
                    match purchase::Purchases::get_from_drive() {
                        Ok(current_purchases) => {
                            println!("Current purchases: ");
                            current_purchases.view(); 
                        },
                        Err(e) => {
                            println!("I got an error on purchases view: {}", e);
                        }
                    }
                },
                _ => { break; }
            }
        }
    } else {
        println!("Wrong username or password!");
        return Ok(());
    } 

    inv.save_to_disk()?; 

    println!("Exiting.."); 
    Ok(())
}
