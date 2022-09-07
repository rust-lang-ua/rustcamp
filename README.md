Rust Incubator
==============

> It wasn’t always so clear, but the Rust programming language is fundamentally about _empowerment_: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before.
_<div align="right">Rust Book's Foreword</div>_

This project represents a hard-way step-by-step [Rust] learning course from language basics to a capability of web backend development.


## Steps


### Before you start

1. [Create][1] a new [GitHub repository] for yourself using this one [as template][2].
2. [Invite as a collaborator][3] of your repository the person you want to review your lessons (lead).

## Additional information

Rust books, articles, libraries and other additinal information repository [learn_rust_together](https://github.com/rust-lang-ua/learn_rust_together).

### Schedule

Each step must be performed as a separate [PR (pull request)][PR] with an appropriate name and checked here in README's schedule after completion. Each step is a [Cargo workspace member][4], so you can run/test it from a project root (i.e. `cargo run -p step_1_8`). Consider to use [rustfmt] and [Clippy] when you're writing [Rust] code.

Each step has estimated time for being completed. If any deeper investigation on step's theme is required for you, then it's on your own.

Do not hesitate to ask your lead with questions, however you won't receive a concrete answer, but rather a direction for investigation. _Lead is the one who asks questions about everything here and demands a concrete answers_.

- [X] [0. Become familiar with Rust basics][Step 0] (3 days)
- [ ] [1. Concepts][Step 1] (2 days)
    - [ ] [1.1. Default values, cloning and copying][Step 1.1] (1 day)
    - [ ] [1.2. Boxing and pinning][Step 1.2] (1 day)
    - [ ] [1.3. Shared ownership and interior mutability][Step 1.3] (1 day)
    - [ ] [1.4. Clone-on-write][Step 1.4] (1 day)
    - [ ] [1.5. Conversions, casting and dereferencing][Step 1.5] (1 day)
    - [ ] [1.6. Static and dynamic dispatch][Step 1.6] (1 day)
    - [ ] [1.7. `Sized` and `?Sized` types][Step 1.7] (1 day)
    - [ ] [1.8. Thread safety][Step 1.8] (1 day)
    - [ ] [1.9. Phantom types][Step 1.9] (1 day)
- [ ] [2. Idioms][Step 2] (2 days)
    - [ ] [2.1. Rich types ensure correctness][Step 2.1] (1 day)
    - [ ] [2.2. Swapping values with `mem::replace`][Step 2.2] (1 day)
    - [ ] [2.3. Bound behavior, not data][Step 2.3] (1 day)
    - [ ] [2.4. Abstract type in, concrete type out][Step 2.4] (1 day)
- [ ] [3. Ecosystem][Step 3] (5 days)
    - [ ] [3.1. Testing and mocking][Step 3.1] (1 day)
    - [ ] [3.2. Declarative and procedural macros][Step 3.2] (1 day)
    - [ ] [3.3. Date and time][Step 3.3] (1 day)
    - [ ] [3.4. Regular expressions and custom parsers][Step 3.4] (1 day)
    - [ ] [3.5. Collections and iterators][Step 3.5] (1 day)
    - [ ] [3.6. Serialization and deserialization][Step 3.6] (1 day)
    - [ ] [3.7. Randomness and cryptography][Step 3.7] (1 day)
    - [ ] [3.8. Logging and tracing][Step 3.8] (1 day)
    - [ ] [3.9. Command-line arguments, environment variables and configs][Step 3.9] (1 day)
    - [ ] [3.10. Multithreading and parallelism][Step 3.10] (1 day)
    - [ ] [3.11. Async I/O, futures and actors][Step 3.11] (2 days)
    - [ ] [3.12. Web frameworks, databases, connection pools and ORMs][Step 3.12] (1 day)





[Step 0]: 0_basics
[Step 1]: 1_concepts
[Step 1.1]: 1_concepts/1_1_default_clone_copy
[Step 1.2]: 1_concepts/1_2_box_pin
[Step 1.3]: 1_concepts/1_3_rc_cell
[Step 1.4]: 1_concepts/1_4_cow
[Step 1.5]: 1_concepts/1_5_convert_cast_deref
[Step 1.6]: 1_concepts/1_6_dispatch
[Step 1.7]: 1_concepts/1_7_sized
[Step 1.8]: 1_concepts/1_8_thread_safety
[Step 1.9]: 1_concepts/1_9_phantom
[Step 2]: 2_idioms
[Step 2.1]: 2_idioms/2_1_type_safety
[Step 2.2]: 2_idioms/2_2_mem_replace
[Step 2.3]: 2_idioms/2_3_bound_impl
[Step 2.4]: 2_idioms/2_4_generic_in_type_out
[Step 3]: 3_ecosystem
[Step 3.1]: 3_ecosystem/3_1_testing
[Step 3.2]: 3_ecosystem/3_2_macro
[Step 3.3]: 3_ecosystem/3_3_date_time
[Step 3.4]: 3_ecosystem/3_4_regex_parsing
[Step 3.5]: 3_ecosystem/3_5_collections
[Step 3.6]: 3_ecosystem/3_6_serde
[Step 3.7]: 3_ecosystem/3_7_rand_crypto
[Step 3.8]: 3_ecosystem/3_8_log
[Step 3.9]: 3_ecosystem/3_9_cmd_env_conf
[Step 3.10]: 3_ecosystem/3_10_threads
[Step 3.11]: 3_ecosystem/3_11_async
[Step 3.12]: 3_ecosystem/3_12_web_db

[Awesome Rust]: https://github.com/rust-unofficial/awesome-rust
[Baby Steps]: http://smallcultfollowing.com/babysteps
[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Cheats.rs]: https://cheats.rs
[CLion]: https://www.jetbrains.com/clion
[Clippy]: https://github.com/rust-lang/rust-clippy
[Effective Rust]: https://www.lurklurk.org/effective-rust
[GitHub repository]: https://help.github.com/articles/github-glossary/#repository
[IntelliJ IDEA]: https://www.jetbrains.com/idea
[IntelliJ Rust]: https://intellij-rust.github.io
[IntelliJ Toml]: https://plugins.jetbrains.com/plugin/8195-toml
[PR]: https://help.github.com/articles/github-glossary/#pull-request
[Rust]: https://www.rust-lang.org
[Rust 2018]: https://doc.rust-lang.org/edition-guide/rust-2018/index.html
[Rust 2021]: https://doc.rust-lang.org/edition-guide/rust-2021/index.html
[Rust API Guidelines]: https://rust-lang.github.io/api-guidelines
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust Cookbook]: https://rust-lang-nursery.github.io/rust-cookbook
[Rust Design Patterns]: https://github.com/rust-unofficial/patterns
[Rust Edition Guide]: https://doc.rust-lang.org/edition-guide
[Rust FAQ]: https://www.rust-lang.org/faq.html
[Rust Playground]: https://play.rust-lang.org
[Rust Reference]: https://doc.rust-lang.org/reference
[Rust std lib]: https://doc.rust-lang.org/std
[Rust SVG Cheatsheet]: https://www.breakdown-notes.com/make/load/rust_cs_canvas/true
[Rustdoc Book]: https://doc.rust-lang.org/rustdoc
[rustfmt]: https://github.com/rust-lang/rustfmt
[rustup]: https://rustup.rs
[This Week in Rust]: https://this-week-in-rust.org

[1]: https://github.com/instrumentisto/rust-incubator/generate
[2]: https://help.github.com/en/articles/creating-a-repository-from-a-template
[3]: https://help.github.com/en/articles/inviting-collaborators-to-a-personal-repository
[4]: https://prev.rust-lang.org/book/ch14-03-cargo-workspaces.html
