use std::env;

fn main() {
    let first = env::args().nth(1).expect("Please give an argument!");
    let n: i32 = first.parse().expect("not an integer!");

    println!("{}", n);
}