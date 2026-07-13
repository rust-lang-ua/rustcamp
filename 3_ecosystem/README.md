Chapter 3: Common ecosystem
========================

These tasks describe common crates and tools in 📚 [Rust]⏱0.25h ecosystem required for application and library development.

> ❗️Before completing this task you should complete all its sub-tasks.

After doing them you should be able to answer the following questions:
1. What testing capabilities does 📚 [Rust]⏱0.25h offer and when should I use them? Why should I follow 📰 [BDD]⏱0.25h style?
2. What are macros? How do they differ? What benefits does their usage give? When should I write one?
3. How to work with date and time in 📚 [Rust]⏱0.25h? How should I store time? How should I return it to other applications?
4. How are regular expressions used in 📚 [Rust]⏱0.25h? When are they not enough? How can I write a custom parser in 📚 [Rust]⏱0.25h?
5. How do iterator and collection compare and differ in 📚 [Rust]⏱0.25h? What is the purpose of immutable collections? Why should I care about using concurrent collections?
6. What should I use for serialization in 📚 [Rust]⏱0.25h? Why this is good or bad?
7. How can I generate randomness in 📚 [Rust]⏱0.25h? Which guarantees of random generator should I choose and when?
8. What should I use for password hashing in 📚 [Rust]⏱0.25h? How can I encrypt a message with 📚 [Rust]⏱0.25h? How should I compare secret values and why?
9. How logging is organized in 📚 [Rust]⏱0.25h ecosystem? Why should I care about structured logging?
10. What should I use for building 📰 [CLI]⏱1.5h interface in 📚 [Rust]⏱0.25h? How can I organize a configuration for my application and why?
11. Why multithreading is required for 📚 [Rust]⏱0.25h programs and what problems does it solve? How threads concurrency differs with parallelism? How can I parallelize code in 📚 [Rust]⏱0.25h?
12. What is asynchronicity and what problems does it solve? How is it compared to threads concurrency? What is 📚 [Rust]⏱0.25h solution for asynchronicity and why it has such design?
13. What are actors? When are they useful?


## Some usefull tools

- [cross-rs/cross](https://github.com/cross-rs/cross)
- [cargo-hack](https://github.com/taiki-e/cargo-hack)
- [Miri: unsafe core interpreter](https://github.com/rust-lang/miri)
- [cargo-outdated](https://crates.io/crates/cargo-outdated)
- [cargo-modules](https://github.com/regexident/cargo-modules)
- [cargo-make](https://github.com/sagiegurari/cargo-make)
- [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [Rust Developer's Toolbox](https://github.com/rust-lang-ua/learn_rust_together/blob/master/toolbox_general.md)

## Task

__Estimated time__: 2 days




Write a 📰 [CLI]⏱1.5h tool for stripping 📰 [JPEG]⏱2h images 📰 [metadata][21]⏱0.25h and minimizing their size (a simplified analogue of 📰 [tinyjpg.com]⏱0.25h).

Requirements:
- Accept input list of files and remote 📰 [URL]⏱0.5hs via: either 📰 [CLI]⏱1.5h arguments, 📰 [STDIN]⏱0.5h, or read it from a specified file (📰 [EOL]⏱0.75h-separated).
- Allow configuring how much images are processed at the same time.
- Allow configuring the output directory to store processed images in.
- Allow configuring the output 📰 [JPEG]⏱2h quality of processed images.
- Read configuration with ascending priority from: a file (format is on your choice), 📰 [environment variables][22]⏱1.5h, 📰 [CLI]⏱1.5h arguments. All are optional for specifying.
- Support `RUST_LOG` environment variable, allowing granular tuning of log levels per module.
- Print execution time in logs, so it's easy to see how much which operation takes during the execution.

If you have enough time after implementing base requirements, consider to add the following to your solution:
- Allow configuring download speed limit for images from remote 📰 [URL]⏱0.5hs.
- Cover your implementation with unit and E2E tests.
- Support 📰 [PNG]⏱1.5h images as well.
- Add comprehensive documentation to your code.




[BDD]: https://en.wikipedia.org/wiki/Behavior-driven_development
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[EOL]: https://en.wikipedia.org/wiki/Newline
[JPEG]: https://en.wikipedia.org/wiki/JPEG
[PNG]: https://en.wikipedia.org/wiki/PNG
[Rust]: https://www.rust-lang.org
[STDIN]: https://en.wikipedia.org/wiki/Standard_streams#Standard_input_(stdin)
[tinyjpg.com]: https://tinyjpg.com
[URL]: https://en.wikipedia.org/wiki/URL

[21]: https://picvario.com/what-is-image-metadata-role-and-benefits
[22]: https://en.wikipedia.org/wiki/Environment_variable
