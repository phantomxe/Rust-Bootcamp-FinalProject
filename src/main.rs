pub mod inventory;

fn main() {
    if let Ok(inv) = inventory::Inventory::get_from_drive() {
        println!("Current inventory: {:?}", inv.items);
    }
}
