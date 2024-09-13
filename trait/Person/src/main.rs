use std::fmt;

trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}

impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} is {} years old.",
            self.name, self.age
        )
    }
}
#[derive(Debug,Clone, Default)] //Default : 구조체 각필드를 디폴트값으로 초기화
struct Book {
    title: String,
    author: String,
    published: u32,
}
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}
impl Printable for Book {
    type Age = u32;
    fn print(&self) {
        println!(
            "Title: {}\nAuthor: {}\nPublished: {}",
            self.title, self.author, self.published
        );
    }
    fn get_age(&self) -> Self::Age {
        self.published
    }
}

// impl Clone for Book {
//     fn clone(&self) -> Self {
//         Book {
//             title: self.title.clone(),
//             author: self.author.clone(),
//             published: self.published,
//         }
//     }
// }

fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}

fn main() {
    // let person = Person::new("Alice", 22);
    // let book = Book {
    //     title: String::from("The Rust Programming Language"),
    //     author: String::from("Steve Klabnik and Carol Nichols"),
    //     published: 20230228,
    // };

    // print_info(&person);
    // print_info(&book);

    // println!("{}", person);
    // println!("{:?}", book);

    let book = Book::default();
    let mut book_clone = book.clone();
    println!("{:?}", book_clone);


}
