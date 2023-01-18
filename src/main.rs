fn main () {
    // creating an empty vector
    let v1 : Vec<i32> = Vec::new();
    println!("{:?}", v1);

    let v2 = vec![1,2,3];
    println!("{:?}", v2);

    // pushing values to vector
    let mut v3 = Vec::new();

    v3.push(0);
    v3.push(1);
    v3.push(2);
    v3.push(3);
    v3.push(4);

    println!("{:?}", v3);


    // reading elements from vector

    let v4 = vec![10,20,30,40,50];
    
    let first = v4[0];
    println!("the first elemnt is {}", first);

    let last = v4.get(4);
    println!("the last elemnt is {:?}", last);  // this will return Some(50)


    // reading an index which do not exist

    let v5 = vec![100, 200, 300, 400];

    // let not_exist_1 = v5[100];
    // println!("{}", not_exist_1);   //thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 100'
    
    let not_exist_2 = v5.get(100);
    println!("{:?}", not_exist_2);   // None



    // iterating a vector

    let v6 = vec![100, 200, 300];
    for i in v6 {
        println!("{i}");
    }

    let v7 = vec![100, 200, 300];
    for mut i in v7 {
        i += 50;

        println!("{}", i);
    }


    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    println!("{:?}", SpreadsheetCell::Int(3));
    println!("{:?}", SpreadsheetCell::Text(String::from("blue")));
    println!("{:?}", SpreadsheetCell::Float(10.12));

}