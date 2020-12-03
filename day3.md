# Day 3 solution

The trick was to get the right data structure and itertool to parse the structure out.

Initially I thought of just a big array with lookups, but really we'd iterate rows and need (sparse) lookup over the columns.

`itertools` continues to have useful things in it.

## Performance

Each `check_slope` calculation is on the order of 6μs. I could maybe speed this up by replacing the `HashSet`s with a `u64`.

Turns out this is 3x faster, which is neat. You might get some gains by hard-coding in some of the constraints and putting as much of the terrain data into a cache line, but I like this fairly fast, general approach.

I wouldn't expect parallelisation to make much of a dent. If you parallelised the main loop you'd have to atomically increase the `trees` count, and the result of that would take very little time and would likely be dwarfed by the thread setup costs.
