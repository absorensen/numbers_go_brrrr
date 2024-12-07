# Less Basic Concepts in Rust
The real contents of this section is the project in ```m0_introduction::code::less_basic_concepts```.
Go into the file corresponding to each function being called in the ```main``` function in ```main.rs```
and read all of the comments in order. The code can also be found [online][0].

# Supplementary Comments
In this section, I'll take you through a few additional notes, which aren't as much about a
specific language construct, but some concepts it might help to know.

## Aliasing
Aliasing is a term for when two pointers or references refer to the same place in memory.
That might not sound like much of a problem at first, but it allows the compiler to make optimizations.
Take a look at this code -

=== "Rust"

    ```rust
    fn compute(input: &u32, output: &mut u32) {
        if 10 < *input {
            *output = 1;
        }
        if 5 < *input {
            *output *= 2;
        }
        // remember that `output` will be `2` if `input > 10`
    }

    fn compute(input: &u32, output: &mut u32) {
        let cached_input: u32 = *input; // keep `*input` in a register
        if 10 < cached_input {
            // If the input is greater than 10, the previous code would set the output to 1 and then double it,
            // resulting in an output of 2 (because `10<>` implies `5<>`).
            // Here, we avoid the double assignment and just set it directly to 2.
            *output = 2;
        } else if 5 < cached_input {
            *output *= 2;
        }
    }
    ```

You can also check the Rustonomicon for a [better explanation of aliasing][1].
It is where the code snippets above are from. The code has been reformatted to preference.
It may be on the more advanced side however.

Basically, whenever you write to a value and there are multiple references to that value hidden away
in different places of the memory hieararchy, such as some threads' registers, or even within the same
function, everything becomes invalidated. This is one of the reasons for the borrow checker to
adamantly enforce that there can be multiple shared (read-only) references to a value,
but only one mutable reference (read/write), and if there is a mutable reference, there cannot
be any shared references to that value. If there were multiple shared references and a mutable reference
it would be impossible to guarantee correctness as just when a value is retrieved from RAM by a shared reference
a write to that value may have ocurred, which in order to make the sequence of operations correct might
necessitate another read, but what if it happens again? Another read! This is not what happens, you
just get a program behaving "weird". In another case, it would also mean you could not read from a value
and save that value in a local variable to do a bunch of operations before writing it somewhere. It already
sounds very headscratching and like you should only ever do single threaded programs. But thankfully,
the borrow checker is there to keep things in check for you. One recommendation - try
to minimize the time that a mutable reference to a value will exist, or in other words, try to
limit the scope of mutable references.

## Multiple Function Definitions Not Allowed
As opposed to languages like C++, you cannot have multiple functions with the same name in Rust.
In C++ this is perfectly legal, and the compiler will attempt to deduce which one you mean based
on the way you are calling function().

=== "C++"

    ```c++
    void function(int argument_a, int argument_b) {}
    void function(int argument_a, int argument_b, int argument_c) {}
    void function(int argument_a, int argument_b, float argument_c) {}
    ```

Rust seems to be designed in a way as to minimize the amount of ambiguity faced by the compiler (and you too).
Sometimes in Rust code you will see several different constructor functions, such as ```build```,
```build_from_ints```, ```new``` and ```default```. In one way, that is a pain in the ass.
In another way, it's actually quite nice. It forces the programmer to be explicit about how the functions
behaviours are different, instead of being unwritten, implicit, or 'well, you can just read the
code, it's not that complicated'. If you ever think or say that. Remember this... *ahem* RED FLAG!
Fix your stuff so people don't have to guess, it will probably make the next person to read your
code hate you slightly less. Which really is something to strive for!

## Index Checking
Whenever you access an element in an indexed collection such as a Vec:

=== "Rust"

    ```rust
    for index in 0..data.len() {
        do_something(data[index]);
    }
    ```

Whenever you do this in Rust, there is a runtime check to make sure this index is not outside of the
memory of the vector. This does have some performance cost, but unless you are absolutely sure this processing
is happening in a hot region (a region of your code where a lot of time is spent), it is not recommended to try
and circumvent this.

=== "Rust"

    ```rust
    for index in 0..data.len() {
        unsafe {
            do_something(data.get_unchecked(index));
        }
    }
    ```

It requires an unsafe region, which is a region in your code where you tell the compiler
to allow you to do some things it would otherwise not allow you to. An unsafe region does not
turn off all checking, but in general, if you are at the level of reading the guide, you don't
need it and I will use it sparingly at most. If you want to read more about unsafe,
the [Rustonomicon][2] is the defacto standard introduction to unsafe in Rust.

The two above functions are equivalent to

=== "C++"

    ```c++
    for(size_t index{ 0 }; index < data.size(); index += 1) {
        do_something(data.at(index));
    }
    ```

=== "C++"

    ```c++
    for(size_t index{ 0 }; index < data.size(); index += 1 )  {
        do_something(data[index]);
    }
    ```

Note however, that square bracket indexing is the defacto standard way of accessing an array element in both
languages. This showcases a core difference between the two languages. One being safety opt-out, and
another being safety opt-in.

[0]: https://github.com/absorensen/the-guide/tree/main/m0_introduction/code/less_basic_concepts
[1]: https://doc.rust-lang.org/nomicon/aliasing.html
[2]: https://doc.rust-lang.org/nomicon/intro.html
