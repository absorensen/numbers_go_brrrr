# Floats
Where as signed integers have two distinct areas of the bits with a uniform precision, floats
have 3 segments and a wildly varying precision. In the more-or-less standard
IEEE-754 specification we also have a number of exceptions and specific behaviors such as in which situations
a float locks to a NaN value. Given the use cases in machine learning new, smaller and less standard floating point
formats have emerged such as bfloat16. A lot of these very small specialized types of floats, or very small integers
are required to work with tensor core accelerators.

I can't explain exactly how a float works much better than the [wiki][7]. Ever so slightly browsing the
page about [IEEE-754][8] also makes good sense.

<figure markdown>
![Image](../figures/invincible_fraction.png){ width="700" }
</figure>

So, now that you've browsed the websites and your are familliar with concepts such as exponent and fraction, I
have a few key concepts for you to make note of. If you can at all keep your floats as close to being between
-1.0 and 1.0, you will keep as much of your precision as possible. 4.0 / 3.0 is not the same in terms of precision
as 4.0 * (1.0 / 3.0), unless the inverse number that you are multiplying with is exactly 1/2^N. This is an often
used optimization as a multiplication is much cheaper than a division. You will typically see this optimization
if a division by a constant is happening in a loop -

=== "Rust"

    ```rust
    let some_constant: f32 = 5.0;
    for element in &mut data {
        *element = *element / some_constant;
    }
    ```

which, again, is not numerically equivalent to

=== "Rust"

    ```rust
    let some_constant: f32 = 5.0;
    let inverse_some_constant: f32 = 1.0 / some_constant;
    for element in &mut data {
        *element = *element * inverse_some_constant;
    }
    ```

But if ```some_constant``` was 4.0, 2.0 or 256.0 or some other version of 2^N, they would be. Finally, ```NaN```'s
propagate. Any operation involving a ```NaN``` value returns a ```NaN``` value, including ```NaN == NaN```.
Division by ```1.0/0.0``` does not return ```NaN``` in Rust, but ```inf```. ```-1.0/0.0``` on the other
hand returns ```-inf```.

Accumulation in floats have some aspects to it which can be quite relevant. Due to the variable amount of
precision in the type, adding a small float to a large float, will yield a significantly higher error
than adding two small floats. By extension, if you have three floats, two small and a large, you would
get a smaller error by first adding the two smaller numbers and then adding the now somewhat less small
number to the big number. Taking things further, if you had a large list of numbers, you could sort them
and add them from smallest to biggest. But, the number in which you accumulate your sum might quickly
become much bigger than the individual elements. An alternative method if you have a large list
of numbers, is summing them pairwise, recursively (like a tree reduction). This will yield a
smaller error. This is one of the reasons for the increased numerical precision in the
[butterfly FFT algorithm][0], compared to naive summation.

Another aspect of reducing errors in summation of floats is the precision at which you accumulate.
This is quite important for working with cooperative matrics/tensor cores. You can reduce the memory
requirements and the memory bandwidth usage, by storing your weights in 16-bit floats, but
accumulating in 16-bit would yield poor results. Instead you can accumulate in a 32-bit float or even
a 64-bit float, which will be kept locally in a register anyway, and then once accumulation is
complete the accumulated sum, in 32-bits, can be cast down to 16-bits and written to memory.

## Additional Reading - Highly Recommended
Every floating point operation incurs some form of error. But if you used specialized floating point operations
such as the [fused multiply-add][1], which can be found [in Rust as well][2], you can get a
single rounding error instead of two rounding errors. The fused multiply-add may be slower in some cases (it is
on my machine) and faster in others. Depending on your use case you might also not see any difference in error. If
you are summing a large list of numbers you can use algorithms for compensating for the accumulated error, such as
[Kahan Summation][3]. You can also keep a [running estimate of the error][4].

### 🧬 Computer Graphics
Depth buffers are usually in need of some thought. You can take a look at [depth precision visualized][5] or
the now fairly common [reverse depth buffer][6].

[0]: https://en.wikipedia.org/wiki/Fast_Fourier_transform#Accuracy
[1]: https://en.wikipedia.org/wiki/Multiply%E2%80%93accumulate_operation
[2]: https://doc.rust-lang.org/std/primitive.f32.html#method.mul_add
[3]: https://en.wikipedia.org/wiki/Kahan_summation_algorithm
[4]: https://pbr-book.org/3ed-2018/Shapes/Managing_Rounding_Error
[5]: https://developer.nvidia.com/content/depth-precision-visualized
[6]: https://www.danielecarbone.com/reverse-depth-buffer-in-opengl/
[7]: https://en.wikipedia.org/wiki/Floating-point_arithmetic
[8]: https://en.wikipedia.org/wiki/IEEE_754
