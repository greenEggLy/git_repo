#![feature(test)]
extern crate test;
// mod minesweeper;
// mod concurrency;
// mod sublist;
// mod clock;
mod circular;

fn main() {
    let mut buffer = circular::CircularBuffer::new(2);
    let _ = buffer.write(String::from("1"));
    let _ = buffer.write(String::from("2"));
    let _ = buffer.write(String::from("3"));
    println!("{:?}", buffer);
    buffer.force_write(String::from("4"));
    println!("{:?}", buffer);
    let res = buffer.read();
    println!("{:?}", res);
}
