Chapter 0: Building Up a Vocabulary
========================================

__Estimated time__: 4 days

> ❗️ To meet the first deadline, please open a Pull Request by Sunday of  the first week and include answers to at least one question in it.

Read through 📚 [Rust Book] and become familiar with basic [Rust] concepts, syntax, memory model, type and module systems.

Polish your familiarity by completing 📚 [Rust By Example] and 📰 [rustlings].

Read through 📚 [Cargo Book] and become familiar with 📰 [Cargo] and its workspaces.

The table below maps Rust concepts to their closest analogue elsewhere, so you can anchor new ideas to what you already know. Treat these as starting points, not equivalences - the "closest analogue" column is often where the differences matter most.

| Rust concept                    | C / C++                                                | Java / C# / Go (GC'd)                                     | Python / JS / Ruby (dynamic)                    |
| ------------------------------- | ------------------------------------------------------ | --------------------------------------------------------- | ----------------------------------------------- |
| Ownership & move semantics      | manual `new`/`delete`, `std::move`                     | no equivalent - GC owns everything                        | no equivalent - GC/refcounting owns everything  |
| Borrowing (`&`, `&mut`)         | raw pointers/references, no compile-time checks        | references (always shared, no exclusivity rule)           | references (always shared, no exclusivity rule) |
| RAII / `Drop`                   | destructors, RAII (C++)                                | `try`-with-resources (Java), `using` (C#), `defer` (Go)   | context managers (`with`, Python), `finally`    |
| Lifetimes                       | nothing enforced - dangling pointers are UB            | GC makes this moot                                        | GC makes this moot                              |
| Traits                          | abstract base classes, concepts (C++20)                | interfaces                                                | duck typing, protocols (Python), mixins         |
| `enum` (sum types / ADTs)       | tagged unions (manual), `std::variant`                 | sealed classes (Java/Kotlin-ish), no true ADT in C#/Go    | no true equivalent - usually classes or dicts   |
| `Option<T>` / `Result<T, E>`    | `nullptr`/error codes, `std::optional`/`std::expected` | `null` + exceptions, `Optional<T>` (Java)                 | `None` + exceptions                             |
| Static vs dynamic dispatch      | virtual functions (dynamic), templates (static)        | interfaces are dynamic dispatch by default                | always dynamic dispatch                         |
| Macros (declarative/procedural) | preprocessor macros, templates                         | annotations/reflection (no real macros)                   | decorators, metaclasses                         |
| Crate / module / package        | translation unit / namespace / library                 | package/namespace + build artifact                        | module/package + package manager entry          |
| Generics & monomorphization     | templates (compile-time, similar)                      | generics via type erasure (Java) or reified (C#/Go 1.18+) | not applicable - dynamic typing                 |

After completing these steps, you should be able to answer (and understand why) the following questions:

1. What 📰 [memory model][31] [Rust] has? Is [Rust] single-threaded or multi-threaded? Is it synchronous or asynchronous? What are the memory layouts of `Box` and `Vector`? What are a heap and a stack? Where, but on heap and stack data could live in RAM?
2. What runtime [Rust] has? Does it use a GC (garbage collector)?
3. What is special about slice? What is the layout of Rust standard data types? Difference between fat and thin pointers?
4. Why does [Rust] have `&str` and `String` types? How do they differ? When should you use them? Why str slice coexists with slice? What is the difference between `String` and `Vec`?
5. What static typing means? What are the benefits of using it? Weak vs strong typing? Implicit vs explicit typing?
6. What are generics and parametric polymorphism? Which problems do they solve?
7. What are nominative typing and structural typing? What is the difference?
8. What are traits? How are they used? How do they compare to interfaces? What are auto trait and blanket impl? Uncovered type? What are marker traits?
9. What are static and dynamic dispatches? Which should you use, and when? What is monomorphization?
10. What are a crate, a module, and a package in Rust? How do they differ? How are they used? What is a workspace?
11. What is cloning? What is copying? How do they compare? What is trait Drop for? What is special about this trait?
12. What is immutability? What is the benefit of using it? What is the difference between immutability and const?
13. What are move semantics? What are borrowing rules? What is the benefit of using them?
14. What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
15. What are lifetimes? Which problems do they solve? Which benefits do they provide?
16. What is an iterator? What is a collection? How do they differ? How are they used?
17. What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
18. How code is tested in [Rust]? Where should you put tests and why?
19. Is [Rust] an OOP language? Is it possible to use SOLID/GRASP? Does it have inheritance? Is Rust a functional language? What variance rules does Rust have?

After you are done, notify your lead in an appropriate PR (pull request), and they will examine what you have learned.

_Additional_ articles, which may help to understand the above topic better:

| Title | Author | Tags |
| --- | --- | --- |
| 📰 [Rust ownership, the hard way][1] | Chris Morgan | type:article |
| 📰 [You are holding it wrong][23] | Adolfo Ochagavía | type:article |
| 📰 [Beyond Pointers: How Rust outshines C++ with its Borrow Checker][30] | Vikram Fugro | type:article |
| 🧭 [A guide to closures in Rust][24] | HashRust | type:guide |
| 📰 [Rusts Module System Explained][2] | Ludwig Stecher | type:article |
| 📰 [Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3] | Tristan Hume | type:article |
| 📰 [Generics Demystified Part 1][4] | Jeff Anderson | type:article |
| 📰 [Generics Demystified Part 2][5] | Jeff Anderson | type:article |
| 📰 [Demystifying trait generics in Rust][25] | Bradford Hovinen | type:article |
| 📰 [Three Kinds of Polymorphism in Rust][6] | Brandon Smith | type:article |
| 📰 [C++ & Rust: Generics and Specialization][7] | Jeremy Steward | type:article |
| 📰 [&stress about &Strings][8] | cooscoos | type:article |
| 📰 [RAII: Compile-Time Memory Management in C++ and Rust][9] | Jimmy Hartzell | type:article |
| 📰 [Trait Drop][10] | — | type:article |
| 📰 [Common Lifetime Misconception][11] | — | type:article |
| 🎥 [Visualizing Memory Layout][12] | — | type:video |
| 📚 [Packages and crates (Rust wiki)][14] | — | type:reference |
| 📋 [Full list of available crates on Rust Playground][16] | — | type:list |
| 📚 [Blanket impl definition][17] | — | type:reference |
| 📚 [Auto-trait definition][18] | — | type:reference |
| 📰 [Rust vs Common C++ Bugs][21] | Georgios Antonopoulos | type:article |
| 📰 [True Observer Pattern with Unsubscribe mechanism using Rust][22] | Yurii Shymon | type:article |
| 🧭 [Rust API guidline checklist][19] | — | type:guide |
| 🧭 [Step-by-step instruction to start development in Rust][26] | — | type:guide |
| 📋 [Awesome collection of crates for productive development in Rust][27] | — | type:list |
| 📋 [Awesome Collection of Materials to Learn Rust][28] | — | type:list |

[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[rustlings]: https://rustlings.cool

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://web.archive.org/web/20251117032322/https://cooscoos.github.io/blog/stress-about-strings/
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://vojtechkral.github.io/blag/rust-drop-order/
[11]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md
[12]: https://youtu.be/7_o-YRxf_cc?si=gSVQ6wWSnr2le6rc
[14]: https://rustwiki.org/en/book/ch07-01-packages-and-crates.html
[15]: https://blog.rust-lang.org/2022/08/05/nll-by-default/
[16]: https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Cargo.toml
[17]: https://doc.rust-lang.org/reference/glossary.html#blanket-implementation
[18]: https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits
[19]: https://rust-lang.github.io/api-guidelines/checklist.html
[21]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
[22]: https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust
[23]: https://ochagavia.nl/blog/you-are-holding-it-wrong
[24]: https://hashrust.com/blog/a-guide-to-closures-in-rust
[25]: https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust
[26]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/introduction.md
[27]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/toolbox_general.md
[28]: https://github.com/rust-lang-ua/learn_rust_together/blob/master/learn.md
[30]: https://dev.to/vikram2784/beyond-pointers-how-rust-outshines-c-with-its-borrow-checker-1mad
[31]: https://en.wikipedia.org/wiki/Memory_model_(programming)
