Step 3.11: Async I/O, futures and actors
========================================

While [threads](../3_10_threads) represent a solution for [CPU-bound] problems, for [I/O-bound] problems, traditionally, the solution is [async (non-blocking) I/O][1].

As of now, [Rust] has no async primitives in its standard library yet, so "by default" `std` I/O works in a synchronous manner (blocks the current [thread][33]). However, it provides [core abstractions][`std::future`] for building ones, using which, ecosystem crates (like [`tokio`]) implement and provide primitives for [async I/O][1].

It's important to note, that async story in [Rust] is [still][2] [maturing][3]. That's why things could be [quite cumbersome][5] at the moment, often [causing frustration][6] (especially, when it [comes to abstractions][7]). [wg-async][4] (async working group) works on making this easier, simpler, more ergonomic and powerful in the future.




## `Future`

The basic primitive of async story in [Rust] is a [future abstraction][8] (also often called "promise" in some other programming languages). There are two major concepts which differ [Rust implementation of futures][9] from other programming languages:
1. Futures are [poll-based][10] rather than push-based. This means that after creation, a future is not going to be executed automatically in-place, but rather should be explicitly executed by some executor (runtime/event-loop for futures). __Future does nothing unless polled__, so generally represents a [lazy computation][12].
2. Futures are [zero cost][11]. This means that the code written on futures compiles down to something equivalent (or better than) a “hand-rolled” implementation, which would typically use manual state machines and careful memory management.

[Rust] provides only basic trait definitions in the [`std::future`] module of its standard library. To use futures with all its power, consider to use the [`futures`] crate (and/or similar ones like [`futures-lite`], [`futures-time`], etc).

To understand [Rust] futures concepts and design better, read through the following articles:
- [Aaron Turon: Zero-cost futures in Rust][11]
- [Aaron Turon: Designing futures for Rust][9]
- [Rust RFC 2592: `futures_api`][13]
- [Asynchronous Programming in Rust: 2.1. The `Future` Trait][20]
- [Conrad Ludgate: Let's talk about this async][14]

It's important to mention, that before [futures design has been stabilized][13], for quite a long period of time [Rust] ecosystem used [`futures@0.1`] crate, which resulted in a big part of ecosystem being built on top of them. Hopefully, as for now, only quite few outdated or dead crates still do use [`futures@0.1`], and, fortunately, they still can be used simultaneously with the modern [`std::future`]-based ecosystem by using the [compatibility layer][15].


### `async`/`.await`

[`async`/`.await` keywords][16] make async programming much more intuitive, ergonomic, and [solves numerous problems with types and borrows][19] (which may be quite tricky when using raw [`futures`]).

> Use `async` in front of `fn`, `closure`, or a `block` to turn the marked code into a `Future`. As such the code will not be run immediately, but will only be evaluated when the returned `Future` is `.await`ed.

[Rust] automatically [desugars `async` functions and blocks into the ones returning a `Future`][17], applying the correct [lifetime capturing and elision rules][18] for the syntax ergonomics.

Though, [`async` keyword in not supported in trait methods yet][2], there is the [`async-trait`] crate, which allows this for traits by desugaring into a [`Box`]ed [`Future`] (the main downside of which is being non-transparent over auto-traits like `Send`/`Sync`). 

For better understanding `async`/`.await` keywords design, desugaring, usage, and features, read through the following articles:
- [Rust RFC 2394: `async_await`][16]
- [Asynchronous Programming in Rust: 3. `async`/`.await`][21]
- [Hayden Stainsby: how I finally understood async/await in Rust (part 1)][63]
- [David Tolnay: Await a minute, why bother?][19]
- [Arpad Borsos: Implementation Details of async Rust][27]
- [Tyler Madry: How Rust optimizes async/await I][29]
- [Tyler Madry: How Rust optimizes async/await II: Program analysis][30]


### Tasks and `Waker`

Except the [future abstraction][8] itself, it's important to understand what is an [asynchronous task][22]:
> Each time a future is polled, it is polled as part of a "task". Tasks are the top-level futures that have been submitted to an executor.

When a task is suspended due to waiting some non-blocking operation to complete (it's used to call it "parked"), there should be a way to signal an executor to continue polling this task once the operation finishes. The [`Waker`] (being provided in the [`task::Context`]) serves exactly this purpose:
> `Waker` provides a `wake()` method that can be used to tell the executor that the associated task should be awoken. When `wake()` is called, the executor knows that the task associated with the `Waker` is ready to make progress, and its future should be polled again.

For better understanding [`Waker`] design, usage, and features, read through the following articles:
- [Official `std::task::Waker` docs][`Waker`]
- [Asynchronous Programming in Rust: 2.2. Task Wakeups with `Waker`][22]
- [Hayden Stainsby: how I finally understood async/await in Rust (part 2)][64]
- [Arpad Borsos: Rust Futures and Tasks][28]


### More reading

- [Matt Sarmiento: Async Rust: Futures, Tasks, Wakers—Oh My!][26]
- [Bert Peters: How does async Rust work][31]
- [Tokio Tutorial: Async in depth][24]
- [Asynchronous Programming in Rust][23]
- [Amos: Understanding Rust futures by going way too deep][25]




## Async I/O

Async I/O in [Rust] is possible due to two main ingredients: __[non-blocking I/O operations][1]__ provided by operating system and an __asynchronous runtime__, which wraps those operations into usable asynchronous abstractions and provides an [event loop][48] for executing and driving them to completion.

For better understanding [mio] and [tokio] design, concepts, usage, and features, read through the following articles:
- [Official `mio` crate docs][mio]
- [Official `tokio` crate docs][tokio]
- [Official `tokio` crate guide][13]
- [Nick Cameron: Asynchronous programming with Rust: Introduction][14]
- [Tokio on asyncronous tasks and executors][15]


### Non-blocking I/O

The async programming is not possible without support for [non-blocking I/O][1], which is represented by various [API]s on different operating systems, for example: [epoll] on [Linux] (or promising [io_uring]), [kqueue] on [macOS]/[iOS], [IOCP] on [Windows].

The low-level crates, like [`mio`] (powering [`tokio`]) and [`polling`] (powering [`async-std`]), provide a single multi-platform unified interface to the majority of those [API]s. There are also low-level crates, specialized on a concrete [API], like [`io-uring`].


### Runtime

The high-level crates, like [`tokio`] (pioneer and most mature, by far) and [`async-std`] (not to be confused by its name, it's neither official, nor `std`-related, just a name chosen by authors), provide not only an [executor implementation][32] for executing [`Future`]s, but also high-level [API]s for [non-blocking I/O][1], [timers][`tokio::time`], and [synchronization primitives][`tokio::sync`] for use in asynchronous contexts ([usual synchronization primitives cannot be used across `.await` points][34] as they will block the whole executor in its current [thread][33]).

All [Rust] asynchronous runtimes for [`Future`]s implement the idea of [cooperative multitasking][35], meaning that the tasks ([`Future`]s in our case) yield control back to their runtime voluntarily (on `.await` points in our case), in contrast with [preemptive multitasking][36] where the runtime can suspend and take control back whenever it decides to (like in [OS threads][33] or [Erlang VM][37]). This gives the benefit of precise control on what is executed and how, but has the disadvantage of requiring to take great care about how [asynchronous tasks][22] are organized (like [avoiding blocking][39] them with synchronous or [CPU-bound] operations and [yielding manually][38] in busy loops).

Also, important to classify [Rust] asynchronous runtimes in the following manner:
- __Single-thread__ runtimes, __scheduling and executing [`Future`]s only on the current [thread][33]__ they're run on.  
  _Examples: [`tokio`'s current-thread scheduler][40], [`tokio-uring`], [`futures::executor::LocalPool`]._
- __Multi-thread__ runtimes, scheduling and executing [`Future`]s on a [thread pool][41]:
    - With __[work-stealing][42]__, where [`Future`]s are __both scheduled and executed on different [threads][33]__, so one [thread][33] can [steal and execute `Future`s initally scheduled on another thread][43], and as the result, workload is distributed more evenly in cost of synchronization overhead ([`Future`]s are required to be [`Send`]).  
      _Examples: [`tokio`'s multi-thread scheduler][44], [`async-executor`] of [`async-std`], [`futures::executor::ThreadPool`]._
    - Using __[thread-per-core][45]__ model, where [`Future`]s are __scheduled on different [threads][33], but never leave their [thread][33] until executed completely__, and so, avoid any synchronization overhead ([`Future`]s are not required to be [`Send`]).  
      _Examples: [`actix-rt`] built on top of multiple [`tokio`'s current-thread schedulers][40], [`glommio`]._

Unfortunately, at the moment, there is no meaningful way to abstract over multiple asynchronous runtimes in [Rust]. That's why authors of the libraries using [non-blocking I/O][1] either stick with a single concrete runtime only ([`tokio`], mostly), or support multiple runtimes via [Cargo features][46].

For better understanding, read through the following articles:
- [Official `tokio` crate docs][`tokio`]
- [Official `async-std` crate docs][`async-std`]
- [Tokio Tutorial][47]
- [Nick Cameron: What is an async runtime?][59]
- [Sylvain Kerkour: Async Rust: Cooperative vs Preemptive scheduling][60]
- [Sylvain Kerkour: Async Rust: What is a runtime? Here is how tokio works under the hood][61]




## Actors

[Actor model][49] is another very spread and famous [concurrency programming paradigm][50]. It fits quite good for solving major concurrent communication problems, so many languages adopted it as their main [concurrency paradigm][50] (the most famous implementations are [Akka][51] and [Erlang][52]).

> [Actor model][53] was put forth by [Carl Hewitt] in 1973 and it adopts the philosophy that everything is an actor. This is similar to the everything is an object philosophy used by some object-oriented programming languages.
>
> It is inherently asynchronous, a message sender will not block whether the reader is ready to pull from the mailbox or not, instead the message goes into a queue usually called a "mailbox". Which is convenient, but it's a bit harder to reason about and mailboxes potentially have to hold a lot of messages.
>
> Each process has a single mailbox, messages are put into the receiver's mailbox by the sender, and fetched by the receiver.

It's somewhat very similar to and interchangeable with [Communicating Sequential Processes (CSP) model][54], as operates on the same level of abstractions, but the main [difference][55] can be described like this:
> [Actors model][49] represents identifiable processes (actors) with non-identifiable communication (message delivery), while [CSP model][54] represents non-identifiable processes with identifiable communication (channels). To deliver a message in [actors model][49] we should "name" the actor, while in [CSP model][54] we should "name" the channel.

In [Rust], [actor abstraction][49] is __mainly useful for expressing some long-living state__ to communicate with (like [background worker][56] or [WebSocket connection][57], for example).

The most famous [actors][49] implementation in [Rust] is [`actix`]. At the time it was designed, it also served as __a "glue" to unite sync and async worlds__, providing both sync and async [actors][49] implementations. Nowadays, however, using [`spawn_blocking()`][39] is usually a more convenient alternative for this.

[`quickwit-actors`] is another simple implementation of [actors][49], with its own advantages, built [specifically for Quickwit needs][62].

More general-purpose and complex [actors system][49] implementations (similar to [Akka]) are [`bastion`] and [`riker`].

For better understanding [actors][49] design, concepts, usage, and implementations, read through the following articles:
- [Karan Pratap Singh: CSP vs Actor model for concurrency][55]
- [Official `actix` crate docs][`actix`]
- [Official `actix` user guide][58]
- [Evance Soumaoro: Efficient indexing with Quickwit Rust actor framework][62]




## Mutlithreading vs Async

Multithreading programming is all about concurrent execution of different functions. Async programming is about non-blocking execution between functions, and we can apply async with single-threaded or multithreaded programming.

So, multithreading is one form of asynchronous programming.

Let’s take a simple analogy:

- Synchronous: you cook the eggs, then you cook the toast.
- Asynchronous, single threaded: you start the eggs cooking and set a timer. You start the toast cooking, and set a timer. While they are both cooking, you clean the kitchen. When the timers go off you take the eggs off the heat and the toast out of the toaster and serve them.
- Asynchronous, multithreaded: you hire two more cooks, one to cook eggs and one to cook toast. Now you have the problem of coordinating the cooks so that they do not conflict with each other in the kitchen when sharing resources. And you have to pay them.

From that analogy, we can conclude that **Multithreading is about workers, Async is about tasks.**

![Synchronous vs Async vs Multithreading](https://i.imgur.com/o0wETfj.png)




## Task

__Estimated time__: 2 days




Implement an async-driven [CLI] tool, which downloads specified web pages:
```bash
cargo run -p step_3_11 -- [--max-threads=<number>] <file>
```
It must read a list of links from the `<file>`, and then concurrently download a content of each link into a separate `.html` file (named by a link).

`--max-threads` argument must control the maximum number of _simultaneously running threads_ in the program (should default to CPUs number).




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is asynchronous programming? How does it relate to multithreading? Which problems does it solve? What are the prerequisites for its existing?
- How does non-blocking I/O works? How does it differs from blocking I/O?
- What is a [`Future`]? Why do we need it? How does it work in [Rust] and how do its semantics differ from other programming languages? What makes it zero-cost?
- What is `async`/`.await`? Ho do they desugar into a [`Future`]? Why are they vital for ergonomics?
- What is an asynchronous task? How does it compare to a [`Future`]?
- What is a [`Waker`]? How does it work? Why is it required?
- What is an asynchronous runtime? From which parts does it usually consist?
- What kind of multitasking is represented by [`Future`]s in [Rust]? Which advantages and disadvantages does it have?
- What kinds of asynchronous runtimes do exist in [Rust] regarding multithreading? Which advantages and disadvantages does each one have?
- Why blocking an asynchronous runtime is bad? How to avoid it in practice?
- What are the key points of actor model concurrency paradigm? How may it be useful in [Rust]?




[`actix`]: https://docs.rs/actix
[`actix-rt`]: https://docs.rs/actix-rt
[`async-executor`]: https://docs.rs/async-executor
[`async-std`]: https://docs.rs/async-std
[`async-trait`]: https://docs.rs/async-trait
[`bastion`]: https://www.bastion-rs.com
[`Box`]: https://doc.rust-lang.org/stable/std/boxed/struct.Box.html
[`Future`]: https://doc.rust-lang.org/stable/std/future/trait.Future.html
[`futures`]: https://docs.rs/futures
[`futures@0.1`]: https://docs.rs/futures/0.1
[`futures::executor::LocalPool`]: https://docs.rs/futures/latest/futures/executor/struct.LocalPool.html
[`futures::executor::ThreadPool`]: https://docs.rs/futures/latest/futures/executor/struct.ThreadPool.html
[`futures-lite`]: https://docs.rs/futures-lite
[`futures-time`]: https://docs.rs/futures-time
[`glommio`]: https://docs.rs/glommio
[`io-uring`]: https://docs.rs/io-uring
[`mio`]: https://docs.rs/mio
[`polling`]: https://docs.rs/polling
[`quickwit-actors`]: https://docs.rs/quickwit-actors
[`riker`]: https://riker.rs
[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`std::future`]: https://doc.rust-lang.org/std/future/index.html
[`task::Context`]: https://doc.rust-lang.org/std/task/struct.Context.html
[`tokio`]: https://docs.rs/tokio
[`tokio::sync`]: https://docs.rs/tokio/latest/tokio/sync/index.html
[`tokio::time`]: https://docs.rs/tokio/latest/tokio/time/index.html
[`tokio-uring`]: https://docs.rs/tokio-uring
[`Waker`]: https://doc.rust-lang.org/stable/std/task/struct.Waker.html
[Akka]: https://akka.io
[API]: https://en.wikipedia.org/wiki/API
[Carl Hewitt]: https://en.wikipedia.org/wiki/Carl_Hewitt
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[CPU-bound]: https://en.wikipedia.org/wiki/CPU-bound
[epoll]: https://en.wikipedia.org/wiki/Epoll
[I/O-bound]: https://en.wikipedia.org/wiki/I/O_bound
[io_uring]: https://en.wikipedia.org/wiki/Io_uring
[IOCP]: https://learn.microsoft.com/windows/win32/fileio/i-o-completion-ports
[iOS]: https://en.wikipedia.org/wiki/IOS
[kqueue]: https://en.wikipedia.org/wiki/Kqueue
[Linux]: https://en.wikipedia.org/wiki/Linux_kernel
[macOS]: https://en.wikipedia.org/wiki/MacOS
[Rust]: https://www.rust-lang.org
[Windows]: https://en.wikipedia.org/wiki/Microsoft_Windows

[1]: https://en.wikipedia.org/wiki/Asynchronous_I/O
[2]: https://areweasyncyet.rs#async-extensions
[3]: https://rust-lang.github.io/wg-async/design_docs.html
[4]: https://rust-lang.github.io/wg-async/welcome.html
[5]: https://eta.st/2021/03/08/async-rust-2.html
[6]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo.html
[7]: https://hirrolot.github.io/posts/rust-is-hard-or-the-misery-of-mainstream-programming.html#waiting-for-better-future
[8]: https://en.wikipedia.org/wiki/Futures_and_promises
[9]: https://aturon.github.io/blog/2016/09/07/futures-design
[10]: http://aturon.github.io/blog/2016/09/07/futures-design#what-worked-the-demand-driven-aka-readiness-based-approach
[11]: https://aturon.github.io/blog/2016/08/11/futures
[12]: https://en.wikipedia.org/wiki/Lazy_evaluation
[13]: https://rust-lang.github.io/rfcs/2592-futures.html
[14]: https://conradludgate.com/posts/async
[15]: https://rust-lang.github.io/futures-rs/blog/2019/04/18/compatibility-layer.html
[16]: https://rust-lang.github.io/rfcs/2394-async_await.html
[17]: https://rust-lang.github.io/rfcs/2394-async_await.html#reference-level-explanation
[18]: https://rust-lang.github.io/rfcs/2394-async_await.html#lifetime-capture-in-the-anonymous-future
[19]: https://docs.rs/dtolnay/latest/dtolnay/macro._01__await_a_minute.html
[20]: https://rust-lang.github.io/async-book/02_execution/02_future.html
[21]: https://rust-lang.github.io/async-book/03_async_await/01_chapter.html
[22]: https://rust-lang.github.io/async-book/02_execution/03_wakeups.html
[23]: https://rust-lang.github.io/async-book
[24]: https://tokio.rs/tokio/tutorial/async
[25]: https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep
[26]: https://msarmi9.github.io/posts/async-rust
[27]: https://swatinem.de/blog/async-codegen
[28]: https://swatinem.de/blog/futures-n-tasks
[29]: https://tmandry.gitlab.io/blog/posts/optimizing-await-1
[30]: https://tmandry.gitlab.io/blog/posts/optimizing-await-2
[31]: https://bertptrs.nl/2023/04/27/how-does-async-rust-work.html
[32]: https://tokio.rs/tokio/tutorial/async#executors
[33]: https://en.wikipedia.org/wiki/Thread_(computing)
[34]: https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html#which-kind-of-mutex-should-you-use
[35]: https://en.wikipedia.org/wiki/Cooperative_multitasking
[36]: https://en.wikipedia.org/wiki/Preemption_(computing)
[37]: https://blog.stenmans.org/theBeamBook#CH-Scheduling
[38]: https://docs.rs/tokio/latest/tokio/task/index.html#yield_now
[39]: https://docs.rs/tokio/latest/tokio/task/index.html#blocking-and-yielding
[40]: https://docs.rs/tokio/latest/tokio/runtime/index.html#current-thread-scheduler
[41]: https://en.wikipedia.org/wiki/Thread_pool
[42]: https://en.wikipedia.org/wiki/Work_stealing
[43]: https://tokio.rs/blog/2019-10-scheduler#work-stealing-scheduler
[44]: https://docs.rs/tokio/latest/tokio/runtime/index.html#multi-thread-scheduler
[45]: https://www.datadoghq.com/blog/engineering/introducing-glommio
[46]: https://doc.rust-lang.org/cargo/reference/features.html
[47]: https://tokio.rs/tokio/tutorial
[48]: https://en.wikipedia.org/wiki/Event_loop
[49]: https://en.wikipedia.org/wiki/Actor_model
[50]: https://en.wikipedia.org/wiki/Concurrency_(computer_science)
[51]: https://doc.akka.io/docs/akka/current/typed/actors.html
[52]: https://www.dmi.unict.it/barba/FOND-LING-PROG-DISTR/PROGRAMMI-TESTI/READING-MATERIAL/shortNotesOnErlang.html
[53]: https://arxiv.org/abs/1008.1459
[54]: https://en.wikipedia.org/wiki/Communicating_sequential_processes
[55]: https://dev.to/karanpratapsingh/csp-vs-actor-model-for-concurrency-1cpg
[56]: https://en.wikipedia.org/wiki/Background_process
[57]: https://levelup.gitconnected.com/websockets-in-actix-web-full-tutorial-websockets-actors-f7f9484f5086
[58]: https://actix.rs/docs/actix/actor
[59]: https://ncameron.org/blog/what-is-an-async-runtime
[60]: https://kerkour.com/cooperative-vs-preemptive-scheduling
[61]: https://kerkour.com/rust-async-await-what-is-a-runtime
[62]: https://quickwit.io/blog/quickwit-actor-framework
[63]: https://hegdenu.net/posts/understanding-async-await-1
[64]: https://hegdenu.net/posts/understanding-async-await-2
