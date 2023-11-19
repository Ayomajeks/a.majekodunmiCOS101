use std::io;

fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();
   let mut input4 = String::new();
   let mut input5 = String::new();
   let mut input6 = String::new();
   let mut input7 = String::new();
   let mut input8 = String::new();
   

   println!("Enter your name: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");

   println!("Enter your Date of Birth: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");

   println!("Enter your email address: ");
   io::stdin().read_line(&mut input3).expect("Not a valid string");

   println!("Enter your number: ");
   io::stdin().read_line(&mut input4).expect("Not a valid string");
   let number:i64 = input4.trim().parse().expect("Not a valid number");

   println!("Enter your number of siblings: ");
   io::stdin().read_line(&mut input5).expect("Not a valid string");
   let siblings:i32 = input5.trim().parse().expect("Not a valid number");

   println!("Enter your medical diagnosis: ");
   io::stdin().read_line(&mut input6).expect("Not a valid string");

   println!("Enter your number of children: ");
   io::stdin().read_line(&mut input7).expect("Not a valid string");
   let children:i32 = input7.trim().parse().expect("Not a valid number");

   println!("Enter your village of residence: ");
   io::stdin().read_line(&mut input8).expect("Not a valid number");

   let amount1 = 1_200_000;
   let amount2 = 550_000;
   let amount3 = 1_500_000;
   let amount4 = 800_000;
   let amount5 = 450_000;

   if medical diagnosis == Alzheimer && age > 50 && number of children > 4 && village of residence == "Akpabom"{
   let discount1 = 1_200_000 - (1_200_000 * (20 / 100));
   
      println!("Your discount is N{}, discount1");
   } 
   else if {
      println!("Your amount is N{}, amount1");
   }
   if medicaldiagnosis == Arrythmia && age == 30 && number of siblings > 4 && village of residence == "Ngbauji"{
   let discount2 = 550_000 - (550_000 * (5 / 100));
      println!("Your discount is N{}, discount2");
   }
   else if {
      println!("Your amount is N{}, amount2");
   }
   if medicaldiagnosis == CKD && age > 40 && number of children > 3 && number of siblings > 3 && village of residence ==" Atabrikang"{
   let discount3 = 1_500_000 - (1_500_000 * (15 / 100));
   
      println!("Your discount is N{}, discount3");
   }
   else if {
      println!("Your amount is N{}, amount3");
   }
   if medicaldiagnosis == Diabetes && age > 28 && age < 45 && number of children = 2..4 && village of residence == "Okorobilom"
   let discount4 = 800_000 - (800_000 * (10 / 100));
   {
      println!("Your discount is N{}, discount4");
   }
   else if {
      println!("Your amount is N{}, amount4");
   }
   if medicaldiagnosis == Arthritis && age > 58 && number of siblings > 5 && number of children > 5 && village of residence == "Emeremen"{
   let discount5 = 450_000 - (450_000 * (10 / 100));
      println!("Your discount is N{}, discount5");
   }
   else if {
      println!("Your amount is N{}, amount5");
   }
}   


