Task 2.4: Abstract type in, concrete type out
=============================================

## Abstracting over input type

The common and obvious rules in [Rust] when choosing type for an input parameter are the following:

- If you need a _read-only access_ to the value, then use a shared reference (`&T`).
- If you want to _mutate the value in-place_, then use a mutable reference (`&mut T`).
- If you want to _consume and own_ the value, then move it (`T`).

Let's illustrate it with the following trivial examples:

```rust
use derive_more::{AsRef, AsMut};

// Read-only access is enough here.
pub fn just_print_stringy(v: &str) {
    println!("{}", v)
}

// We want to change `v`, but don't own.
pub fn add_hi(v: &mut String) {
    v.push_str(" Hi")
}

#[derive(AsMut, AsRef)]
pub struct Nickname(String);


impl Nickname {
    // We want to own `nickname` inside `Nickname` value. 
    pub fn new(nickname: String) -> Self {
        Self(nickname)
    }
}
```

However, due to the need of explicit type conversions in [Rust], such API can lack ergonomics in use (notice the explicit conversion methods that API user has to use):

```rust
let mut nickname = Nickname::new("Vasya".to_string());
add_hi(nickname.as_mut());
just_print_stringy(nickname.as_ref());
```

The most standard way to improve ergonomics here is to __hide type conversions under-the-hood by abstracting over input types__ in our APIs:

```rust
use derive_more::{AsRef, AsMut};

pub fn just_print_stringy<S: AsRef<String>>(v: S) {
    println!("{}", v.as_ref())
}

pub fn add_hi<S: AsMut<String>>(mut v: S) {
    v.as_mut().push_str(" Hi")
}

#[derive(AsMut, AsRef)]
pub struct Nickname(String);

impl Nickname {
    pub fn new<S: Into<String>>(nickname: S) -> Self {
        Self(nickname.into())
    }
}

```

And now our API is pleasant to use:

```rust
let mut nickname = Nickname::new("Vasya");
add_hi(&mut nickname);
just_print_stringy(&nickname);
```

This is one of the key features, which drive [Rust] expressiveness and ergonomics. Just look over `std` library to see how widely it's used: [`Iterator::eq()`][1], [`Vec::drain()`][2], [`HashMap::extend()`][3], etc.

The downside of this idiom is that compiler generates more code due to monomorphization, so potentially leads to code bloating. The way it can be optimized has already been [explained in "Reducing code bloat optimization" section of 1.6 task][6].

Further reading on theme:

- [Joe Wilm: From &str to Cow][4]
- [Pascal Hertleif: Elegant Library APIs in Rust: Use conversion traits][5]
- [Carl M. Kadie: Nine Rules for Elegant Rust Library APIs][10]

## Returning concrete type

While for an input parameter we want to accept as much types as possible, for return types a vice versa will be a good practice: __return the most concrete type to provide as much information as possible__, so prefer returning a concrete type instead of `impl Trait` or a type parameter. This extends usage API of a returned type (as it's concrete type not erased), and, potentially, may reduce monomorphization (actually, it shouldn't, as monomorphization mostly happens due to input type parameters).

Consider [`Iterator`] adapter methods as an example: [`Iterator::map()`][7], [`Iterator::enumerate()`][8], [`Iterator::filter()`][9], etc. They all return a concrete adapter type, rather than `impl Iterator<..>` abstraction.

However, this is not a strict rule, so should not be applied blindly. If you _really need_ to abstract over a return type (for example, to future-proof your API), then just do it.

## Task

__Estimated time__: 1 day

Refactor the code contained in [this task's crate](src/main.rs) to make it more efficient, idiomatic, simple and pleasant to use.

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- Why abstracting over input type is good? Which problems does it have and how can they be overcome?
- When returning a concrete type is good? When aren't? What are the trade-offs?

[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.eq
[2]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain
[3]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.extend
[4]: https://jwilm.io/blog/from-str-to-cow
[5]: https://deterministic.space/elegant-apis-in-rust.html#use-conversion-traits
[6]: ../../1_concepts/1_6_dispatch#reducing-code-bloat-optimization
[7]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
[8]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
[9]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[10]: https://towardsdatascience.com/nine-rules-for-elegant-rust-library-apis-9b986a465247
