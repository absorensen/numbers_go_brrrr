# 👨🏼‍💻 Projects
A project is a good way to put in to practice all of the various tools, methods and concepts
we have looked at previously. This section will get into what makes for a good project.

This is where you get to play!

You can either write your own proposal, which is hopefully relevant to what you normally work
with/are interested in, or you can get some inspiration from [the list][14].

## What makes for a good real-time project?
Below you will find a few questions to ask yourself when determining either how to write your own
proposal or picking a pre-existing one.

* Which concepts from the previously seen material do you think are relevant to your project and why?
* Can the system eventually (end goal) take in sensor data, such as images from a camera or sound
from a microphone, and keep a real-time processing rate?
* Can the system be manipulated by a user through a GUI?
* Can the system be optimized by preprocessing your data?
* How do you adapt to your chosen/available platform?
* Which libraries are available for this problem?
* How fast can you get to your minimum viable product?
* Which steps would you take from there and why?
* How did you determine which parts of your system to optimize?
* Is there room to add more features to your system?

## How to create real time systems

* Start with a simple prototype.
* Identify your components.
* Create your first prototype as a single threaded correct implementation.
* Start with simple, reproducible data. If you are working with sound, load a single file.
* Implement testing (preferably automated) to avoid any regressions. Regressions are likely to occur
when optimizing.
* Start ingesting real-time data from a sensor or getting user interaction.
* Profile and optimize in a loop.
* Optionally, add more features and optimize.
* Optionally, if you are using machine learning, is there anything you could do to optmize your
model as a preprocessing step?

## Components
Given how easy it is to include libraries in Rust projects, Rust has a relatively small standard
library. Instead a de-facto standard library emerges through a pseudo-democratic process driven
by usage statistics and GitHub stars. A list of these de-facto standard crates can be
found on the [blessed][0] website.

Additionally, there's a set of overview sites tracking the state of Rust's nascent ecosystem.
These have the "Arewe____yet" form. Examples like [Arewelearningyet][1] can also be found
in [AreWeRustYet][2].

An additional list of libraries and frameworks you might find useful for doing a project -  
[rayon][3]  
[ultraviolet][9]  
[egui][4]  
[winit][7]  
[wonnx][5]  
[tch][6]  
[burn][10]  
[cv][8]  
[time][11]  
[chrono][12]  
[hifitime][13]  

[0]: https://blessed.rs/crates
[1]: https://www.arewelearningyet.com/
[2]: https://github.com/UgurcanAkkok/AreWeRustYet
[3]: https://github.com/rayon-rs/rayon
[4]: https://github.com/emilk/egui
[5]: https://github.com/webonnx/wonnx
[6]: https://github.com/LaurentMazare/tch-rs
[7]: https://github.com/rust-windowing/winit
[8]: https://github.com/rust-cv/cv
[9]: https://github.com/fu5ha/ultraviolet
[10]: https://github.com/burn-rs/burn
[11]: https://doc.rust-lang.org/std/time/index.html
[12]: https://docs.rs/chrono/latest/chrono/
[13]: https://github.com/nyx-space/hifitime
[14]: https://absorensen.github.io/the-guide/m6_projects/s0_project_ideas
