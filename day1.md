# Day 1 solution

Hitting this problem directly isn't too hard. Find all combinations. Filter on `a+b==2020` and early-return with `a*b`.

You can however spend a little time in the file-reading to partition the numbers into odds and evens. Then you can search a smaller space because 2020 is even.

2020 (Even) = Even + Even = Odd + Odd

Also:
Even = Even + Even + Even = Even + Odd + Odd

So you can iterate on triples of evens, or pairs of odds with an even.

## Post-sleep realizations

I was very much asleep last night, especially when coding! **Why am I doing all these linear scans?!?**

I replaced the linear scans with hash lookups and it's vastly improved timings. I also caught a dumb error with how the partition function split the two lists (evens first then odds).
