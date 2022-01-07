fn dump(s: &str) {
    println!("str is {}", s);
}

fn arr_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn main() {
    let s = "hello"; // slice string
    let t = s.to_string(); // allocated string

    dump(s);
    dump(&t);

    let mut s2 = String::new();
    s2.push('H');
    s2.push_str("ello");
    s2 += " world!";
    s2.pop();

    dump(&s2);

    let arr = arr_to_str(&[1,2,3,4,5]);
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [1,2,3,4,5]");
}