Step 1.8: Thread safety
=======================

[Rust] has [`Send`] and [`Sync`] marker traits which are fundamental for concurrency and thread safety story in [Rust] and represent one of [fearless concurrency][2] corner stones (which allow to [avoid data races][1] at compile time).

For better understanding [`Send`]/[`Sync`] purpose, design, limitations and use cases, read through the following articles:
- [Official `Send` docs][`Send`]
- [Official `Sync` docs][`Sync`]
- [Rust Book: 16.4. Extensible Concurrency with the Sync and Send Traits][3]
- [Rustonomicon: 8.2. Send and Sync][4]
- [Huon Wilson: Some notes on Send and Sync][5]
- [Piotr Sarnacki: Arc and Mutex in Rust][9]
- [nyanpasu64: An unsafe tour of Rust's Send and Sync][6]
- [Josh Haberman: Thread Safety in C++ and Rust][7]
- [Cliff L. Biffle: Safely writing code that isn't thread-safe][8]




## Task

__Estimated time__: 1 day




Implement the following types, which meet conditions:
1. `OnlySync` is `Sync`, but `!Send`.
2. `OnlySend` is `Send`, but `!Sync`.
3. `SyncAndSend` is both `Sync` and `Send`.
4. `NotSyncNotSend` is both `!Sync` and `!Send`.

All inner details of implementation are on your choice.

Play with these types from multiple threads to see how compile time [fearless concurrency][2] works in practice.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What does "fearless concurrency" mean in [Rust]? With which mechanisms does [Rust] fulfill this guarantee exactly?
- Why do [`Send`] and [`Sync`] exist at all? How is it related to interior mutability?




[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/nomicon/races.html
[2]: https://doc.rust-lang.org/book/ch16-00-concurrency.html
[3]: https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html
[4]: https://doc.rust-lang.org/stable/nomicon/send-and-sync.html
[5]: http://huonw.github.io/blog/2015/02/some-notes-on-send-and-sync
[6]: https://nyanpasu64.github.io/blog/an-unsafe-tour-of-rust-s-send-and-sync
[7]: https://blog.reverberate.org/2021/12/18/thread-safety-cpp-rust.html
[8]: https://cliffle.com/blog/not-thread-safe
[9]: https://web.archive.org/web/20220929143451/https://itsallaboutthebit.com/arc-mutex
