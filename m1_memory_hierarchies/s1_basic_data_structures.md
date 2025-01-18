# Basic Data Structures
In this section I will take you through some common constructs like dynamic arrays,  vectors, stacks
and queues, seen through the lens of the stack and the heap from the previous section.

## The Dynamic Array
The dynamic array is ubiquitous in C++ and Rust. It is quite often what we think about, when we think of
arrays in those languages. C++ has [```vector<T>```][0] and Rust has [```Vec<T>```][1]. I highly recommend
reading the first parts of the Rust Vec page. They are basically the same though and I will refer to them
as vector from here on out. A dynamic array bundles up the behavior we saw earlier with pointers,
allocations and deallocations, but adds the ability to automatically create a new array that is larger
(usually by a factor of 2) than the old array and move the old values over to the new array. The vector has
three values. How much memory is in its allocation, ```capacity```, how much of the memory is currently in
use, ```length```, and a pointer to the data which lives on the heap. The vector itself can live on the
stack and make sure to free the memory it points to once the vector is dropped from the stack. The vector
supports quite a few operations, but the core ones are ```push```, ```pop```, array access ```[]```,
```reserve``` and ```shrink_to_fit```.

Let's start off with how we allocate a vector in Rust.

=== "Rust"

    ```rust
    let mut data: Vec<i32> = Vec::<i32>::new();
    ```

In this case we should get a completely empty vector. It will have a default ```capacity```, because
we didn't specify any capacity it should start with. Let's just say this ```capacity``` is 4.
However, if we want to print the current size

=== "Rust"

    ```rust
    let mut data: Vec<i32> = Vec::<i32>::new();
    println("{}", data.len());
    ```

we would get an output of 0! We have a ```capacity``` of 4, but a ```size``` of 0. Meaning,
we have 4 integers of 4 bytes each on the heap, but they are unitialized (containing garbage values),
and we have not used any of them. If we however use ```push``` to add some actual data and then print

=== "Rust"

    ```rust
    let mut data: Vec<i32> = Vec::<i32>::new();
    data.push(0);
    data.push(1);
    println("{}", data.len());
    ```

we would print the number 2. Now we have live, initialized values on the heap at indices 0 and 1.
We can print them by accessing the values directly.

=== "Rust"

    ```rust
    let mut data: Vec<i32> = Vec::<i32>::new();
    data.push(0);
    data.push(1);
    println("{}", data.len());
    println("{}", data[0]);
    println("{}", data[1]);
    ```

In this case we print 2, 0 and 1. Push finds the first unused index, which is conveniently indicated
by the ```size``` value, increments ```size``` and puts the value into the designated index. If we pushed
5 values however, once we reached the 5th push, assuming the default capacity was 4, we would see the
5th push taking a lot of time compared to the other 4 pushes. In this case the vector would allocate
a new memory segment on the heap with a size of 8, copy all of the values from elements 0-3 and then add
the 5th value to the vector. Conversely, we can also use the ```pop``` function.

=== "Rust"

    ```rust
    let mut data: Vec<i32> = Vec::<i32>::new();
    data.push(0);
    data.push(1);
    data.pop();
    println("{}", data.len());
    println("{}", data[0]);
    ```

Now we end up printing the values 1 and 0. In theory, a dynamic array should move to a smaller array at some point.
Such as, when at a quarter of the reserved capacity. But in practice, Rust doesn't move to a smaller array
unless explicitly asked to do so using the ´´´shrink_to_fit´´´ function. In that case it will allocate and move
to an array that is exactly the size of ```size```, thus also making ```capacity``` the same. In practice,
you should only do this for large arrays which are unlikely to see more elements added to it.

But, in the case of knowing how many elements we actually want to put in our vector, or at least an expcected
minimum amount, we can just create the vector in a way where it has already reserved that amount of capcity.
If you can at all do this, which requires you to at the very least estimate how much data you need at the time
of creating the vector, it is one of the easiest ways to get better performance as you remove a whole
bunch of allocations, deallocations and copying.

=== "Rust"

    ```rust
    let mut data: Vec<i32> = Vec::with_capacity(5);
    data.push(0);
    data.push(1);
    data.push(2);
    data.push(3);
    data.push(4);
    ```

In this case, we have been unambigously up front about how many elements we will put in the vector.
It was created with a ```capacity``` of 5 and a ```size``` of 0. We can also tell the vector to make sure we
have a ```capacity``` of at least N. If it already has ```capacity``` to meet the minimum, nothing happens.
If it doesn't it will allocate, copy and deallocate.

=== "Rust"

    ```rust
    let mut data: Vec<i32> = Vec::<i32>::new();
    let element_count: usize = 42;
    data.reserve(element_count);
    for index in 0..element_count {
        data.push(index as i32);
    }
    ```

There are more idiomatic ways to do this in Rust, which might also be faster, but you get the gist!

## Enter the Matrix
But, we aren't just interested in single lists of numbers, sometimes, we would even like a matrix.
In Rust we can have fixed size arrays defined like so:

=== "Rust"

    ```rust
    let data: [i32; 2] = [0, 1];
    ```

If the sizes given to the array definition are constants, known at compile time, the array will be
stack allocated. From what we have learned previously, the elements will be stored in memory in the
order of 0 and 1. But what if we create a two-dimensional array?

=== "Rust"

    ```rust
    let data: [[i32; 2]; 2] = 
                            [
                                [0, 1], 
                                [2, 3]
                            ];
    ```

In Rust the elements will be ordered in memory 0, 1, 2, 3. But that is not a universal truth.
This is called row-major ordering and is the standard layout in C, C++, Rust, Python and most modern languages.
The alternative is column-major which is seen in Fortran and Matlab.
In column-major ordering the elements would be ordered in memory as 0, 2, 1, 3.
With row-major ordering the memory will be most tightly packed in the last dimension from the left.
In Rust, which, again is row-major ordered, we can iterate through a 3 dimensional matrix in the same order as its
memory with the following triple for-loop.

=== "Rust"

    ```rust
    let data: [[[i32; 2]; 2]; 2] = 
                                [
                                    [[1, 2], [3, 4]],
                                    [[5, 6], [7, 8]]
                                ];

    let x_dimension: usize = 2;
    let y_dimension: usize = 2;
    let z_dimension: usize = 2;

    for x_index in 0..x_dimension {
        for y_index in 0..y_dimension {
            for z_index in 0..z_dimension {
                println!("{}", data[x_index][y_index][z_index]);
            }
        }
    }
    ```

Where as if Rust favored column-major ordering the in-memory-order traversal would be

=== "Rust"

    ```rust
    let data: [[[i32; 2]; 2]; 2] = 
                                [
                                    [
                                        [[1, 2], [3, 4]],
                                        [[5, 6], [7, 8]]
                                    ]
                                ];

    let x_dimension: usize = 2;
    let y_dimension: usize = 2;
    let z_dimension: usize = 2;

    for z_index in 0..z_dimension {
        for y_index in 0..y_dimension {
            for x_index in 0..x_dimension {
                println!("{}", data[x_index][y_index][z_index]);
            }
        }
    }
    ```

If you think back to stride and cache lines, traversing our 3-dimensional array like the above
in the actual case, where Rust is row-major, would be like the stride access we looked at earlier.
We could also do this with nested vectors.

=== "Rust"

    ```rust
    let mut data: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    data.push(vec![0, 1]);
    data.push(vec![2, 3]);


    let x_dimension: usize = 2;
    let y_dimension: usize = 2;

    for x_index in 0..x_dimension {
        for y_index in 0..y_dimension {
            println!("{}", data[x_index][y_index]);
        }
    }
    ```

This is even worse though. We now have a 2-dimensional array, which is highly flexible, we can
even have uneven dimensions, but we have to dereference two pointers for every access.

There is another way of doing this with a vector, which is the way I will be using
multi-dimensional arrays in this module. It involves using a single dimensional vector
as if it had more dimensions. This is also known as linearization.

=== "Rust"

    ```rust
    let mut data: Vec<i32> = Vec::<i32>::new();
    data.push(vec![0, 1, 2, 3]);

    let column_count: usize = 2;
    let row_count: usize = 2;

    for x_index in 0..row_count {
        for y_index in 0..column_count {
            println!("{}", data[x_index * column_count + y_index]);
        }
    }
    ```

We just create a vector with as much room as we need and then access it with a bit of calculation.
We've flattened our matrix and can now both have it dynamic and with arbitrary dimensions. We
can even dynamically decide to see the matrix in a different way, for example by deciding
to swap the number of columns and rows. The formula to access each element is to multiply
the index by the dimensions that come after it multiplied and add it to the next index.
For example with three dimensions ```x```, ```y``` and ```z```, the index would be
calculated by

=== "Rust"

    ```rust
    x_index * y_size * z_size + y_index * z_size + z_index
    ```

and for the two dimensions ```x``` and ```y```, we would access the 2-dimensional matrix with

=== "Rust"

    ```rust
    x_index * y_size + y_index
    ```

I really hope this makes sense. Once it clicks it is a very simple formula, if a bit wordy.
Some libraries will work like this under the hood but wrap it in an interface
for you to simply access it like it was a multi-dimensional array.

To wrap it up I have made a performance test of these approaches. The code
doesn't match completely as we need bigger dimensions to get a good test.
The code is at ```m1_memory_hierarchies/code/the_vector/``` or [online][2].

Implementing all of the methods described above in both row-major and column-major form,
as well as an element-wise version, where we flatten the multidimensionality to save
the administration of two of the for-loops, so we just get one for-loop running across
a vector, we get the following numbers.

<figure markdown>
![Image](../figures/the_multidimensional_vector.png){ width="500" }
<figcaption>
Access times for multidimensional arrays.
</figcaption>
</figure>

The functions named Multi-Array are stack allocated instead of heap, which is why
they are that fast. I was however unable to run them for 64x64x64 and 128x128x128.
Rust refused citing a stack overflow. Interestingly as well, the element-wise function
can be quite fast as it saves two of the for-loops. So, if you can, use element-wise.
Otherwise, the row-major single vector function seemed to work the best. How much
is saved by not having the two extra for-loops depends on how much work you are
actually doing in each iteration. In this benchmark we do pretty much nothing.

## Move, Copy, Clone, Soldier, Spy
Now that we have examined how we can deal with a more expensive type,
compared to the simpler integer or float, let's expand the scope a little bit.
How do we actually move around these vectors as data? In each language there are
some implicit rules, which can have wide reaching ramifications, both in terms of
correctness and performance.

In Python, variables are all references to an underlying object, which is freed
when there are no longer any references to said object. Don't worry about it too
much, it's a concept I will introduce further down the page.
But, it does have consequences when this happens

=== "Python"

    ```python
    x = [5, 5, 3, 42]
    y = x
    ```

There aren't actually two lists, but two references to a list which has some data on the heap.
This can be a bit problematic, as you now have two variables, which can both write to the same list without
the other knowing. Once both ```x``` and ```y``` go out of scope, the list on the heap will be deallocated
(eventually). This happens because Python keeps count of how many live references there currently are to
the data element. Once ```x``` goes out of scope, that counter is decremented. Same goes for ```y```.
Whichever one decrements the counter to 0 will trigger a cleanup.

In C and C++, the following actually results in two different lists on the heap, kept by two different
variables. C++ is copy by default, and this is a deep copy.

=== "C++"

    ```c++
    vector<int> x{5, 5, 3, 42};
    vector<int> y = x;
    ```

Which is what Rust would call a clone. Rust however, is move by default.

=== "Rust"

    ```rust
    let x: Vec<i32> = Vec::from([5, 5, 3, 42]);
    let y: Vec<i32> = x;
    ```

Once the values in ```x```, the ```capacity```, ```size``` and the pointer to the memory on the heap, have
been moved from ```x``` into ```y```, ```x``` is no longer accessible. The Rust compiler will complain. We
can however, move it right back.

=== "Rust"

    ```rust
    let mut x: Vec<i32> = Vec::from([5, 5, 3, 42]);
    let y: Vec<i32> = x;
    x = y;
    ```

Now, ```y``` is inaccessible at the end. We could also create a scope, after which ```y``` is dropped,
but the ownership is not moved back to ```x```.

=== "Rust"

    ```rust
    let x: Vec<i32> = Vec::from([5, 5, 3, 42]);
    {
        let y: Vec<i32> = x;
    }
    ```

Unless we move the values back ourselves.

=== "Rust"

    ```rust
    let mut x: Vec<i32> = Vec::from([5, 5, 3, 42]);
    {
        let y: Vec<i32> = x;

        x = y
    }
    ```

To actually create two lists, like we did in the C++ example, we have to explicitly ask for a deep copy.
A clone in Rust terminology.

=== "Rust"

    ```rust
    let x: Vec<i32> = Vec::from([5, 5, 3, 42]);
    let y: Vec<i32> = x.clone();
    ```

Usually, in Rust at least, adding lots of clones everywhere is an easy way to get around the borrow checker and
have everything be correct. But once your first prototype is finished, one of the easiest improvements to
your performance will be to search for all instances of .clone() and see whether there is some other solution
that might work better. Rust isn't fighting you in this case, even if it can be strict, it is trying to
protect you from having multiple write-enabled references to the same data, as in the Python example,
which could make for incorrect code. While performance is great, in most cases, it should not come before
correctness. C++ does have these [move operations][3] as well, it is even highly
recommended a lot of the time. It is however, not the default behavior of the language.

Rust has something called traits (don't worry about it). One of these traits is the ```Copy``` trait.
If a type implements the ```Copy``` trait, it will be [copied rather than moved][4] when assigned to a new
value or passed as an argument to a function. It is sort of like an implicit version of ```.clone()```,
except in the case of deeper structures, such as ```Vec<T>```, in that case, it would copy all of the
stack values, ```capacity```, ```size``` and the pointer to the memory on the heap.

But hold on a minute! That is illegal! We would have two pointers with full write rights. Which is illegal
in Rust! Which is also why ```Vec<T>``` doesn't actually implement ```Copy``` and this has all been a ruse,
for your edification.

## Stacks
Now let's look at the stack, not that one, the other stack. The fundamental data structure. It isn't an array, but
most implementations are just an array used in a restricted fashion. A stack is what is called a Last In,
First Out (LIFO) data structure. The usual example is, imagine a stack of cantina trays. If you put a tray
into the stack, in order to get a tray, you have to take the top tray, you can't remove a tray that is
below the top tray. At least not without picking up the top tray, taking the now top tray and putting back
the former top tray, which now becomes the top tray again. Top tray.

<figure markdown>
![Image](../figures/stack_push.png){ width="500" }
<figcaption>
Pushing a value on to the stack. The states are from before the push.
</figcaption>
</figure>

If we implement this using a vector, we need at least the following 3 functions - ```push```, ```pop``` and
```peek```. ```push``` you might already know as the default mechanism for adding an individual element to
a vector. The element to push is inserted at index ```size``` and ```size``` is incremented. With a ```pop```,
the element at index ```size - 1``` is returned and ```size``` is decremented. With a call to ```peek```, either
a copy or a reference to the element at ```size - 1``` is returned. Most, if not all functions are
already implemented on the vector types, but if we want to maintain the invariant that all of the elements from
indices 0 to ```size - 1``` are all valid, you need to make sure that only the stack related functions are called.
In that way, if you need a stack, you should use not just a vector type, but a stack type, which might just be a
wrapper around a vector, but also restricts anyone using that type to maintain the invariants needed for a valid
stack. Sending a ```Stack<T>``` from function to function, instead of a ```Vec<T>```, will also communicate
how the value is supposed to be used.

<figure markdown>
![Image](../figures/stack_pop.png){ width="500" }
<figcaption>
Popping a value from the top (end) of the stack. The states are from before the pop, and were the result of
the previous push.
</figcaption>
</figure>

Stacks scale well and all operations would be constant time, except when enough values have been pushed to
necessitate a resize. However, the cost of this is low enough that across all of the operations it averages out
and becomes what is known as amortized constant time. Amortized constant time is a concept found in the
[big O notation system][9].

## Queues
Queues, just like stacks, are a fundamental data type centered around constant time operations mostly implemented
on top of dynamic arrays. Queues maintain a First In, First Out (FIFO) principle, just like queues of people.
The first person to enter a queue, should be the first person to leave it. Now we no longer have ```pop```
and ```push```, but ```enqueue``` and ```dequeue```. Enqueueing is basically the same as ```push``` on a stack.
An element is added to the index at ```size```, except, the queue needs two new variables, ```front``` and
```back```. Once the ```back``` index extends beyond the ```size``` or ```capacity```, it can just wrap
back around and starting again from 0, as long as it does not become equal to the ```front``` value. If it does so
and ```capacity < back - front```, it can resize itself and adjust.

<figure markdown>
![Image](../figures/queue_enqueue.png){ width="500" }
<figcaption>
Enqueueing a value from to the back of the queue. The states are from before the enqueue.
</figcaption>
</figure>

Resizing is just one way to handle the overlap. In quite a few real-time systems, we don't want the system to be
overwhelmable. If data comes in too fast to process, and it keeps coming in faster than we can process, we might
instead say that the ```front``` will move with the ```back``` if they become equal, thus letting the older data
be overwritten. Other options could be to have whatever is trying to submit an element, wait until a spot opens up
in the queue or the element could be "added", but not actually added to the queue, essentially dropping the given
element into the void. You'd of course like to be certain of how your queue type would handle being full.
It's a central property and you should make sure if you are constructing systems with lots of data that you use
a queue with the right behavior for your system.

<figure markdown>
![Image](../figures/queue_dequeue.png){ width="500" }
<figcaption>
Dequeueing a value from to the front of the queue. The states are from before the dequeue.
</figcaption>
</figure>

Just like the stack, your local vector type probably has the functionality, but if you use it as a queue, you
should probably just use a queue type, restricting any usage to maintain and communicate that it's a queue.

## Additional Reading
For more on implementing a [heap with an array][5], [priority queues][6], [binary trees][7],
[binary trees using arrays in Python][8]. These pages have implementation details in C/C++/Python.

[0]: https://en.cppreference.com/w/cpp/container/vector
[1]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[2]: https://github.com/absorensen/the-guide/tree/main/m1_memory_hierarchies/code/the_vector
[3]: https://en.cppreference.com/w/cpp/utility/move
[4]: https://blog.logrocket.com/disambiguating-rust-traits-copy-clone-dynamic/
[5]: https://www.programiz.com/dsa/heap-data-structure
[6]: https://www.programiz.com/dsa/priority-queue
[7]: https://www.programiz.com/dsa/binary-tree
[8]: https://programmingoneonone.com/array-representation-of-binary-tree.html
[9]: https://en.wikipedia.org/wiki/Big_O_notation
