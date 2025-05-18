# Bitwise Operations
Bitwise operations are operations we do on individual bits of integers. In general, doing bitwise operations on
floats are frowned upon, if not undefined behavior.

<figure markdown>
![Image](../figures/bitwise_float_to_jail.jpg){ width="700" }
</figure>

Doing individual bit operations allows us to squeeze lots of data into very little space which means we can
keep more data cached and cache lines better utilized. As we'll see later on, bitwise operations pose an
alternative to standard binary operations such as && and ||, which is a huge boon when optimizing code with
[branchless programming][3].

After that little aside, let's get back to a more direct description of how we work with types.
We don't just have the mathematical operators which we can use on integers and floats, taken
directly from mathematics, like *, /, +, -, %, but we can operate directly on the bits themselves, which
is specific to the underlying implementation of types. Before we start, I should probably say (write?) that
it is, at minimum, bad practice, if not outright triggering a compiler error, to do bitwise operations on
floating point numbers. In most cases it is recommended to use integers, and if you do not view the
variable as a number at all but as an array of bits, also known as a
[bit array][5], a bit mask, bit map, bit set, bit string or bit vector
(make up your minds people!), it is recommended to use unsigned integers, or sometimes languages will have
specific types just for better ergonomics when directly manipulating bits. In that case they might mimic
the vector operations quite closely, such as indexing the bit array, like ```bit_vector[3]``` to get the fourth
bit. This is quite inefficient if you are manipulating more than a single bit at a time though. A bit vector type
might also add some other operations, like counting the number of zeros and ones. In fact, the Rust ```uxx```
types like ```u32```, have functions like ```.count_ones()``` and ```.count_zeros()``` available.

Anyways. We have 5 of the most common bitwise operations right here: >> << & | ^ ~.

The >> and << operators are known as bit shifts. You might hastily assume that that ```let shifted = input >> 2;```
merely shifts all of the bits 2 places to the right. This CAN be equivalent to integer division by 4 as the
bits fall off to the right. But what if ```input``` is a signed integer? In that case it can vary. Does
it shift the sign along with it or does ignore the sign bit? The same with ```let shifted = input << 2;```.
If input was ```-2```, with a bit representation of ```0b1111_1110``` and we shift two places to the right we
get, wait, hold on... we still get ```-8```! Thank you 2's complement! What it actually does is shift the 1 to
the right, but puts in another 1 into the sign bit. But shifting downwards would get us ```0b0011_1111```,
which is ```63```. Which wasn't what we intended. If you think this is a bit messy, that's because it is
and if you are playing around with bits for signed integers you should always look up the language you
are using's definitions of shifts on various types. That got a bit headscratchy, because one number
representation had a special bit and 2's complement. Now imagine this, but with floating point numbers with
three segments of bits instead. That's why we don't do bitwise operations on floats!

The type of shift where the sign bit gets special treatment is called an *arithmetic shift*, where as the one
treating the whole thing as just bits, is called [*logical shift*][4], but
I would recommend you stick to unsigned integers for bit manipulating stuff. It's less stuff to keep in your head.

Next, let's look at the bitwise and (&), usually written as bitwise AND, probably to ensure you don't mistake it for
linguistic addition. Bitwise AND has the same logical implications as the boolean &&. Two trues, or two 1's, result
in a true or a 1. All other cases result in false or 0. But, the common && does something else. It short circuits.
If we take the statement ```let is_correct: bool = eval_a() && eval_b();``` and ```eval_a()``` returns false,
```eval_b()``` is never called. This can be a boon for performance if ```eval_b()``` is very expensive, but
can actually lead to worse performance in the case of two cheap evaluations such as
```let is_correct: b = 0 < some_number && some_number < max;```. I'll get back to that in
the section on branchless programming. It's an advanced topic, which requires understanding some stuff about
branch prediction, and especially bitwise- and conditional operations.

Anyways. When we do the bitwise AND we don't supply two booleans, we supply two integers or other bit
representations. Don't do it with floats, even if the language allows you, it is needlessly complicated. The
other number we supply is usually called a mask. We could for example have a number where we wanted to
isolate the first 8 bits and not be bothered by the rest. In that case we might write something like -

=== "Rust"

    ```rust
    let initial_value: u16 = 0b10101010_11110101;
    let mask: u16 =                   0b11111111;
    let masked_value: u16 = initial_value & mask; // masked value should now be 0b11110101
    ```

Masks is also where hexadecimal numbers can really clean up. I used u16 in the example, but if we used
64-bit numbers things could get very out of hand -

=== "Rust"

    ```rust
    // Damn you 64-bits!
    let initial_value: u64 = 0b10101010_10101010_10101010_10101010_10101010_10101010_10101010_11110101;
    let mask: u64 =          0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
    let masked_value: u64 = initial_value & mask; // masked value should now be 0b11110101
    ```

... let's use hexadecimal numbers instead -

=== "Rust"

    ```rust
    // Damn you 64-bits!
    let initial_value: u64 = 0x2AAA_AAAA_AAAA_AAF5; // or 3_074_457_345_618_258_677 
    let mask: u64 = 0xFF00 0000 0000 0000; // FF is the same as turning 8 bits completely on. So 0xF == 0xb1111.
    let masked_value: u64 = initial_value & mask; // Masked value should be 0x2A00 0000 0000 0000
    ```

Or with a slightly more sane example.

=== "Rust"

    ```rust
    let initial_value: u16 = 43_765;
    let mask: u16 = 0xFF;
    let masked_value: u16 = initial_value & mask; // masked value should now be 0b11110101
    ```

We could also use it to isolate specific bits. Like checking whether a single bit is turned on -

=== "Rust"

    ```rust
    let initial_value: u16 = 0b10101010_11110101;
    let mask: u16 = 0b00000100;
    let third_bit_turned_on: bool = 0 < (initial_value & mask); // true
    ```

This is really useful in embedded systems or any system where we have lots of flags and want to conserve
bandwidth and/or space, like "the router is in an error state" or "package received". All of a sudden we can
have 64 flags in 64 bits! Or if we wanted to have a bunch of 3d voxels or an occupancy map which only carried
a is there/is not there value, we could vastly reduce memory consumption by using bit operations. | and ^
are the same except with a different logic table. || is sort of like |, or bitwise OR. One or more values
need to be true. Where as with bitwise XOR (^) it only returns 1 if a single bit is 1, but not if both are
1's.

Finally, we have the bitwise NOT. Usually written as ~. It is the bitwise version of !. It just flips all bits
from 0 to 1 or from 1 to 0.

_________________

# Bitwise Rust
Rust does not have a bit vector implementation in the standard library, but what it does instead is implement
quite a number of bitwise or bitwise-adjacent operations directly on
[integers][0]. Rust does have the bitwise NOT, usually denoted ~, but it is ! like on boolean types. Like so -

=== "Rust"

    ```rust
    let initial_value: u8 = 0b01010101;
    let flipped_value: u8 = !initial_value; // 0b10101010;
    ```

## Additional Reading
If you want to go into greater depths about bitwise operators check out [this wiki][1] or the Rust language
reference for [operators][2].

[0]: https://doc.rust-lang.org/std/primitive.u32.html#method.rotate_left
[1]: https://en.wikipedia.org/wiki/Bitwise_operation
[2]: https://doc.rust-lang.org/reference/expressions/operator-expr.html
[3]: https://absorensen.github.io/numbers_go_brrrr/m4_real_time_systems/s4_branchless_programming/
[4]: https://open4tech.com/logical-vs-arithmetic-shift/
[5]: https://en.wikipedia.org/wiki/Bit_array
