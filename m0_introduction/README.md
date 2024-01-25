# Introduction
Hello there!  
If you are reading this you might have been enticed by promises of performance and other
some such advanced black magic, but first, a digression...

There are so many things to keep track of as a modern day programmer and most systems hide these things from
the user. You call something called... ```?numba?``` and annotate a few functions and it magically makes your
code faster. You use something called ```?Py?...?Torch?...``` and it seems really slow for some reason.
You're sure you have a GPU in your machine, but it's still slow.
```PyTorch``` has something called a profiler, but you don't know how it works and you don't
know what the hell ```DtoHMemCpy``` even is.
It can be hard to reason about what is going on inside these black boxes.
On top of that you might not be able to find anything to walk you through all of the
stuff you don't know that you don't know.
As scary as it can sometimes seem to get your hands dirty and take on what might
seem an insurmountable obstacle, not doing so can have consequences.

The speed of execution of a program is approximately correlated to the energy consumption of that program.
Until we use 100% green, renewable, energy in all of computing we have a shared responsibility to at the very least
practice some modicum of restraint and sensibility in our resource consumption. Taken to the limit by putting
large scale machine learning, with its [massive energy consumption][0] for both training and inference, in
everything without necessarily generating value comensurate to the expended resources is an
[irresponsible use of resources][1].

<figure markdown>
![Image](../figures/chatgpt-is-so.jpg){ width="500" }
<figcaption>
<a href="https://makeameme.org/meme/chatgpt-is-so"> Image credit </a>
</figcaption>
</figure>

If someone trains a deep learning model for two weeks on eight huge data center GPUs in a cluster, it is their
responsibility that that training process is fairly well optimized, and that all data is responsibly retrieved,
such that that training does not have to run again because of sloppiness.

And thus stops the finger pointing!

Optimizing code, especially on systems you might share with others both means that you can get your results faster,
but that others can have use of the system in a reasonable time as well.
If you are creating large models, optimizing them to be smaller or more efficient also results in entities with profits less
than the GDP of a small nation actually being able to train and run inference with your model,
increasing the democratization of your work and its reach. If its able to run on a consumer grade desktop - even better!

This guide was made to be an iterative process, taking you by the hand,
speaking to you at the level at which you are following it, trying not to overwhelm you.
Reading that back, it could sound a bit condescending, but it basically means that the
types of concepts you are assumed to know about will gradually increase.  
Due to the guide gradually introducing certain concepts, jumping around the material
is not recommended.
I also acknowledge that some people have different interests.  
As such, some portions of the guide will be tailored to people who are into deep learning,
people who like computer vision, or computer graphics or other some such.  
You are more than welcome to read and do all of it, but no one says you have to do anything.  
If you just follow along the path that is most relevant to you, you will be just fine.  
The guide does contain code, sometimes just small snippets, but also frameworks in
which most of a module will take place.

Most importantly - Don't Panic! The guide is here for you!

And now for something completely different... practicalities!

# 🧬 Specializations
Throughout the guide there are sections and exercises prefixed by 🧬. These exercises and topics
are meant to be up to you to follow. If you are mostly interested in deep learning, by all means
only read and do the sections and exercises which are relevant to deep learning. Which section and
exercise is relevant to which specialization will be explained in each section.

# Additional Reading
The secret bonus level! It is in no way expected that you read all of the links in additional reading,
but if you find a topic interesting some additional links to help you along are present in
the "Additional Reading" sections at the bottom of select pages.
The references here will generally be more advanced. Typically these references will be to book
chapters, university courses, scientific papers and in-depth blog posts. In a lot of cases
they are also the basis for the content in the section above it.

# 👨🏼‍💻 Exercises and Teaching
!!! warning "WSL"
    
    I recommend you do not use WSL on Windows. It makes GUI and GPU programming difficult.

Sections indicated by 👨🏼‍💻 are recommended exercises if you want to really learn the topics in-depth or if you
are following a course based on the material.

If you are a teacher who wants to make use of this material, feel free to use the [course site][2]
for inspiration.
The course focuses on teaching real-time systems for deep learners and visual systems programmers.  
It allocates half of 15 days to go through the material in the guide and the
other half to making a project relevant to the students' specialization, which is the [contents of m6][3].  
It is designed to give shallow introductions to a wide range of concepts with a
handful of varying exercises.
The student is to then reach back and explore the topics relevant to their specialized
project in greater detail.
The breadth of topics is quite wide, and each student shouldn't be expected to
pass an exam in every topic.
In most cases they might remember that a thing exists and that they can search for it.
The thing they hopefully all learn is how to reason about performance, systems
design and that getting your hands dirty can be both fun and motivating.

It is important to note that the material and exercises have been made with a range of potential students
in mind. From having taught themselves Python to experienced C++ programmers, from specialized in computer
graphics to being a deep learning PhD. There is a range in everything, including the exercises and
hand-ins. As such it is very important to specify to students which parts of the exercises and hand-ins
they are expected to do and which parts they are invited to do if they find it interesting.

[0]: https://towardsdatascience.com/chatgpts-electricity-consumption-7873483feac4
[1]: https://towardsdatascience.com/environmental-impact-of-ubiquitous-generative-ai-9e061bac6800
[2]: https://absorensen.github.io/real-time-visual-and-machine-learning-systems/
[3]: https://absorensen.github.io/the-guide/m6_projects/
