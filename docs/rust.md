# doc-rust

## 基本概念

- 简单类型放在stack上，如：i32, i64, f32, f64, bool, char, tuple, enum, array, pointer, reference
- 大类型放在heap上，如Vec，HashMap, String等，这些类型都有一个指针指向heap上的数据
- 大类型的指针放在stack上

### 使用方法

- `&` 表示引用，不会转移所有权，不会发生数据拷贝，只是借用
- `&mut` 表示可变引用，不会转移所有权，不会发生数据拷贝，只是借用
  - 可变引用只能有一个, 可变引用和不可变引用不能同时出现
- `*` 表示解引用，将引用转换为数据
- `Box` 表示将数据放在heap上，会转移所有权，会发生数据拷贝
- `Rc` 表示引用计数，会转移所有权，会发生数据拷贝
- `Arc` 表示原子引用计数，会转移所有权，会发生数据拷贝
- `RefCell` 表示内部可变性，会转移所有权，会发生数据拷贝
- `Cell` 表示内部可变性，会转移所有权，会发生数据拷贝

### case

- 最直白的例子： &str and String
&str是一个不可变指针，指向了字符串常量，同时不具有数据的所有权
String是一个堆上的对象，同时具有数据的所有权，会随着离开作用域而被销毁
但是通过所有权的转移可以扩展变量的生命周期

~~~rust
fn test() -> String{
    let s = String::from("test");
    s
}
// String("test")的生命周期是t的生命周期
let t = test();
~~~

- 智能指针的理解
以`Box`为例，它拥有对所知对象的所有权，并把它放在堆上

~~~rust
pub fn test() -> Box<String> {
    let str = String::from("hello");
    let ret = Box::new(str); 
    // 这里只发生所有权的转移，
    // 并把String包在Box的结构中
    println!("{:?}",str); 
    //这里会报错，因为所有权已经转移
    ret
}
~~~

~~~rust
pub fn test() -> (Box<String>, Box<i32>) {
    let str = String::from("hello");
    let num = 1;
    let ret = Box::new(str);
    let rett = Box::new(num);
    println!("{:?}", num); 
    // 这里不会报错，因为i32是基本类型，存在栈上
    // 直接调用copy()方法，并且在堆上多一个i32的副本
    (ret, rett)
}
~~~

### 题目解答

~~~rust
pub fn two_boxes() -> (i32, i32, i32) {
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    (**first, **second, sum)
}
~~~

- `arr` 是一个 Vec，它在堆上分配内存来存储 Box 包装的整数。
- `first` 和 `second` 是`&arr[0]` 和`&arr[1]` 的引用，它们位于栈上。这些引用指向堆上的 Box 对象。
- sum 是 `**first` + `**second` 表达式的结果，它也是一个 i32 类型的值，位于栈上。这个表达式会将 `first` 和 `second` 引用解引用两次，然后将它们的值相加。
- 函数结束后 first, second, sum会被立即销毁，arr没有将使用权传递给任何一个变量，因此也不会再被引用，也会被销毁

## 生命周期

### 引用

~~~rust
pub fn anagrams(word: & str, candidates: & [& str]) -> Vec<&str> {...}
~~~

一个这样的函数会出现需要约束生命周期的问题，因为返回的Vec中的元素的生命周期是不确定的

~~~rust
pub fn anagrams<'a>(word: &'a str, candidates: & [&'a str]) -> Vec<&'a str> {...}
~~~

这样就好了

### 闭包

闭包中move关键字强制闭包获取其所捕获变量的所有权
当闭包中出现了外部的变量时编译器会强迫你使用`move`

在并行的实现中，需要注意保护全局变量不要被move进闭包

### 模式匹配

~~~rust
match vec {
        moved_vec => do_something(moved_vec) // 这里就不能用vec了
    }
~~~

## trait struct

trait是一种接口，可以用来约束行为，但是没有具体的数据结构内容
struct 是一种结构体，可以用来定义数据结构，但是没有具体的行为

~~~rust
trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
    age: u32,
}

impl Printable for Person {
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}
~~~

## enum的艺术

~~~rust
pub enum Option<T> {
    /// No value.
    #[lang = "None"]
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value of type `T`.
    #[lang = "Some"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}
~~~

是枚举类
在写链表的时候用来判断是否为空指针`:)`的方法也是放一个枚举类，例如可以分为`Empty`和`More(Box<T>)`两种情况（快说谢谢rust book）
如果直接用`Option<T>`来套的话会出现在通过下标取值时很难处理的问题，返回值会被嵌套成`Option<Option<T>>`....

~~~rust
pub struct MyLinkedList {
    head: LinkedList_,
}

enum LinkedList_ {
    Empty,
    More(Box<Node>),
}

struct Node {
    data: i32,
    next: LinkedList_,
}
~~~

链表的数据结构最后是这样的

## 集合类型的处理

### map

按照原来的结构处理，返回一个新的集合

### flat_map

展开成一个新的集合
flat_map和map一起使用可以按字符处理字符串

例如扫雷题

~~~rust
input: &[&str]
result: Vec<char>
let result = input
        .iter()
        .enumerate()
        .flat_map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, c)|{do_something})
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>()
        .chunks(k)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<char>>>();
~~~
