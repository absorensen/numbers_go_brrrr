# Project Ideas
Sometimes you might just need a little inspiration.

## Speech Recognition System
Classify incoming speech audio as one of the digits 0-9.
There is a [free spoken digit dataset][1]. You can start with loading
that data one at a time. Once you have a nice solution, try classifying
in real-time with a microphone input.

If you are able to run that in real-time, can you add more classes?
Could you do real-time transcription or could you denoise the incoming
signal under worse acoustic conditions
(such as the speaker standing further away from the microphone)?

Alternatively, use a different type of data, such as classifying bird noises found in the
[xeno-canto database][8].

Another variant could be to classify and show, in real-time, out of 1 or more people on a web cam
feed, who is currently making noise?

## Real-Time Hot Dog/Real-Time Not Hot Dog
Run classification using a neural network in a loop on a web cam feed.
Choose a single class to focus on. Determine whether images contain the class you are focusing on.

While maintaining classification of the incoming images using model A, use user inputs for
ground truth classification of incoming images for fine tuning model B on another thread.
This user input could for example be -
"The space bar toggles whether there is a spider in the image or not".
Once you have fine tuned model B enough, maintain a copy of model B which you can keep training,
and switch out model A with your newly trained model B.

If you have more time, you can add another fine tuning stage by optimizing the fine-tuned model B
by introducing pruning and quantification.

## Style Transfer
Load two images. One representing incoming data and one representing the style to be transferred.
Use a neural network to perform style transfer, rendering and presenting a new image.
Implement a UI allowing you to load various images and tweak parameters.
Start using images from a web cam as your input for style transfer.
Optimize the process, first without changing the network, are you sure of your paralleism,
are you sure of your handling of memory?
Finally, what can you do to optimize the inference speed or quality of the neural network?

## Eye tracking
Present a web cam feed to screen.
Use feature extraction and a classifier to locate the eyes of a person reading and any additional
metrics you can extract.
Try to classify whether how focused, tired or distracted the reader might be.
Have the UI display when an intervention is needed.

Some inspiration - [Pistol][6], [saccades][7] and [Dr. Wolfgang Fuhl][5].

Alternatively, you can try to classify whether the reader is dyslexic or extract features and
visualize features of cognitive load. What happens if the system itself intervenes by playing sounds?

## Virtual 3D Scanner for a Point Cloud Dataset
Implement rendering of points.
Build up a 3D environment of points from a series of scans - [sample datasets][0], by
incrementally adding these new points into the scene representation.
Consolidate similar points. A suggestion would be binning through spatial hashing. If
points are within some distance of each other, consolidate them. Once you hit the
performance wall, try hierarchical spatial hashing (cells within cells).
Assume that the camera poses are correct and solved.
Allow the user to navigate the scene with camera controls.

## Point cloud renderer
Load a point cloud.
Render the points using primitives like GL_POINT or similar in a classic vertex-/fragment shader
pipeline.
Render the points to a buffer using compute shaders instead and use a fragment shader to render
the points to screen. How do you handle which points are closest in terms of depth? Which
type of synchronization might you need?
Could you preprocess the points to compress the data?
One way to start this project is to create a renderer using [LearnWGPU][9] and [WebGPU Fundamentals][10].

## Path tracer
Create an interactive GPU-based path tracer.
There are lots of GPU-based path tracers for inspiration on [Shadertoy][2].
Make sure there is a UI to allow the user to navigate the scene and edit settings.
Transfer the path tracer to a compute shader writing to a texture and a fragment shader reading and
presenting the texture. Try to implement wavefront path tracing or other advanced methods. Shadertoy
has more sources of inspiration, or you can check out [this chapter][3] from
Physically Based Rendering fourth edition.

## Rendering fractals influenced by sensor inputs
Render fractals in real-time. You can find inspiration with [Shadertoy][2] and [Inigo Quilez][4].
Identify key parts of the equations.
Create a GUI with which you can modify those key elements of the equations.
Start taking sensor inputs, either web cam or microphone, perform FFT to get the spectral
domain signal in real-time on the input and modify the fractal equations using the
spectral domain signal.

[0]: https://www.thinkautonomous.ai/blog/lidar-datasets/
[1]: https://github.com/Jakobovski/free-spoken-digit-dataset
[2]: https://www.shadertoy.com/
[3]: https://pbr-book.org/4ed/Wavefront_Rendering_on_GPUs
[4]: https://iquilezles.org/articles/
[5]: https://www.hci.uni-tuebingen.de/chair/team/wolfgang-fuhl
[6]: https://arxiv.org/abs/2201.06799
[7]: https://github.com/tmalsburg/saccades
[8]: https://xeno-canto.org/
[9]: https://sotrh.github.io/learn-wgpu/
[10]: https://webgpufundamentals.org/webgpu/lessons/webgpu-points.html
