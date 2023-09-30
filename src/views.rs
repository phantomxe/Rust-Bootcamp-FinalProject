use crate::{inventory::Inventory, sales::{Transaction, CompleteTransaction, Sales}, purchase::Purchases};

pub trait TableGenerator {
    fn view(&self);
}

impl TableGenerator for Inventory {
    fn view(&self) {   
        let index = "Idx".to_string();
        let name = "Name".to_string();
        let desc = "Description".to_string();
        let quantity = "Quantity".to_string();
        let price = "Price".to_string();

        let max_index_len = {
            let b = self.items.len().to_string().len();
            if b > index.len() {
                b
            } else {
                index.len()
            }
        };
        let max_name_len = self.items.iter().fold(name.len(), |acc, item| { if item.name.len() > acc { item.name.len() } else { acc } });
        let max_desc_len = self.items.iter().fold(desc.len(), |acc, item| { if item.description.len() > acc { item.description.len() } else { acc } });
        let max_quantity_len = self.items.iter().fold(quantity.len(), |acc, item| { if (item.quantity.to_string()).len() > acc { (item.quantity.to_string()).len() } else { acc } });
        let max_price_len = self.items.iter().fold(price.len(), |acc, item| { if (item.price.to_string()).len() > acc { format!("{:.2}", item.price).len() } else { acc } });
        
        let total_len = (4 * 3) + max_name_len + max_desc_len + max_quantity_len + max_price_len + max_index_len;
 
        let header = "Products".to_string(); 
        let header_remaining_len = header.len();
        let header_end_len = (total_len-header_remaining_len-2)/2;
        println!("| {} {} {} |", "#".repeat((total_len-header_remaining_len-2)/2), header, "#".repeat(total_len-header_end_len-header_remaining_len-2));

        let index_remaining_len = {
            let b: i32 = max_index_len as i32 - index.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", index, " ".repeat(index_remaining_len));
 
        let name_remaining_len = {
            let b: i32 = max_name_len as i32 - name.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", name, " ".repeat(name_remaining_len));
 
        let desc_remaining_len = {
            let b: i32 = max_desc_len as i32 - desc.len() as i32 - 3;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", desc, " ".repeat(desc_remaining_len));

        let quantity_remaining_len = {
            let b: i32 = max_quantity_len as i32 - quantity.len() as i32 - 3;
            if b > 0 {
                b as usize 
            } else {
                0
            }
        };
        print!("| {}{} ", quantity, " ".repeat(quantity_remaining_len));

        let price_remaining_len = {
            let b: i32 = max_price_len as i32 - price.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        println!("| {}{} |", price, " ".repeat(price_remaining_len));


        for (idx, item) in self.items.iter().enumerate() {
            print!("| {}", idx);
            print!("{} ", " ".repeat(max_index_len - (idx.to_string()).len())); 
            print!("| {}", item.name);
            print!("{} ", " ".repeat(max_name_len - item.name.len()));
            print!("| {}", item.description);
            print!("{} ", " ".repeat(max_desc_len - item.description.len()));
            print!("| {}", item.quantity);
            print!("{} ", " ".repeat(max_quantity_len - (item.quantity.to_string()).len()));
            let price = format!("{:.2}", item.price);
            print!("| {}", price);
            println!("{} |", " ".repeat(max_price_len - price.len()));
        } 

        if self.items.len() == 0 {
            println!("| There is no products found!");
        }
        println!("|{}|", "#".repeat(total_len+2));
    }
}

impl TableGenerator for Sales {
    fn view(&self) {
        let index = "Idx".to_string();
        let name = "Manager Name".to_string(); 
        let sale = "Total Sale".to_string();
        let profit = "Total Profit".to_string();

        let max_index_len = {
            let b = self.items.len().to_string().len();
            if b > index.len() {
                b
            } else {
                index.len()
            }
        };
        let max_name_len = self.items.iter().fold(name.len(), |acc, item| { if item.manager.name.len() > acc { item.manager.name.len() } else { acc } });
        let max_sale_len = self.items.iter().fold(sale.len(), |acc, item| { if (item.total_sale.to_string()).len() > acc { format!("{:.2}", item.total_sale).len() } else { acc } });
        let max_profit_len = self.items.iter().fold(profit.len(), |acc, item| { if (item.total_profit.to_string()).len() > acc { format!("{:.2}", item.total_profit).len() } else { acc } });
        
        let total_len = (3 * 3) + max_name_len + max_sale_len + max_profit_len + max_index_len;
 
        let header = "Sales (Completed Transactions)".to_string(); 
        let header_remaining_len = header.len();
        let header_end_len = (total_len-header_remaining_len-2)/2;
        println!("| {} {} {} |", "#".repeat((total_len-header_remaining_len-2)/2), header, "#".repeat(total_len-header_end_len-header_remaining_len-2));

        let index_remaining_len = {
            let b: i32 = max_index_len as i32 - index.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", index, " ".repeat(index_remaining_len));
 
        let name_remaining_len = {
            let b: i32 = max_name_len as i32 - name.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", name, " ".repeat(name_remaining_len));
 
        let sale_remaining_len = {
            let b: i32 = max_sale_len as i32 - sale.len() as i32 - 3;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", sale, " ".repeat(sale_remaining_len));

        let profit_remaining_len = {
            let b: i32 = max_profit_len as i32 - profit.len() as i32 - 3;
            if b > 0 {
                b as usize 
            } else {
                0
            }
        };
        println!("| {}{} |", profit, " ".repeat(profit_remaining_len)); 

        let mut general_total_sale = 0.0;
        let mut general_total_profit = 0.0;

        for (idx, item) in self.items.iter().enumerate() {
            print!("| {}", idx); 
            print!("{} ", " ".repeat(max_index_len - (idx.to_string()).len())); 

            print!("| {}", item.manager.name);
            print!("{} ", " ".repeat(max_name_len - item.manager.name.len()));

            let tsale = format!("{:.2}", item.total_sale);
            print!("| {}", tsale);
            print!("{} ", " ".repeat(max_sale_len - tsale.len())); 

            let tprofit = format!("{:.2}", item.total_profit);
            print!("| {}", tprofit);
            println!("{} |", " ".repeat(max_profit_len - tprofit.len()));

            general_total_sale += item.total_sale;
            general_total_profit += item.total_profit;
        } 

        if self.items.len() == 0 {
            println!("| There is no sales found!");
        }

        
        let msg = format!(" # Total Sale: {:.2} # Total Profit: {:.2} ", general_total_sale, general_total_profit);
        let msg_remaining_len = {
            let b: i32 = total_len as i32-msg.len() as i32;
            if b > 0 { 
                b as usize
            } else { 
                0
            }
        };

        println!("|{}{}|", msg, " ".repeat(msg_remaining_len));

        
        println!("|{}|", "#".repeat(total_len+2));
    }
}

impl TableGenerator for Purchases {
    fn view(&self) {
        let index = "Idx".to_string();
        let manager_name = "Manager Name".to_string();
        let firm_name = "Firm Name".to_string();
        let product_name = "Product Name".to_string(); 
        let quantity = "Quantity".to_string();
        let price = "Price".to_string();
        let total_price = "Total Purchase".to_string();

        let max_index_len = {
            let b = self.items.len().to_string().len();
            if b > index.len() {
                b
            } else {
                index.len()
            }
        };
        let max_manager_name_len = self.items.iter().fold(manager_name.len(), |acc, item| { if item.manager.name.len() > acc { item.manager.name.len() } else { acc } });
        let max_firm_name_len = self.items.iter().fold(firm_name.len(), |acc, item| { if item.firm.name.len() > acc { item.firm.name.len() } else { acc } });
        let max_product_name_len = self.items.iter().fold(product_name.len(), |acc, item| { if item.product_name.len() > acc { item.product_name.len() } else { acc } });
        let max_quantity_len = self.items.iter().fold(quantity.len(), |acc, item| { if format!("{:.2}", item.purchase_quantity).len() > acc { format!("{:.2}", item.purchase_quantity).len() } else { acc } });
        let max_price_len = self.items.iter().fold(price.len(), |acc, item| { if format!("{:.2}", item.purchase_price).len() > acc { format!("{:.2}", item.purchase_price).len() } else { acc } });
        let max_total_price_len = self.items.iter().fold(total_price.len(), |acc, item| { let b = item.purchase_quantity as f64 * item.purchase_price; if format!("{:.2}", b).len() > acc { format!("{:.2}", b).len() } else { acc } });


        let total_len = (3 * 6) + max_manager_name_len + max_firm_name_len + max_product_name_len + max_quantity_len + max_price_len + max_total_price_len + max_index_len;
 
        let header = "Purchases (Completed Purchases)".to_string(); 
        let header_remaining_len = header.len();
        let header_end_len = (total_len-header_remaining_len-2)/2;
        println!("| {} {} {} |", "#".repeat((total_len-header_remaining_len-2)/2), header, "#".repeat(total_len-header_end_len-header_remaining_len-2));

        let index_remaining_len = {
            let b: i32 = max_index_len as i32 - index.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", index, " ".repeat(index_remaining_len));
 
        let manager_name_remaining_len = {
            let b: i32 = max_manager_name_len as i32 - manager_name.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", manager_name, " ".repeat(manager_name_remaining_len));

        let firm_name_remaining_len = {
            let b: i32 = max_firm_name_len as i32 - firm_name.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", firm_name, " ".repeat(firm_name_remaining_len));

        let product_name_remaining_len = {
            let b: i32 = max_product_name_len as i32 - product_name.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", product_name, " ".repeat(product_name_remaining_len)); 
 
        let quantity_remaining_len = {
            let b: i32 = max_quantity_len as i32 - quantity.len() as i32 - 3;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", quantity, " ".repeat(quantity_remaining_len)); 

        let price_remaining_len = {
            let b: i32 = max_price_len as i32 - price.len() as i32;
            if b > 0 {
                b as usize
            } else {
                0
            }
        };
        print!("| {}{} ", price, " ".repeat(price_remaining_len));

        let total_price_remaining_len = {
            let b: i32 = max_total_price_len as i32 - total_price.len() as i32 - 3;
            if b > 0 {
                b as usize 
            } else {
                0
            }
        };
        println!("| {}{} |", total_price, " ".repeat(total_price_remaining_len));

        for (idx, item) in self.items.iter().enumerate() {
            print!("| {}", idx); 
            print!("{} ", " ".repeat(max_index_len - (idx.to_string()).len())); 

            print!("| {}", item.manager.name);
            print!("{} ", " ".repeat(max_manager_name_len - item.manager.name.len()));

            print!("| {}", item.firm.name);
            print!("{} ", " ".repeat(max_firm_name_len - item.firm.name.len()));

            print!("| {}", item.product_name);
            print!("{} ", " ".repeat(max_product_name_len - item.product_name.len())); 

            let quantity = format!("{:.2}", item.purchase_quantity);
            print!("| {}", quantity);
            print!("{} ", " ".repeat(max_quantity_len - quantity.len()));

            let price = format!("{:.2}", item.purchase_price);
            print!("| {}", price);
            print!("{} ", " ".repeat(max_price_len - price.len()));

            let total_price = format!("{:.2}", item.purchase_quantity as f64 * item.purchase_price);
            println!("| {}{} |", total_price, " ".repeat(max_total_price_len - total_price.len()));
        } 

        if self.items.len() == 0 {
            println!("| There is no purchase found!");
        }  
        
        println!("|{}|", "#".repeat(total_len+2));
    }
}