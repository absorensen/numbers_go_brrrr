# Async
Async operations are basically non-blocking operations. They allow us to set multiple processes in motion instead of
waiting for each operation to finish, we get to decide when to wait for the results. We keep track of these
processes in flight by using something called futures to come back to the process.

If you went into the code from the memory hierachies module you will have already seen some async operations, which
was solely to interact with the GPU, as the WebGPU specification, on which WGPU is based, requires that all
interactions are async. This makes sense, as the CPU we are calling it from, does not have direct control of the
GPU, but makes requests. Synchronized execution is expensive. Later on, you might see async as part of GUI
programming.

Where async is most pervasive however, is when interacting with the internet. Getting stuff from the internet
is quite slow compared to getting things from RAM, so it makes good sense, if we are sending and receiving
lots of requests to make a note of what we did and come back later. This is the core of async. Make a note
of what you just asked something to do and either wait now (blocking) until that request has been completed
or come back at a later time to see whether your request has completed, perhaps even pick up the results and
send them somewhere else.

This makes async programming really shine in web servers, handling thousands of requests per second. We wouldn't
want to launch thousands of threads, our system would be massively oversubscribed, unless we had at least
hundreds of physical cores. Async and futures on the other hand are lightweight and can instead be executed by
a handful of threads. Scheduling and keeping track of when work needs to be performed on which futures, the note
that a request has been made and what will be the result, will require an async runtime. This runtime
can itself be relatively big, so you have to imagine a linear function with a bias of the size of the runtime
plus the amount of futures. Big runtime, small futures. Thus a big, if fast runtime, like the very
popular [tokio][4], only makes sense if we are in a minimum of 10's of requests per second.
Otherwise, use a lighter runtime or whatever comes with Rust by default.

_________________

## Async in Rust
Async is still relatively new in Rust and is likely to see significant changes. The documentation reflects that,
despite there being an async Rust book, it is not complete at the time of writing.

In order to call functions denoted ```async``` we either need to use a ```block_on(my_async_function())```
call in our synchronous code and the call, as surprising as that may be, blocks on the asynchronous
function call. An ```async``` function will return ```impl Future<Output = T>```. Basically, this means,
this is a future returning type ```T```... eventually! Within ```async``` functions we are free to call other async
functions without using ```block_on()```. Instead each call to an ```async``` function returns the ```Future``` I
mentioned earlier. You can either use ```.await``` immmediately like so
```let result: u8 = my_async_func().await;```, or you can store it and ```.await``` later, like so

=== "Rust"

    ```rust
        let future_result = my_async_func();
        let result: u8 = future_result.await;
    ```

If you think back to the earlier page ```m2::s1``` about threads, you can imagine the same scenario as threading.
You launch a bunch of jobs, store their handles, then when you are done launching jobs, you might even have some
other work to do in the mean time, you await all of your handles until you are ready to move on. But, allow
me to quote Rust's [async book][5] -

> The most common way to run a Future is to .await it. When .await is called on a Future, it will attempt to run it
> to completion. If the Future is blocked, it will yield control of the current thread. When more progress can be
> made, the Future will be picked up by the executor and will resume running, allowing the .await to resolve.

if a future, for example representing download of a file, in which case there may be multiple other factors than
just the system we are in control of, calling ```.await``` may result in the current thread yielding. Another thing
we could do is the ```join!()``` macro. This is sort of like calling ```.await``` on a bunch of futures at the same
time. Like so -

=== "Rust"

    ```rust
    let future_a = download_file_async(url_a);
    let future_b = download_file_async(url_b);
    let future_c = download_file_async(url_c);

    (file_a, file_b, file_c) = join!(future_a, future_b, future_c);

    println!("Successfully downloaded files from {}, {} and {}, url_a, url_b, url_c);
    ```

You can read more about [join!()][6]. Async is its own paradigm and
again, in this course you are most likely to see it when interacting with the GPU and a GUI system. In the
real world you might see it very pervasibely in web servers and anything to do with stuff that happens
outside of your computer. In any case, you should try to limit how big the async portions of your code are,
or very soon all of your code, including your main, will be async.

## Additional Reading
For an intro to the async functionality available in the core Rust library, there's a [book for that][0].

[Tokio][1] is a widely used async runtime. It is suggested, by Tokio itself, to not use it for cases
where you are CPU compute bound, they suggest Rayon in that case, to not use it for accessing lots of files as the
operating systems generally do not have asynchronous file APIs, and to not use it for single web requests. So, when
thinking about stuff like web servers, or something else making lots of web requests, Tokio might be the right tool.

[async-std][2] is a newer library which seeks to act as an async extension of Rust. It is also
mostly interesting if you need to handle lots of networking.

[Green Threads][3] are like virtualized threads, like threads emulated in software, to make them extremely lightweight.

[0]: https://rust-lang.github.io/async-book/
[1]: https://tokio.rs/
[2]: https://async.rs/
[3]: https://en.wikipedia.org/wiki/Green_thread
[4]: https://tokio.rs/
[5]: https://rust-lang.github.io/async-book/
[6]: https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html
