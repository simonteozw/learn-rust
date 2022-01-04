fn dump(arr: &[i32]) {
    println!("arr = {:?}", arr);
}

fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    let first = v[0];
    let maybe_first = v.get(0);

    println!("first = {}", first);
    println!("maybe_first = {:?}", maybe_first);

    dump(&v);
}