#Fibonacci
#===

##Objectives
##---

Calculate [Leonardo Fibonacci's](http://en.wikipedia.org/wiki/Fibonacci) sequence up to Nth iteration.

##The algorithm
##---
The algoritm remembers values of last two fibonacci numbers, adding them together in next iteration. For the very first two iterations, there are no values to add. Fortunately, for the first two iterations, the output values is exactly the same as the number of current iteration (i).


##Rust integer representation
##---
Keyword u64 means "unsigned integer of 64 bits". With signed integers, there is always one bit reserved for the plus or minus sign. Because fibonacci sequence reaches sky-rocket-high numbers very quickly, 32 bits (u32) are exhausted very quickly.

For u64, there are valid values from 0 to 2^64 = 18 446 744 073 709 551 616
For u32, there are valid values from 0 to 2^32 = 4 294 967 296
For i32 (signed integer), there are valid values from -max to +max, where max = 2^31 = (2^32)/2 = 2 147 483 648