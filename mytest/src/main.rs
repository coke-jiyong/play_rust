// //use mytest::loop_test;
// use mytest::fizzbuzz_2;
// fn main() {
// //    모듈::public();
    
//     // let result = loop_test::sum(1..=10);
//     // println!("{result}");

//     //fizzbuzz_2(15);
//     for i in 1..=100 {
//         let msg = match i {
//             m if m % 15 == 0 => format!("{} - FizzBizz", m),
//             m if m % 3 == 0 => format!("{} - Fizz", m),
//             m if m % 5 == 0 => format!("{} - Buzz", m),
//             _ => format!("{}", i),
//         };
//         println!("{}", msg);
//     }

// }



// fn make_greeting(name: &mut String) {
//     name.push_str("씨 안녕하세요");
// }

// fn main() {
//     let x: Option<i32> = Some(5);
//     let y: Option<f32> = None;
    
//     println!("x is {}", x.unwrap_or(-1));
//     println!("y is {}", y.unwrap_or_default());
//     let some_option: Option<i32> = Some(5);

//     let option_mapped = some_option.map(|num| num + 1);

//     match option_mapped {   
//         Some(x) => println!("result is {}", x),
//         None => println!("result is None"),
//     }
// }


//try 
// use std::fs::File;
// use std::io::prelude::*;

// fn read_file_contents(filename: &str) -> std::io::Result<String> {
//     let mut file = File::open(filename)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// fn main() {
//     match read_file_contents("example.txt") {
//         Ok(contents) => println!("File contents: {}", contents),
//         Err(error) => println!("Error reading file: {}", error),
//     }
// }


//함수포인터
use mytest::fn_pointer::{threesixnine}; 

fn main () {
    //fizzbuzz_fn(fizzcheck,buzzcheck);
    let mut _v=vec![Some(1),Some(2),None,Some(4),Some(5),None,Some(7),Some(8),None,Some(10)];
    
    threesixnine(_v);


}