
// // use std::io;
// use colored::*;
// fn main() {
//     println!("{}", "Hello, world!".red());

//     // let a : f64 = 2.0;
//     // let b : f64 = 3.7;

//     // let sum = a + b;

//     // print!("the sim is, {}", sum);

//     // let arr = [1,2,3,4,5];

//     // print!("{:?}", arr);

//     // let mut input = String::new();
//     // io::stdin().read_line(&mut input).expect("kuchh bhi");

//     // println!("{}", input);

//     let mut counter  = 0;

//     loop {
//         counter += 1;
//         println!("{}", "i do not like Rust".green());

//         if counter == 10 {
//             break;
//         }       
//     }

// }




use std::io;
use rand::Rng; 
use std::cmp::Ordering; 
fn main() {
    println!("guessing Game!!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        
        println!("you guessed : {guess}");
    
        let mut guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
           } 
       
    }
} 



