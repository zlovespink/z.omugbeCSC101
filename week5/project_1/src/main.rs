use std::io;

fn main() {
    println!("WELCOME TO Z'S CUISINE!");

    // Menu items with their codes and prices stored in an array
    let menu = [
        ("P", "Poundo-yam/Edikainko-soup", 3200),
        ("F", "Fried Rice & Chicken", 3000),
        ("A", "Amala & Ewedu Soup", 2500),
        ("E", "Eba & Egusi Soup", 2000),
        ("W", "White Rice & Stew", 2500),
    ];

    // Display the menu
    println!("\nMenu:");
    for &(code, name, price) in &menu {
        println!("{}: {} - ₦{}", code, name, price);
    }

    // Get the food code
    println!("\nEnter the code of the food you want to order:");
    let mut food_code = String::new();
    io::stdin().read_line(&mut food_code).expect("Failed to read input");
    let food_code = food_code.trim().to_uppercase();

    // Validate the food code
    let selected_item = menu.iter().find(|&&(code, _, _)| code == food_code);
    match selected_item {
        Some((_, name, price)) => {
            println!("You selected: {}", name);

            // Get quantity
            println!("Enter the quantity:");
            let mut quantity_input = String::new();
            io::stdin().read_line(&mut quantity_input).expect("Failed to read input");
            let quantity: i32 = quantity_input.trim().parse().expect("Not a valid integer");

            // Calculate total
            let mut total = price * quantity;
            println!("Added {} x ₦{} = ₦{} to your order.", quantity, price, total);

            // Apply discount if applicable
            if total > 10_000 {
                let discount = total as f64 * 0.05;
                total = (total as f64 - discount) as i32;
                println!("\nYou qualify for a 5% discount! Discount applied: ₦{:.2}", discount);
            }

            // Display final total
            println!("\nTotal charges for your order: ₦{}", total);
        }
        None => {
            println!("Invalid code! Please restart the program and select a valid menu item.");
        }
    }

    println!("Thank you for ordering!");
}