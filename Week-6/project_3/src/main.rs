use std::io;

fn main() {
    println!("Enter the limit for the 2 multiplication table:");
    
    let mut limit = String::new();
    
    io::stdin().read_line(&mut limit)
        .expect("Failed to read line");
    
    let limit: u32 = limit.trim().parse()
        .expect("Please enter a valid number");
    
    println!("2 Multiplication Table up to {}: ", limit);
    
    for i in 1..=limit {
        let result = 2 * i;
        println!("2 * {} = {}", i, result);
    }
}             