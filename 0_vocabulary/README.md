Chapter 0: Building Up a Vocabulary
========================================

__Estimated time__: 4 days

> ❗️ To meet the first deadline, please open a Pull Request by Sunday of  the first week and include answers to at least one question in it.

Read through 🧭 [Rust Book]⏱0.25h and become familiar with basic 🏠 [Rust] concepts, syntax, memory model, type and module systems.

Polish your familiarity by completing 🧭 [Rust By Example]⏱0.25h and 🏠 [rustlings].

Read through 🧭 [Cargo Book] and become familiar with 📚 [Cargo] and its workspaces.

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

1. What 📰 [memory model][31]⏱0.25h 🏠 [Rust] has? Is 🏠 [Rust] single-threaded or multi-threaded? Is it synchronous or asynchronous? What are the memory layouts of `Box` and `Vector`? What are a heap and a stack? Where, but on heap and stack data could live in RAM?


In the context of the wiki page provided, Rust has a C11 or C++11/20 memory model, according to Rustonomicon. Rust exposes next orderings: SeqCst, Release, Acquire, Relaxed, with SeqCst being the strictest; as I understand it, all read/write access happens in strict order. Acquire is paired with Release, and means that when a thread wants to access something, it requests to lock this resource. And releases it when all work with it is finished, for other threads to pick up. And Relaxed I'm less sure about, since we can't have multiple mutable accesses in Rust, but can have multiple read accesses.

Rust feels like both single- and multi-threaded. Because we can do everything in one thread no problem, and at the same time can spawn real OS threads with thread::spawn to do work there.

Core is sync; for it to work as async, we need an executor, like the "Tokio" crate, which changes the "main" fn in Rust to work asynchronously.

Box may have a pointer and len; Vec will have a pointer, len, and capacity, since we can add more elements to the Vec. And capacity is needed to extend it later. As I understand it doubles most commonly. So 1,2,4,8,16 and so on, starting with the base capacity we initialize it with.

We can only put things of known size into the stack. Like base types i32, etc. Or handles to the heap. And on the heap, we can put anything we're not sure about size, like Vec leaves on the heap exactly because its size is unknown and we can add new elements, while a "handle" to this Vec may live in the stack. Because we know it has a pointer, len, and capacity, 

Binary static memory, consts, and literals live there.


2. What runtime 🏠 [Rust] has? Does it use a GC (garbage collector)?


From what I understand, there is almost no runtime in Rust. Only some startup and panic handling. 

No GC in Rust, because it's not needed thanks to ownership rules and borrowing checkers in Rust. The lifetime of everything is known at compile time, and drop methods will be called automatically.


3. What is special about slice? What is the layout of Rust standard data types? Difference between fat and thin pointers?


Slice is a reference that has a pointer to the start of the sequence and its length. So we can read data without owning it.
Not sure what layout means in this context. But primitive types like i32 and bool are stored on the stack and are thin pointers, cause it is just a pointer and size known from the type itself.
Vec - fat pointer; it has a pointer, length, and capacity. Vec may live on the stack, while the data it is pointing to lives only on the heap.


4. Why does 🏠 [Rust] have `&str` and `String` types? How do they differ? When should you use them? Why str slice coexists with slice? What is the difference between `String` and `Vec`?


String and &str are very interesting) String is Vec<u8>, and &str - string slice. str is used to work with strings at a "low cost", so we don't clone it every time.
String slices and slices as well as String and Vec<u8> differ in a way that String and &str also ensure that the data inside is valid UTF-8, since one character may use 1-4 bytes.


5. What static typing means? What are the benefits of using it? Weak vs strong typing? Implicit vs explicit typing?


It means that we know all the types at compile time.
Benefits: we will never have a wrong type error at runtime.
Weak and strong: it's when we need exactly that type with strong, and with weak we can, for example, write something like "1" + 1 and it will autocast, and it will be valid(not the case for Rust)
Explicit is when we write types by ourselves; implicit is when, for example, the compiler picks it up from the function return type, and we don't need to specify it explicitly.


6. What are generics and parametric polymorphism? Which problems do they solve?


Generics and parametric polymorphism allow us to write code that works with any type implementing something, for example, so we don't need to write separate code for each type.
Well, not quite) For example, with generics, the compiler will create functions for each type implementing some Trait, and it will be part of the binary. 


7. What are nominative typing and structural typing? What is the difference?


Nominative is when we accept only a specific type. 
Structural is when we use "duck" typing: if it quacks like a duck, it's a duck. For example, in Rust, if we expect type Car {goose_power: i32}, we can't accept Bike {goose_power: i32} because it's different types, but in languages with structural typing, they can be used instead of each other since they have the same number of fields with the same types and names.


8. What are traits? How are they used? How do they compare to interfaces? What are auto trait and blanket impl? Uncovered type? What are marker traits?


Trait is Rust's version of interfaces, aka contracts. It describes methods(with or without some default implementation). And we can use it to write one implementation for all types that implement some trait.

Blanket implementation is like we can say that we implement some trait for each type that implements another trait. Like, for example, we can create a Logger trait with a log method, and implement it for each type that implements the std Display trait, since it may be displayed. So we can print it in the terminal.

Auto traits look like structural typing a little. Because these traits are implemented for types, all elements of which implement them. Unless we have a negative implement tag.

An uncovered type is T, for example. And Vec<T> - is covered type.

Marker traits are traits without fields or methods used to tag something. Like std Copy


9. What are static and dynamic dispatches? Which should you use, and when? What is monomorphization?


Static dispatch is T: Trait; dynamic is dyn Trait. The difference is that under T: Trait, we can only path one type at a time; dyn Trait allows mixing different types, all of which implement this Trait.


10. What are a crate, a module, and a package in Rust? How do they differ? How are they used? What is a workspace?


A crate is a compilation unit in Rust, like one file. A module is something like an internal organization inside a crate. Also - implements encapsulation with modules. A package is what is combined under one Cargo.toml. And a workspace is packages that are combined together under one Cargo.toml or higher level)


11. What is cloning? What is copying? How do they compare? What is trait Drop for? What is special about this trait?


Cloning is creating a copy of heap data and a pointer to this data. 
Copying is allowed on stack-only data. It is automatic since it's cheap. 
But cloning is not automatic and needs to be called explicitly, because it is not that cheap, since we are creating a copy of heap data. Also, we need a Drop trait implementation for cloning. Because after the owner goes out of scope, the drop function will be executed to drop data we will not use anymore and free memory. 


12. What is immutability? What is the benefit of using it? What is the difference between immutability and const?


Immutability means that the value cannot be changed after creation. 

The benefit is that we don't have to worry about the value ever changing because it is immutable. 

The difference, I would say, is where it is live in memory. All consts live in static memory and are part of the binary; data inside a variable may live on the stack or on the heap.


13. What are move semantics? What are borrowing rules? What is the benefit of using them?


Moving is passing ownership somewhere. Borrowing is allowing access(depending on the signature: mutable or immutable; can be only one mutable or however many immutable). Benefit: we can efficiently pass data to functions and such. If we don't need to move ownership, we can just let it borrow data and work with it. And after it finishes, we can use it with a variable that owns it. Also, when the owner goes out of scope, the drop is called.


14. What is RAII? How is it implemented in 🏠 [Rust]? What is the benefit of using it?


RAII is Resource Acquisition Is Initialization, which is strange because it is exactly what guarantees that whenever an object goes out ofscope, its destructor is called, and its owned resources are freed. Benefit: no leftover memory needs to be cleaned, and we don't need GC.


15. What are lifetimes? Which problems do they solve? Which benefits do they provide?


Lifetime is like a mark for the compiler that whatever we pass into the function should live for how much time. It checks at compile time that variables passed to the method should not go out of scope before it finishes and while the results are still used. So we won't have a null pointer exception at runtime, for example, as in Java)


16. What is an iterator? What is a collection? How do they differ? How are they used?


A collection owns values or references to values inside it. An iterator gets items one at a time on demand from the collection and returns an option, since we will eventually reach the end ot the iterating collection. An iterator is like a built-in mechanism to go over some collection without the need to use a traditional loop; for example, very comfortable when you get it, I would say.


17. What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?


Macros are like shortcuts to generate code. They are expanded at compile time and are not present in the resulting binary in their "macro" form. They just produce some Rust code.

Declarative is something like vec!. ```let v: Vec<u32> = vec![1, 2, 3];``` For example, we can create however big vec as we want with this macro. And it will return a Vec<u32> including all values. Something like push under it, I guess.

Procedural is one that creates Rust code. Like #[tokio::main] that changes the main function to be asynchronous. And executed at compile time to change it.


18. How code is tested in 🏠 [Rust]? Where should you put tests and why?


Unit tests -same file as the code. So we can test privates of it. Integration - separate directory and files, usually, to test the public API of our code.


19. Is 🏠 [Rust] an OOP language? Is it possible to use SOLID/GRASP? Does it have inheritance? Is Rust a functional language? What variance rules does Rust have?


Well, yes, but actually no. Rust is taking better from different paradigms. For example, it has everything from OOP except inheritance. Since it produces weird bugs when something breaks at some point of cascade inherited objects.
Totally possible, and some of the SOLID principles exist in Rust by design) Traits rules. 
I will be honest, I need to learn more about GRASP and functional languages' traits to understand the last part of this question. 


After you are done, notify your lead in an appropriate PR (pull request), and they will examine what you have learned.

_Additional_ articles, which may help to understand the above topic better:

| Title | Author |
| --- | --- |
| 📰 [Rust ownership, the hard way][1]⏱0.5h | Chris Morgan |
| 📰 [You are holding it wrong][23]⏱0.25h | Adolfo Ochagavía |
| 📰 [Beyond Pointers: How Rust outshines C++ with its Borrow Checker][30]⏱0.5h | Vikram Fugro |
| 🧭 [A guide to closures in Rust][24]⏱0.75h | HashRust |
| 📰 [Rusts Module System Explained][2]⏱0.25h | Ludwig Stecher |
| 📰 [Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]⏱0.5h | Tristan Hume |
| 📰 [Generics Demystified Part 1][4] | Jeff Anderson |
| 📰 [Generics Demystified Part 2][5] | Jeff Anderson |
| 📰 [Demystifying trait generics in Rust][25]⏱0.25h | Bradford Hovinen |
| 📰 [Three Kinds of Polymorphism in Rust][6]⏱0.25h | Brandon Smith |
| 📰 [C++ & Rust: Generics and Specialization][7]⏱0.5h | Jeremy Steward |
| 📰 [&stress about &Strings][8]⏱0.5h | cooscoos |
| 📰 [RAII: Compile-Time Memory Management in C++ and Rust][9]⏱0.75h | Jimmy Hartzell |
| 📰 [Trait Drop][10]⏱0.25h | — |
| 📰 [Common Lifetime Misconception][11]⏱0.75h | — |
| 🎥 [Crust of Rust: Subtyping and Variance][32]⏱1.75h | Jon Gjengset |
| 🎥 [Visualizing Memory Layout][12]⏱0.75h | — |
| 📚 [Packages and crates (Rust wiki)][14]⏱0.25h | — |
| 📰 [Non-lexical lifetimes (NLL) fully stable][15]⏱0.25h | Niko Matsakis |
| 📚 [Full list of available crates on Rust Playground][16] | — |
| 📚 [Blanket impl definition][17] | — |
| 📚 [Auto-trait definition][18] | — |
| 📰 [Rust vs Common C++ Bugs][21]⏱0.5h | Georgios Antonopoulos |
| 📰 [True Observer Pattern with Unsubscribe mechanism using Rust][22]⏱0.25h | Yurii Shymon |
| 🧭 [Rust API guidline checklist][19] | — |
| 🧭 [Step-by-step instruction to start development in Rust][26]⏱0.25h | — |
| 📋 [Awesome collection of crates for productive development in Rust][27] | — |
| 📋 [Awesome Collection of Materials to Learn Rust][28] | — |

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
[32]: https://www.youtube.com/watch?v=iVYWDIW71jk
