# Optimization
Generally, when optimizing code, try to see the memory hierarchy and optimize
from the bottom up. Sure, you can optimize your RAM usage and memory allocations,
but it won't matter much if your program spends most of its time waiting on the disk.
Likewise, microoptimizing your calculations for optimal cache usage probably won't
matter much if you are constantly asking the operating system for more RAM in a
loop.

Much like the memory hierarchies there's levels to measuring performance.
The easiest is just using your system's own task manager. On Windows you just click
"ctrl+shift+esc" and click on the performance tab. No putting in print statements,
no timing of functions, no installation, no nothing. You can see many of your cores
are being used, how much memory (RAM) is being used, how is you GPU doing, how much
is the disk being used. All of that. You will have to deduce what is happening, but
if you are training a neural network, and you wonder why training is so slow, whip
up task managers performance tab, see that the disk is being used quite aggresively
while you are using hardly any of the RAM or GPU RAM, then you are probably loading
all of your data from disk at all times without trying to save it in any type of RAM.
Keeping your data in a higher level of the memory hierarchy should probably be your
first move without spending too much additional time figuring out what the problem is.

Using something like task manager requires very little work and is easy to do,
but it will get you a view of all of the programs currently running on your system.
At best you can check what the system is using without your program and deduce what
your program is doing. But much like the black box approach to understanding the
performance of someone else's library, trying to deduce the characteristics of your
program through the task manager might not be enough.

Next up, you can insert print statements and measure the timing of functions yourself.
This won't give you any information about how many threads are in play, how much
memory is used, or how many L2 cache misses you have, but it will allow you to
measure the functions in your own code directly. This might require you to radically
change your code in order to get adequate measurements and you might have to examine
more closely what it is you want to measure. If you are developing a real-time system,
the variance in the time to compute each frame might be relevant, pipeline bubbles could
be relevant, memory usage over time (to ensure you don't have a memory leak) could be
important, you may want to automatically create graphs of the system's performance or
you might decide to benchmark functions individually, perhaps even automatically
as part of your automated CI to ensure that none of your nifty, high performing, functions
get slower with new commits. Creating benchmarks for individual functions can take
a bit more work as you also have to generate some test cases to use as input.

Depending on what system you are on, you can have easy access to programs like
[perf][0], which is available on Linux.
This command line tool allows you to run your full program and can output performance
statistics like which functions are the "hottest", as in, in which functions does your
program spend the most amount of time. To get meaningful output in situations like these
your program needs to be compiled with debug symbols. This means that every function
in your program doesn't just have a mangled named like ```xYz289421```, but a human readable
name like ```my_function()```. The computer doesn't care, that name is for you.

Things like that, and which line is run in which order can easily get mangled based on which flags
you gave the compiler. You should always debug in debug mode, but for measuring performance,
you should in most cases measure it as close to the final program as possible, but with
the caveat of retaining symbols and other likely relevant information. What tools
like ```perf``` might not be able to help you with (not quite sure) is the nitty gritty
details of exactly what is going on in your CPU. In that case you may need to get use
a profiler from the specific hardware vendor who made the CPU, or in the case of GPU
programming (which ```perf``` is not attuned to), a GPU profiler from the GPU vendor.
Thus for GPU programs you might end up profiling twice, separately. Once for your
CPU code and once for your GPU code. These sorts of profilers, which can be great,
awesome and very visual tools, can be a pain to install and are so platform specific
that I won't go into too great detail. They are specific tools you have to install
and learn for your specific platform. You should make sure that you really need that
extra information before spending the extra time, and possibly frustration.

Finally, some frameworks might even have their own profiler, such as the profiler
that is in your browser for web programming, or the profiler that is provided with
deep learning frameworks such as PyTorch.

## Additional Reading
For some step-by-step introductions to optimization check out [this series of videos][1] about
optimizing the performance of machine learning code or [this video][2] about writing performant concurrent
data structures.

[0]: https://en.wikipedia.org/wiki/Perf_(Linux)
[1]: https://nvlabs.github.io/EfficientDL/
[2]: https://www.youtube.com/watch?v=XKODaZgKcnE
