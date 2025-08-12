Task 2.1: Rich types ensure correctness
=======================================

[Rust] has a rich type system which allows to express our program primitives, entities, notions, logic and semantics mostly in types, rather than in data/values, which is known as a "programming with types" concept. The benefits of this are obvious: the more compiler knows about our problem - the more false programs it will decline. Or, rephrased: __the more we describe about the program in types - the more we reduce the probability for the program to be incorrect__.

"Programming with types" inevitably implies its own idioms and patterns. The most common are described below.

## Newtype

Consider the following example, which demonstrates a possible bug:

```rust
#[derive(Clone)]
struct Post {
    id: u64,
    user_id: u64,
    title: String,
    body: String,
}

fn repost(post: &Post, new_author_id: u64) -> Post {
    let mut new_post = post.clone();
    new_post.id = new_author_id;  // Oops!
    new_post
}
```

Here the problem occurs because our entities are expressed in values, so compiler makes no difference between `Post::id` and `Post::user_id` as they have the same type.

Let's express those entities in types:

```rust
mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(String);
}
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(u64);
}

#[derive(Clone)]
struct Post {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

fn repost(post: &Post, new_author_id: user::Id) -> Post {
    let mut new_post = post.clone();
    new_post.id = new_author_id;  // Does not compile!
    new_post
}
```

Now, compiler is able to cut off this type of bugs _totally_ at compile time, and to be quite informative with errors:

```rust
error[E0308]: mismatched types
  --> src/main.rs:27:19
   |
27 |     new_post.id = new_author_id;
   |                   ^^^^^^^^^^^^^ expected struct `post::Id`, found struct `user::Id`
   |
   = note: expected type `post::Id`
              found type `user::Id`
```

This is what is called ["newtype pattern"][1]. [Newtypes][1] are a zero-cost abstraction - __there is no runtime overhead__. Additionally, you may __enforce desired invariants on values of the type__ (for example, `Email` type may allow only valid email address strings to be its values, and another good example is [`uom`] crate). Also, [newtype pattern][1] __makes code more understandable for developers__, as domain knowledge is reflected in types, so is described and documented more explicitly.

The downside of using [newtype pattern][1] is a necessity of writing _more boilerplate code_, because you should provide common traits implementations by yourself (like `Clone`, `Copy`, `From`/`Into`/`AsRef`/`AsMut`), as without them the type won't be ergonomic in use. However, most of them can be _derived automatically_ with `std` capabilities or third-party derive-crates (like [`derive_more`]), so the cost is acceptable in most cases. Furthermore, the excellent [`nutype`] crate pushes this idea even further, aiming to provide the best ergonomics for [newtype pattern][1] without compromising any guarantees it gives.

For better understanding [newtype pattern][1], read through the following articles:

- [Rust Design Patterns: Newtype][1]
- [Rust By Example: 14.7. New Type Idiom][2]
- [Alexis King: Parse, don’t validate][7] ([ru][7_ru])
- [Alexis King: Parse, don’t validate][7] ([eng][7_eng])
- [Stefan Baumgartner: Refactoring in Rust: Abstraction with the Newtype Pattern][10]
- [Official `nutype` crate docs][`nutype`]

## Typestates

[Newtype pattern][1] prevents us from invalid use of data. But what about behavior? Can we _enforce some behavioral invariants at compile time_, so compiler is able to _cut off incorrect behavior totally_?

Not always, but _yes_ in some cases. One possible way is to use [typestates][3] to represent (in types) a _sequence of states_ our type is able to be in, and to declare transitions (via functions) between these states. Doing so will allow compiler to __cut off incorrect state transitions at compile time__.

A real-world example of applying this idiom in [Rust] would be the awesome [`state_machine_future`] crate.

For better understanding [typestates][3], read through the following articles:

- [David Teller: Typestates in Rust][3]
- [Cliff L. Biffle: The Typestate Pattern in Rust][5]
- [Ana Hobden: Pretty State Machine Patterns in Rust][4]
- [Will Crichton: Type-level Programming in Rust][6]
- [Sergey Potapov: Builder with typestate in Rust][8]
- [State Pattern - Design Patterns][9]
- [Azriel Hoh: Compile Time Correctness: Type State][11]

## Task

__Estimated time__: 1 day

For the `Post` type described above, assume the following behavior in our application:

```text
+-----+              +-------------+            +-----------+
| New |--publish()-->| Unmoderated |--allow()-->| Published |
+-----+              +-------------+            +-----------+
                           |                          |
                         deny()                    delete()
                           |       +---------+        |
                           +------>| Deleted |<-------+
                                   +---------+
```

Implement this behavior using [typestates idiom][3], so that calling `delete()` on `New` post (or calling `deny()` on `Deleted` post) will be a compile-time error.
Write simple tests for the task.

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- Why expressing semantics in types is good? What are the benefits and downsides?
- What is `newtype` pattern? How does it work? Which guarantees does it give?
- What is `typestates` pattern? How does it work? Which guarantees does it give?

[`derive_more`]: https://docs.rs/derive_more
[`nutype`]: https://docs.rs/nutype
[`state_machine_future`]: https://docs.rs/state_machine_future
[`uom`]: https://docs.rs/uom
[Rust]: https://www.rust-lang.org

[1]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
[2]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
[3]: https://yoric.github.io/post/rust-typestate
[4]: https://hoverbear.org/2016/10/12/rust-state-machine-pattern
[5]: https://cliffle.com/blog/rust-typestate
[6]: https://willcrichton.net/notes/type-level-programming
[7]: https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate
[7_ru]: https://habr.com/ru/post/498042
[8]: https://www.greyblake.com/blog/builder-with-typestate-in-rust
[9]: https://www.youtube.com/watch?v=N12L5D78MAA
[11]: https://peace.mk/blog/compile-time-correctness-type-state
[10]: https://fettblog.eu/refactoring-rust-abstraction-newtype
