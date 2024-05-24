// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("not work");
    let x:i32 = input.trim().parse().expect("not work!");
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
