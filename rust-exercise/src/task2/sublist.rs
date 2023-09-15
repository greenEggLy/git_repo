pub fn is_sublist(str1: &Vec<u32>, str2: &Vec<u32>) {
    let len1 = str1.len();
    let len2 = str2.len();
    if len2 > len1 {
        for i in 0..len2 - len1 {
            if str2[i..i + len1] == str1[..] {
                print!("A is a sublist of B\n");
                return;
            }
        }
    } else if len1 > len2 {
        for i in 0..len1 - len2 {
            if str1[i..i + len2] == str2[..] {
                print!("A is a superlist of B\n");
                return;
            }
        }
    } else {
        if str1[..] == str2[..] {
            print!("A is a equal to B\n");
            return;
        }
    }
    println!("A is not a superlist of, sublist of or equal to B\n");
    return;
}
