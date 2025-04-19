Task 3.3: Date and time
=======================

[Rust] has a simple [`std::time`] module which contains very basic primitives for time measurements. To operate with dates, time zones, epochs, and other related stuff, the [`time`] and [`chrono`] crates are used in [Rust] ecosystem.

The main difference between them (except the API, ergonomics and maintaining activity) is that [`chrono`] crate parametrizes time zone in types, while [`time`] crate handles it in runtime. In practice, we recommend to use [`time`] crate (unless [`chrono`] better suits your needs), as it's much actively maintained and evolved.

If you hit limitations of [`time`] and [`chrono`] crates regarding their accuracy (like swallowing [leap seconds][3]) or supported formats/standards (like [TAI]), consider using the [`hifitime`] crate, representing a scientifically accurate and [formally verified][4] date and time library.

For better understanding and familiarity, read through the following documentation:

- [Official `std::time` docs][`std::time`]
- [Official `time` crate docs][`time`]
- [Official `chrono` crate docs][`chrono`]
- [Official `hifitime` crate docs][`hifitime`]

## Duration measurements for code

Beware, that to measure duration of some operation, you should not use [`time`] crate primitives or an [`std::time::SystemTime`], but only an [`std::time::Instant`] instead, as it provides [monotonic clock][1] measurement (otherwise, your time measurement may be inconsistent due to [system clock drift][2]).

## Task

__Estimated time__: 1 day

Provide implementations for `User::age()` and `User::is_adult()` methods in [this task's crate](src/main.rs).

Prove your implementation correctness with additional tests. For tests reproducibility consider that "now time" is the date specified in the `NOW` constant.

> 💡 The structure needs to be modified

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- How does system clock and monotonic clock differ? What are use-cases for both?
- Why is system clock is not reliable for measuring duration? What causes its drift?
- What is the main practical difference between [`chrono`] and [`time`] crates?
- When [`hifitime`] crate could be useful?

[`chrono`]: https://docs.rs/chrono
[`hifitime`]: https://docs.rs/hifitime
[`std::time`]: https://doc.rust-lang.org/std/time/index.html
[`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[`time`]: https://docs.rs/time
[Rust]: https://www.rust-lang.org
[TAI]: https://en.wikipedia.org/wiki/International_Atomic_Time

[1]: https://stackoverflow.com/questions/3523442/difference-between-clock-realtime-and-clock-monotonic
[2]: https://en.wikipedia.org/wiki/Clock_drift
[3]: https://en.wikipedia.org/wiki/Leap_second
[4]: https://model-checking.github.io/kani-verifier-blog/2023/03/31/how-kani-helped-find-bugs-in-hifitime.html
