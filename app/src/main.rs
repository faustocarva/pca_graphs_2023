//use graphlib::*;

fn main() {}

// fn recursive_function(n: i32) {
//     let buffer = [0u8; 1024]; // declare a large buffer on the stack
//     println!("Stack frame {} at {:p}", n, &buffer as *const _); // print the address of the stack frame
//     recursive_function(n+1); // call the function recursively
// }

// fn main() {
//     recursive_function(1); // call the recursive function
// }

// fn main() {
//     let mut v = vec![
//         String::from("hello"),
//         // String::from("world"),
//         String::from("rust")
//     ];
//     borrow(&v);
//     println!("{}", v.get(0).unwrap());
//     v.push(String::from("Bazinga"));
// }

// fn borrow(v: &Vec<String>) {
//     for i in v {
//         println!("{}", i)
//     }
// }
