Task 3.3: Date and time
=======================

📚 [Rust]⏱0.25h has a simple 📚 [`std::time`]⏱0.25h module which contains very basic primitives for time measurements. To operate with dates, time zones, epochs, and other related stuff, the 📚 [`time`]⏱0.25h and 📚 [`chrono`]⏱0.25h crates are used in 📚 [Rust]⏱0.25h ecosystem.

The main difference between them (except the API, ergonomics and maintaining activity) is that 📚 [`chrono`]⏱0.25h crate parametrizes time zone in types, while 📚 [`time`]⏱0.25h crate handles it in runtime. In practice, we recommend to use 📚 [`time`]⏱0.25h crate (unless 📚 [`chrono`]⏱0.25h better suits your needs), as it's much actively maintained and evolved.

If you hit limitations of 📚 [`time`]⏱0.25h and 📚 [`chrono`]⏱0.25h crates regarding their accuracy (like swallowing 📰 [leap seconds][3]⏱1.5h) or supported formats/standards (like 📰 [TAI]⏱0.5h), consider using the 📚 [`hifitime`]⏱0.25h crate, representing a scientifically accurate and 📰 [formally verified][4]⏱0.25h date and time library.

For better understanding and familiarity, read through the following documentation:
- 📚 [Official `std::time` docs][`std::time`]⏱0.25h
- 📚 [Official `time` crate docs][`time`]⏱0.25h
- 📚 [Official `chrono` crate docs][`chrono`]⏱0.25h
- 📚 [Official `hifitime` crate docs][`hifitime`]⏱0.25h




## Duration measurements for code

Beware, that to measure duration of some operation, you should not use 📚 [`time`]⏱0.25h crate primitives or an 📚 [`std::time::SystemTime`]⏱0.25h, but only an 📚 [`std::time::Instant`]⏱0.25h instead, as it provides ❓ [monotonic clock][1]⏱0.25h measurement (otherwise, your time measurement may be inconsistent due to 📰 [system clock drift][2]⏱0.25h).




## Task

__Estimated time__: 1 day




Provide implementations for `User::age()` and `User::is_adult()` methods in [this task's crate](src/main.rs).

Prove your implementation correctness with additional tests. For tests reproducibility consider that "now time" is the date specified in the `NOW` constant.

> 💡 The structure needs to be modified


## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
1. How does system clock and monotonic clock differ? What are use-cases for both?
2. Why is system clock is not reliable for measuring duration? What causes its drift?
3. What is the main practical difference between 📚 [`chrono`]⏱0.25h and 📚 [`time`]⏱0.25h crates?
4. When 📚 [`hifitime`]⏱0.25h crate could be useful?




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
