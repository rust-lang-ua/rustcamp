Chapter 3: Common ecosystem
========================

These tasks describe common crates and tools in [Rust] ecosystem required for application and library development.

> ❗️Before completing this task you should complete all its sub-tasks.

After doing them you should be able to answer the following questions:

- What testing capabilities does [Rust] offer and when should I use them? Why should I follow [BDD] style?
- What are macros? How do they differ? What benefits does their usage give? When should I write one?
- How to work with date and time in [Rust]? How should I store time? How should I return it to other applications?
- How are regular expressions used in [Rust]? When are they not enough? How can I write a custom parser in [Rust]?
- How do iterator and collection compare and differ in [Rust]? What is the purpose of immutable collections? Why should I care about using concurrent collections?
- What should I use for serialization in [Rust]? Why this is good or bad?
- How can I generate randomness in [Rust]? Which guarantees of random generator should I choose and when?
- What should I use for password hashing in [Rust]? How can I encrypt a message with [Rust]? How should I compare secret values and why?
- How logging is organized in [Rust] ecosystem? Why should I care about structured logging?
- What should I use for building [CLI] interface in [Rust]? How can I organize a configuration for my application and why?
- Why multithreading is required for [Rust] programs and what problems does it solve? How threads concurrency differs with parallelism? How can I parallelize code in [Rust]?
- What is asynchronicity and what problems does it solve? How is it compared to threads concurrency? What is [Rust] solution for asynchronicity and why it has such design?
- What are actors? When are they useful?

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

Write a [CLI] tool for stripping [JPEG] images [metadata][21] and minimizing their size (a simplified analogue of [tinyjpg.com]).

Requirements:

- Accept input list of files and remote [URL]s via: either [CLI] arguments, [STDIN], or read it from a specified file ([EOL]-separated).
- Allow configuring how much images are processed at the same time.
- Allow configuring the output directory to store processed images in.
- Allow configuring the output [JPEG] quality of processed images.
- Read configuration with ascending priority from: a file (format is on your choice), [environment variables][22], [CLI] arguments. All are optional for specifying.
- Support `RUST_LOG` environment variable, allowing granular tuning of log levels per module.
- Print execution time in logs, so it's easy to see how much which operation takes during the execution.

If you have enough time after implementing base requirements, consider to add the following to your solution:

- Allow configuring download speed limit for images from remote [URL]s.
- Cover your implementation with unit and E2E tests.
- Support [PNG] images as well.
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
