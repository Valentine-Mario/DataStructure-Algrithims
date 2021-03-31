mod queue;
mod stack;
use crate::queue::*;
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
    println!("the item in queue is {:?}", b.item);

    let mut c = CircularQueue::<i32>::new(10);
    c.enqueue(89);
    c.enqueue(100);
    c.enqueue(101);
    c.enqueue(141);


    println!("circular queue {:?}", c.dequeue());
    println!("circular queue {:?}", c.dequeue());
    
    c.print_circular_queue();
}
