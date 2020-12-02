# Day 2 solution

The main difficulty here was making sure I was parsing the lines correctly, and interpreting the "password protocol".

My approach was to parse each line into a `Policy` object, and then endow them with validity checkers. Getting a count after that was trivial.

My parsing is very gung-ho with `unwrap()`s galore. I also use the very nice `collect_tuple` from `itertools`. The other trick is that Rust doesn't let you index strings at will (because of the careful distinction between bytes and characters).

I wish there was a `count_predicate` function rather than `filter( ... ).count()`, but it's no biggie.

## Performance

Using `cargo aoc bench` I got an average performance of:
- Part 1: 16.147μs
- Part 2: 27.703μs

Since there are 1,000 lines (with 20,648 characters) we're processing a character in roughly 8 ticks. Not too bad.

I tried to use rayon to improve performance... it made it worse so perhaps the multithreading overhead was not worth it.
