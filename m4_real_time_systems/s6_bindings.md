# Bindings
In many cases you, as the dilligent systems programmer I am confident you are,
might be perfectly happy to stay in low level languages like C++ or Rust. Sometimes, it might be more convenient
to configure your real-time system in a higher level language before you verify all the settings, save them to
a file and then give the file as an argument when you want to run just your super optimized pipeline. You might
even be able to use some of the principles from the computational graph sections to optimize on the given
configuration and then once you have prepared everything, allocated every single piece of memory you might need in
your hot loop, you are ready to run at hyper speed. This would also be a good way to handle having users of your
system who might not be systems programmers.

Another case could of course if we inverted the scenario. You might be able to use a low level language to
accelerate whatever you are doing in a high level language such as Python. Other than writing entire
libraries, like PyTorch, you could be writing your own custom data loader.

Another reason for creating [bindings][0] could be if you need the functionality of a library that
isn't available in the language you are using. Rust still has a relatively young ecosystem. It is not uncommon
that a library isn't available or isn't as good as one that is in C++. In that case you will need to use the
[foreign function interface][1] ([Rust FFI][2]) to call into other languages. Specifically, it can call to C
from which you can call to other languages like C++. C generally defines the glue between languages, but
there are multiple crates for creating language bindings found on [blessed.rs][6], like [Maturin][3], which uses
[PyO3][4] ([docs][5]).

Enough motivation! What should you be aware of when creating bindings for your project, outside of the specifics
of using a crate like PyO3? First off, if you are creating bindings, what type of information are you exchanging?
Commands with no inherent information like ```my_lib.terminate()```, calling a Rust library from Python, are easy
to decode. A function was called, no other information needed. Ok, so what if we instead called a function in the
same setup like this ```my_lib.do_something(5)```. Ok, that's fairly trivial, the representation of the number 5
is likely to be the exact same in both Rust and Python (but what about ```String``` and your own custom structs?).
Additionally, because we use a literal, the ownership of that 5 is not an issue. But then we go do something
crazy like this -

=== "Python"

    ```Python
    element_count = 5
    my_lib.do_something(element_count)
    ```

In this scenario, we have created a variable, which in Python is reference counted so the garbage collector can
free it later on, and then given that variable to a function. That function calls some Rust code. Does that Rust
code take ownership of ```element_count```? If it does, we have to be sure that Python no longer sees
```element_count``` as something it has to garbage collect. If the Rust library instead manipulates the value
of ```element_count```, are sure that is unproblematic? As you might recall, Rust's borrow checker is quite strict
that you cannot have multiple references to a piece of data to which you can write. Did we just violate that?
What if, even worse, there were multiple references to ```element_count``` in Python, and we just gave that
value to Rust to manipulate? Basically, you have to make sure to do all of the work that the borrow checker would
do for you. Rust cannot see what happens on the other side of the call and you have to make sure. In some cases,
it might make best sense to make sure that all variables have their ownership moved to Rust when calling functions
like that.

If you were to use a library for calculating a physics problem and you had defined a large matrix for
the initial conditions, it might make perfect sense to hand over the ownership of that matrix to your library.
The alternative could be to copy all incoming data to not manipulate it across language boundaries and to not
mess with the garbage collector, but in that case, a large matrix copy like that might become very slow and
expensive. So yes, to make it worse, creating bindings doesn't just have memory safety and correctness implications,
but performance implications.

Different libraries and methods for creating bindings result in different overhead
costs when calling into the other language. And while you definitely will see the difference between between
implementing a call with or without copying of big variables, there's another key performance issue. How you
design your bindings can make a huge difference. Every single time you cross over from Python to Rust, you pay a
fee. A bit like when we transferred data from the CPU to the GPU. And just like those transfers, we have to try
to stay in one domain, finish the job and transfer back, instead of hopping from one to the other all the time.
How many calls from Python to Rust do you need to make to do something? Do you ever need to do it in a loop?
As you might know, libraries like [Numpy][7] are written in C or C++. When using Numpy, you can do something
with every element of a vector with a for-loop like anywhere else. But, it is a lot faster to remove the for-loop
and use ```[:]```. In that case, you've just moved the for-loop from happening in Python, to happening in C,
reducing the amount of switching from Python to C and back from N to 1, resulting in a large speedup.

There's of course many more subtleties and nuances to creating bindings, but that should be enough to set you
on your way and let you reuse some of the skills you have learned here in other contexts.

[0]: https://en.wikipedia.org/wiki/Language_binding
[1]: https://en.wikipedia.org/wiki/Foreign_function_interface
[2]: https://doc.rust-lang.org/nomicon/ffi.html
[3]: https://www.maturin.rs/
[4]: https://github.com/PyO3/pyo3
[5]: https://pyo3.rs/v0.20.0/
[6]: https://blessed.rs
[7]: https://numpy.org/
