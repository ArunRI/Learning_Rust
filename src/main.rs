// fn main () {
    
//     let s = String::from("hello world");
//     change_owenrship(s);
    
//     //println!("{}", s);   // gives error -> - move occurs because `s` has type `String`, which does not implement the `Copy` trait change_owenrship(s); - value moved here
    
//     let i:i32 = 23;
//     make_copy(i);
    
//     println!("{}", i);
// }

// fn change_owenrship(input: String){
//     println!("{}", input);
// }

// fn make_copy(input: i32){
//     println!("{}", input);
// }


fn main () {
    
    let s = String::from("hello world");
    change_owenrship(&s);
    
    println!("{}", s);   // gives error -> - move occurs because `s` has type `String`, which does not implement the `Copy` trait change_owenrship(s); - value moved here
    
    let i:i32 = 23;
    make_copy(i);
    
    println!("{}", i);
}

fn change_owenrship(input: &String){
    println!("{}", input);
}

fn make_copy(input: i32){
    println!("{}", input);
}