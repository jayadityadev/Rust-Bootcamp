// fn main() {
//     let mut num: i8 = 124;
//     for _i in 0..100 {
//         num += 127;
//     }
//     print!("Number: {}", num)
// }

// fn main() {
//     let greeting = String::from("hello world");
//     println!("{}", greeting);

    // print!("{}", greeting.chars().nth(1000))
// }

// pub fn main() {
//     let x = 98;
//     let is_even = is_even(x);
//     if is_even {
//         print!("{} is even", x);
//     } else {
//         print!("{} is odd", x);
//     }
// }

// pub fn is_even(x: i32) -> bool {
//     return x % 2 == 0;
// }

// pub fn main() {
//     let str = String::from("harkirat singh");
//     println!("First name {}", get_first_name(str))
    
// }

// pub fn get_first_name(str: String) -> String {
//     let mut first_name = String::from("");
//     for c in str.chars() {
//         if c == ' ' {
//             break
//         }
//         first_name.push(c);
//     }
//     return first_name;
// }

fn main() {
    // let is_even: bool = true;
    // if is_even {
    //     println!("It's even");
    // } else {
    //     println!("It's odd");
    // }

    for i in 0..10 {
        print!("{} ", i);
    }
}