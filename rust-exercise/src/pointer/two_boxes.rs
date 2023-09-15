pub fn two_boxes() -> (i32, i32, i32) {
    // arr: Vec; first, second: Box heap
    // sum: i32 stack
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    (**first, **second, sum)
}
// arr, first, second 生命周期结束，释放
// **first, **second, **sum 栈上的值所有权转交到函数外