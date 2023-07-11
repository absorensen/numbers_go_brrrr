use std::{rc::Rc, sync::Arc};

pub fn smart_pointers() {
    // Whenver you can't quite get what you want past the borrow checker
    // the next thing to try is smart pointers. They allow data to be shared,
    // but in order to modify them, you will have to combine them 
    // with atomics or mutexes to allow for modifications. For those concepts
    // see the concepts in parallelism module.

    // Box<T> is equivalent to unique_ptr<T> in C++.
    // It's basically a form of ownership of an element
    // of type T, which can be any size. You no longer
    // need to know the size of your element at compile time,
    // a Box<T> is always the same size.
    let our_integer: Box<u32> = Box::<u32>::new(42);

    // We can hand over ownership of our_integer to another,
    // which means our_integer is no longer accesible.
    let another_integer: Box<u32> = our_integer;

    // We can access the underlying value
    let copy_the_integer: u32 = *another_integer.clone();

    // Once the Box goes out of scope and is dropped, it drops whatever value it carries.
    // Read more about Box here: https://doc.rust-lang.org/rust-by-example/std/box.html


    // Rc<T> is sort of like Box, but it can be shared. Rc stands for reference counted.
    // Rc keeps track of how many references there are floating around to its carried
    // value. Every Rc that is dropped decrements the counter. The last decrementer
    // also drops the underlying element.
    let reference_one: Rc<u32> = Rc::<u32>::new(42);
    let references_count: usize = Rc::strong_count(&reference_one); // references_count == 1
    let reference_two: Rc<u32> = Rc::clone(&reference_one);
    let references_count: usize = Rc::strong_count(&reference_one); // references_count == 2
    {
        let reference_three: Rc<u32> = Rc::clone(&reference_one);
        let references_count: usize = Rc::strong_count(&reference_one); // references_count == 3
        let reference_four: Rc<u32> = Rc::clone(&reference_one);
        let references_count: usize = Rc::strong_count(&reference_one); // references_count == 4
    }
    let references_count: usize = Rc::strong_count(&reference_one); // references_count == 2
    // What Rc is not good at however, is when several threads will try to clone the
    // reference. Read more about Rc here: https://doc.rust-lang.org/rust-by-example/std/rc.html.


    // For that we have Arc<T>. Arc stands for atomically reference counted.
    let arc_one: Arc<u32> = Arc::<u32>::new(42);
    let arc_count: usize = Arc::strong_count(&arc_one); // arc_count == 1
    let arc_two: Arc<u32> = Arc::clone(&arc_one);
    let arc_count: usize = Arc::strong_count(&arc_one); // arc_count == 2

    // For an introduction to atomics see the concepts in parallelism module.
    // Arc does come with a performance hit compared to Rc, though, so use
    // only if you are multithreading.
    
    // In general you should use the minimum version of smart pointers that you need.
    // The performance is Arc < Rc < Box.

    // Read more about Arc here: https://doc.rust-lang.org/rust-by-example/std/arc.html
}