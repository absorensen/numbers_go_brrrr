# Graphics for Graphical User Interfaces
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
most used are usually lines, triangles and triangle strips. When we know we have a contiguous set of triangles we
can save a lot of indices by using a moving window interpretation for the faces. Face 0 will be vertex 0, 1 and 2,
face 1 will be vertex 1, 2 and 3 and so on. To stop the strip and start a new one, you can usually make the
strip degenerate by repeating a vertex. So making face 1 be vertex 1, 2 and 2, would make it clear that a new
triangle strip needs to be started. Sometimes to maximize numerical accuracy or to help with the quantization
process, we might create a model matrix for the mesh and normalize all coordinates to maximally touch the
surface of a unit sphere and then quantize.

## What's in drawing a series of triangles?
Vertex and Fragment Shaders  
Coordinates and spaces  
Normalized device coordinates  
Rasterization  
Depth testing  

## What's in drawing a series of triangles for a GUI?
How to draw a circle  
Tesselation and curves  
Subdivision surfaces [intro][4] [pixar][5]  
Contention  
Draw Order and the Painters algorithm  

## What's in drawing a series of triangles for a GUI with egui?
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