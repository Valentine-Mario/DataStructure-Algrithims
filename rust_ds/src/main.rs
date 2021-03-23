mod stack;
use crate::stack::Stack;

fn main() {
    let mut a = Stack::<String>::new(20);
    println!("stack empty {}", a.check_empty());
    a.push(String::from("hi"));
    a.push(String::from("hello"));
    println!("stack empty {}", a.check_empty());
    let poped_item= a.pop();
    println!("popped item {:?}", poped_item.unwrap());
}
