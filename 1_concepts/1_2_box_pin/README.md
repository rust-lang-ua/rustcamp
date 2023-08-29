Step 1.2: Boxing and pinning
============================




## Boxing

[`Box`] is a pointer that owns heap-allocated data. This is the most common and simples form of [heap] allocation in [Rust].

It's more idiomatic to use references (`&T`/`&mut T`) for pointing to the data, however they often come with lifetimes complexity. [`Box`] allows to avoid this complexity at the cost of heap allocation.

[`Box`] is also a way to go if an owned [slice] is needed, but is not intended to be resized. For example, `Box<str>`/`Box<[T]>` are often used instead `String`/`Vec<T>` in such cases.

For better understanding [`Box`] purpose, design, limitations and use cases read through:
- [Rust Book: 15.1. Using Box to Point to Data on the Heap][1]
- [Official `std::boxed` docs][`std::boxed`]
- [Amos: What's in the box?][3]
- [Mahdi Dibaiee: What is `Box<str>` and how is it different from `String` in Rust?][8]




## Pinning

It is sometimes useful to have objects that are guaranteed to not move, in the sense that their placement in memory does not change, and can thus be relied upon. A prime example of such a scenario would be building self-referential structs, since moving an object with pointers to itself will invalidate them, which could cause undefined behavior.

[`Pin<P>`][`Pin`] ensures that the pointee of any pointer type `P` has a stable location in memory, meaning it cannot be moved elsewhere and its memory cannot be deallocated until it gets dropped. We say that the pointee is "pinned".

However, many types are always freely movable, even when pinned, because they do not rely on having a stable address. This includes all the basic types (like `bool`, `i32`, references) as well as types consisting solely of these types. Types that do not care about pinning implement the [`Unpin`] marker trait, which cancels the effect of [`Pin`]. For `T: Unpin`, `Pin<Box<T>>` and `Box<T>` function identically, as do `Pin<&mut T>` and `&mut T`.

Note, that pinning and [`Unpin`] only affect the pointed-to type `P::Target`, not the pointer type `P` itself that got wrapped in `Pin<P>`. For example, whether or not `Box<T>` is `Unpin` has no effect on the behavior of `Pin<Box<T>>` (here, `T` is the pointed-to type).

For better understanding [`Pin`] purpose, design, limitations and use cases read through:
- [Official `std::pin` docs][`std::pin`]
- [Reddit: Pinned objects ELI5?][2]
- [SoByte: Pin and Unpin in Rust][10]
- [Adam Chalmers: Pin, Unpin, and why Rust needs them][4]
- [Tamme Schichler: Pinning in plain English][5]
- [Yoshua Wuyts: Safe Pin Projections Through View Types][6]
- [Official `#[pin_project]` docs][7]
- [Alice Ryhl answers on "Pin tutorial are confusing me"][9]
- [Rust Forum: Why is it unsafe to pin a shared reference?][11]
- [Ohad Ravid: Put a Pin on That][12]




## Task

__Estimated time__: 1 day




   Implement the following traits:
1. Implement the `SayHi` and `MutMeSomehow` traits **for the following types**: `Box<T>`, `Rc<T>`, `Vec<T>`, `String`, `&[u8]`, `T`.
    
   #### Important:
   ##### THERE HAS TO BE NO `unsafe` CODE (DON'T USE `unsafe`)
   > - `mut_me_somehow` must mutate self somehow.
   > - You can add trait bounds to the types.
   > - Write simple tests to demonstrate mut_me_somehow.
   > - you may use modules to avoid conflicting implementations
  

   ```rust
   trait SayHi: fmt::Debug {
       fn say_hi(self: Pin<&Self>) {
           println!("Hi from {:?}", self)
       }
   }
   ```
   ```rust
   trait MutMeSomehow {
       fn mut_me_somehow(self: Pin<&mut Self>) {
           // Implementation must be meaningful, and
           // obviously call something requiring `&mut self`.
           // The point here is to practice dealing with
           // `Pin<&mut Self>` -> `&mut self` conversion
           // in different contexts, without introducing 
           // any `Unpin` trait bounds.
       }
   }
   ```


3. For the following structure:
   ```rust
   struct MeasurableFuture<Fut> {
       inner_future: Fut,
       started_at: Option<std::time::Instant>,
   }
   ```
   Provide a [`Future`] trait implementation, transparently polling the `inner_future`, and printing its execution time in nanoseconds once it's ready. Using `Fut: Unpin` trait bound (or similar) is not allowed.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What does "boxing" mean in [Rust]? How is it useful? When and why is it required?
- What is [`Pin`] and why is it required? What guarantees does it provide? How it fulfills them?
- How does [`Unpin`] affect the [`Pin`]? What does it mean?
- Is it allowed to move pinned data after the [`Pin`] dies? Why?
- What is structural pinning? When it should be used and why?
- What is [`Pin`] projection? Why does it exist? How is it used?




[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
[`Pin`]: https://doc.rust-lang.org/std/pin/struct.Pin.html
[`std::boxed`]: https://doc.rust-lang.org/std/boxed/index.html
[`std::pin`]: https://doc.rust-lang.org/std/pin/index.html
[`Unpin`]: https://doc.rust-lang.org/std/marker/trait.Unpin.html
[heap]: https://en.wikipedia.org/wiki/Memory_management#HEAP
[Rust]: https://www.rust-lang.org
[slice]: https://doc.rust-lang.org/std/primitive.slice.html

[1]: https://doc.rust-lang.org/book/ch15-01-box.html
[2]: https://www.reddit.com/r/rust/comments/9akmqv/pinned_objects_eli5
[3]: https://fasterthanli.me/articles/whats-in-the-box
[4]: https://blog.adamchalmers.com/pin-unpin
[5]: https://blog.schichler.dev/pinning-in-plain-english-ckwdq3pd0065zwks10raohh85
[6]: https://blog.yoshuawuyts.com/safe-pin-projections-through-view-types
[7]: https://docs.rs/pin-project/latest/pin_project/attr.pin_project.html
[8]: https://mahdi.blog/rust-box-str-vs-string
[9]: https://users.rust-lang.org/t/pin-tutorial-are-confusing-me/91003/18
[10]: https://www.sobyte.net/post/2022-07/rust-pin-unpin
[11]: https://users.rust-lang.org/t/why-is-it-unsafe-to-pin-a-shared-reference/40309
[12]: https://ohadravid.github.io/posts/2023-07-put-a-pin-on-that
