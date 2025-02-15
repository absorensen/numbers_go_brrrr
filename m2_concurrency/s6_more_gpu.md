# More GPU
This section is about giving a bit of additional context around GPUs. It isn't a prerequisite for
understanding any of the material after it.

## The Graphics Pipeline
Originally, GPU's were made just for graphics. They were intiially quite closed in their
form, with a lot of the internal pipeline hidden away from the user.
The classic vertex and fragment shader setup can be seen below.

<figure markdown>
![Image](../figures/graphics_pipeline.png){ width="700" }
<figcaption>
A simplified view of the traditional vertex/fragment shader setup. The geometry shader
is optional and not that popular anymore.
<a href="https://learnopengl.com/Getting-started/Hello-Triangle">
Image credit </a>
</figcaption>
</figure>

The geometry shader is a later addition and has largely fallen out of favor due to
dynamically generating more data in a way that yielded quite poor performance. But.
If you look at the figure you can see that vertex data is passed in. This is
transferred from the GPU. This will be all of the corners of the triangles
that make up a mesh. Aside from the purely positional data, it can also have
attributes like the normal vector, the texture coordinates or the color. The vertices
are passed in and the vertex shader has access to one vertex at a time. Each
vertex is transformed from the objects own space into the world, view and projection
spaces. The vertices are sent along. A geometry shader could create additional vertices
and resulting triangles, with a view of more than one vertex. This is optional.
In shape assembly, vertices are assembled to full triangles, afterwhich the triangle
is rasterized into fragments. Think of these fragments as potential pixel candidates.
Each of them have values which are barycentric interpolations of the three vertices
they are constructed from. Each fragment is then sent to the fragment shader,
which can see a single fragment at a time. Lighting can then be calculated per fragment.
Importantly, it is also possible for the fragment to know where its potential position
is in the coordinate space of the rectangle to which it will be rendered. Once
lighting and other fragment related information has been calculated the fragment can
be sent further along where it will be tested whether the fragment is the fragment
closest to the viewpoint of the camera, or whether it might be behind another fragment
and can safely be discarded. The fragment shader could also write or read from textures
or buffers. Other shaders can do that too, but I'm building to something.

Additional shader types include mesh, task, raytracing and tesselation. Possibly more.
Additional behind the scenes processes are performed to transform the data, doing
things like clipping to determine whether a fragment is even in view. If it is not
it can safely be discarded. This is not programmable.

Why do you think it was important that the fragment in the fragment shader had access
to its coordinate on the output surface? Take a second to think about it.

The fragment shader is the precursor to the compute shader. Knowing where you are on
the 2D output surface is tantamount to finding your thread ID. Allowing the fragment
shader to write to buffers and not just send the fragment to be rendered allows
you to save the calculations. Once upon a time, scientists thought it would be kind
of nice if you could use the GPU, which is a massively parallel device, for scientific
computing. So they used fragment shaders in the early 2000's to do stuff like
matrix multiplication and FFT's. There were a number of research projects
in how to do GPGPU, eventually culminating in the official release of CUDA in 2007.
Nowadays compute shaders are a standard part of graphics APIs and workloads, with
an increasing amount of rendering pipelines using one or more compute shaders in their
pipelines. These compute shaders can happen at any point. In fact, in launching
the new Nanite system for Unreal Engine 5, one of the tech leads on the project,
Brian Karis, [revealed][2] that for select triangles, they would prevent them from
being hardware rasterized and instead write their own software based rasterizer
which could outperform hardware rasterization for very small triangles.

For a more thorough explanation of the graphics pipeline you can read this page
from [Learn OpenGL][3].

## SPIR-V & GLSL
WGSL is somewhat limited in what it can do. If you would like access to more features
or to have a greater codebase you can copy from directly, WGPU [supports][4] the GPU
intermediate representation SPIR-V. The shading languages GLSL, HLSL or Slang can all be
compiled to SPIR-V.

## Additional Levels in the Memory Hierarchy
Last time I introduced concepts in GPU memory to you, we had two options.
A thread could either write its data to global memory (VRAM) from which
other threads could read the data, or it could share data with other
threads in the same work group, by writing to the user programmable
shared memory, residing in the L1 cache (on Nvidia systems). There are two other options.

When sharing data through shared memory, the cost to access shared memory is 10 cycles,
or it was last time I checked. Work group shuffling, is sharing
directly between the registers of each thread in a work group. It was 1 cycle to
access last time I checked. This is immensely useful in reductions and prefix sums.
You would still need to write data to global memory in order to share data between
work groups. Unless you are using a GPU supporting [distributed shared memory][7].
At the time of writing, the Nvidia H100 is the only card supporting it.
Distributed shared memory allows accessing memory residing in the shared memory of
other work groups, as if it was one big shared memory. Hopefully, this feature
will make its way into other cards soon.

## Additional Reading
To learn more about the graphics pipeline you can check out [Learn OpenGL][0] or [Learn WGPU][1].

[0]: https://learnopengl.com/
[1]: https://sotrh.github.io/learn-wgpu/
[2]: https://www.youtube.com/watch?v=eviSykqSUUw
[3]: https://learnopengl.com/Getting-started/Hello-Triangle
[4]: https://docs.rs/wgpu/latest/wgpu/enum.ShaderSource.html#
[5]: https://github.com/gfx-rs/wgpu/tree/trunk/naga
[6]: https://developer.nvidia.com/blog/using-cuda-warp-level-primitives/
[7]: https://developer.nvidia.com/blog/nvidia-hopper-architecture-in-depth/
