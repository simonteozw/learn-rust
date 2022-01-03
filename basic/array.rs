fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0 .. values.len() {
        res += values[i];
    }

    return res;
}

fn main() {
    let arr = [1,2,3,4,5];
    let first: i32 = arr[0];
    let s1 = &arr[0..2];
    let s2 = &arr[2..];

    println!("first: {}", first);

    for i in 0 .. 5 {
        println!("arr[{}]: {}", i, arr[i]);
    }

    println!("{}", sum(&arr));

    println!("arr: {:?}", &arr);
    println!("s1: {:?}", &s1);
    println!("s2: {:?}", &s2);
}