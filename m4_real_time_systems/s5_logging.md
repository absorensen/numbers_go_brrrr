# Logging
Logging is not really a hard concept. It's just a useful tool to be aware of.
You have a project in mind, you have made the first prototype, and you are now
ready to remove all of the print statements. But wait! You can do something else.
Use logging! Instead of printing all of your checks and what nots to the terminal
with commands like ```println!()```, you can use a logging crate, like the defacto
standard, [tracing][0], to print messages to a log file, which you can inspect once
your program is done running. In some cases this should always be running allowing
you to inspect the system after the fact. Logging also has some performance side
effects. First of all, anytime you print to the terminal, it involves a system call.
As you might recall, anytime we interact with the operating system, like printing
to a terminal, it is likely to be costly in terms of performance. If you have ever
timed a loop before and after putting in print statements, you'll know what I mean.

If the logging library has been implemented correctly (please don't implement it
yourself), it will be keeping most messages in memory, and then once in a while
writing a large chunk of text to the output file. Preferably it should do this
on another thread to not hold up the rest of the program. If the logging crate
is working correctly it should flush all remaining lines in memory to the file
once it is about to be dropped/go out of scope/be destroyed.

<figure markdown>
![Image](../figures/logging.png){ width="600" }
<figcaption>
Levels of logging.
<a href="https://skaftenicki.github.io/dtu_mlops/s4_debugging_and_logging/logging/">
Image credit </a>
</figcaption>
</figure>

Logging usually happens at various levels. Levels like debug, warning and error
allow you to filter which messages are presented at runtime, which are written to
the log file and, when inspecting the log file, you can filter which messages you
actually care about.

As an example, at runtime, you might want the logger to only print critical errors
to the terminal or to some other user facing display mechanism you have put in place,
but once your application is done, you might want to clean up your program by checking
that there are of course no errors, but fixing all the code that results in warnings.
Logging is definitely a useful tool in most cases, and the performance implications
are likely to be neglible, but just like printing, unless you really need it, you
probably shouldn't be calling it in a loop by default.

[0]: https://docs.rs/tracing/latest/tracing/
