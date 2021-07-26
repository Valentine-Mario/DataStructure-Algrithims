mod queue;
mod search;
mod sort;
mod stack;
use crate::queue::*;
use crate::sort::*;
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

    println!(
        "the sorted algo is {:?}",
        bubble_sort(&mut vec![3, 1, 6, 1, 9, 3])
    );
    let mut list = vec![12, 1, 34, 1, 90, 100, 19, 865, 34, 67];
    list.sort();

    let exist = search::binary_search(&list, 304, 0, list.len() as u32);
    println!("{:?}", exist);
}
