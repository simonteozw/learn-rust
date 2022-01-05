fn main() {
    let mut iter = 0 .. 3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
}