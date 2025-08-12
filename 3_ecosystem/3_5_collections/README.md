Task 3.5: Collections and iterators
===================================

## `std` collections

[Rust] provides [implementations for commonly used collections][`std::collections`] in its `std` library. They come with [different guarantees][2] and for [different purposes][1], and are usually applicable for 90% use cases.

For better understanding [`std::collections`] purpose, design, limitations and use cases, read through the following articles:

- [Rust Book: 8. Common Collections][5]
- [Rust By Example: 19.2. Vectors][3]
- [Rust By Example: 19.7. HashMap][4]
- [Official `std::collections` docs][`std::collections`]

## Iterators

> Iterators are heavily used in idiomatic Rust code, so it's worth becoming familiar with them.

While collection represents a some complete set of data, an [`Iterator`] is a way of iteration over its elements.

> An iterator has a method, [`next`][7], which when called, returns an `Option<Item>`. [`next`][7] will return `Some(Item)` as long as there are elements, and once they've all been exhausted, will return `None` to indicate that iteration is finished. Individual iterators may choose to resume iteration, and so calling [`next`][7] again may or may not eventually start returning `Some(Item)` again at some point.
>
> Iterators are also composable, and it's common to chain them together to do more complex forms of processing.

There are three forms of iteration over a collection in [Rust]:

- `iter()` iterates over _borrowed_ elements (`&T`), so used for read-only operations with a collection.
- `iter_mut()` iterates over _mutably borrowed_ elements (`&mut T`), so used when in-place elements mutation is required.
- `into_iter()` iterates over _owned_ element (`T`), so used when whole collection transformation and/or moving is required.

It's important to remember, that __iterators (and their adapters) are lazy__. [`Iterator`] does _nothing_, unless its [`next()`][7] method is called. This property leads to the next one: __iterators do not have to be finite__. So, if you need a sort of an infinite collection (like endless [fibonacci sequence][8]), an [`Iterator`] implementation is a way to go, as each new element will be evaluated lazily on request.

[`Iterator`] comes with a lot of powerful and useful [adapters][9] in `std` library, which makes them highly composable and pleasant to use. If `std` capabilities are not enough for your needs, consider to use [`itertools`] crate, which provides more non-trivial adapters.

For better understanding [Rust] iterators purpose, design, limitations and use cases, read through the following articles:

- [Rust By Example: 16.4. Iterators][6]
- [Official `std::iter` docs][`std::iter`]

## Immutable collections

[Immutable collections][10] (aka "persistent data structures") are collections which preserve interface and behavior of its mutable analogues, but have a different implementation under-the-hood, which __allows each piece of code to work with its own copy of a whole collection without worrying about accidentally changing elements for others__. The key feature is in implicit data deduplication. This inevitably comes in a price of performance, so immutable collection has [other performance guarantees][11] than mutable ones.

[Rust] ecosystem has [`im`] and [`rpds`] crates, which provide immutable implementations for some collections.

For better understanding immutable collections' nature, design, and a motivation behind them, read through the following articles:

- [Official `im` crate docs][`im`]
- [Wikipedia: Persistent data structure][10]
- [Jean Niklas L'orange: Understanding Clojure's Persistent Vectors, pt. 1][15_1]
- [Jean Niklas L'orange: Understanding Clojure's Persistent Vectors, pt. 2][15_2]
- [Jean Niklas L'orange: Understanding Clojure's Persistent Vectors, pt. 3][15_3]
- [Immutable Data Structures and why You want them][15_4]

## Concurrent collections

When you need to operate with the same collection from multiple threads, the most common and obvious way to go is to put it behind some synchronization primitive (like `Arc<RwLock<VecDeque<T>>>`, for example). However, this _performs too bad_ for an extensive use of a collection. That's why concurrent collections exist: they __allow usage of a collection from multiple threads without explicit synchronization__ and __provide efficient synchronization mechanism under-the-hood__ (usually, leveraging lock-free algorithms).

[Rust] ecosystem has [`crossbeam`] and [`lockfree`] crates, providing efficient lock-free implementations for some collections usually used in a concurrent context. Also, consider [`flurry`] and [`chashmap`] crates for a concurrent [hash map][`HashMap`] implementation.

For better understanding concurrent collections' nature, design, and a motivation behind them, read through the following articles:

- [Aaron Turon: Lock-freedom without garbage collection][13]
- [Stjepan Glavina: Lock-free Rust: Crossbeam in 2019][14]
- [Wikipedia: Non-blocking algorithm][12]
- [Ibraheem Ahmed: A Lock-Free Vector][16]

## Task

__Estimated time__: 1 day

Write a simple `UsersRepository` trait, which supports 3 operations (consider to chose correct collections):

- returns single `User` by its ID;
- returns multiple `User`s by their IDs;
- return IDs of `User`s which `nickname` contains given string (search function).

Provide an implementation of `UsersRepository` trait backed by some [immutable collection](#immutable-collections).

Prove your implementation correctness with tests.

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- What is a collection? What is an iterator? How do they differ? How are they used? Which limitations does each one have?
- What are immutable collections? How do they work? Why shouldn't we use them all the time? When does it make sense to use them?
- What are concurrent collections? How do they work? Why are they better than explicit synchronization on a normal collection?

[`chashmap`]: https://docs.rs/chashmap
[`crossbeam`]: https://docs.rs/crossbeam
[`flurry`]: https://docs.rs/flurry
[`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
[`im`]: https://docs.rs/im
[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[`itertools`]: https://docs.rs/itertools
[`lockfree`]: https://docs.rs/lockfree
[`rpds`]: https://docs.rs/rpds
[`std::collections`]: https://doc.rust-lang.org/std/collections/index.html
[`std::iter`]: https://doc.rust-lang.org/std/iter/index.html
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/std/collections/index.html#when-should-you-use-which-collection
[2]: https://doc.rust-lang.org/std/collections/index.html#performance
[3]: https://doc.rust-lang.org/rust-by-example/std/vec.html
[4]: https://doc.rust-lang.org/rust-by-example/std/hash.html
[5]: https://doc.rust-lang.org/book/ch08-00-common-collections.html
[6]: https://doc.rust-lang.org/rust-by-example/trait/iter.html
[7]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
[8]: https://en.wikipedia.org/wiki/Fibonacci_number
[9]: https://doc.rust-lang.org/std/iter/index.html#adapters
[10]: https://en.wikipedia.org/wiki/Persistent_data_structure
[11]: https://docs.rs/im/#performance-notes
[12]: https://en.wikipedia.org/wiki/Non-blocking_algorithm
[13]: https://aturon.github.io/blog/2015/08/27/epoch
[14]: ../../archive/Stjepan_Glavina_Lock-free_Rust_Crossbeam_in_2019.md
[15_1]: https://hypirion.com/musings/understanding-persistent-vector-pt-1
[15_2]: https://hypirion.com/musings/understanding-persistent-vector-pt-2
[15_3]: https://hypirion.com/musings/understanding-persistent-vector-pt-3
[15_4]: https://www.youtube.com/watch?v=Gfe-JKn7G0I
[16]: https://ibraheem.ca/posts/a-lock-free-vector
