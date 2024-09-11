
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

// pub fn fizzbuzz_2(max: i32) {
//     for i in 1..=max {
//         match (i % 3, i % 5) {
//             (0, 0) => println!("{} - FizzBuzz", i),
//             (0, _) => println!("{} - Fizz", i),
//             (_, 0) => println!("{} - Buzz", i),
//             (_, _) => (),
//         }
//     }
// }

pub mod fn_pointer{
    pub fn fizzcheck(n:i32) -> bool {
        n % 3 == 0
    }

    pub fn buzzcheck(n:i32) -> bool {
        n % 5 == 0
    }

    pub fn fizzbuzz_fn(fizz:fn(i32)->bool , buzz:fn(i32)->bool) {
        for n in 1..=100 {
            if fizz(n) && buzz(n) {
                println!("Fizz_Buzz!");
            }
            else if fizz(n) {
                println!("Fizz!");
            }
            else if buzz(n) {
                println!("Buzz!");
            }
            else {
                println!("anything!");
            }
        }
    }


    pub fn fizzbuzz_2(max:i32) {
        for n in 1..=max {
            match (n%3 , n%5) {
                (0,0) => {println!("{n} - FizzBuzz!")},
                (0,_) => {println!("{n} - Fizz!")},
                (_,0) => {println!("{n} - Bizz!")},
                (_,_) => {println!("{n} - Nothing!")},
            }
        }
    }

    pub fn fizzbuzz_3(max:i32) {
        let iter = 1..=max;
        let fizzbuzzcheck = (|x:i32| match (x % 3 , x % 5) {
            (0,0) => {format!("{x} - FizzBuzz!")},
            (0,_) => {format!("{x} - Fizz!")},
            (_,0) => {format!("{x} - Buzz!")},
            (_,_) => {format!("{x} - Anything")},
        });
        for n in iter {
            println!("{}", fizzbuzzcheck(n));
        }
    }


    //practice
    pub fn threesixnine(nums: Vec<Option<i32>>) {
        // What happens if switch .iter and .into_iter?
        // What are the types of first i and the second i?
        for op in nums {
            println!("{}", op.map_or("짝!".to_string(), |n| format!(" {}! ", n)));
        }
        
    
        // let ret = nums
        //     .into_iter()
        //     .map(|i| format!("{}", i))
        //     .collect::<Vec<String>>()
        //     .join("|");
        // println!("{}", ret);
    }
    
    
}