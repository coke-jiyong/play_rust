fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    
    
    for item in list {
        if max < item {
            max = item;
        }
    }
    max
}

struct Point<T> {
    x: T,
    y:  T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
