Task 1.1: Default values, cloning and copying
=============================================




## Default values

📚 [Rust]⏱0.25h has a standard way to deal with default values of a type via [`Default`]⏱0.25h trait. Read through 📚 [its official docs][`Default`]⏱0.25h to understand the design.

It can be auto-derived, but only for a `struct` whose all members have 📚 [`Default`]⏱0.25h implementations. It is implemented for a great many types in the standard library, and also used in a surprising number of places. So if your type has a value that can be construed as being "default", it is a good idea to implement this trait.

If you're not satisfied with 📚 [std]⏱0.25h deriving capabilities for 📚 [`Default`]⏱0.25h, consider using 📚 [smart-default]⏱0.25h crate. An example is quite self-explanatory:
```rust
#[derive(SmartDefault)]
enum Foo {
    Bar,
    #[default]
    Baz {
        #[default = 12]
        a: i32,
        b: i32,
        #[default(Some(Default::default()))]
        c: Option<i32>,
        #[default(_code = "vec![1, 2, 3]")]
        d: Vec<u32>,
        #[default = "four"]
        e: String,
    },
    Qux(i32),
}
```

A great thing that having a 📚 [`Default`]⏱0.25h implementation you can instantiate your `struct` with only the non-default values and have all other fields filled with default values:
```rust
let x = Foo { bar: baz, ..Default::default() };
```




## Cloning and copying

By default, all types in 📚 [Rust]⏱0.25h follow ❓ ['move semantics'][1]⏱0.5h.

If you need a duplicate of a value, then its type should implement [`Clone`]⏱0.75h trait (see 📚 [official docs][`Clone`]⏱0.75h), and a duplicate is created by calling [`Clone`]⏱0.75h methods __explicitly__. Cloning can be __either cheap or expensive__ operation depending on type semantics.

However, [`Copy`]⏱0.25h marker trait (see 📚 [official docs][`Copy`]⏱0.25h) enables 'copy semantics' for a type, so a value is __copied implicitly__ every time is passed. That's why copying must always perform a __simple bit-to-bit copy operation__.

📚 [Official `Copy` docs][`Copy`]⏱0.25h are quite explanatory about which types _should_ be [`Copy`]⏱0.25h and which types _cannot_:

> Some types can't be copied safely. For example, copying `&mut T` would create an aliased mutable reference. Copying `String` would duplicate responsibility for managing the `String`'s buffer, leading to a double free.
>
> Generalizing the latter case, any type implementing `Drop` can't be `Copy`, because it's managing some resource besides its own `size_of::<T>` bytes.

> Generally speaking, if your type can implement `Copy`, it should. Keep in mind, though, that implementing `Copy` is part of the public API of your type. If the type might become non-`Copy` in the future, it could be prudent to omit the `Copy` implementation now, to avoid a breaking API change.

For better understanding the topic, read through:
- 📚 [Official `Clone` docs][`Clone`]⏱0.75h
- 📚 [Official `Copy` docs][`Copy`]⏱0.25h
- 📰 [HashRust: Moves, copies and clones in Rust][2]⏱0.25h




## Task

__Estimated time__: 1 day




- Create a `Point` type which represents a 2D point (`x` and `y` coordinates). This type has to be `Copy` and `Default`.
- Create a `Polyline` type which represents a non-empty collection(whichever you want) of `Point`s of unknown size. This type has to be `Clone` and non-`Default`.
    - Polyline must implement basic collection methods: addition of an item, removal of an item,
      getting an item. You may add additional methods & features if you desire.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
1. What purpose does 📚 [`Default`]⏱0.25h trait serve in 📚 [Rust]⏱0.25h?
2. What is `#[derive(Default)]` from `std` capable of? What does it wrong? Which are alternatives?
3. What does 📚 [`Clone`]⏱0.75h mean semantically?
4. What does 📚 [`Copy`]⏱0.25h mean semantically? How is it connected with 📚 [`Clone`]⏱0.75h? Which limitations does it have and why?




[`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[std]: https://doc.rust-lang.org/stable/std
[smart-default]: https://docs.rs/smart-default
[Rust]: https://www.rust-lang.org

[1]: https://stackoverflow.com/a/30290070/1828012
[2]: https://hashrust.com/blog/moves-copies-and-clones-in-rust
