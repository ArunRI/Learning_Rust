// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);

// }

// ----------------------------- using generics--------------

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    //have to use std::cmp::PartialOrd, because  the body of largest won’t work for all possible types that T could be, T can be of any type and we can not compare all types
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // --------------------------- generics in structs

    struct point<T> {
        x: T,
        y: T,
    }

    let point_int = point { x: 3, y: 5 };
    let point_float = point { x: 1.2, y: 3.2 };

    struct point2<T, U> {
        x: T,
        y: U,
    }

    let point2_int = point2 { x: 3, y: 4.5 };
    let point2_float = point2 { x: 2.1, y: 5 }; 



    

    impl<T> point<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    let p = point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());


}
