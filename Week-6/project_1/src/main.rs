use std::io;

fn main() {

   for _ in 0..150 {

    println!("Enter Name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter E-mail: ");
    let mut e_mail = String::new();
    io::stdin().read_line(&mut e_mail).expect("Failed to read input");

    println!("Enter Department: ");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read input");

    println!("Enter State of Origin: ");
    let mut state = String::new();
    io::stdin().read_line(&mut state).expect("Failed to read input");

    let mut input1 = String::new();
    println!("Are you a current class rep? (yes/no)");
    io::stdin().read_line(&mut input1).expect("Not existent");
    let class_rep = input1.trim();

    let mut level = String::new();
    println!("What level are you: ");
    io::stdin().read_line(&mut level).expect("Failed to read input");
    let level:i32 = level.trim().parse().expect("Failed to read input");
   
    let mut input3 = String::new();
    println!("What is your CGPA: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let cgpa:f32 = input3.trim().parse().expect("Not a valid number");

    if class_rep == "yes" && level > 100 && cgpa > 4.0 {

    println!("Name: {} , E-mail: {} , Department: {} , State of Origin: {}", name , e_mail , department , state);
    println!("You can vote");
    } else {
      println!("Sorry, you are not eligible to vote");
    }
   }
}   

