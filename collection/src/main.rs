use std::io;
use std::collections::HashMap;

fn add_(_hash: &mut HashMap<String,String> , _vec: &mut Vec<String> ,_cmd: &str) -> bool {
    let mut v : Vec<&str> = Vec::new();
    for word in _cmd.split_whitespace() {
        v.push(word);
    }
    match v[0] {
        "add" | "Add" => {
            match v[2] {
                "to" => {
                    _hash.insert(v[1].to_string(), v[3].to_string());
                    _vec.push(v[1].to_string());
                    return true;
                },
                _ => {
                    return false;
                },
            }
        },
        _ => {
            return false;
        }
    }
    //println!("{:?}",v);
}
fn main()
{

    let mut my_hash: HashMap<String,String> = HashMap::new();
    let mut my_vec: Vec<String> = Vec::new();

    loop {
        println!("Input number!\n1.add 2.search all employee 3.search department 4.quit");
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        let input = input.trim();
        match input {
            "1" => {
                let mut input: String = String::new();
                io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
                let mut input = input.trim();
                if add_(&mut my_hash, &mut my_vec ,&mut input) == false {
                    println!("please check your input (Add <name> to <department>)");
                    continue;
                }
            },
            "2" => {
                my_vec.sort();
                println!("{:?}", my_vec);
            },
            "3" => {
                println!("{:?}", my_hash);
            },
            "4" => {
                println!("Bye!");
                break;
            },
            _ => {
                println!("please check your input");
                continue;
            }
        }
    }
    // my_vec.sort();
    // println!("[Debug]");
    // println!("{:?}\n{:?}", my_hash, my_vec);
}

// fn main() {
//     let mut input = "apple".to_string();

//     let first = &input[0..1];
//     let pig_latin = &input[1..input.len()];
//     let res = match first {
//         "a" | "e" | "i" | "o" | "u" |
//         "A" | "E" | "I" | "O" | "U" => true,
//         _ => false,
//     };

//     match res {
//         true => {
//             input.push_str("-hey");
//             println!("{input}");
//         }
//         _ =>{
//             let pig_latin =  pig_latin.to_string() + "-" + first + "ay";
//             println!("{pig_latin}");
//         }
//     }
    

//     // println!("{}", input);

// }


//use std::collections::HashMap;

// fn main() {

    

//     let mut my = [2,2,4,7,5,1,9];
//     my.sort();

//     println!("{:?}",my);   
//     let len = my.len();
    
    
//     if len % 2 == 1 {
//         println!("median: {}", my[len/2]);
//     }
//     else{
//         println!("medians: {} {}", my[len/2 -1] , my[len/2]);
//     }

//     let mut my_hash = HashMap::new();

//     for i in my {
//         let count = my_hash.entry(i).or_insert(0);
//         *count +=1;
//     }
    


//     for (k ,v) in &my_hash {
//         println!("{}:{}", k,v);
//     }


//     let mut max  = 0;
//     let mut max_idx  = 0;

//     for (k ,v) in my_hash {
//         if max < v {
//             max = v;
//             max_idx = k;
//         }
//     }

    

//     println!("mode: {}", max_idx);

// }

