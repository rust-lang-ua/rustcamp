Task 3.8: Logging and tracing
=============================

🏠 [Rust] has flexible type system and 📰 [metaprogramming][1]⏱0.25h capabilities, allowing to build both efficient and highly reusable log system. The idea is very similar to 📚 [`serde`] and is introduced in a widely used 📚 [`log`], 📚 [`slog`] and 📚 [`tracing`] crates.




## Simple logging

📚 [`log`] crate represents a __single unified frontend interface (facade)__ which is used by all libraries at the same time, but is backed by one actual backend implementation on your choice. This allows to control all the logs (of application and its dependencies) from a single place and in a unified manner: opt-in and opt-out logs of libraries, separate logs by destinations, etc.

> - Libraries should link only to the `log` crate, and use the provided macros to log whatever information will be useful to downstream consumers.
> - Executables should choose a logger implementation and initialize it early in the runtime of the program. Logging implementations will typically include a function to do this.

One interesting part is that log levels can be 📚 [disabled at compile time][3], thus have __no runtime performance impact at all__, unless you're debugging.

For better understanding and familiarity with 📚 [`log`]'s design, concepts, usage, and features, read through the following articles:
- 📚 [Official `log` crate docs][`log`]
- 📰 [Jimmy Hartzell: Using the Log Crate in Your Rust Projects][12]⏱0.25h




## Structured logging

For 📰 [structured logging][4] there is the excellent 📚 [`slog`] crate in 🏠 [Rust] ecosystem.

> The ambition is to be The Logging Library for Rust. `slog` should accommodate a variety of logging features and requirements. If there is a feature that you need and standard `log` crate is missing, `slog` should have it.

It's __backward and forward compatible with 📚 [`log`]__ crate, extending its ideas and is baked with an 📰 [excellent performance][5]⏱0.25h.

For better understanding and familiarity with 📚 [`slog`]'s design, concepts, usage, and features, read through the following articles:
- 📚 [Official `slog` crate docs][`slog`]
- ❓ [Official `slog` crate wiki][6]⏱0.25h




## Tracing

The famous 📚 [`tracing`] crate is fabulous at both 📰 [tracing][10]⏱0.25h and 📰 [structured logging][4].

> `tracing` expands upon logging-style diagnostics by allowing libraries and applications to record structured events with additional information about _temporality_ and _causality_ — unlike a log message, a span in `tracing` has a beginning and end time, may be entered and exited by the flow of execution, and may exist within a nested tree of similar spans. In addition, `tracing` spans are _structured_, with the ability to record typed data as well as textual messages.

Its "killer feature", undoubtedly, is 📚 [spans functionality][7], so ❓ [people tend to prefer it over `slog`][9]⏱0.25h even for usual logging. It's also __📚 [backward and forward compatible][8] with 📚 [`log`]__ crate.

Speaking of 📰 [tracing][10]⏱0.25h, the 📚 [`tracing`] crate has good integrations with 🏠 [OpenTelemetry]-compatible distributed tracing systems (and similar ones). All this allows to reuse the same solution both for logging, tracing (like 🏠 [Jaeger], 🏠 [Zipkin]), profiling (like 📰 [coz]⏱0.25h, 🏠 [Tracy]), error reporting (like 🏠 [Sentry]), etc.

For better understanding and familiarity with 📚 [`tracing`]'s design, concepts, usage, and features, read through the following articles:
- 📚 [Official `tracing` crate docs][`tracing`]
- 📰 [Joshua Mo: Getting Started with Tracing in Rust][13]⏱0.25h
- 📰 [Yoav Danieli: Guide to OpenTelemetry Distributed Tracing in Rust][11]
- 📰 [Tokio Blog: Diagnostics with Tracing][14]⏱0.5h
- 📰 [Hayden Stainsby: debugging tokio instrumentation][15]⏱0.25h




## Task

__Estimated time__: 1 day




Implement two loggers:
1. Global main `app.log` logger which prints all its logs to `STDOUT`, but `WARN` level (and higher) logs to `STDERR`.
2. Local `access.log` logger which writes all its logs to `access.log` file.

All logs should be structured and logged in a JSON format, and have time field with nanoseconds (📰 [RFC 3339] formatted).

Examples:
```json
{"lvl":"ERROR","file":"app.log","time":"2018-07-30T12:14:14.196483657Z","msg":"Error occurred"}
{"lvl":"INFO","file":"access.log","time":"2018-07-30T12:17:18.721127239Z","msg":"http","method":"POST","path":"/some"}
```




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
1. How does 📚 [`log`] crate achieve its reusability over ecosystem? What are the ideas behind it?
2. Why logging is preferred over printing (`println!` usage)? When it's not?
3. What is structured logging? What benefits does it provide?
4. Why 📚 [`tracing`] crate is good for logging? What makes it preferred over 📚 [`slog`] and 📚 [`log`] crates?
5. What is tracing? Why is it beneficial for observability?




[`log`]: https://docs.rs/log
[`serde`]: https://docs.rs/serde
[`slog`]: https://docs.rs/slog
[`tracing`]: https://docs.rs/tracing
[coz]: https://github.com/plasma-umass/coz
[Jaeger]: https://www.jaegertracing.io
[OpenTelemetry]: https://opentelemetry.io
[Rust]: https://www.rust-lang.org
[RFC 3339]: https://www.ietf.org/rfc/rfc3339.txt
[Sentry]: https://sentry.io
[Tracy]: https://github.com/wolfpld/tracy
[Zipkin]: https://zipkin.io

[1]: https://en.wikipedia.org/wiki/Metaprogramming
[3]: https://docs.rs/log/#compile-time-filters
[4]: https://web.archive.org/web/20170527175640/https://dzone.com/articles/what-is-structured-logging
[5]: https://github.com/slog-rs/slog/wiki/What-makes-slog-fast
[6]: https://github.com/slog-rs/slog/wiki/FAQ
[7]: https://docs.rs/tracing#spans
[8]: https://docs.rs/tracing#log-compatibility
[9]: https://www.reddit.com/r/rust/comments/kdo29n/slog_vs_tracing_which_one_do_you_prefer
[10]: https://en.wikipedia.org/wiki/Tracing_(software)
[11]: https://www.aspecto.io/blog/distributed-tracing-with-opentelemetry-rust
[12]: https://www.thecodedmessage.com/posts/logging
[13]: https://www.shuttle.rs/blog/2024/01/09/getting-started-tracing-rust
[14]: https://tokio.rs/blog/2019-08-tracing
[15]: https://hegdenu.net/posts/debugging-tokio-instrumentation
