# Concurrency
Ok, so in the memory hierarchies module we looked at parallelism in the form of GPU-parallelism. In many ways,
I find it to be an easier introduction to the concept of parallelism. The threads need to be tightly coordinated
and you need to make sure you have a clear model of which memory is available to all threads, which memory is
available inside a work group and which memory is exclusive to the single thread. Programming GPU's can be a bit
mind bending and as such I wanted to put it in early to allow you some time to get used to that way of thinking.

Parallelism and concurrency are often used interchangably, but they aren't necessarily the same.
[Concurrency][0] is when we run several calls at once, but they aren't necessarily running on two
different physical cores. This could for example be the downloading of several files at once. Things are
happening in the background, the process doesn't necessarily need to sit and wait for the first file
to download and then request the second file for download and so on. Instead it can ask to download
all of the files and then wait for all of them to be done, or for the first one to be done so it can
begin processing the files.

Parallelism on the other hand, implies that we are actually running different cores and threads.
So far, I have introduced parallelism in small pockets inside a function which cannot do anything
too complicated. The programs aren't long running and we choose a specific subset of problems to
use the GPU for. In this module, I'll mainly introduce you to CPU-based parallelism with different
mechanisms. In creating longer running CPU-based parallel programs you will likely need to combine
a bunch of these mechanisms along with your accrued knowledge of data races, as enforced by the
borrow checker in Rust. Additionally, I will introduce a few more concepts in GPU programming in ```m2::s6```.

Anyways, why do we need parallelism in CPU's? Eventually, the clock frequencies, as in how many times
per second a processor can do something, more or less flattened out. We get increased performance by
either doing things in a smarter way or by increasing the amount of processors, either through
a massive amount of parallelism in an accelerator, such as a GPU or through adding more processors.

But parallel programming and parallel-friendly algorithms put a much greater cognitive strain on
you, the programmer. The more you learn about parallel programming, the more you will see that
the basic components are actually quite simple. The strain lies in thinking about
parallelism and who owns what memory at which time. This is critical in not just getting
faster programs, but retaining the correctness of your program from before you started parallelizing it.

## Algorithms and Systems Design

<figure markdown>
![Image](../figures/AmdahlsLaw.svg){ width="500" }
<figcaption>
Amdahl's Law
<a href="https://en.wikipedia.org/wiki/Amdahl%27s_law">
Image credit</a>.
</figcaption>
</figure>

[Amdahl's Law][1] is a fundamental concept in parallelism. You should just skim the link, but the concept is
very simple. If 90% of your program is infinitely parallelizable, you will still be left with a runtime of
10% of the original runtime - if you take parallelization to the absolute limit. But how do you actually
gauge which parts of your system are parallelizable? The answer is quite frustrating.

??? success "The Answer"

    *It depends.*
    It depends on what type of algorithms are in play in your system, what sort of hardware platform
    you are running on, it depends on what amount of development time and skill you have available.

Sometimes when you think about optimizing your code you might visualize it as explosions and
speed, flamethrowers and excess!

<figure markdown>
![Image](../figures/doof-warrior-from-mad-max.jpg){ width="600" }
<figcaption>
Witness Concurrency!
<a href="https://www.ultimate-guitar.com/news/community_feed/mad_max_doof_warrior_inspired_flamethrower_ukulele.html">
Image credit</a>.
</figcaption>
</figure>

But in actuality, working with parallelism takes restraint and consideration. Like a watchmaker placing
tiny gears with a pincette. If we look back at the way we constructed computational graphs in
```m1```, we were able to parallelize internally in each node/operator, but if we had very small
matrices with a big depth, we would more or less be unable to do any parallelization, as the launching
of threads to parallelize the matrices themselves, might cost more than simply having a single thread just
handle the whole thing.

Some elements in your system you might be able to parallelize lock free, wherein you find a solution
without needing synchronization primitives like scopes, barriers, locks or mutexes. You might get
away with having no synchronization, as the shared data might be read-only or you might have to use a
simple synchronization mechanism with hardware support like atomics. Some parts of your system might be
amenable to fine-grained parallelism, such as a matrix multiplication, whereas other parts might only be
amenable to coarse grained parallelism, such as a SLAM system pipelined into 4 stages, thus only being
able to utilize 4 long running threads.

All of these put one thing into the center of everything. Can you guess it?

??? success "The Answer"

    *Memory!*

    Some ways of accessing memory can seem completely fine when single threaded, but break down under the scrutiny
    of parallization. Trees can be hard, especially if you also have to modify them. As one researcher found it,
    a [hierarchical hash map][https://www.researchgate.net/publication/354065094_Practical_Spatial_Hash_Map_Updates]
    performed siginifcantly better for some types of algorithms on the GPU.

Once you have the correct CPU based implementation, you should start asking yourself, where is this going to
run and how is the memory accessed in order to accomplish what I want to do?

## Here Be Dragons
Some of the things you have to get used to in concurrency programming is the sudden lack of serialized
interactions. Thread 1 won't necessarily execute before thread 8, and the way you debug and verify your code
will have to take that into account.

Along the way, you will encounter a number of hazards. Especially race hazards are prevalent. The race condition
happens when at least one thread is writing while one or more are writing or reading. Typically, these types
of bugs can be very hard to find due to some part of your code being serialized once you try to find the bug or
due to the multithreading, the execution might be non-deterministic.

Take a few minutes to familiarize yourself with race conditions in software and data races [here][3].

## Platforms
When you decide you want to parallelize your application, you almost always have to consider the platform you
will be running on. Do you expect to run it on users' laptops, will you have GPUs available, will it be running
in a data center with powerful, very expensive GPUs, will you be using an integrated GPU on a laptop, will
it run on the edge or in the cloud? I will assume you are running on your own laptop or desktop for following
along, but running on multiple data center GPUs seems to be all the rage these days, so I will keep that in mind.

## Rust and Concurrency
Each language has its own implementations of concepts in concurrency, but I will focus on showing you the ones
in Rust and WGSL. All of them exist in other languages, but some may for example be more ergonomic to work with
for concepts like ```async``` or ```channels```. What the other languages do not have is the borrow checker to
ensure the validity of your code. Often this results in parallelized Rust code looking or feeling slightly
different, as the borrow checker forces you down a certain path. Also Rust has traits, such as ```Send```
and ```Sync```, but these are specific to Rust and I have tried to avoid getting too far into traits,
so I won't be explaining them. If interested you are most welcome to read about them [here][4].
This is mostly relevant if implementing your own types which need to be shared by threads. In most
cases, ```Send``` and ```Sync``` are automatically derived.

[0]: https://en.wikipedia.org/wiki/Concurrency_(computer_science)
[1]: https://en.wikipedia.org/wiki/Amdahl%27s_law
[3]: https://en.wikipedia.org/wiki/Race_condition
[4]: https://doc.rust-lang.org/nomicon/send-and-sync.html
