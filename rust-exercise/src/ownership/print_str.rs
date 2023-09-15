fn print_str(s: &String) {
    println!("{}", s);
}

pub fn call_print_str() {
    let s = String::from("hello, world");
    print_str(&s);
    println!("{}", s);
}