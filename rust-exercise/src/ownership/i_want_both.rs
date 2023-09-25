pub fn i_want_both_1() -> String {
    let x = String::from("hello, world");
    let y = &x;
    let output = format!("{}, {}", x, *y);
    println!("{}, {}", x, *y);
    output
}

pub fn i_want_both_2() -> String {
    let x = "hello, world";
    let y = x;
    let output = format!("{}, {}", x, y);
    println!("{}, {}", x, y);
    output
}

pub fn i_want_both_3() -> String {
    let x = String::from("hello, world");
    let y = x.clone();
    let output = format!("{}, {}", x, y);
    println!("{}, {}", x, y);
    output
}
