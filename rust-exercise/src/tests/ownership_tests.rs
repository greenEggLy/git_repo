#[cfg(test)]
mod test {
    #[test]
    fn test_i_want_both_1() {
        use crate::ownership::i_want_both::*;
        let output = i_want_both_1();
        assert_eq!(output, "hello, world, hello, world");
    }

    #[test]
    fn test_i_want_both_2() {
        use crate::ownership::i_want_both::*;
        let output = i_want_both_2();
        assert_eq!(output, "hello, world, hello, world");
    }

    #[test]
    fn test_i_want_both_3() {
        use crate::ownership::i_want_both::*;
        let output = i_want_both_3();
        assert_eq!(output, "hello, world, hello, world");
    }
}
