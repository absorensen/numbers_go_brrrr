# 👨🏼‍💻 Exercises
Try out the profilers relevant to your own system with some sample programs.  
Now try it with some of your own code from before you started on the guide!

## m5::e0 - Setting Up a Profiler
Find and setup a profiler that works on your system.
Find some Rust code that uses the GPU and run it.
If you don't have any code relevant to the kind of projects you do normally, for GPU code you can try out
the ```egui-winit-template``` project, the ```computational_graphs``` framework or the ```gpu_add``` project.

* Does it work with Rust?
* Which metrics can you get out of it?
* Can you see what is happening on the GPU?
* Do you need a different profiler to see what is going on on the GPU?
* Can you see L2 cache hits/misses?
* Can it show you the hot spots in your code?

## m5::e1 - Professor Bærentzen and the Geometry Factory: A Performance Easter Egg Hunt in G Minor
I have conjured up a playground of functions for you to profile and optimize. I present to you
Professor Bærentzen and the Geometry Factory. In it, the Willy Wonka of Geometry Processing
(don't be alarmed, he is completely fictional) has a factory which constructs geometry doodads
and supresses the work force (as is tradition). In the end, he uses the supression of the work
force and various geometry operations to calculate the curvature of rainbows at various points.
He doesn't care much about it's color, that's the other guy.

Figure out why, how and what, in order to optimize the time per loop iteration. You will need the
different concepts introduced throughout the course to figure out how to progress. What the functions
are called and what they do don't matter. Some of the code is real world code, some of it is nonsense.
Try to spot which concept from the text could improve the performance of each section. Also try to
structure the loop. Which of the calculations are completely deterministic and are repeated with
each loop iteration? You are allowed to move things to the GPU if it makes sense, you are also allowed
to completely restructure and cache data. Try to figure out each function individually before you start
using the hints. You can also remove all the timing if you have a working profiler.

Go to the code in ```m5_optimization::code::geometry_factory```.
To get the code to work you need to download the [Stanford bunny][0] as an .obj file through
[Morgan McGuire's Compute Graphics archive][1] and put it in the ```geometry_factory::resources```
folder. You are allowed to change function signatures as long as the overall results stay the same.

??? note "Hints - Overall"

    * You are free to change how you measure things, the timings already there are there to get you started.
    * You are allowed to try and replace the used random number generation solution with something more performant.
    Crytographic security isn't a priority. The generated values should be reasonably uniformly distributed.
    * Some functions do the same thing every loop iteration and can be moved to preprocessing.
    * Some functions don't depend on other functions for their results or are only needed until the very end.
    Parhaps the work could be transferred to other threads.
    * You are free to restructure the flow of data to cache generated or allocated data.
    * How can you see whether results depending on RNG are the same with every iteration?
    * Look for branching everywhere.
    * Look at data oriented design to rewrite the data structures to be optimal for caching.
    * Try to avoid resizing in loops.
    * Is there a more performant way of printing text?
    * Are you sure your data is cache aligned?
    * Try to remove branching and if-statements inside loops.
    * Try to remove all values represented as String.
    * See whether loops can be fused or fissioned.
    * If you parallelize a loop, see whether you gain more performance by using chunks instead.

??? note "Hints - Workers"

    * Restructure Vec<Worker> into Workers using data oriented design.
    * Change in_a_union to bool.

??? note "Hints - Rainbow Curvature"

    * This function doesn't really matter except to create a data dependence ensuring that the compiler
    doesn't remove the other functions for not having their results used.

??? note "Hints - Pretty Screensaver"

    * If you formulate the loops as an iterator instead you can use Rayon to parallelize it, or you can
    move the whole thing to the GPU.

??? note "Hints - Point Cloud Processing"

    * Point sampling of a mesh means sampling random points on a triangle with the number of points relative to
    the area of the triangle. What can you do to minimize the amount of allocations? (Hint: resize as soon as you
    know how much data you will need instead of always just pushing)

??? note "Hints - Pandemonium Machine"

    * As written in the code, the order in which the functions are executed isn't important. You can
    cache this list of functions and sort it.
    * Sort the list of functions so you execute all of the functions of type A before moving on to
    functions of type B.
    * Using enums instead of dyn.

??? note "Hints - Model Loading"

    * In this function the main run time is loading the bunny with a library.
    * What you can do is to make sure you cache and save the bunny model once in preprocessing.

??? note "Hints - Kettles"

    * Does the early returns and extra if-statements yield a performance gain or penalty?

??? note "Hints - Geometry Machinery"

    * Threads being spun up for each individual work task
    * Could you change the gizmo work to use SIMD and data oriented design instead?
    * In the geometry machinery add_work function, could you add a return type to reduce the amount of contention
    on the lock or could you reformulate the interactions using atomics?

[0]: http://www.graphics.stanford.edu/data/3Dscanrep/
[1]: https://casual-effects.com/g3d/data10/index.html#mesh4
