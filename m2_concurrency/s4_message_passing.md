# Message Passing
Now let's look at a different way of doing parallelism. What if we didn't share memory at all and just sent it
between threads? That would make things so much easier! We will do that with [message passing][2]!

Message passing is one of the easiest ways to share data between multiple threads. Message passing does not
not use locks but sends data from one thread to another. Whoever owns the data gets to use it.
Once that thread is done using it, it can send it to another thread. Synchronization through
ownership. Easy peasy.

It might also make things slower, but I will just assume that it is slower than the data parallelism, threads,
locks and atomics we just looked at. On the other hand it allows us to work simply while maintaining a very high
degree of freedom between threads, allowing us to take that to the extreme with [task parallelism][3].

So why is message passing easier to work with than the mutexes and atomics we just looked at? Until now, we have
either had a library like Rayon split some shared data into segments which each thread could consume or we have used
more or less complex locking mechanisms to synchronize which thread had access to which data. With message passing
we instead use some of the things we learned earlier about move semantics. We partition the data somehow and move
the data completely to whichever entity will use the data for processing. The processing thread can then return the
result through message passing as well. These messages could be anything, they could be a tuple of a command, such
such as "multiply all data by 2" along with the data to be processed.

This is quite a simple way of doing things and we don't have to worry about a single lock. The implementation takes
care of that. There are some other caveats, however. The specific version of message passing we will be looking at
is called ```channels``` in Rust. What happens is that the message is basically moved into a queue from which the receiver
can dequeue messages. The Rust's borrow checker would no longer need to worry about who or what owns what, as the
data is now fully owned by the receiving thread.
But what happens if the queue is full? We can have multiple threads enqueueing messages to
the same channel (queue), if the receiving thread is unable to dequeue fast enough, should the channel overwrite the
oldest messages in the queue to allow the transmitting threads to move on and no longer block on the transmission,
should the enqueued message be dropped/ignored or should the threads wanting to enqueue a message wait around until
space opens up in the channel? If you are designing an efficient system using message passing this is absolutely
something you should think about.

Two potential solutions are the synchronous and the asynchronous channels. With the asynchronous channel the
transmitting thread will send the message to the channel and then move on, as in not block. Remember blocking
behavior? The asynchronous channel then has two methods of handling message overflow. It can either resize the
queue (think back to dynamically sized arrays) or begin dropping messages. The synchronous channel on the other
hand requires the transmitter to wait until either the message has been successfully transmitted or received,
depending on the interpretation.

Another hazard is what happens if one side of the channel stops interacting with the channel?
If you imagine one thread holding a transmitter to the channel. It sends data every once in a while to be processed.
On the other end a thread might have nothing to do but receive messages from the channel and process them. If
the transmitting thread moves on and no longer transmits data to the channel, we need to be aware of that
possibility and handle it. It could either be the channel itself communicating that one end of the channel had been
dropped or it could be the transmission of an exit message. This only works if it is the transmitting thread
moving on. If the receiving thread will move on, or end, it cannot transmit an exit message, unless it has a way to
send an exit message to the transmitting thread. The channel itself handling whether both ends are still alive is
probably the way to go in most cases.

Another main use of message passing is sharing data between computers/nodes/servers. If you have
multiple computers working on the same problem, they don't have any physical memory to share,
so instead they can send messages to each other. This is the basis of the MPI standard.

_________________

## Rust Channels
The first ways to do message passing you will meet in Rust are the ```mpsc::channel``` and ```mpsc::sync_channel```
structs. ```mpsc``` stands for multiple producer, single consumer. When creating an ```mpsc::channel``` both a
sender and a receiver are returned. The sender can be cloned multiple times, to allow several threads to send
messages using the same channel, whereas there can only ever be one receiver.

[mpsc::channel][4] is the one most often used and is an asynchronous channel. It is what is known as
*unbounded* and if given too many messages will resize to accommodate. A transmitting thread will not block.

The [mpsc::sync_channel][5] on the other hand is *bounded* and won't change its size. A transmitting thread
will have to block until the message as been sent successfully. This might be best if you are running at high
speeds and can't drop packages. You could very quickly accummulate a massive amount of memory.

## Real-Time Message Passing
I made a code example for you. You can either go to [m2_concurrency::code::message_passing][6].

In this example I use Rust's ```mpsc::channel``` to get two pairs of senders/receivers. The main thread
generates some data and sends the ```Vec<f32>``` to a processing thread, which performs some in-place
computations and sends them back on another channel. The main thread can then receive the result and prints it
out. Note that each thread does not block and instead of using ```.rcv()```, which is a blocking receive, uses
```.try_rcv()``` and handles the case where there wasn't any new message to receive. In the case of no message
being received it also prints.

There are three values at the top -

=== "Rust"

    ```rust
        let max_work: usize = 100;
        let master_wait_time: u64 = 200;
        let worker_wait_time: u64 = 200;
    ```

Max work is the most amount of work messages will be processed, otherwise the program would run forever.
```master_wait_time``` is the amount of miliseconds the master thread will wait after sending a new
work message. The ```worker_wait_time``` is the amount of time the processing thread will wait before
attempting to receive another message.

Try and run the code, see what happens!

Now try and adjusting the wait times. How does it run when you reduce ```worker_wait_time```. Why do you think
that is?

## Additional Reading
Here's a longer, very friendly introduction to [channels and message passing][0] in Rust.

This is slightly larger scale, but when running code on multiple compute nodes in an HPC installation,
the most used method for sharing data between nodes is a standard called [MPI][1].

[0]: https://doc.rust-lang.org/book/ch16-02-message-passing.html
[1]: https://en.wikipedia.org/wiki/Message_Passing_Interface
[2]: https://en.wikipedia.org/wiki/Message_passing
[3]: https://en.wikipedia.org/wiki/Task_parallelism
[4]: https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
[5]: https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html
[6]: https://github.com/absorensen/numbers_go_brrrr/tree/main/m2_concurrency/code/message_passing
