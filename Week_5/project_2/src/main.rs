use std::io;

fn main() {
	
   let mut input1= String::new();
   let mut input2= String::new();

   println!("Enter your age: ");
   io::stdin().read_line(&mut input1).expect("Not existent");
   let age:i32 = input1.trim().parse().expect("Not existent");

   println!("Enter your experience (experienced or inexperienced) : ");
   io::stdin().read_line(&mut input2).expect("Not existent");
   let experienced = input2.trim();

   if experienced == "yes"

   {if age >= 40
   {
      println!("The incentive of employee is N1_560_000");
   }
   }
   else if age >= 30 && age < 40
   {
      println!("The incentive of employee is N1_480_000");
   }
   else if age <28
   {
      println!("Your incentive is N1_300_000");
      }
   if experienced == "no"
   {
      println!("Your incentive is N100_000");
   }
}            
        