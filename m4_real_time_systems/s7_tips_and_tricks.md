# Tips and Tricks
Wrapping up this module I have a random bag of tips and tricks that I didn't know where else to put.
Real-time systems are often complex heterogenous things. Not everything will fit neatly into a box.
But as always, we should at least try to sort them into clusters...

## Software Engineering
I'll get to the performance stuff in a second. But one way for performance analysis to get easier, is for your
code and your process to get simpler. Not simple in the way where you just hard code everything or name every
single variable ```i```, but simpler in the sense that your code is simpler to read for other people. Other people
also includes future you. Try to minimize the amount of abbreviations you use. Sure, people can figure out
that ```img``` means an image, but it isn't easier to write ```img``` than ```image``` and you get used to
making these concessions everywhere instead of making code that is as easy to understand as possible.

If you are doing equations, it is ok to name the variables something else, but in that case you should put
a link in the comment that makes it absolutely clear what you are doing. Making other people have to do detective
work to understand your code doesn't make you anything other than a target for a coffee fueled grievance rant.
The computer doesn't care what you name your variables. Of course you can go overboard and have overly verbose
naming, but that's something that you'll have to work on. Naming variables can be a difficult process. We do
have to make concessions in other areas though, like reformating our code to follow data oriented design instead.
But you should only do so when seeing a clear value gained from deviating from the simple and
the easily comprehendible.

Another software engineering-y tip is to always use version control, even for solo development. Once you finally
get all of your code working correctly, make sure to commit it. As soon as you start optimizing and refactoring
your code, you are likely to introduce bugs, get exasperated and lost. Then once you've given up, you can just
revert to your commit when it was working and retry your optimization attempt. Usually at that point you've tried
out some stuff and can now see a clearer, simpler, solution and it will come to you much quicker. Thank you,
version control! Or, even better, create branches when you try out new features and merge them into trunk once
you're assured that they didn't introduce errors.

## Minimizing Interaction with the Operating System
Previously I have mentioned concepts such as object pools and green threads. Often times a key performance
improvement can be simply to make less, but bigger interactions with the operating system. In the simplest form,
the object pool, once we allocate an object and we are done using it, we can move it to some data structure instead
of freeing it. Then when we need another instance of the object we can just reset it and put it into play.
Occasionally, we might have to make sure the object pool doesn't grow too big and deallocate enough to just
hold on to a reasonable number of objects. This can save quite a lot of memory allocations and frees. The simplicity
of the object pool is allowed by the specific pool holding one specific type of object. All objects are the same
size and can be interpreted as (hopefully) valid instances of that object.

If we had to keep an object pool of every single type of object we might need, we would probably use a lot
of memory. A different approach, especially seen in game engines, is to implement your own custom memory allocator.
That memory allocator asks the operating system for a huge chunk of memory at a time and keeps track of them. Which
bytes are taken of the chunk and which are free. Then all other instances of memory allocation in the program will
be to the custom memory allocator. This is so much more complicated that in most cases, if you are a reader of this
guide, you should probably focus on some other stuff first... or see if there are other user space memory allocators
available to speed you up. One example is found in the Vulkan ecosystem. In that case the
[Vulkan Memory Allocator][0] can allocate one big chunk on the GPU and give the user a chunk at a time,
defragment and all that jazz.

Another way to reduce the amount of interactions you have with the operating system is to reduce the amount of
files you need to interact with at run time, this could just be about preferring bigger files or if you are doing
machine learning, preprocessing your data set into one big file and then writing a loader to handle this huge
merged file with as few file reads as possible. If you are on an HPC file system, it is likely a big performance
boost to have a few, or one, really big files, load them into memory and get your files from that mapped file
instead of having > 100 files and loading them individually.

## Hot Loops
Once you have built up enough features that you feel you have a working prototype, and you are ready to move on
to improve things on the performance side you should check your system monitor whether you are disk bound. Once
you have removed being disk bound, if you were that is, you should move on to finding your hot loop.

In most systems this is a while loop doing something until you terminate your program. You can verify this
with a profiler. A rule of thumb is, you should be doing as few interactions with the operating system as possible
in your hot loop. Are you allocating in side the loop? Could you just figure out what is the maximum size needed
and preallocate it before the loop? In the case of too much data in coming, you could decide that your containers
should net resize and decide whether to write over existing data or just drop incoming data.

You should also remember that printing to the terminal is a system call. Don't do it in a loop unless you really
really need to. Likewise reading from files should be minimized, or done asynchronously on other threads.
You also want to minimize synchronization. If you are sending lots of work to the GPU, don't wait and synchronize
after every interaction. Send a bunch of instructions, then either don't synchronize or wait until the last possible
moment. Remember, interacting with the GPU is also an asynchronous process.

Finally, try to reduce the amount of branching in your hot loop. Can you move any of the if-statements and checks
out of the loop and assume they are invariant inside the loop? Can you use any of the techniques in the branchless
programming section to improve performance? Once you have tried out these things, you might want to profile in
greater detail. Preferably with a view of which lines in the loop are the hottest, as in which lines take up
the most amount of time.

## Random Numbers
When creating systems, like in machine learning and path tracing, which are significantly dependent on
random number generation, I have two tips. First off, make sure you use seeds for testing correctness,
but also for performance reasons. Especially path tracing can have a changed workload based on the seed.
Generating the random numbers themselves can also result in a significant performance cost, in which case
it can make sense to look for faster methods for generation. Take special note to whether your generation
needs to be cryptographically secure, if it doesn't you can likely find a faster version, such as [this one][1]
or get a simpler version you can just copy paste into a file.

[0]: https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator
[1]: https://youtu.be/5_RAHZQCPjE
