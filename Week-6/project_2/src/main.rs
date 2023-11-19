use std::io;

fn main() {

   for _ in 0..500 {

        println!("What is your name: ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Not a valid string");

        println!("Number of papers published: ");
        let mut paper = String::new();
        io::stdin().read_line(&mut paper).expect("Not a valid string");
        let p:i32 = paper.trim().parse().expect("Not a valid number");

        let incentive1:i32 = 500_000;
        let incentive2:i32 = 800_000;
        let incentive3:i32 = 1_000_000;
        let incentive4:i32 = 100_000;

      if p >= 3 && p <= 5 {
         println!("Your incentive is N{}", incentive1);
      }
      else if p > 5 && p <= 10 {
          println!("Your incentive is N{}", incentive2);
      }
      else if p >= 10 {
         println!("Your incentive is N{}", incentive3);
      }
      else if p < 3 {
         println!("Your incentive is N{}", incentive4);
      }
   }
}                       