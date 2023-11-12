use std::io;

fn main() {
    let p = "Poundo Yam/Edinkaiko Soup";
    let f = "Fried Rice & Chicken";
    let a = "Amala & Egusi Soup";
    let e = "Eba & Egusi Soup";
    let w = "White Rice & Stew";

    let mut input = String::new();

    println!("Menu: \n{} = N3_200 \n{} = 3_000 \n{} = 2_500 \n{} = 2_000 \n{} = 2_500", p,f,a,e,w);

    println!("Enter your price: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let price:f64 = input.trim().parse().expect("Not a valid number");

    if price == 3_200.0{
      println!("Your order is: {}",p);
    }    
    else if price == 3_000.0{
      println!("Your order is: {}",f);
   }
    else if price == 2_500.0{
	   println!("Your order is: {} or {}",a,w);
   } 
   else if price == 2_000.0{
	   println!("Your order is: {}",e);
   }
   else if price >= 10_000.0{
	  let discount:f64 = price - (price * 0.05);
	  println!("You have earned a discount from your purchase");
	  println!("Your price is N{}",discount);
   }
   else {
     println!("Your price  is N{}", price); 
   }
}   
       




 
   	       
            
   	       	
