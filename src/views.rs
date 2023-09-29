use crate::{inventory::Inventory, sales::{Transaction, CompleteTransaction, Sales}};

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
        } 

        if self.items.len() == 0 {
            println!("| There is no sales found!");
        }
        println!("|{}|", "#".repeat(total_len+2));
    }
}