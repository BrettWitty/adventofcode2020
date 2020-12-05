# Day 4 solution

I think the biggest issue here today was wrangling the return types of various iterator calls. It *should* all be compatible, but sometimes it needed a poke in the right type direction, or mutability.

Interestingly the solution got easier to work with when I used a more complete `Passport` type. In part 1 I threw away all the values and kept the keys, which was messier to assemble than just mapping pairs.

I feel that if I had experience with a Rust equivalent of the `pyparsing` library, I'd smash this a bit easier.

I didn't have a lot of time to work on this one, but was caught in an annoying bug. I ignored the `cid` field. In fact I ignored it so thoroughly it fell into the error trap for unknown fields, so anything with a `cid` field would fail to validate. Setting up unit tests was handy.
