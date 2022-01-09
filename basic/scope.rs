fn main() {
    let mut s1 = "hello dolly".to_string();
    let rs1 = &s1;
    s1 = "hello world".to_string();
    println!("s1: {}", &s1);
    println!("rs1: {}", &rs1);
}