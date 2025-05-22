Chapter 1: Concepts
================

These tasks describe common and necessary-to-know concepts for everyday programming in [Rust].

> ❗️Before completing this task you should complete all its sub-tasks.

After doing them you should be able to answer the following questions:

- How do I recognize that data is allocated at the heap rather than at the stack? When data should be allocated at the heap?
- What is copying and cloning data in [Rust]? What's the difference? When and why should I use them?
- How can a single piece of data be owned by multiple parts of program? When and why is this commonly required?
- How borrowing rules may be violated? In what price? When and why is this commonly required?
- How to deal with owned and borrowed data simultaneously? When and why is this commonly required?
- How to share values between threads? What is `Send` and `Sync` markers? Why are they required, when should be used?
- How do static and dynamic dispatches differ? Why do they exist? When and why should I choose between them?
- Why `?Sized` types exist? How are they used? Why should I care about them?
- Why phantom types exist? What problems do they solve?

The following articles may help you to sum up your experience:

- [Wrapper Types in Rust: Choosing Your Guarantees][1]
- [Rust, Builder Pattern, Trait Objects, `Box<T>` and `Rc<T>`][2]
- [Rust's Built-in Traits, the When, How & Why][3]
- [Learn Rust With Entirely Too Many Linked Lists][4]
- [Rustonomicon: Subtyping and Variance][13]
- [Crust of Rust: Subtyping and Variance][14]

## Task

__Estimated time__: 2 days

Provide your own implementation of [doubly linked list][11] data structure. It should be [thread safe][12] without a necessity to use explicit synchronization primitives (like `Arc<Mutex<T>>`) on top of it.

> ❗️ Don't use unsafe for this task.

Prove your implementation correctness with tests. Provide both single-threaded and multi-threaded examples of usage.  

[Rust]: https://www.rust-lang.org

[1]: https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees
[2]: https://abronan.com/rust-trait-objects-box-and-rc
[3]: https://llogiq.github.io/2015/07/30/traits.html
[4]: https://rust-unofficial.github.io/too-many-lists/
[11]: https://en.wikipedia.org/wiki/Doubly_linked_list
[12]: https://en.wikipedia.org/wiki/Thread_safety
[13]: https://doc.rust-lang.org/nomicon/subtyping.html
[14]: https://www.youtube.com/watch?v=iVYWDIW71jk
