
// pub mod loop_test {
//     //재귀함수
//     pub fn sum(start:i32, end:i32) -> i32{
//         if start == end {
//             panic!("Same number cant be recursive")
//         }
//         if end == start + 1 {
//             start + end 
//         }
//         else {
//             end + sum(start , end-1)
//         }
//     }

//     //Range
//     use std::ops::RangeInclusive;
//     pub fn sum(range: RangeInclusive<i32>) ->i32 {
//         let mut result = 0 ;
//         for i in range {
//             result += i;
//         }

//         result
//     }
// }

pub fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} - FizzBuzz", i),
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            (_, _) => (),
        }
    }
}

