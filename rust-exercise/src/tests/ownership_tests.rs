fn testfn() {
    let yesyes: i32 = 2;
    print!("{}", yesyes);
    print!("{}", yesyes);
    print!("{}", yesyes);
    print!("{}", yesyes);
    let test = 4;
}

// #[cfg(test)]
// mod ownership_test {
//     use super::*;
//     use crate::ownership::*;
//     use crate::ownership::clone_or_copy::clone_or_copy;
//     use crate::ownership::i_want_both::{i_want_both_1, i_want_both_2, i_want_both_3};

//     fn ownership_test1() {
//         let y1 = i_want_both_1();
//         assert_eq!(y1, "hello, world, hello, world");
//         let y2 = i_want_both_2();
//         assert_eq!(y2, "hello, world, hello, world");
//         let y3 = i_want_both_3();
//         assert_eq!(y3, "hello, world, hello, world");
//     }

//     fn ownership_test2() {
//         let (output, x) = clone_or_copy();
//         assert_eq!(output, format!["{:?}", x]);
//     }
// }
