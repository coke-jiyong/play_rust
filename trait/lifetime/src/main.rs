fn main() {
    // let string1 = String::from("long string is long");

    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str()); //더 작은 라이프 타임을 가지는쪼기 `a의 라이프 타임이 됨.
    //     println!("The longest string is {}", result);
    // }



    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result); //error: longeset 함수가 반환 할 참조자의 라이프타임은 
    // //                                              매개변수의 라이프타임중 작은 것과 동일하다.


    let string1 ;
    let string2 ;
    let result;
    {
        string1 = String::from("xyz");
    }
    {
        string2 = String::from("long string is long");
        result = longest(string1.as_str(), string2.as_str());
    }

    

    println!("The longest string is {}", result); 
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}