pub fn doc1() -> String {
    let str = String::from("hello, world");
    return str;
}

pub fn doc2() -> (Box<String>, Box<i32>) {
    let str = String::from("hello");
    let num = 1;
    let ret = Box::new(str);
    let rett = Box::new(num);
    println!("{:?}", num);
    (ret, rett)
}
