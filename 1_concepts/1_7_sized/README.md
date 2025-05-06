Task 1.7: `Sized` and `?Sized` types
====================================

Most types in [Rust] have a particular size, in bytes, that is knowable at compile time. For example, an `i32` is 32 bits big, or 4 bytes. However, there are some types which are useful to express, but do not have a defined size (called "unsized" or "dynamically sized" types). One example is `[T]`: it represents a certain number of `T` in a sequence, but we don’t know how many there are, so the size is not known.

All types with a constant size known at compile time in [Rust] implement [`Sized`] marker trait. And all type parameters (except `Self` in traits) have always an implicit bound of [`Sized`]. So, you should not bother about specifying [`Sized`] marker trait in code, usually.

For better understanding [`Sized`] and `?Sized` purpose, design, limitations and use cases, read through the following articles:

- [Official `Sized` docs][`Sized`]
- [Old Rust Book: 3.31. Unsized Types][4]
- [Rust Forum: Trait Objects and the Sized Trait][5]
- [pretzelhammer: Sizedness in Rust][6]

## Using `?Sized` to accept more types

The more important concept to understand for day-to-day routine is a `?Sized` trait bound, which __lifts the implicit [`Sized`] bound allowing to use more types__ in generic code (so provide better API and ergonomics).

A real-world example would be:

```rust
trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}
```

which allows to use "unsized" types like [trait objects][3]

```rust
impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), UserError>;
    
    fn handle_command(&self, cmd: &CreateUser, user_repo: &Self::Context) -> Self::Result {
        // Here we operate with the `UserRepository`
        // via its trait object `&dyn UserRepository`
    }
}
```

## Task

__Estimated time__: 1 day

Given the [`User` and `UserRepository` implementations from the previous task](../1_6_dispatch#task), write the actual code for `CommandHandler<CreateUser>` implementation described above.

Provide tests for `CommandHandler<CreateUser>` implementation where `dyn UserRepository` is mocked with another hand-written type for testing purposes (you will need to transform the `UserRepository` type into a trait).

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- What is [`Sized`] trait about? When [Rust] implies it? And when aren't?
- Why `?Sized` trait bound is important? When should we use it and why?

[Rust]: https://www.rust-lang.org
[`Sized`]: https://doc.rust-lang.org/std/marker/trait.Sized.html

[3]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
[4]: https://doc.rust-lang.org/1.26.0/book/first-edition/unsized-types.html
[5]: https://users.rust-lang.org/t/trait-objects-and-the-sized-trait/14410
[6]: https://github.com/pretzelhammer/rust-blog/blob/master/posts/sizedness-in-rust.md
