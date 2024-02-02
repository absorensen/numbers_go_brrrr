# Graphics in Two Dimensions
In most cases, you can probably get away with some fairly rudimentary code to create, draw and interact
with some buttons for your application. But, in this context, we care about performance. It's simple to
redraw every single button, every single frame, but it isn't necessarily performant. As such, we have to
straddle the line between knowing enough about computer graphics to reason about the performance of GUI's
in interactive systems and not explaining so much that this become a full-on graphics course.

So, for this gentle introduction to computer graphics, I will try and constrain things to mainly deal with
geometry and 2D rendering without any form of lighting. In the end I look at what this means in the context
of an immediate mode GUI library like [egui][2] and the accompanying app framework [eframe][3].

## What's in a triangle?
Most of what can be seen on screen is made of two primitives. Lines and images. Lines are a sparse representation,
whereas an image is dense representation. Lines, aside from their sparsity, have the added side effect that they
are scalable. If we zoom in on an image, densely represented as a two dimensional array of colors, we eventually
zoom in to a point where the image doesn't look very nice anymore. If the image was compressed with a lossy
format such as JPEG, we will eventually also become very aware of unnatural compression artifacts. Lines on the
other hand, can be zoomed in on in perpetuity. This scaling is exemplified by vector graphics formats like PDF and
SVG. There's also other representations such as NURBS, splines and voxels, but they are out of scope.

Back in the before time where GPU's were really just for graphics... GPU's were made for rendering triangles.
Lots of them. All the time. Triangles are conceptually simple and sparse in nature and we can infer lots of
information from them. GPU's also have helpful hardware support for turning triangles into a bunch of fragments
covering the area of the triangle, while matching the resolution of the output image. GPU's are good at drawing
straight (not curved!!!) lines and images. Which we will get back to later on, when I take you through what goes
into rendering a UI.

The most commonly used definition of a triangle is three vertices and three edges. The vertices are the
corners of the triangles and the edges are the lines between them. One key observation can be made though.
We can define the triangle instead by an ordering. If we define just the three vertices, we have also
defined a triangle because we implicitly assume that all three vertices are in the same triangle. However,
in graphics and geometry, it is really important to know which direction of a triangle's surface is
facing outwards. What is commonly seen is that triangles have their faces defined by the ordering of the
vertices and whether the face is constructed in clockwise or counterclockwise fashion. You might ask
yourself why this is important. For one, this allows us to determine whether we are inside
or outside a surface/volume and it allows us to not render the backface of triangles. Without it we would have to
render each triangle twice as we aren't allowed to assume the two faces match. If you think about something like
the impact of lighting, having the same lighting whether something was facing towards or away from a light source,
would be completely inconsistent with the world we know.

<figure markdown>
![Image](../figures/interpolated_colors.png){ width="600" }
<figcaption>
Each vertex is given a color. Each fragment has its color interpolated by the GPU.
<a href="https://en.wikipedia.org/wiki/Barycentric_coordinate_system">
Image credit </a>
</figcaption>
</figure>

Triangles can have auxiliary data like normals, UV coordinates, color, material (might be defined per draw call)
and anything else you might think of. This auxiliary data is most often defined per vertex and interpolated between
to generate smooth values across the triangle face. Normals are directional vectors representing which way a
surface is pointing. If we give vertices A, B and C their own normal each, despite a vertex not have a real normal,
most GPU's would find the barycentric coordinates of a given fragment of the triangle face and use it to construct
a new normal.

<figure markdown>
![Image](../figures/interpolated_normals.jpg){ width="600" }
<figcaption>
Phong shading, using interpolated normals to change the lighting per fragment.
<a href="https://en.wikipedia.org/wiki/Phong_shading">
Image credit </a>
</figcaption>
</figure>

Barycentric coordinates means finding a given position in the triangle's own 3D space where all values
are between 0 and 1 and the sum of the three values is always 1.

<figure markdown>
![Image](../figures/barycentric_coordinates.png){ width="600" }
<figcaption>
Barycentric coordinates for triangles.
<a href="https://en.wikipedia.org/wiki/Barycentric_coordinate_system">
Image credit </a>
</figcaption>
</figure>

UV coordinates are coordinates into a texture (image) which allows us to wrap a bunch of triangles
(mesh) in an image. You can also render a simple rectangle, which is just two triangles, and have
its surfaces colors be from a texture. For example, we might like to render a 2D button in a 3D world.
In which case we might render a 3D rectangle and use UV coordinates to look up values in a texture.

## What's in a series of triangles?
Usually when we deal with more than a handful of triangles we are talking meshes. Or more accurately,
[polygon meshes][9]. We have a bunch of vertices and edges, which make up a lot of faces, which share a border.
Note that a mesh isn't necessarily completely connected, you can have islands of smaller meshes floating around
somewhere. This is usually not a good thing though. Especially if those outliers are far from the main attraction.
Generally, these meshes aren't just randomly interconnected but make up some nice surfaces wherein faces don't
cross. Not having faces which cross is part of what can make a mesh [manifold][10] or non-manifold. From this
point on, just assume that I am always talking about manifold polygonal meshes.

Once we move away from merely describing handfuls of triangles we should, as we always should when scaling things,
make sure that we don't naively reuse too much information which could either be described once, or inferred.
Part of this optimization is performed by recognizing that in a mesh, a lot of vertices occur as part of more than
one face. In fact, in most meshes a vertex will occur in an average of six faces. A vertex is likely to cost
anywhere between 2 or 3 times 16- to 32-bits, depending on whether the mesh is 2D or 3D. 6 entries of the same
vertex would cost us 196- to 576-bits. Where as if we instead used a 16-bit integer to represent an index into
a list of vertices, we could describe the same vertex six times with 32 to 96 bits for the core description, and
72 bits for the indices, for a total of 104 to 168 bits, saving us 1.88x to 3.42x depending on the dimensionality
of the data. The savings will of course be greater if we also have auxiliary data for our geometry, such as
normals, UV coordinates and color. What we gain for austerity is a layer of indirection, which is why, when we
call a graphics API to draw triangles we can draw them with or without indices. Usually there will be a function
called ```draw()``` and ```draw_indexed()```.

The ```draw()``` function will work from a vertex buffer, which is just an array of all vertices and their
attributes. Before usage, how to interpret the vertex buffer(s) has to be specified. These attributes can either be
interleaved, as in ```x0y0z0r0g0b0x1y1z1r1g1b1``` or be split by type
```x0x1..xNy0y1..yNz0z1..zNr0r1..and..so..on..```. The last one can be advantageous for compressing our data.
When using ```draw_indexed()``` we have another buffer, called the index buffer, which indexes into the vertex
buffer. Naively, we could simply interpret every three vertex indices as being a triangle face. As I wrote earlier,
this would be interpreted either in clockwise or counterclockwise fashion. But, depending on the GPU API you are
using a number of different primitive interpretations might be available.

<figure markdown>
![Image](../figures/geometric_primitives.png){ width="600" }
<figcaption>
Geometric primitives in OpenGL.
<a href="https://www3.ntu.edu.sg/home/ehchua/programming/opengl/CG_Introduction.html">
Image credit </a>
</figcaption>
</figure>

The point primitive is sometimes available. It will usually draw a quad of a size you have to set per vertex. The
most used are usually lines, triangles and triangle strips. When we know we have a contiguous set of triangles, we
can save a lot of indices by using a moving window interpretation for the faces. Face 0 will be vertex 0, 1 and 2,
face 1 will be vertex 1, 2 and 3 and so on. To stop the strip and start a new one, you can usually make the
strip degenerate by repeating a vertex. So making face 1 be vertex 1, 2 and 2, would make it clear that a new
triangle strip needs to be started. Sometimes to maximize numerical accuracy or to help with the quantization
process, we might create a model matrix for the mesh and normalize all coordinates to maximally touch the
surface of a unit sphere and then quantize to lower precision integers.

## What's in drawing a series of triangles?
Now we have a mesh to draw. How do we draw it? First of all, just like when we looked at the more generic
GPGPU programming in ```m1```, any time we want data on the GPU, we have to allocate and transfer. For vertex
data, it is the exact same. We have to allocate a buffer on the GPU and transfer data from the CPU in a format
that works for the GPU. If you update the vertex data you should reuse the buffer to avoid another allocation
or use a handful of buffers you can switch between. If you want to redraw the same model, don't reallocate
and retransfer, just reuse. If you can avoid any allocations and transfers, you should.

Once the vertex buffer has been properly described, allocated, transferred and received, we can start using
it on the GPU. There are more possible variations of the drawing process, but here is a basic version from
[earlier][11] -

<figure markdown>
![Image](../figures/graphics_pipeline.png){ width="700" }
<figcaption>
A simplified view of the traditional vertex/fragment shader setup. The geometry shader
is optional and not that popular anymore.
<a href="https://learnopengl.com/Getting-started/Hello-Triangle">
Image credit </a>
</figcaption>
</figure>

When we call the correct draw function with the properly created and bound render pipeline, a traditional
vertex and fragment shading pipeline might look like above, minus the geometry shader, which has fallen out
of favor. So, we send all of the data we want to draw through the vertex shader. Each instance of the vertex
shader gets a single vertex to process. Often it might have global associated data, such as various transformation
matrices which can be applied to each vertex, such as taking into account moving the mesh around in world space
or taking into account where the camera actually is. If you have quantized coordinates, such as 16-bit
signed integers, this would also typically be where you would dequantize these values and get floating point
coordinates.

Once the vertices are through being processed in the vertex shader, they are sent to shape assembly. This
is where the geometry primitives result in a shape, such as a line or triangle, is assembled. Once the
shape is assembled, culling can take place. If a triangle is completely outside the field of view of the camera,
there is no need to use more compute on it and it can be discarded. If it is partially or wholly within the view
frustum (box which contains all the camera can see), the primitive can be rasterized. The surface of the primitive
is sliced and diced into fragments. For primitives only partially within view, the fragments what are wholly outside
can safely be discarded. This is called clipping.

The surviving fragments are sent to the fragment shader. In the fragment shader, you can do all sorts of coloring
and lighting. But we can also change the depth of the fragment (more on that in a few lines) and even discard the
fragment. This can be really useful if we want to draw circles (also more on that later). Meshes are really
expensive if you want to draw a nice round circle. You have to generate enough vertices to match the resolution
of the image to get something that might look like a real circle. What you can do instead, is to draw circles
as a single quad (two triangles) and keep track of the center point and radius. For each fragment you can
discard the fragment from the quad if it is outside of the radius. Voila. You just drew a much cheaper circle.
This could also be used for drawing rounded corners on otherwise square windows.

Once we have done what we needed with the fragments they are sent for testing and blending. In traditional
rasterization based graphics, we render to a framebuffer. A framebuffer consists of an image in one or more
colors, a depth buffer and auxiliary information. The depth buffer is a log compressed image containing depth
values. Fragments which are passed on from the fragment shader are tested against this depth buffer.
Depending on how you set up your system, it will discard or accept new fragments replacing the currently
held fragment. A small example; if we submit a new fragment for pixel (3, 4) with depth value 0.1, and
the currently closest (to the camera) fragment has a depth value of 0.3, the new fragment will replace
the current one. As such, the depth value will be updated and the other image, which is the one which
we will eventually present to the screen, will have pixel (3, 4) replaced with this new fragment as well.
There is a slight bit of complexity to this as fragments can have an alpha value, which determines its opacity.
If alpha blending is enabled, the new fragment might be mixed in with the current value. If you think about
the various windows in your operating system, quite a few can be made transparent. Which requires that we don't
completely overwrite former values, but blend in the new ones. This also requires that we sort all the elements
we are rendering from back to front in what is called [Painter's algorithm][12]. When we aren't doing
transparency, the reverse is more efficient. Writes are generally more expensive than reads, and that is
also true in this process of proposing new fragments and testing them against the depth buffer. If we could
only write to every single pixel once, and then have all other fragments rejected, while still getting the correct
image, that would be quite a bit more efficient. This goes back to contention which was introduced in both
```m1``` and ```m2```.

Once all elements have been rendered, the image part of the framebuffer can be presented to the screen.
The framebuffer used in the last frame can be used to render the next frame. At the beginning of that
render process, the new framebuffer will have its former values cleared.

One rendering concept which has largely been ignored for simplicity's sake, is coordinate spaces.
The geometry we put into our vertex buffers at the beginning are of course not defined relative
to the world it exists in. It is defined independently. To move it about, we don't change the
value of every single vertex, we change a model matrix which moves the model about in the world.
Usually, that world space will be centered around the camera. When we move the camera throughout
the world, we are actually moving the world and not the camera.

<figure markdown>
![Image](../figures/spoon-boy.jpg){ width="700" }
<figcaption>
We just have to realize that there is no spoon.
<a href="https://www.matrixfans.net/there-is-no-spoon-spoon-boy-actor-rowan-witt/">
Image credit </a>
</figcaption>
</figure>

If you think back to ```m3``` and what I wrote about the precision of floating point
numbers, this should make sense. By keeping the world centered around the camera and moving everything
else, the elements that are close to the camera (0.0) have a higher numerical precision and the things
that are far away have a lower numerical precision. To get from the world space to the cameras space,
also known as view space, another matrix is involved; the view matrix.

<figure markdown>
![Image](../figures/coordinate_systems.png){ width="700" }
<figcaption>
LearnOpenGL's explanation of the coordinate spaces in the vertex/fragment render pipeline.
<a href="https://learnopengl.com/Getting-started/Coordinate-Systems">
Image credit </a>
</figcaption>
</figure>

Now we are seeing things from the cameras perspective and we can take into account the projection
of the camera with the projection matrix. Now we should be in clip space, which resides in 2D from
-1 to 1 for both axes. Once the primitives and have been culled, clipped and rasterized, we are in
normalized device coordinates. In GUI libraries you might see something called NDC. This is it.
If you are rendering GUI elements, which are always in the same place, you can circumvent all of
these matrices and define your geometry directly in normalized device coordinates. Once the fragments
are passed through the fragment shader they are moved into screen space which translates them directly
to the actual pixels they correspond to.

<figure markdown>
![Image](../figures/vertex-transform-pipeline.png){ width="600" }
<figcaption>
Scratchapixel's explanation of the coordinate spaces in the vertex/fragment render pipeline.
<a href="https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/projection-matrix-GPU-rendering-pipeline-clipping.html">
Image credit </a>
</figcaption>
</figure>

## What's in drawing a series of triangles for a GUI?
How to draw a circle  
Tesselation and curves  
Subdivision surfaces [intro][4] [pixar][5]  

## What's in drawing a series of triangles for a GUI using libraries?
Running through egui usage in egui-winit-wgpu-template

We can also find a simpler way to do things by relinquishing fine-grained
control and use an app framework made for egui, called [eframe Hello World][6]
Which significantly shortens the time it takes to get some buttons on screen

## Additional Reading
If you would like to know more about classic rasterization based computer
graphics using GPU's, I can recommend the following websites - [LearnWGPU][0]
and [LearnOpenGL][1].

[Bresenhams line algorithm][7], [Xiaolin Wu's line algorithm][8], while more expensive draws, nicer lines.

[0]: https://learnopengl.com/
[1]: https://sotrh.github.io/learn-wgpu/
[2]: https://github.com/emilk/egui
[3]: https://github.com/emilk/egui/tree/master/crates/eframe
[4]: https://en.wikipedia.org/wiki/Subdivision_surface
[5]: https://graphics.pixar.com/opensubdiv/docs/subdivision_surfaces.html
[6]: https://github.com/emilk/egui/blob/master/examples/hello_world/src/main.rs
[7]: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
[8]: https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
[9]: https://en.wikipedia.org/wiki/Polygon_mesh
[10]: https://en.wikipedia.org/wiki/Manifold
[11]: https://absorensen.github.io/the-guide/m2_concurrency/s6_more_gpu/
[12]: https://en.wikipedia.org/wiki/Painter's_algorithm
