use std::ops::Index;

use crate::pointer::linked_list::LinkedList_::More;

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


impl MyLinkedList {
    pub(crate) fn new() -> MyLinkedList {
        MyLinkedList { head: LinkedList_::Empty }
    }

    pub fn push(&mut self, data: i32) {
        let old_header = std::mem::replace(&mut self.head, LinkedList_::Empty);
        self.head = LinkedList_::More(Box::new(Node { data, next: old_header }))
    }

    pub fn pop(&mut self) -> i32 {
        let pop_node = std::mem::replace(&mut self.head, LinkedList_::Empty);

        return match pop_node {
            LinkedList_::Empty => {
                panic!("out of bounds!")
            }
            More(node) => {
                self.head = node.next;
                node.data
            }
        };
    }
}

impl Index<usize> for MyLinkedList {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        let mut curr_head = &self.head;
        let mut ret_val: Option<i32> = None;
        for i in 0..=index {
            match curr_head {
                LinkedList_::Empty => {
                    panic!("out of bounds")
                }
                More(node) => {
                    if i != index {
                        curr_head = &node.next
                    } else {
                        return &node.data;
                    }
                }
            }
        }
        panic!("out of bound!")
    }
}


#[cfg(test)]
mod link_test {
    use crate::pointer::linked_list::MyLinkedList;

    #[test]
    fn test1() {
        let mut my_list = MyLinkedList::new();
        my_list.push(2);
        my_list.push(3);
        assert_eq!(3, my_list.pop());
        assert_eq!(2, my_list[0]);
    }
}