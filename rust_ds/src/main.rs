mod queue;
mod stack;
use crate::queue::Queue;
use crate::stack::Stack;

fn main() {
    let mut a = Stack::<String>::new(20);
    println!("stack empty {}", a.check_empty());
    a.push(String::from("hi"));
    a.push(String::from("hello"));
    println!("stack empty {}", a.check_empty());
    let poped_item = a.pop();
    println!("popped item {:?}", poped_item.unwrap());

    let mut b = Queue::<i32>::new(10);

    b.enqueue(100);
    println!("the item in queue is {:?}", b.item);
    b.dequeue();
    println!("the item in queue is {:?}", b.item)
}
