# Events and Interactions
[Events][0] are, well, events. Something happens, and the event is processed somehow, by one or more actors,
reacting to that event. Usually, events are asynchronous in origin. Distributed architectures can also be
based on events with different elements reacting to, and generating their own, events. There is even a type of systems
architecture based around events, called [event driven architecture][1], which I won't be going into at all. I
will be focusing on events in the context of the kind of real-time systems you can easily make for your own
projects.

If we import some event library, we could generate events. An example could be every time a user clicks the
left mouse button an event could be emitted. Subsequently, any actor subscribing to that event would receive
the signal that that event had happened. Then each actor might have their own way of handling such an event.
Multiple actors can be subscribed to the same event, sort of like the reverse of the multiple producer,
single consumer channel we looked at earlier. A single producer for an event, with potentially multiple
not consumers, but listeners. This is a very reactive way of handling events. Essentially, every time you
subscribed to an event, such as the left mouse button click, you would supply a callback function to be
called, whenever that event was fired. Say you have 3 components, ```A```, ```B``` and ```C```. Component
```A``` generates some type of event ```a```. In order to react to event ```a```, components ```B``` and
```C``` each subscribe to all occurences of event ```a```. ```B``` and ```C``` each supply their own callback
to handle whenever event ```a``` is emitted.

This can be quite complicated, especially if you don't really need it and you might find yourself swimming in
a hard-to-debug sea of callbacks and asynchronicity. How many components did you sign up for callbacks and
what are the side effects of all those callbacks? If all of this is handled in a single threaded manner,
isn't the order in which your code is executed getting yanked around quite a lot? See the branchless
programming section for more on how the order of execution can improve performance.

Often you can define your own events, or a framework you might be using, such as [winit][2] for window handling,
might have its own version of events. As such it can be hard to say anything specifically about events, except
that it depends. What framework you use will define how you have to work with events. Events are usually integral
to interactive systems, if you do not have interactivity, you might want to stick to async.

See the event loop section below for a much easier way of handling events, which is easier when you have a smaller
amount of components.

# Interactions
Interactivity can be a major source of events, especially in the types of systems I'll focus on here.
You might get emitted events every time you hover a mouse over something, moving your mouse, typing on your
keyboard, etc. It could also be an external sensor communicating that it is ready with a new sample, such as
a webcam. It might send an event everytime it has another image for you to process, or a microphone might
have another audio signal ready. When these types of events come in, which have a substantial amount of associated
data, you'll have to carefully take into account where that data is coming in and how to handle it.

In the case of a webcam, hopefully that data is coming in very close to the method with which you will process it.
If you are using wgpu for instance, you might hope that the incoming images are given to you in WGPU-compatible
buffers allowing you to not have to transfer, manipulate or copy the data, either to another buffer or back to the CPU.

# Event Loops
An easier way of handling events, compared to emitters and listeners is an event loop. The event loop will handle
all incoming events and how to distribute them. It allows the event loop to direct the flow of the program directly.

In practice, a useable first event loop could just be a while loop with a match statement inside. This can be a
bit of an ugly match statement with lots of cases and sending data or commands to other components for them to
react to the incoming events.

In the example below, the event loop will instead be used. Every iteration of the loop, any and all events
will be handled and for each type of event a branch will handle what to do given that event. In the case
of the example for later on is that given a event for a mouse button press, the event handler might find out
which window the mouse was hovering over and which event handler should receive a signal.
This is a lot easier to debug and follow with a clear separation of flow and state.

I made some code for showing how you can work with events in practice. You can find the code in
[```m4_real_time_systems::code::egui-winit-wgpu-template```][3]. Note that the events stem from
Rust's more-or-less defacto window handling library, [winit][2]. Try and make sense of what is happening
and run the code!

When handling events like this, it can sometimes be worth it to ensure that a user holding down a button
doesn't generate thousands of events, but a couple of events in order to not completely flood your system. If you
are using bounded channels to distribute your events, you might also incur the risk of overflowing your channels.

Is that acceptable behavior in the case of taking in mouse drag events?

[0]: https://en.wikipedia.org/wiki/Event_(computing)
[1]: https://en.wikipedia.org/wiki/Event-driven_architecture
[2]: https://github.com/rust-windowing/winit
[3]: https://github.com/absorensen/numbers_go_brrrr/tree/main/m4_real_time_systems/code/egui-winit-wgpu-template
