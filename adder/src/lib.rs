// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger: Rectangle = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller: Rectangle = Rectangle {
//             width: 5,
//             height: 2,
//         };

//         assert!(larger.can_hold(&smaller));
//     }
// }
// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }


// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(3));
//     }
// }

// pub fn greeting(name: &str) -> String {
//     format!("Hello {}!", name)
// }

// #[test]
// fn greeting_contains_name() {
//     let result: String = greeting("Carol");
//     assert!(
//         result.contains("Park"),
//         "Greeting did not contain name, value was `{}`",
//         result
//     );
// }
// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic]
//     fn greater_than_100() {
//         Guess::new(3);
//     }
// }

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}