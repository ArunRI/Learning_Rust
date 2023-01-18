fn main() {

    // creating a new string
    let s1 = String::new();

    // creating a string from a string literal
    let s2 = "my name is rust".to_string();
    println!("{}", s2);

    // using String::from
    let s3 = String::from("my name is rust");
    println!("{}", s3);


    // updating rust
    let mut s4 = String::from("hello");
    s4.push_str(" rust");
    println!("{}", s4);

    let mut s5 = String::from("hello ");
    let str1 = "world";
    s5.push_str(str1);    // it only take slice of str not ownership, we can also use str1 later
    println!("{}", s5);
    println!("{}", str1);


    // concatenation
    let cs1 = String::from("hello ");
    let cs2 = String::from("concatenation");

    let cs3 = cs1 + &cs2;   // cs1 is moved can no longer be used
    println!("{}", cs3);
    println!("{}", cs2);
    // println!("{}", cs1);  // error


    // slicing strings
    let s6 = String::from("rust_language");

    let slc1 = &s6[0..5];
    println!("{}", slc1);

    let slc2 = &s6[..];
    println!("{}", slc2);



    // itrating over strings
    for char in s6.chars(){
        println!("{}", char);
    } 
    
}
