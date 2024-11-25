use std::io;

fn main() {
    // Display the menu
    println!("Welcome to the Restaurant!");
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken      - N3,000");
    println!("A = Amala & Ewedu Soup        - N2,500");
    println!("E = Eba & Egusi Soup          - N2,000");
    println!("W = White Rice & Stew         - N2,500");

    // Read user input
    let mut choice = String::new();
    println!("Enter your choice (P, F, A, E, W):");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice = choice.trim().to_uppercase();

    // Match user input to menu items
    match choice.as_str() {
        "P" => println!("You selected Poundo Yam/Edinkaiko Soup. Price: N3,200"),
        "F" => println!("You selected Fried Rice & Chicken. Price: N3,000"),
        "A" => println!("You selected Amala & Ewedu Soup. Price: N2,500"),
        "E" => println!("You selected Eba & Egusi Soup. Price: N2,000"),
        "W" => println!("You selected White Rice & Stew. Price: N2,500"),
        _ => println!("Invalid choice! Please select a valid option."),
    }
}