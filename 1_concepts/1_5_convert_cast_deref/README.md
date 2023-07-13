Step 1.5: Conversions, casting and dereferencing
================================================

As [Rust] is a [strongly typed][1] language, all type conversions must be performed explicitly in the code. As [Rust] has a rich type system (programming logic and semantics are mostly expressed in types rather than in values), type conversions are inevitable in almost every single line of code. Fortunately, [Rust] offers [well-designed type conversion capabilities][`std::convert`], which are quite ergonomic, intuitive and are pleasant to use.




## Value-to-value conversion

Value-to-value conversion in [Rust] is done with [`From`] and [`Into`] mirrored traits (implementing one automatically implements another one). These traits provide __non-fallible conversion__.

If your conversion may fail, then you should use [`TryFrom`]/[`TryInto`] analogues, which __allow failing in a controlled way__.

```rust
let num: u32 = 5;
let big_num: u64 = num.into();
let small_num: u16 = big_num.try_into().expect("Value is too big");
```

Note, that __all these traits consume ownership__ of a passed value. However, they [can be implemented for references too][2] if you're treating a reference as a value.

For better understanding [`From`]/[`Into`] and [`TryFrom`]/[`TryInto`] purpose, design, limitations and use cases read through:
- [Rust By Example: 6.1. From and Into][8]
- [Official `From` docs][`From`]
- [Official `Into` docs][`Into`]
- [Official `TryFrom` docs][`TryFrom`]
- [Official `TryInto` docs][`TryInto`]




## Reference-to-reference conversion

Quite often you don't want to consume ownership of a value for conversion, but rather to refer it as another type. In such case [`AsRef`]/[`AsMut`] should be used. They allow to do a __cheap non-fallible reference-to-reference conversion__.

```rust
let string: String = "some text".into();
let bytes: &[u8] = string.as_ref();
```

[`AsRef`]/[`AsMut`] are commonly implemented for smart pointers to allow referring a data behind it via regular [Rust] references.

For better understanding [`AsRef`]/[`AsMut`] purpose, design, limitations and use cases read through:
- [Official `AsRef` docs][`AsRef`]
- [Official `AsMut` docs][`AsMut`]
- [Ricardo Martins: Convenient and idiomatic conversions in Rust][10]


### Difference from [`Borrow`]

Novices in [Rust] are often confused with the fact that [`AsRef`]/[`AsMut`] and [`Borrow`]/[`BorrowMut`] traits have the same signatures, because it may not be clear which trait to use or implement for their needs.

See [explanation in `Borrow` trait docs][`Borrow`]:

> Further, when providing implementations for additional traits, it needs to be considered whether they should behave identical to those of the underlying type as a consequence of acting as a representation of that underlying type. Generic code typically uses `Borrow<T>` when it relies on the identical behavior of these additional trait implementations. These traits will likely appear as additional trait bounds.
> 
> In particular `Eq`, `Ord` and `Hash` must be equivalent for borrowed and owned values: `x.borrow() == y.borrow()` should give the same result as `x == y`.
> 
> If generic code merely needs to work for all types that can provide a reference to related type `T`, it is often better to use `AsRef<T>` as more types can safely implement it.

And [another one in `AsRef` trait docs][`AsRef`]:

> - Unlike `AsRef`, `Borrow` has a blanket impl for any `T`, and can be used to accept either a reference or a value.
> - `Borrow` also requires that `Hash`, `Eq` and `Ord` for a borrowed value are equivalent to those of the owned value. For this reason, if you want to borrow only a single field of a struct you can implement `AsRef`, but not `Borrow`.

So, as a conclusion:
- [`AsRef`]/[`AsMut`] means that the implementor type may be represented as a reference to the implemented type. More like one type contains another one, or is just generally reference-convertible to the one.
- [`Borrow`]/[`BorrowMut`] means that the implementor type is equivalent to the implemented type in its semantics, differing only in how its data is stored. More like one type is just a pointer to another one.

For example, it's natural for an `UserEmail` type to implement `Borrow<str>`, so it may be easily consumed in the code accepting `&str` (converted to `&str`), as they're semantically equivalent regarding `Hash`, `Eq` and `Ord`. And it's good for some execution `Context` to implement `AsRef<dyn Repository>`, so it can be extracted and used where needed, without using the whole `Context`.

For better understanding [`AsRef`]/[`Borrow`] differences, read through:
- [Anup Jadhav: AsRef vs Borrow trait (ft. ChatGPT)][12]


### Inner-to-outer conversion

[`AsRef`]/[`AsMut`] are able to do only outer-to-inner reference conversion, but obviously not the opposite.

```rust
struct Id(u8);

impl AsRef<u8> for Id {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

impl AsRef<Id> for u8 {
    fn as_ref(&self) -> &Id {
        &Id(*self)
    }
}
```
```
error[E0515]: cannot return reference to temporary value
  --> src/lib.rs:11:9
   |
11 |         &Id(*self)
   |         ^---------
   |         ||
   |         |temporary value created here
   |         returns a reference to data owned by the current function
```

However, there is nothing wrong with such conversion as long as memory layout of the inner type is the same for the outer type.

```rust
#[repr(transparent)]
struct Id(u8);

impl AsRef<Id> for u8 {
    fn as_ref(&self) -> &Id {
        unsafe { mem::transmute(self) }
    }
}
```

That's exactly what [`ref-cast`] crate checks and does, without necessity of writing `unsafe` explicitly. See [crate's documentation][`ref-cast`] for more explanations.




## Dereferencing

[`Deref`]/[`DerefMut`] standard library trait __allows to implicitly coerce from a custom type to a reference__ when dereferencing (operator `*v`) is used. The most common example of this is using [`Box<T>`][`Box`] where `&T` is expected.

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

let m = Box::new(String::from("Rust"));
hello(&m);
```

For better understanding [`Deref`] purpose, design, limitations and use cases read through:
- [Rust Book: 15.2. Treating Smart Pointers Like Regular References with the Deref Trait][3]
- [Official `Deref` docs][`Deref`]
- [Deref vs AsRef vs Borrow vs Cow][12]


### Incorrect usage

The implicit coercion that [Rust] implements for [`Deref`] is a sweet honey pot which may lead you to misuse of this feature.

The common temptation is to use [`Deref`] in a combination with [newtype pattern][4], so you can use your inner type via outer type without any explicit requirements. However, this is considered to be a bad practice, and [official `Deref` docs][`Deref`] clearly states:

> __`Deref` should only be implemented for smart pointers.__

The wider explanation of this bad practice is given in [this SO answer][5] and [`Deref` polymorphism anti-pattern][6] description.




## Casting

For casting between types the [`as` keyword][`as`] is used in [Rust].

```rust
fn average(values: &[f64]) -> f64 {
    let sum: f64 = sum(values);
    let size: f64 = len(values) as f64;
    sum / size
}
```

However, it supports only a [small, fixed set of transformations][7], and __is [not idiomatic][11] to use when other conversion possibilities are available__ (like [`From`], [`TryFrom`], [`AsRef`]).

See also:
- [Rust By Example: 5.1. Casting][9]
- [Rust Reference: 8.2.4. Type cast expressions][7]




## Task


__Estimated time__: 1 day




Implement the following types:
1. `EmailString` - a type, which value can be only a valid email address string.
2. `Random<T>` - a smart pointer, which takes 3 values of the pointed-to type on creation and points to one of them randomly every time is used.

Provide conversion and `Deref` implementations for these types on your choice, to make their usage and interoperability with `std` types easy and ergonomic.

Write simple tests for the task.



## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- How value-to-value conversion is represented in [Rust]? What is relation between fallible and infallible one?
- How reference-to-reference conversion is represented in [Rust]? How its traits differ? When and which one should be used?
- How can inner-to-outer reference conversion be achieved in [Rust]? Which prerequisites does it have?
- What is dereferencing in [Rust]? How it can be abused? Why it shouldn't be abused?
- Why using [`as`] keyword is not a good practice in [Rust]? Why do we still use it?




[`as`]: https://doc.rust-lang.org/std/keyword.as.html
[`AsMut`]: https://doc.rust-lang.org/std/convert/trait.AsMut.html
[`AsRef`]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
[`Borrow`]: https://doc.rust-lang.org/std/borrow/trait.Borrow.html
[`BorrowMut`]: https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html
[`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
[Rust]: https://www.rust-lang.org
[`ref-cast`]: https://docs.rs/ref-cast
[`std::convert`]: https://doc.rust-lang.org/std/convert/index.html
[`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
[`TryInto`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html

[1]: https://en.wikipedia.org/wiki/Strong_and_weak_typing
[2]: https://doc.rust-lang.org/std/string/struct.String.html#impl-From%3C%26%27_%20str%3E
[3]: https://doc.rust-lang.org/book/ch15-02-deref.html
[4]: https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
[5]: https://stackoverflow.com/questions/45086595/is-it-considered-a-bad-practice-to-implement-deref-for-newtypes
[6]: https://rust-unofficial.github.io/patterns/anti_patterns/deref.html
[7]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
[8]: https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
[9]: https://doc.rust-lang.org/rust-by-example/types/cast.html
[10]: https://ricardomartins.cc/2016/08/03/convenient_and_idiomatic_conversions_in_rust
[11]: https://rust-lang.github.io/rust-clippy/master/index.html#as_conversions
[12]: https://rusty-ferris.pages.dev/blog/asref-vs-borrow-trait
