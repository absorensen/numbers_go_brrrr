# Types
Types might not seem to be the sexiest topic of all time, but don't you worry. Knowing about types
will be a good investment of your time. Whether to use a 32-bit or 64-bit number is not just a matter
of whether your program has enough precision to go around, but can be a source of bugs and buggy
behavior. Sometimes you may even need to guarantee that your system yields at minimum some level
of precision in very real world terms for your users to be satisfied using your system.

You can reduce the strain on your memory bandwidth by using smaller types (think back to
cache lines) and get more data elements per cache line resulting in being less memory bound
which might in turn increase the speed of your program. If you are on a GPU, your performance
might improve significantly (factor 32) by going from 64-bit floats to 32-bits, you can
even get access to tensor cores by reducing precision even further. Knowing more about types
allows you to sort, order and quantize your processed data in a way that has as small an
impact on precision as possible while decreasing the size of your data. This could result
in faster download times or you could stream your data from disk directly to the GPU,
where the GPU itself might be able to unpack the data.

Knowing which transformations of your data are acceptable can also allow you to minimize the total
size of your data at run time, allowing you to fit everything into memory which will greatly
decrease the amount of disk activity and pressure on memory bandwidth.

Which types you are using has an impact not just on speed and size, but also the energy
consumption of your programs. In general, less bits mean less energy consumed. You don't
have to micromanage every single variable all the time, but one of the first places to
look when optimizing should be arrays. ```f64``` rarely matters, but ```[f64]``` sure does.

Finally, knowing about types allows us to operate directly on the underlying bits, casting
from one type to another, to create tightly packed information, which we couldn't otherwise
have, such as packing three dimensional indices into a single integer or 32 boolean values
into a 32 bit integer.
