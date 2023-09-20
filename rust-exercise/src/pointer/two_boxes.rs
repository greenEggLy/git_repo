pub fn two_boxes() -> (i32, i32, i32) {
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    (**first, **second, sum)
}