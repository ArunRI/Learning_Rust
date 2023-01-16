// -------------------- enums--------------------------//

// #[derive(Debug)]
// enum IpAddr {
//     V4,
//     V6
// }


// #[derive(Debug)]
// struct IpAddrStruct {
//     kind: IpAddr,
//     address: String
// }

// fn main() {

//     let home = IpAddrStruct{
//         kind : IpAddr::V4,
//         address : String::from("127.0.0.1")
//     };

//     println!("{:?}", home);
// }


// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String)
// }


// fn main() {

//     let home1 = IpAddr::V4(127, 0, 0, 1);
//     let home2 = IpAddr::V6(String::from("127.0.0.1"));

//     println!("{:?}", home1);
//     println!("{:?}", home2);
// }




// ------------------------------------------- options--------------

// fn main () {
//     let x = 5;
//     let y = Some(10);

//     // let sum = x + y; // can not sum

//     let sum = x + y.unwrap_or(0);  // if value is none then will add 0

//     println!("{}", sum);
// }




// ------------------------- match ------------------//

fn main () {
    let one = Some(1);
    let two = plus_one(one);
    let none = plus_one(None);

    println!("the value of one is: {:?}", one);
    println!("the value of two is: {:?}", two);
    println!("the value of none is: {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    
    match x {
        None => None,
        Some(i) => Some(i+1),
        _ => None  // for all other cases 
    }
}