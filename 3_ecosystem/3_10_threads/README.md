Task 3.10: Multithreading and parallelism
=========================================

One of main 📚 [Rust]⏱0.25h's design goals is a 📰 [concurrency][1]⏱0.25h. 📚 [Rust]⏱0.25h has a 📰 [strong opinion][2]⏱0.5h about that, while allows different concurrent models to coexist.




## Threads

📚 [Rust]⏱0.25h has built-in support for 📰 [native threads][3]⏱0.75h in form of the 📚 [`std::thread`]⏱0.25h module of its standard library.

Traditionally, 📰 [threads][3]⏱0.75h are used for solving 📰 [CPU-bound]⏱0.25h problems, as they allow to execute tasks in parallel. However, in practice, threads are often used to solve 📰 [I/O-bound]⏱0.25h problems too, especially when 📰 [asynchronous I/O][4]⏱0.5h is not supported well (which is true for 📚 [Rust]⏱0.25h `std` library at the moment).

📚 [`crossbeam`]⏱0.25h crate also provides implementation of 📚 [scoped threads][5]⏱0.25h, which allow to borrow values from a stack. They are also available in form of 📚 [`std::thread::scope`]⏱0.25h, as of 📚 [Rust]⏱0.25h 1.63. 

For better understanding 📚 [Rust]⏱0.25h threads design, concepts, usage, and features (especially 📰 [TLS][4]⏱0.5h is important and widely used one), read through the following articles:
- 📚 [Rust Book: 16.1. Using Threads to Run Code Simultaneously][6]⏱0.25h
- 📚 [Rust By Example: 20.1. Threads][7]⏱0.25h
- 📚 [Official `std::thread` docs][`std::thread`]⏱0.25h
- 📰 [Nicky Meuleman: Multithreading in Rust][29]⏱0.25h




## Synchronization

The 📰 [threads synchronization][11]⏱0.5h is a wide topic, but generally it's done via 📰 [atomic operations][12]⏱0.5h, shared state with an 📚 [exclusive access][13]⏱0.25h, or by 📰 [threads communication][14]⏱0.25h. 📚 [Rust]⏱0.25h has built-in support for all of them.

📰 [Atomic operations][12]⏱0.5h are represented by 📚 [`std::sync::atomic`]⏱0.25h module of 📚 [Rust]⏱0.25h standard library (and, additionally, 📚 [`atomic`]⏱0.25h crate).

📚 [Exclusive access][13]⏱0.25h may be controlled via primitives of 📚 [`std::sync`]⏱0.25h module of 📚 [Rust]⏱0.25h standard library.

Threads communication is commonly represented via 📰 [channels][14]⏱0.25h and is implemented in 📚 [`std::sync::mpsc`]⏱0.25h module of 📚 [Rust]⏱0.25h standard library. 

Despite that, there is also the 📚 [`crossbeam`]⏱0.25h crate, providing more feature-rich and optimized concurrency and synchronization primitives. The most notable is 📚 [`crossbeam-channel`]⏱0.25h as [an enhancement][15] of `std` channel implementations.

For better understanding and familiarity with 📚 [Rust]⏱0.25h synchronization primitives design, concepts, usage, and features, read through the following articles:
- 📚 [Rust Book: 16.2. Using Message Passing to Transfer Data Between Threads][16]⏱0.25h
- 📚 [Rust Book: 16.3. Shared-State Concurrency][13]⏱0.25h
- 📰 [Rust Blog: Fearless Concurrency with Rust][2]⏱0.5h
- 📚 [Official `std::sync` docs][`std::sync`]⏱0.25h
- 📚 [Official `std::sync::atomic` docs][`std::sync::atomic`]⏱0.25h
- 📚 [Official `std::sync::mpsc` docs][`std::sync::mpsc`]⏱0.25h
- 📚 [Official `atomic` crate docs][`atomic`]⏱0.25h
- 📚 [Official `crossbeam-channel` crate docs][`crossbeam-channel`]⏱0.25h
- 📰 [Nicky Meuleman: Multithreading in Rust][29]⏱0.25h
- 📰 [Carl Fredrik Samson: Explaining Atomics in Rust][26]⏱0.75h
- 📰 [Aleksey Kladov: Mutexes Are Faster Than Spinlocks][27]⏱0.25h
- 📰 [Mara Bos: Comparing Rust's and C++'s Concurrency Library][31]⏱0.25h
- 📰 [Mahmoud Al-Qudsi: Implementing truly safe semaphores in rust][32]⏱1.25h
- 📰 [Michael Snoyman: My Best and Worst Deadlock in Rust][35]⏱0.75h




## Parallelism

The important concept to understand is ❓ [how concurrency and parallelism differ][21]⏱1h.

📚 [Rust]⏱0.25h ecosystem has support for parallelism in form of 📚 [`rayon`]⏱0.25h and 📚 [`dpc-pariter`]⏱0.25h crates, which make it easy to convert a sequential iterator to _execute in parallel threads_.

Another way to perform parallel data processing _without using 📰 [threads][3]⏱0.75h_ is 📰 [SIMD]⏱0.75h instructions usage. If an algorithm is parallelizable enough, applying 📰 [SIMD]⏱0.75h instructions may 📰 [increase performance drastically][24]⏱0.5h. 📚 [Rust]⏱0.25h ecosystem provides basic support for 📰 [SIMD]⏱0.75h instructions in a form of 📚 [`packed_simd`]⏱0.25h crate.

For better understanding and familiarity with parallelism in 📚 [Rust]⏱0.25h, read through the following articles:
- 📰 [Nicky Meuleman: Concurrent vs parallel][28]⏱0.25h
- 📚 [Official `rayon` crate docs][`rayon`]⏱0.25h
- 📰 [`rayon` crate FAQ][22]⏱0.25h
- 📰 [`rayon` crate demos][23]⏱0.25h
- 📰 [Kofi Otuo: Implementing data parallelism with Rayon Rust][34]⏱0.5h
- 📰 [Dawid Ciężarkiewicz: Adding parallelism to your Rust iterators with `dpc-pariter`][30]⏱0.25h
- 📚 [Official `dpc-pariter` crate docs][`dpc-pariter`]⏱0.25h
- 📚 [Rust Edition Guide: 3.9. SIMD for faster computing][25]⏱0.25h
- 📚 [Official `packed_simd` crate docs][`packed_simd`]⏱0.25h
- 📰 [vgatherps: Parsing numbers into base-10 decimals with SIMD][33]⏱0.25h




## Multiprocessing

Multiprocessing is a system that has more than one or two processors. In Multiprocessing, CPUs are added for increasing computing speed of the system. Because of Multiprocessing, There are many processes are executed simultaneously.




## Multiprocessing vs Multithreading

|     Parameter    |                                                              Multiprocessing                                                              |                                                        Multithreading                                                        |
|:----------------:|:-----------------------------------------------------------------------------------------------------------------------------------------:|:----------------------------------------------------------------------------------------------------------------------------:|
| Basic            | Multiprocessing helps you to increase computing power.                                                                                    | Multithreading helps you to create computing threads of a single process to increase computing power.                        |
| Execution        | It allows you to execute multiple processes concurrently.                                                                                 | Multiple threads of a single process are executed concurrently.                                                              |
| CPU switching    | In Multiprocessing, CPU has to switch between multiple programs so  that it looks like that multiple programs are running simultaneously. | In multithreading, CPU has to switch between multiple threads to make it appear that all threads are running simultaneously. |
| Creation         | The creation of a process is slow and resource-specific.                                                                                  | The creation of a thread is economical in time and resource.                                                                 |
| Classification   | Multiprocessing can be symmetric or asymmetric.                                                                                           | Multithreading is not classified.                                                                                            |
| Memory           | Multiprocessing allocates separate memory and resources for each process or program.                                                      | Multithreading threads belonging to the same process share the same memory and resources as that of the process.             |
| Pickling objects | Multiprocessing relies on pickling objects in memory to send to other processes.                                                          | Multithreading avoids pickling.                                                                                              |
| Program          | Multiprocessing system allows executing multiple programs and tasks.                                                                      | Multithreading system executes multiple threads of the same or different processes.                                          |


## Task

__Estimated time__: 1 day




Write a program with the following workflow:
- `Producer` is a separate thread, which continuously generates square matrixes of random `u8` elements and size `4096`.
- `Consumer` is a separate thread, which takes a generated matrix, counts sum of all its elements and prints the sum to STDOUT.
- There are only 1 `Producer` and 2 `Consumer`s.
- Counting sum of matrix elements should be parallelized.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
1. What is concurrency? What is parallelism? How do they relate to each other and how do they differ?
2. How parallelism is represented in 📚 [Rust]⏱0.25h? Which are common crates for using it?
3. What are the main ways of threads synchronization in 📚 [Rust]⏱0.25h? Which advantages and disadvantages does each one have? What are the use-cases for each one?




[`atomic`]: https://docs.rs/atomic
[`crossbeam`]: https://docs.rs/crossbeam
[`crossbeam-channel`]: https://docs.rs/crossbeam-channel
[`dpc-pariter`]: https://docs.rs/dpc-pariter
[`packed_simd`]: https://docs.rs/packed_simd
[`rayon`]: https://docs.rs/rayon
[`std::sync`]: https://doc.rust-lang.org/std/sync/index.html
[`std::sync::atomic`]: https://doc.rust-lang.org/std/sync/atomic/index.html
[`std::sync::mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html
[`std::thread`]: https://doc.rust-lang.org/std/thread/index.html
[`std::thread::scope`]: https://doc.rust-lang.org/std/thread/fn.scope.html
[CPU-bound]: https://en.wikipedia.org/wiki/CPU-bound
[I/O-bound]: https://en.wikipedia.org/wiki/I/O_bound
[Rust]: https://www.rust-lang.org
[SIMD]: https://en.wikipedia.org/wiki/SIMD

[1]: https://en.wikipedia.org/wiki/Concurrency_(computer_science)
[2]: https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html
[3]: https://en.wikipedia.org/wiki/Thread_(computing)
[4]: https://en.wikipedia.org/wiki/Asynchronous_I/O
[5]: https://docs.rs/crossbeam/0.7.1/crossbeam/thread/index.html
[6]: https://doc.rust-lang.org/book/ch16-01-threads.html
[7]: https://doc.rust-lang.org/rust-by-example/std_misc/threads.html
[8]: https://doc.rust-lang.org/std/thread/index.html#thread-local-storage
[11]: https://en.wikipedia.org/wiki/Synchronization_(computer_science)#Thread_or_process_synchronization
[12]: https://en.wikipedia.org/wiki/Linearizability
[13]: https://doc.rust-lang.org/book/ch16-03-shared-state.html
[14]: https://en.wikipedia.org/wiki/Channel_(programming)
[15]: ../../archive/Stjepan_Glavina_Designing_a_channel.md
[16]: https://doc.rust-lang.org/book/ch16-02-message-passing.html
[21]: https://stackoverflow.com/a/1050257/1828012
[22]: https://github.com/rayon-rs/rayon/blob/master/FAQ.md
[23]: https://github.com/rayon-rs/rayon/tree/master/rayon-demo
[23]: https://doc.rust-lang.org/edition-guide/rust-2018/simd-for-faster-computing.html
[24]: https://branchfree.org/2019/02/25/paper-parsing-gigabytes-of-json-per-second
[25]: https://doc.rust-lang.org/edition-guide/rust-2018/simd-for-faster-computing.html
[26]: https://cfsamsonbooks.gitbook.io/explaining-atomics-in-rust
[27]: https://matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html
[28]: https://nickymeuleman.netlify.app/garden/concurrent-vs-parallel
[29]: https://nickymeuleman.netlify.app/blog/multithreading-rust
[30]: https://dpc.pw/adding-parallelism-to-your-rust-iterators
[31]: https://blog.m-ou.se/rust-cpp-concurrency
[32]: https://neosmart.net/blog/implementing-truly-safe-semaphores-in-rust/
[33]: https://vgatherps.github.io/2022-11-28-dec
[34]: https://blog.logrocket.com/implementing-data-parallelism-rayon-rust
[35]: https://www.snoyman.com/blog/2024/01/best-worst-deadlock-rust
