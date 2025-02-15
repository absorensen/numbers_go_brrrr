# 👨🏼‍💻 Exercises
While you might think to yourself - I thought this was about making fast systems, why do I need to do stuff with
precision and number representations? It is actually quite useful. If you know how much precision you can afford to
lose you might be able to drop down from f64 to f32, you might be able to use fewer iterations in your solution
or you might be able to quantize data in order to compress it better, or even use it with cooperative matrix
hardware.

## Rounding Error Mitigation
### m3::e0 - Summation
Let's take a look at compensating for errors in extremely large summations. To do this we need to have a ground
truth result we can hold up the quality of our summation to. I have a code snippet ready for you to generate
your data -

=== "Rust"

    ```rust
        let element_count: usize = 100000000;
        let element_value: f32 = 0.1;
        let ground_truth: f32 = 10000000.0;

        let data: Vec<f32> = (0..element_count).into_iter().map(| _ | element_value).collect();
    ```

Now we are going to try out three different techniques for summing these numbers. Start using f32 for the data
and the accumulation. What happens if you accumulate in f64 instead? What if you use f16 for data and f32 for
accumulation? What if you use f16 for both? You will have to find a crate for f16, just make sure it doesn't convert
to f32 for arithmetic operations.

First up is [Kahan summation][2] and a supplemental [blog post][14]. Next try the Kahan-Babushka-Klein sum, from the
same page. Finally, try the [pairwise summation][4]. Iteratively add every two numbers to get a new list that is
half the size until you have 1 number.

Pairwise summation is also the principle behind the [butterfly FFT algorithm][6] and the defacto standard
method of summation on GPU's. If you want to get nuts, we can get nuts, you can also try to implement the
[iterative tree reduction][5] on the GPU.

## Sorting
### m3::e1 - Radix
Simply but [Radix][7]-, or buckets sorting numbers sorts them one digit at a time. Eventually, once all digits
have been sorted, all of the numbers will be sorted. There are some very advanced, very parallelizable, versions
of Radix sort, but try to do [a simple one][8].

Generate a list of integers. Start with a small one where you can visually verify that your sorting is correct.
Then for one digit at a time sort them into different buckets. The number of buckets depends on which number
representation you choose. Then sort each of those buckets. At which point do you begin to merge the buckets
with each other? Do you do it while iterating or at the end?

Next generate a really big list of integers. Once the algorithm has executed and you have one big sorted list,
verify programmatically, that every element is equal to or bigger than its predecessor. Unless it is element 0,
of course.

### m3::e2 - Morton
[Morton coding][9] is not quite sorting, but we can generate a new number which gives better spatial coherence to data.
Which can then be sorted by the [Radix sort][7]. Briefly put, in the two dimensional case, we interleave the bits of
two 4 bit numbers, ```x``` and ```y```, as ```y3x3y2x2y1x1y0x0```. This results in a z-order curve as described in
[bit tricks][10]. Note that you have a maximum amount of precision in your input numbers. If you are generating
a 64-bit Morton code at most for 3 dimensions, you have at most 64-bits to spare. If you want an isotropic
resolution in all dimensions, your input numbers cannot exceed 21-bits.

* Generate a list of integers and generate Morton codes for each of them.
* Sort the Morton codes with your Radix sort. What number representation would be best at first glance? What is the disadvantage?
* Reconstruct your initial numbers from the Morton codes.

Generating Morton codes as efficiently as possible also makes for an [interesting read][11].

## m3::e3 - Compression and Quantization
Generate a list of points (```Vec3<f32>```). Make them big, don't just center them around 0 to 1. Start off with
just a few numbers you verify yourself, but then generate a couple of million numbers. You can
get the ```Vec3``` from libraries like [nalgebra][12] and [ultraviolet][13].
Find the minimum and maximum values in the ranges. Use spatial hashing as described in [bit tricks][10] to sort the
points into buckets in a ```HashMap<u32, Vec<Vec3<f32>>```. Use 10 bits of precision for each coordinate.

Once you have done this, perhaps too literal, bucket sort, quantize each point in each bucket's collection of points
relative to the placement of the bucket in which it resides. From there turn the quantized points into
Morton codes. Do you need to adjust the resolution of your quantization to accomodate the range of the Morton codes?

Once you have the Morton code version of your points, use Radix sorting on each bucket's list of Morton codes.

Now, use delta encoding to limit the variety of numbers present in each list. Next, find a compression crate to
write the lists to disk in zipped format. Write one file for each bucket and name each file the corresponding
spatial hash of the bucket.

Load all of the buckets from disk, de-delta encode all points, un-interleave them and finally de-quantize all of the
numbers.

Can you calculate the maximum induced loss of precision for the points?

[2]: https://en.wikipedia.org/wiki/Kahan_summation_algorithm
[4]: https://en.wikipedia.org/wiki/Pairwise_summation
[5]: https://developer.download.nvidia.com/assets/cuda/files/reduction.pdf
[6]: https://en.wikipedia.org/wiki/Fast_Fourier_transform
[7]: https://en.wikipedia.org/wiki/Radix_sort
[8]: https://brilliant.org/wiki/radix-sort/
[9]: https://en.wikipedia.org/wiki/Z-order_curve
[10]: https://absorensen.github.io/the-guide/m3_types/s4_bit_tricks/
[11]: https://www.forceflow.be/2013/10/07/morton-encodingdecoding-through-bit-interleaving-implementations/
[12]: https://www.nalgebra.org/
[13]: https://docs.rs/ultraviolet/latest/ultraviolet/
[14]: https://chrisjameswalker.com/2022/01/28/kahans-summation-algorithm/
