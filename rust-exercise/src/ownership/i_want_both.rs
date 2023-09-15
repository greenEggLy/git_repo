pub fn i_want_both_1() {
    let x = String::from("hello, world");
    let y = &x;
    println!("{}, {}", x, *y);
}

pub fn i_want_both_2() {
    let x = "hello, world";
    let y = x;
    println!("{}, {}", x, y);
}

pub fn i_want_both_3() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{}, {}", x, y);
}