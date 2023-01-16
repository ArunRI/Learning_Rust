// ---------------------------------- type casting ------------------------

fn main() {

    let mut a : u32 = 12;
    
    let mut b = a as i32;
    
    println!("the value of a is {}", a);
    println!("the value of b is {}", b);
    
    // a = -12;
    b= -12;
    println!("the value of a is {}", a);
    println!("the value of b is {}", b);
    
    
    
    
    
    // let a = 45 as i64;
    let a = 45_i64;
    let b = 32i32;
    
    
    // let c = a/b;    //cannot divide `i64` by `i32`
    
    let c = a / (b as i64);
    
    println!("c is {}", c);

}
