fn main() {

    use std::collections::HashMap;

    // creating a new empty hashmap
    let mut scores = HashMap::new();

    // inserting key values in hashmap
    scores.insert("India", 300);
    scores.insert("SriLanka", 250);

    println!("{:?}", scores);


    // accessing values of hashmap
    let score = scores.get("India"); // will return an option
    println!("{:?}", score);  // Some(300);

    let score2 = scores.get("SriLanka").copied().unwrap_or(0);  //This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key.
    println!("{}", score2); // 250



    // itrating over hashmap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }


    // overwriting a value
    scores.insert("India", 350);
    println!("{:?}", scores);

    // entry method
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("SA"), 400);

    scores2.entry(String::from("Australia")).or_insert(380);  //entry will check f Australia present or not of not will insert 380
    scores2.entry(String::from("SA")).or_insert(444);  // SA is still 400, bcoz SA was already present
    println!("{:?}", scores2);





    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    
}
