# 👨🏼‍💻 Exercises
These exercises are supposed to get you to write some Rust code yourself. Advent of Code is a good place to start
for learning the basic syntax and stuff like reading from files. After a few exercises you'll see that you start focusing
more and more on the problems themselves instead on Rust. At this point you can learn more about the challenges
of structure and the borrow checker through doing either m0::e1::a or m0::e1::b. One is more visually inclined,
wheras the other is more neural network oriented.

## m0::e0 - Advent of Code
In order to get a hang of the basic of Rust, I recommend doing [Advent of Code][0]
problems until you feel comfortable moving forward.

Give yourself a bigger challenge:

* Implement testing
* Rewrite your solutions to use iterators whenever possible. Use testing to ensure your solution stays correct
* Handle errors properly with match statements instead of using ```.unwrap()``` everywhere

## m0::e1 - Traits
Traits are one of the ways in which we can begin to create structure in our programs. In Rust, traits
are somewhat like interfaces seen in other languages. A struct can implement multiple traits and we can enable
lots of different types of input as function arguments by specifying them somewhat generically. Imagine a
function definition. Argument A might be an ```f32```, argument B might be a ```&Vec<i64>```, but argument
C could support a range of possible inputs, by specifying that all arguments in the third spot (C), should
implement the interface Alpha. Meaning that they all must implement some form of interface that is
needed by this function. It could have lots of other behavior, but we don't care about that behavior.
Or we could even require multiple kinds of traits such as argument C must implement
trait Alpha + trait Beta + trait Omega. This is known as composition. Now for the exercise.

* Read [this link][1] about traits
* Create a new project (```cargo new name```)
* Create 3 different traits
* Create 3 different structs, each implementing 2 different traits
* Create 3 functions each requiring one or more arguments to implement one or more of those traits
* Create a test program which uses all structs, all functions and all traits

## m0::e2::a - Ray Tracing in One Weekend
Doing this exercise will be quite challenging if you don't know how to read C++. To better practice
structure, smart pointers, dynamic dispatch and traits, I recommend moving on to doing
[Ray Tracing in One Weekend][2] in Rust. There is a code snippet which shows your image on screen
instead of writing it to file in [m0_introduction::code::image_on_screen][3].

Once you have completed this part 1 out of 3, I suggest modifying the resulting code in the following ways -

* Remove the use of ```dyn```
* Remove the use of smart pointers
* Parallelize computation of pixels through ```rayon```

??? note "Hints"

    * You can remove the use of dyn through using enums to represent the material interface
    * My solution to remove smart pointers was to use indices and a geometry service provided
    through dependency injection
    * Formulate your rendering as a base iterator that calls the inner function with the correct
    pixel indices, then trade .iter() for .par_iter() 

## m0::e2::b - Learning to Fly
If you are more inclined towards neural networks, [Learning to Fly][4], which focuses on neural networks and genetic
algorithms, might be more stimulating. The code is already in Rust and should be fairly easy to follow.

[0]: https://adventofcode.com/
[1]: https://github.com/absorensen/the-guide/tree/main/m0_introduction/code/image_on_screen
[2]: https://raytracing.github.io/books/RayTracingInOneWeekend.html
[3]: https://github.com/absorensen/the-guide/tree/main/m0_introduction/code/image_on_screen
[4]: https://pwy.io/posts/learning-to-fly-pt1/
