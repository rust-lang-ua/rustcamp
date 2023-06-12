Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 3 days

Read through [Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, memory model, type and module systems.

Polish your familiarity by completing [Rust By Example].

Read through [Cargo Book] and become familiar with [Cargo] and its workspaces.

After completing these steps, consider the following terms and questions. Try to answer and explain them providing useful context like examples, use cases are, comparisons.

## Basics
- What is Rust?
- What it is used for?
- Does it have a runtime?
- Is it compiled or interpreted?
- Pros and cons.

## Types & variables

- What primitive data types Rust has? What are their uses and properties?
- Name all the compound data types in Rust. Describe them.
- Constants, variables.
- Smart pointers.
- What is a type? *
- Static variables *

## Working with memory

- Stack and the heap.
- `Clone`, `Drop`, `Copy`.
- References, slices, lifetimes.
- Ownership and borrowing.
- Mutability.
- Pointers.
- Unsafe.

## Programming techniques (todo: naming)

- Expressions, statements.
- What is pattern matching?
- Where you can use it?
- Refutability.
- Matching operators.
- Dereferencing.
- Never type. *

## Error handling

- Runtime error.
- Compile time error.
- Error propagation, ? operator.
- Panic.
- Unwrap, when it's fine to use it.
- Useful shortcuts.

## Generics & Polymorphism

- Traits.
- Auto traits.
- Blanket impl, uncovered type. *
- Marker traits.
- Static dispatch VS dynamic dispatch.


## Bugs
Explain what these are, do you know how to avoid them?
- Memory leak
- Data race
- Use after free
- Deadlock
- Out of memory

todo: add some more (important) bugs

## Project structure

- Crates and modules.
- What is a workspace? Name the benefits of using workspaces.
- Visibility.
- How do you add dependencies to your project?



- What is an iterator? What is a collection? How do they differ? How are they used?
- What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?


- Why [Rust] has `&str` and `String` types? How do they differ? When should you use them? Why str slice coexist with slice? 
- Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance? Is Rust functional language?



After you're done notify your lead in an appropriate PR (pull request), and he will exam what you have learned.

_Additional_ articles, which may help to understand the above topic better:
- [Chris Morgan: Rust ownership, the hard way][1]
- [Adolfo Ochagavía: You are holding it wrong][23]
- [HashRust: A guide to closures in Rust][24]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [Trait Drop][10]
- [Common Lifetime Misconception][11]
- [Visualizing Memory Layout][12]
- [Package vs. Crate terminology (r/rust)][13]
- [Packages and crates (Rust wiki)][14]
- [Full list of available crates on Rust Playground][16]
- [Blanket impl definition][17]
- [Auto-trait definition][18]
- [Georgios Antonopoulos: Rust vs Common C++ Bugs][21]
- [Yurii Shymon: True Observer Pattern with Unsubscribe mechanism using Rust][22]

Additional:
- [Rust API guidline checklist][19]
- [Interview Questions on Rust Programming][20]

[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://vojtechkral.github.io/blag/rust-drop-order/
[11]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
[12]: https://www.youtube.com/watch?v=rDoqT-a6UFg
[13]: https://www.reddit.com/r/rust/comments/lvtzri/confused_about_package_vs_crate_terminology/
[14]: https://rustwiki.org/en/book/ch07-01-packages-and-crates.html
[16]: https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Cargo.toml
[17]: https://doc.rust-lang.org/reference/glossary.html#blanket-implementation
[18]: https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits
[19]: https://rust-lang.github.io/api-guidelines/checklist.html
[20]: https://iq.opengenus.org/questions-on-rust/
[21]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
[22]: https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust
[23]: https://ochagavia.nl/blog/you-are-holding-it-wrong
[24]: https://hashrust.com/blog/a-guide-to-closures-in-rust
