fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { //std::cmp::PartialOrd 값을 정렬할 수 있는 타입
    let mut max = &list[0];
    
    
    for item in list {
        if max < item {
            max = item;
        }
    }
    max
}



// enum Option<T> { Option 열거형 또한 제네릭
//     Some(T),
//     None,
// }
// enum Result<T, E> { Result 열거형 또한 제네릭
//     Ok(T),
//     Err(E),
// }

struct Point2<T,U> { //x 와 y는 다른타입일 수 있음
    //제네릭 타입 매개변수는 원하는 만큼 정의가능 <T,U,I,C,Z ...>
    //하지만 가독성이 떨어지고 좋은 코드가 아님. 코드 리팩터링이 고려.
    x: T,
    y:  U,
}

impl <T,U> Point2<T,U>{
    fn x(&self) -> &T{
        &self.x
    }
    fn y(&self) -> &U{
        &self.y
    }
}


struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> { //꼭 구조체의 제네릭 타입 매개변수와 메서드의 제네릭타입 매개변수가 항상 같은것은 아님.
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
    
    fn x(&self) -> &X1 {
        &self.x
    }
    fn y(&self) -> &Y1 {
        &self.y
    }
}

fn main() {
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };

    // let integer = Point2 { x: 5, y: 10.1 };
    // let float = Point2 { x: 1.0, y: 4 };

    // println!("{:?}\n{:?}", integer.x,integer.y);
    // println!("{:?}\n{:?}", float.x,float.y);

//     let test = Point2{x:"test" ,y:28};
//     println!("test.x: {}\ntest.y: {}", test.x(), test.y());

    let test1 = Point{x:1,y:3.3};
    let test2 = Point {x:"Best",y:"Player"};

    let result = test1.mixup(test2);

    println!("x:{}\ny:{}",result.x(), result.y());

    
}

