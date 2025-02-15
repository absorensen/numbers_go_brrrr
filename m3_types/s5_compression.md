# Compression
Why should you care about compression? In most cases it will be handled for you, but if you are implementing a real-time
system, in some cases you will have time for preprocessing. In preprocessing you can compress or pack your data which,
while requiring some encoding/decoding to then use your data, can relax the pressure on bandwidth,
reduce download or upload times and reduce the amount of memory used. It is already heavily used in everything
to do with video and audio.

In compression, you have to distinguish between lossy and lossless compression. With lossless compression the
original data is recoverable down to the bit, where as with lossy compression we have some, usually variable,
amount of loss of quality. This quality might not even be perceptible, or you can calculate how much is
acceptable for your situation, allowing you to reap a vast amount of memory savings. Choosing the
right compression scheme and transforming your data, with techniques like quantization, packing and sorting
to maximize the compression scheme's effectiveness often requires domain knowledge, but the results can
be spectactular.

You will find lossy compression in formats like JPEG (2D images) and MPEG (2D videos), using codecs like H.264.
You will find lossless compression in stuff like PNG and ZIP. Some of these are ubiquitous enough that dedicated
hardware is available. For example, a lot of GPU's have dedicated hardware for video encoding and decoding for
stuff like streaming. Compression times and characteristics vary wildly. Some are really good with text, some
are good with numerical data. Some take a really long time to compress or decompress.
Knowing how external compression libraries work can help you do transforms on your own data to maximize the
performance of the compression libraries which you might apply after your own transformations.

Ok, so what is compression actually? In short, exploiting the structure and repetition of data to encode it in a
smaller representation. This is tightly connected to types, which is why it is in this module. For example, floats
are really hard to compress well, why? Because of repetition. As you might recall, it is generally bad form to
compare floats exactly. Instead you need to take the absolute difference and confirm that the two numbers are
within some threshold distance of each other, not to be considered the same, but similar enough. Compression
thrives on sameness. If we had a row of 1,000 pixels in an image, all of which were the exact same color, we could
instead represent the entire row by the amount of pixels in the row and the value itself. Thus replacing 1,000
values with just two values! Think of the savings! This is what is known as run length encoding and is one of the
easiest ways of transforming your data before running it through a more general compression scheme (which might
itself use run length encoding). This also somewhat echoes the various ways we tried to do jagged arrays in ```m1::s2```.

If we wanted to use the run length transformation, we could then in turn begin transforming our input data
(the image) to maximize this similarity. After all, if we had a row of 1,000 pixels and they all had different
values, our simple run-length encoding would balloon up to double the size. We could
make ensure that the values became more similiar. If all of the colors were in 16-bit, we could lower the precision
to 8-bits by either dividing all of the values by 256, or we could introduce a defacto variable precision reduction
by nulling bits from the right in all of the values (I'm assuming integers here) to get the precision we could live
with. If we could at most live with a reduction from 16-bit to 11-bit precision in our colors, we could set the
5 least significant bits in all of the values to 0. We have then effectively increased the likelihood of any
given value being similar to its neighbor. We still have to keep it in memory as 16-bits per element, mind you, but
when compressed/encoded it will take up much less space.

We could take things even further and delta encode all of the values. We do this by having the starting value
be the initial value, and every subsequent value being the difference of that value relative to the preceeding
value. In that case we now have even more numbers that are similar. This does however make the decompression
take longer. You now have to de-delta encode all of the values sequentially starting from each reference
value to use them.

For a brief introduction to both the classic Lempel-Ziv schemes look under
week 10 on this [page][7]. Another classic is [Huffman Encoding][8].

_________________

## 🧬 Point Cloud Compression
This is mostly for people interested in spatial data. I will showcase a few examples of transformations you can do to
compress point clouds, which is based on some real-world experience.

First up, let's set the scene. A user uploads a point cloud to your system. You have time to preprocess the point
cloud and at some point the user will see that the point cloud has been processed and is ready for viewing. The
point clouds can be massive. Much bigger than can fit in the VRAM of a GPU, or even the RAM of the users system.
Progressive rendering is chosen to render. The frame buffer is what we will render to. It is an image buffer that
mimics the image that is output on screen, but it is not that image. To show an image on screen we copy just the
color values from the frame buffer. The frame buffer also contains depth values per pixel, which means we can
iteratively suggest new candidates for a pixel closer to the viewing point. The depth values are then used to
determine whether the new pixel candidate is actually closer to the viewing point. In the case of progressive
rendering the frame buffer is cleared every time the user moves the camera, but the relevant parts of the scene
are accumulated into the frame buffer, using the depth buffer to discriminate between which points should be in
view, across multiple frames. So imagine this - whenever the camera moves we start over rendering our scene, but
whenever the camera stands still we stop clearing the image every frame and starting over. We might not have a
a converged image, a rendering of all the relevant geometry for that specific point of view, until some number
of frames later. By not restricting ourselves to having to render the full image in every frame we allow for fast
movement and full quality once the user has settled on a point of view.

Frustum culling is also implemented to prioritize which points to render first, and which points to not render
at all. We need a data structure which lets us make decisions about which packages to even download to the
user's system before we download them. We choose a type of level-of-detail octree. The regular structure of
the octree allows us to make some nice decisions later on, which are encoded implcitly by our knowledge of the
chosen structure. A level-of-detail structure for point clouds means that the top node of the octree will
cover the entire scene. We will choose some maximum amount of points which can reside in this root node.
Those points have to cover as much of the scene as possible, preferably in as visually nice a way as possible.
The fewer nodes at as low levels-of-detail as we can get away with, will allow for a very high frame rate during
navigation of the scene so the user can find the angle they need and then set the camera to rest so we can
accumulate the relevant points and converge on the "true" image.

We will build this level-of-detail octree by using spatial hashing. Instead of starting with a tree representation,
think back to nodes and pointers, we will start with a list of hash maps. In Rust the pseudo tree representation
would look something like ```let mut pseudo_tree : Vec<HashMap<u64, Vec<Point>> = ...```. Alternatively, given
that we know how much each list maxes out at we could just use a fixed size array and an index to which points
are active. So for each level-of-detail we will keep a hash map. Each level-of-detail will have the same offset
vector, but a diminishing scale. We can pick any factor, but if we choose 2.0 we get some nice side effects
later on. So if we set the scale of the root node to 100.0 meters for each axis, level-of-detail 1 would
have a scale of 50.0 meters per node for each axis. This will continue on until level-of-detail 10 where
we have a scale of 0.09765625 meters, or 9.7 cm per node. Clearly, we now have a very small area,
being covered by quite a lot of bits, 20! And how we will turn that to an advantage I will get back to.

We just have to clear up one thing, subsampling. It is not that important in terms of compression, but this
data structure will reuse the spatial hashing concept quite a bit, and we will do so too when choosing which
points go where.

To build our tree we start finding the minimum and maximum of the scene covered by the
given point cloud. We create a node defined by the minimum and maximum. The offset becomes the minimum vector and
the scale vector will be defined by the vector from the minimum to the maximum value. Sometimes, the subsampling
can come out nice if we just use isometric axes. In which case the scale vector would be the same as the maximum
value in the difference between the minimum and maximum values. Then we might as well not use a scale vector, but
a single scale value.

Ok, so we found the area that our root node needs to cover, now we start adding points until we hit a chosen
threshold value. Lets say 32k points. Once we hit that limit we subsample and keep only half that amount of points.
The subsampling strategy should be reasonably performant and make for a reasonably representative coverage of the scene.
We will start with an empty hash map to subsample the points. Having the cells only be occupied by one point at
a time is a good basis for subsampling. A more visually optimal technique would be the [3D poisson disc subsampling][9].
It looks very nice, but to minimize the distances between neighbors, we would need to query for any active neighboring
cells, which in 3D is 27 cells we need to query for every single one of our initial 32k points. Instead, if we need
a bit of speed, we can get a much better performance and a reasonable aesthetic by minimizing the distance to the
center of the individual cell.

So. Our algorithm for subsampling a list of points will be this. Get list of points, offset and scale. Create
an empty hash map, almost like the one we saw earlier, the keys are ```u64``` and the values are ```Point```.
For each point in our list we compute the spatial hash of that point. We query the hash map for whether that
cell has already been created. If it has not, we create it and insert the point as the point held by the cell.
If there was already a point in the cell, we compare their distances to the center of the cell. Whichever point
is rejected, is added to a list of subsampled points. Once we are done with this process we have two lists of points.
One list we keep in the node, the other list we send to the next level-of-detail. The second list will then be
distributed among the up to 8 child nodes. The child nodes are implicitly linked by their places in the list of
hash maps and not by pointers. Once we have done this for all nodes and we no longer have either new or subsampled
points to be processed, we can finalize the structure. We do this by now enforcing a maximum amount of points and continue adding
and subsampling until each node holds at most N points.

Phew, that was a lot, but now we are ready to do some compression! Throughout this process we have kept all of the
points in floating point coordinates. Ideally, as close to their original precision as possible. But, once we
are done. We know where each point should be and we can act accordingly and only once. This data will be streamed
to a users computer. They will be viewing the model through the web and won't have all of the model on their
own system. So we will zip all of the points with a web friendly lossless compression scheme. [Brotli][10] is well
supported by browsers and was tried, this was some years ago, so the performance may be different now, but it
required a lot more compression and decompression time. Instead the venerable [gzip][11] was chosen.

So we know we are going to compress each node in our implicit octree. The rendering details aren't
important/relevant to this section, but there is a limit to the resolution a user can resonably demand to see
in a web browser, while at the same time we don't want to induce too big of an error as users may want to use the
system for measurements. A maximum induced error of 0.5 mm is probably reasonable. Our area covered per node
at level-of-detail 10 was 9.7 cm. Each node can have up to 32k points, so let's start off with the easiest
optimization we can always make. Doing less. Having 32k points covering a 9.7x9.7x9.7 cm cube, is quite a bit.
So let's just throw away all points after that. We enforce the limitation that we have at most 11 levels-of-detail.
We could go even further and start merging all points which were sufficiently close to each other whenever we
subsample, some users are bound to be mad lads and send you point clouds with a precision greater than 0.1mm.
But we would need some form of policy for doing so which the user would have to find acceptable.
Anyways, if we know we level-of-detail 10 will be our most precise level, we can make another observation.

Do users really care if you have better precision, due to the smaller area covered by the same amount of bits,
at higher levels-of-detail compared to the root node?

The answer is of course, no. Most users will not care. And given that we previously chose that each successive
level-of-detail will have a scale that is exactly half of the preceeding level-of-detail, we can now do bit nulling.
First we will quantize all of the floats to 16-bit unsigned integers specific to the node, using the individual
nodes' scale and offset. The node will carry around some information, but that isn't too important. We won't do
bit nulling with level-of-detail 0, but for level-of-detail 1 we will set the single least significant bit to 0.
For level-of-detail 2, we will set the two least significant bits to zero. In the end, we will only have 6 bits of
precision per coordinate axis for level-of-detail 10. So 6 bits, 64 values to cover 9.7 cm each. This leaves us
with a precision of 0.1515625 cm, or 1.515 mm. Given that we quantize through flooring, we end up with at most
inducing an error of 1.515mm. If we instead started our bit nulling at level-of-detail 2, we would have double
the precision and have a maximum induced error of 0.75mm.

But hold up, we can only guarantee this precision if our scene is at most 100 m in scale. What if your scene is
bigger? This is where we veer off the standard octree path, once again. We will just have more than one root node.

Think back to the part about making numbers more similar. We can do this for our list of points by sorting
the points with techniques like Morton codes, Radix sorting or other schemes. As found in [this paper][12] by Schütz,
et al., sorting the points, which we can do in preprocessing, can not only have an effect on the effectiveness
of compression, but also the effectiveness of rendering.

We could also do delta encoding of all of the points, which would work better now that the points were all sorted.
This would be fine in the preprocessing step, but incurred too much CPU work to de-delta encode on the users'
systems. Once each node has been received by the users' systems, unzipped and sent to the GPU, the 16-bit
unsigned integers will be dequantized back into floats using a transformation matrix. This is a rendering detail.

Another option to make the numbers more similar would be to deinterleave the list of points and putting all of the X
values next to each other, followed by all of the Y values, all of the Z values and all of the colors. This again,
requires the users' system to touch every single point before uploading to the GPU, or to use 4 bindings per list
of points in the shader.

And here we are. This was a small example of how domain knowledge can help you compress your data, even if you
aren't writing the compressor yourself.

## Additional Reading
[Data Compression][1] has its basis in [information theory][0]. The two seminal basis algorithms are
[Lempel-Ziv 77 & 78][2] and [Huffman coding][3]. [Zip][4] is universally used and is natively supported in most
browsers through Gzip for [HTTP compression][5].

Lots of interesting things are happening in real-time decompression [using neural networks][6].

[0]: https://en.wikipedia.org/wiki/Information_theory
[1]: https://en.wikipedia.org/wiki/Data_compression
[2]: https://en.wikipedia.org/wiki/LZ77_and_LZ78
[3]: https://en.wikipedia.org/wiki/Huffman_coding
[4]: https://en.wikipedia.org/wiki/ZIP_(file_format)
[5]: https://en.wikipedia.org/wiki/HTTP_compression
[6]: https://research.nvidia.com/labs/rtr/neural_texture_compression/
[7]: http://www2.compute.dtu.dk/courses/02282/2023/
[8]: https://web.stanford.edu/class/archive/cs/cs106b/cs106b.1132/handouts/34-Huffman-Encoding.pdf
[9]: https://www.jasondavies.com/poisson-disc/
[10]: https://en.wikipedia.org/wiki/Brotli
[11]: https://en.wikipedia.org/wiki/Gzip
[12]: https://www.cg.tuwien.ac.at/research/publications/2021/SCHUETZ-2021-PCC/

