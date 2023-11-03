fn main() {
   let miles1:f32 = 80.0;
   let time1:f32 = 2.0;
   let kilometers1:f32 = miles1 * 1.609;  // Convert miles to kilometers
   let speed1 = kilometers1 / time1;

   let miles2:f32 = 120.0;
   let time2:f32 = 4.0;
   let kilometers2:f32 = miles2 * 1.609;  // Convert miles to kilometers
   let speed2 = kilometers2 / time2;

   //calculations
   println!("A car travels {} km/hr if it goes 80 miles in 2 hours",speed1);
   println!("A car travels {} km/hr if it goes 120 miles in 4 hours",speed2);
}   
