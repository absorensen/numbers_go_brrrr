# Don't Panic
Hello there!

You've found a guide for learning about all the stuff needed to either program or reason about data-oriented
and real-time systems. It will help you with things like what memory allocations are, why using computational graphs
to program neural networks are a good idea, different concepts in parallelization of code, what types are,
how to profile and optimize code and how to create real-time systems. All running on your laptop!

To make things more complicated, because everyone loves complicated, there are some sections which are meant to
tailor to you, the reader! These specialization sections could for example have one set of tasks for people
interested in compute graphics and a different set of tasks for people interested in deep learning.
These sections are indicated by this DNA emoji - 🧬. You will find additional sections marked with 👨🏼‍💻.
These sections are meant as exercises or hand-ins for a course or if you want to explore the concepts
in practice.

I did a prototype course based on this material, which can be found [here][0]. The material was being written
during the course, so I wouldn't follow everything 1-to-1.

The material comprising the guide is divided into 7 modules.

* Intro to the course, Rust and wgpu.
* Memory hierarchies and computational graphs
* Concurrency
* Types, energy usage, bit tricks and compression
* Aspects of real time systems and additional components to use for your own projects
* Optimization with tips on profiling
* Project ideas for trying out the various tools you have learned throughout the material

So let's get started!

🌌 Queue Eric Idle singing while wearing a white wig 🌌

## Updates
The last major update was 20/03/24.
Recently updated features:

* A reworking of the project template ```egui-winit-wgpu-template```due to poor performance on Linux. It now
focuses more on stuff like logging, command line arguments and a bit of structure. Also added some exercises
to go with it.

The next features will likely be another template and a real-time case study/tutorial, followed by a full
write through (to add more memes).

## License
The content of this course is free to use under the Apache 2.0 license.
If you use parts of this course in your work, please cite using:

```bibtex
@misc{absorensen_the_guide,
    author       = {Anders Bo Sørensen},
    title        = {The Real-Timers Guide to the Computational Galaxy},
    howpublished = {\url{https://github.com/absorensen/the-guide}},
    year         = {2024}
}
```

[0]: https://absorensen.github.io/real-time-visual-and-machine-learning-systems/
