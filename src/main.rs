// // temprature convertor

// use std::io;

// fn main() {
//     loop {
//         println!("Please enter 0 for fahrenheit to celcius, or 1 for celcius to fahrenheit.");

//         let mut input1 = String::new();
//         io::stdin()
//             .read_line(&mut input1)
//             .expect("failed to read line");

//         let mut input1: u32 = input1.trim().parse().expect("Please type a number!");

//         if (input1 == 0) {
//             println!("Please enter the value in farenheit");

//             let mut input2 = String::new();
//             io::stdin()
//                 .read_line(&mut input2)
//                 .expect("failed to read line");

//             let input2: f32 = input2.trim().parse().expect("Please type a number!");
            
//             println!("temprature in celcius is {} degree", fahrenheit_to_celcius(input2));

//         }

//         if (input1 == 1) {
//             println!("Please enter the value in celcius");

//             let mut input2 = String::new();
//             io::stdin()
//                 .read_line(&mut input2)
//                 .expect("failed to read line");

//             let input2: f32 = input2.trim().parse().expect("Please type a number!");
            
//             println!("temprature in fahrenheit is {} degree", celcius_to_fahrenheit(input2));

//         }
//     }
// }

// fn celcius_to_fahrenheit(celcius: f32) -> f32 {
//     let fahrenheit = (celcius * 1.8) + 32.0;
//     return fahrenheit;
// }

// fn fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
//     let celcius = (fahrenheit - 32.0) / 1.8;
//     return celcius;
// }


// // ---------------------------------------------------------------------print the nth fibnoccci
// use std::io;

// fn main(){

//     loop{
    
//     println!("enter the number upto which you want fibonacci");
    
//     let mut input2 = String::new();
//     io::stdin()
//                 .read_line(&mut input2)
//                 .expect("failed to read line");

//     let input2: u32 = input2.trim().parse().expect("Please type a number!");
            

//     for uint in 0..input2+1 {
//          println! ( "fibonacci of {} is {}", uint, fib(uint));
//      }
    
//     } 
// }

// fn fib (n: u32) -> u32 {
//     if n <= 0 {
//           return 0;
//     } else if n== 1{
//           return 1;
// } else {
//     return fib (n-1)  + fib(n-2);
//  }
// }