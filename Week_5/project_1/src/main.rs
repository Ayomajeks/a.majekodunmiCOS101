use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:f32 = a.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let b:f32 = b.trim().parse().expect("Not a valid number");


    println!("Enter c: ");
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let c:f32 = c.trim().parse().expect("Not a valid number");

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    //Check discriminants for real roots
    if discriminant > 0.0 {
        // Calculate the roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

        // Print the roots
        println!("Root 1: {}",root1);
        println!("Root 2: {}",root2);
    }
    else if discriminant == 0.0
    { 
      let root = -b / (2.0 * a);
      println!("There is one real root: {}", root);
    }
    else if discriminant < 0.0
    {
      println!("There are no real roots");
    }
}          
           