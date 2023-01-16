// Write a function that takes a string of words separated by spaces and returns 
// the first word it finds in that string. If the function doesn’t find a space in the 
// string, the whole string must be one word, so the entire string should be 
// returned. (use slice)


fn main() {

    let s = String::from("hello world");
    
    let word = get_first_word(&s);
    
    println!("the first word is: {}", word);
    
    }
    
    
    
    fn get_first_word(input: &String) -> &str {
    let bytes = input.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[0..i];
        }
    }
    
    &input[..]
    }
    
    


    // build a car


struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]

enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {

    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0
    }
}

fn main() {
    let mut car = car_factory(String::from("Black"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
    
    car = car_factory(String::from("Blue"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    
    
    car = car_factory(String::from("Gray"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    
}






// ----------------------- area of rectangle -------------------------- //


// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }


// ---------------------------------------------------------//

// fn main() {
//     let rect_tup = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect_tup)
//     );
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// } 

// ---------------------------------------------------------//


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} 
