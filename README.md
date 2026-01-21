Rust Rustcamp
==============

> "It wasn't always so clear, but the Rust programming language is fundamentally about _empowerment_: no matter what kind of code you are writing now, Rust empowers you to reach farther, to program with confidence in a wider variety of domains than you did before."
_<div align="right">Rust Book's Foreword</div>_

In this rustcamp, you will participate in the [Rustcamp], a rigorous, step-by-step Rust certification program 🏆. The aim of this program is to shape you into a proficient **Strong Junior** Rust Developer. Indeed, with this rigorous training, you will find yourself better prepared than developers switching from other languages without a similar level of instruction. If you are a mid- or senior-level developer transitioning to Rust, you can confidently target **Middle-level** positions after achieving this certification.

| Description                     | Number |
|---------------------------------|-------:|
| Total applications to Rustcamp  |    812 |
| Total number of Participants    |    323 |
| Mentors    |    47 |
| Certified Professionals         |     73 |

## Prerequisites

Before you begin the certification program, you should have a solid understanding of Rust and Git/GitHub. We strongly recommend completing the [Rust Book]. It's also beneficial if you have completed [Rustlings] and [Exercism], and have some hands-on practice. If you're transitioning from another programming language, you'll also find this certification program particularly useful.

## Process

Participants in the boot camp will be divided into closely-knit small groups. Meetings within these groups will occur four times a week, excluding weekends. These meetings are an opportunity for dynamic discussion, mutual learning, and answering any questions you might have. Additionally, participants can expect to answer insightful questions posed by their mentors, fostering a proactive learning environment.

Our learning process is designed around daily skill enhancement, leveraging the highest quality learning materials available. We provide an interactive platform for tracking progress, ensuring a clear path of advancement and accomplishment.

Participants will also be challenged with designing, developing, and presenting a capstone project. This project provides a fantastic opportunity to apply the skills learned throughout the boot camp in a practical, impactful way.

Finally, a comprehensive assessment will be conducted at the end of the certification program. Upon successful completion, participants will earn a certificate. This certificate is more than just a document; it's a testament to your ability to utilize best practices and write idiomatic code that employers will value highly. Imagine the doors that could open with this certificate in hand!

## Getting Started

Please, read [instructions][Getting Started].

## Submitting Solutions

Please, read [instructions][Submitting Solutions].

### Toolchain

- [rustup] for installing [Rust] toolchain and keeping it up-to-date.
- [RustRover] as development environment (or any other on your choice).


### Bookshelf

- [Rust Book] teaches and explains [Rust] basics.
- [Rust By Example] teaches you [Rust] basics using editable examples.
- [Rust Reference] is not a formal spec, but is more detailed and comprehensive than the [Rust Book].
- [Cheats.rs] and [Rust SVG Cheatsheet] for quick reference.
- [Rust Edition Guide] for considering improvements of [Rust 2018] and [Rust 2021].
- [Rust std lib] documentation.
- [Cargo Book] is a guide to [Cargo], [Rust]'s build tool and dependency manager.
- [Rustdoc Book] is a guide to `rustdoc` documentation tool.
- [Rust Cookbook] is a collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the [Rust] ecosystem.
- [Rust Design Patterns] is an open source repository of [Rust] design patterns and idioms.
- [Effective Rust] is a collection of guidelines that had been learned from real world experience of creating software in [Rust].
- [Rust API Guidelines] is a set of recommendations on how to design and present APIs for [Rust].
- [Rust FAQ] answers common question about [Rust].
- [Rust Playground] allows to share and check runnable [Rust] code snippets online.
- [Awesome Rust] is a curated list of [Rust] code and resources.
- [This Week in Rust] represents handpicked and subscribable [Rust] weekly updates.
- [Baby Steps] blog of [Nicholas Matsakis](https://github.com/nikomatsakis) shares useful [Rust] patterns, ideas and design decisions.




### Schedule

Each task must be performed as a separate [PR (pull request)][PR] with an appropriate name and checkmarked here in README's schedule after completion. Each task is a [Cargo workspace member][13], so you can run/test it from the project root (i.e. `cargo run -p task_1_8`). __Consider to use [rustfmt] and [Clippy] when you're writing [Rust] code.__

Each task has the estimated time for completion. If any deeper investigation on task's theme is needed by you, then it's on your own.

Do not hesitate to ask your mentor/lead with questions, however you won't receive any concrete answer, but rather a direction for your own investigation. _Mentor/Lead is the one who asks questions here, demanding concrete and complete answers._

## Curriculum

- [ ] [0. Become familiar with Rust basics][Task 0] (1 week)
- [ ] [1. Concepts][Task 1] (2 weeks)
    - [ ] [1.1. Default values, cloning and copying][Task 1.1]
    - [ ] [1.2. Boxing and pinning][Task 1.2]
    - [ ] [1.3. Shared ownership and interior mutability][Task 1.3]
    - [ ] [1.4. Clone-on-write][Task 1.4]
    - [ ] [1.5. Conversions, casting and dereferencing][Task 1.5]
    - [ ] [1.6. Static and dynamic dispatch][Task 1.6]
    - [ ] [1.7. `Sized` and `?Sized` types][Task 1.7]
    - [ ] [1.8. Thread safety][Task 1.8]
    - [ ] [1.9. Phantom types][Task 1.9]
    - [ ] [1.10. Summary Task][Task 1 Summary Task]
- [ ] [2. Idioms][Task 2] (2 weeks)
    - [ ] [2.1. Rich types ensure correctness][Task 2.1]
    - [ ] [2.2. Swapping values with `mem::replace`][Task 2.2]
    - [ ] [2.3. Bound behavior, not data][Task 2.3]
    - [ ] [2.4. Abstract type in, concrete type out][Task 2.4]
    - [ ] [2.5. Exhaustivity][Task 2.5]
    - [ ] [2.6. Sealing][Task 2.6]
    - [ ] [2.7. Summary Task][Task 2 Summary Task]
- [ ] [3. Ecosystem][Task 3] (3 weeks)
    - [ ] [3.1. Testing and mocking][Task 3.1]
    - [ ] [3.2. Declarative and procedural macros][Task 3.2]
    - [ ] [3.3. Date and time][Task 3.3]
    - [ ] [3.4. Regular expressions and custom parsers][Task 3.4]
    - [ ] [3.5. Collections and iterators][Task 3.5]
    - [ ] [3.6. Serialization and deserialization][Task 3.6]
    - [ ] [3.7. Randomness and cryptography][Task 3.7]
    - [ ] [3.8. Logging and tracing][Task 3.8]
    - [ ] [3.9. Command-line arguments, environment variables and configs][Task 3.9]
    - [ ] [3.10. Multithreading and parallelism][Task 3.10]
    - [ ] [3.11. Async I/O, futures and actors][Task 3.11]
    - [ ] [3.12. Summary Task][Task 3 Summary Task]
- [ ] [4. Backend ecosystem][Task 4] (2 weeks)
    - [ ] [4.1. Databases, connection pools and ORMs][Task 4.1]
    - [ ] [4.2. HTTP servers and clients][Task 4.2]
    - [ ] [4.3. API servers, clients and tools][Task 4.3]
    - [ ] [4.4. gRPC][Task 4.4]
    - [ ] [4.5. Summary Task][Task 4 Summary Task]
- [ ] [Agree your capstone project][Task 6.1]
- [ ] [5. Zero To Production][Task 5] (3 weeks)
    - [ ] [5.1. Basic actix-web][Task 5.1]
    - [ ] [5.2. Logging][Task 5.2]
    - [ ] [5.3. Docker and deployment][Task 5.3]
    - [ ] [5.4. Type-Driven Development and testing][Task 5.4]
    - [ ] [5.5. Advanced actix-web and error handling][Task 5.5]
    - [ ] [5.6. Authorization][Task 5.6]
- [ ] [6. Capstone Project][Task 6] (1 week)
    - [ ] [6.1. Agree your capstone project][Task 6.1]
    - [ ] [6.2. Get approvement of basic implementation from the first mentor][Task 6.2]
    - [ ] [6.3. Get 4 reviews from peers and mentors][Task 6.3]
    - [ ] [6.4. Present your project][Task 6.4]

## Useful links

- 🧭 [Orientation] - Maximize your rustcamp experience with these tips.
- ⏩ [Getting Started][Getting Started] - Kick off your learning journey here.
- 🎓 [Exercism] - Dive into coding exercises with the guidance of mentors.
- 📚 [Learning Materials][Learning Materials] - A curated collection of top-notch Rust learning materials.
- 🔧 [Ecosystem][Ecosystem] - A curated collection of frequently used crates, representing essential parts of the Rust ecosystem.
- 🌐 [Zero To Production] - A back-end focused book to broaden your knowledge.
  - Access the book as a [series of articles][Zero To Production as a series of articles].
- ❔ [FAQ] - Answers to frequently asked questions at your fingertips.
- 🔄 [Progress Board] - Participant progress board
- 🚀 [Graduates' Capstone Projects] - Graduates' сapstone зrojects
  
## Too hard?

- 👣 [Step-by-Step Introduction] - A gradual introduction to Rust programming.
- 🐣 [Rustlings] - Small exercises designed to familiarize you with reading and writing Rust code.
- 📘 [Rust Book] - An indispensable guide to Rust. While it's a must-have starting point, we strongly encourage revisiting it throughout your learning journey.
<!-- - 🗂️ Awesome List of [Learning Materials on Git][Learning Materials on Git] - Essential resources for mastering Git. -->

## Credits

Rustcamp materials were based on the [Rust Incubator](https://github.com/instrumentisto/rust-incubator) program created by the Legendary [Kai](https://github.com/tyranron) 💜 and Member of the Jedi High Council [Luca Palmieri](https://github.com/LukeMathWalker) 🧙‍♂️

[Task 0]: 0_vocabulary
[Task 1]: 1_concepts
[Task 1.1]: 1_concepts/1_1_default_clone_copy
[Task 1.2]: 1_concepts/1_2_box_pin
[Task 1.3]: 1_concepts/1_3_rc_cell
[Task 1.4]: 1_concepts/1_4_cow
[Task 1.5]: 1_concepts/1_5_convert_cast_deref
[Task 1.6]: 1_concepts/1_6_dispatch
[Task 1.7]: 1_concepts/1_7_sized
[Task 1.8]: 1_concepts/1_8_thread_safety
[Task 1.9]: 1_concepts/1_9_phantom
[Task 1 Summary Task]: 1_concepts/README.md#task
[Task 2]: 2_idioms
[Task 2.1]: 2_idioms/2_1_type_safety
[Task 2.2]: 2_idioms/2_2_mem_replace
[Task 2.3]: 2_idioms/2_3_bound_impl
[Task 2.4]: 2_idioms/2_4_generic_in_type_out
[Task 2.5]: 2_idioms/2_5_exhaustivity
[Task 2.6]: 2_idioms/2_6_sealing
[Task 2 Summary Task]: 2_idioms/README.md#task
[Task 3]: 3_ecosystem
[Task 3.1]: 3_ecosystem/3_1_testing
[Task 3.2]: 3_ecosystem/3_2_macro
[Task 3.3]: 3_ecosystem/3_3_date_time
[Task 3.4]: 3_ecosystem/3_4_regex_parsing
[Task 3.5]: 3_ecosystem/3_5_collections
[Task 3.6]: 3_ecosystem/3_6_serde
[Task 3.7]: 3_ecosystem/3_7_rand_crypto
[Task 3.8]: 3_ecosystem/3_8_log
[Task 3.9]: 3_ecosystem/3_9_cmd_env_conf
[Task 3.10]: 3_ecosystem/3_10_threads
[Task 3.11]: 3_ecosystem/3_11_async
[Task 3 Summary Task]: 3_ecosystem/README.md#task
[Task 4]: 4_backend
[Task 4.1]: 4_backend/4_1_db
[Task 4.2]: 4_backend/4_2_http
[Task 4.3]: 4_backend/4_3_api
[Task 4.4]: 4_backend/4_4_grpc
[Task 4 Summary Task]: 4_backend/README.md#task
[Task 5]: 5_zero2prod
[Task 5.1]: 5_zero2prod/3_chapter
[Task 5.2]: 5_zero2prod/4_chapter
[Task 5.3]: 5_zero2prod/5_chapter
[Task 5.4]: 5_zero2prod/6_chapter
[Task 5.5]: 5_zero2prod/7_chapter
[Task 5.6]: 5_zero2prod/10_chapter
[Task 6]: 6_project
[Task 6.1]: 6_project/README.md#task-61-agree-on-your-capstone-project
[Task 6.2]: 6_project/README.md#task-62-get-approvement-of-basic-implementation-from-the-first-mentor
[Task 6.3]: 6_project/README.md#task-63-get-4-reviews-from-peers-and-mentors
[Task 6.4]: 6_project/README.md#task-64-present-your-project

[Awesome Rust]: https://github.com/rust-unofficial/awesome-rust
[Baby Steps]: http://smallcultfollowing.com/babysteps
[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Cheats.rs]: https://cheats.rs
[RustRover]: https://www.jetbrains.com/rust/
[Effective Rust]: https://www.lurklurk.org/effective-rust
[GitHub repository]: https://help.github.com/articles/github-glossary/#repository
[PR]: https://help.github.com/articles/github-glossary/#pull-request
[workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[Rust]: https://www.rust-lang.org
[Rust 2018]: https://doc.rust-lang.org/edition-guide/rust-2018/index.html
[Rust 2021]: https://doc.rust-lang.org/edition-guide/rust-2021/index.html
[Rust API Guidelines]: https://rust-lang.github.io/api-guidelines
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust Cookbook]: https://rust-lang-nursery.github.io/rust-cookbook
[Rust Design Patterns]: https://rust-unofficial.github.io/patterns
[Rust Edition Guide]: https://doc.rust-lang.org/edition-guide
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[Rust Playground]: https://play.rust-lang.org
[Rust Reference]: https://doc.rust-lang.org/reference
[Rust std lib]: https://doc.rust-lang.org/std
[Rust SVG Cheatsheet]: https://www.breakdown-notes.com/make/load/rust_cs_canvas/true
[Rustdoc Book]: https://doc.rust-lang.org/rustdoc
[Clippy]: https://github.com/rust-lang/rust-clippy
[rustfmt]: https://github.com/rust-lang/rustfmt
[Rustcamp ]: https://github.com/rust-lang-ua/rustcamp
[Fork As Template]: https://help.github.com/en/articles/creating-a-repository-from-a-template
[rustup]: https://rustup.rs
[This Week in Rust]: https://this-week-in-rust.org
[bot]: https://github.com/1tbot
[how to fork - step by step]: ./how_to_fork.md

[Orientation]: ./orientation.md
[Getting Started]: ./orientation.md#getting-started
[Submitting Solutions]: ./orientation.md#submitting-solutions
[Exercism]: https://exercism.org/tracks/rust
[Rust Quiz]: https://github.com/dtolnay/rust-quiz
[Learning Materials]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/learn.md
[Ecosystem]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/toolbox_general.md
[Zero To Production]: https://www.zero2prod.com/index.html?country=Ukraine&discount_code=EEU60
[Zero To Production as a series of articles]: ./backend_book.md
[FAQ]: ./faq.md
[Step-by-Step Introduction]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/introduction.md
[Rustlings]: https://github.com/rust-lang/rustlings
[Learning Materials on Git]: https://github.com/Learn-Together-Pro/LearnGitTogether
[Rust Book]: https://doc.rust-lang.org/book
[Progress Board]: https://github.com/rust-lang-ua/rustcamp_progress/blob/master/README.md
[Graduates' Capstone Projects]: https://github.com/rust-lang-ua/rustcamp_projects


[13]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
