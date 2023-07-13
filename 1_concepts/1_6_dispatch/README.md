Step 1.6: Static and dynamic dispatch
=====================================

[Static][1] and [dynamic][2] dispatches are important concepts to understand how your code is compiled and works in runtime, and how you can solve certain day-to-day coding problems (related to polymorphism).

__[Static dispatch][1]__ (also called "early binding") __happens only at compile time__. The compiler generates separate code for each concrete type that is used. In [Rust] static dispatch is a __default way for polymorphism__ and is introduced simply by generics (parametric polymorphism): `MyType<T, S, F>`.

__[Dynamic dispatch][2]__ (sometimes called "late binding") __happens at runtime__. The concrete used __[type is erased][4] at compile time__, so compiler doesn't know it, therefore generates `vtable` which dispatches call at runtime and __comes with a performance penalty__. In [Rust] dynamic dispatch is introduced via [trait objects][3]: `&dyn MyTrait`, `Box<dyn MyTrait>`.

You _have to_ use [dynamic dispatch][2] in situations where [type erasure][4] is required. If the problem can be solved with a [static dispatch][1] then you'd better to do so to avoid performance penalties. The most common example when you cannot use [static dispatch][1] and have to go with [dynamic dispatch][2] are _heterogeneous_ collections (where each item is potentially a different concrete type, but each one implements `MyTrait`).

For better understanding [static][1] and [dynamic][2] dispatches purpose, design, limitations and use cases, read through the following articles:
- [Rust Blog: Abstraction without overhead: traits in Rust][11]
- [Joshleeb: Traits and Trait Objects in Rust][12]
- [Rust Book: 17.2. Using Trait Objects That Allow for Values of Different Types][3]
- [Adam Schwalm: Exploring Dynamic Dispatch in Rust][13]
- [Marco Amann: Rust Dynamic Dispatching deep-dive][20]
- [Nicholas Matsakis: Dyn async traits, part 2][17]
- [Armin Ronacher: Rust Any Part 1: Extension Maps in Rust][18]
- [Armin Ronacher: Rust Any Part 2: As-Any Hack][19]




## Object safety

The other reason to go with [static dispatch][1] is that except performance penalties, [trait objects][3] have the other major downside: not all traits can be used for creating [trait objects][3]. A trait needs to meet special [object safety requirements][6]:

> - The trait cannot require `Self: Sized`.
> - Method references the `Self` type in its arguments or return type.
> - Method has generic type parameters.
> - Method has no receiver.
> - The trait cannot contain associated constants.
> - The trait cannot use `Self` as a type parameter in the supertrait listing.

This can lead to quite tricky and non-obvious situations when writing code.

For better understanding [object safety][5] purpose, design and limitations, read through the following articles:
- [Rust Book: 17.2. Object Safety Is Required for Trait Objects][5]
- [Rust Reference: 6.1. Traits: Object Safety][6]
- [Nicholas Matsakis: Dyn async traits, part 2][17]




## Dynamic-to-static optimization for closed types set

In situations where you need to deal with different types, but all possible types form a [closed set][14] (you know _all_ the used types), [dynamic dispatch][2] can be replaced with a [static dispatch][1] in a price of some `enum`-based boilerplate.

For example the following [dynamically dispatched][2] code:
```rust
trait SayHello {
    fn say_hello(&self);
}

struct English;
impl SayHello for English {
    fn say_hello(&self) {
        println!("Hello!")
    }
}

struct Spanish;
impl SayHello for Spanish {
    fn say_hello(&self) {
        println!("Hola!")
    }
}

// We have to use trait object here to contain different types.
let greetings: Vec<Box<dyn SayHello>> = vec![
    Box::new(English),
    Box::new(Spanish),
];
```

Can be refactored in the following way (as far as we know that only `English` and `Spanish` types will be used):
```rust
trait SayHello {
    fn say_hello(&self);
}

struct English;
impl SayHello for English {
    fn say_hello(&self) {
        println!("Hello!")
    }
}

struct Spanish;
impl SayHello for Spanish {
    fn say_hello(&self) {
        println!("Hola!")
    }
}

enum Language {
    English(English),
    Spanish(Spanish),
}
impl SayHello for Language {
    fn say_hello(&self) {
        match self {
            Language::English(l) => l.say_hello(),
            Language::Spanish(l) => l.say_hello(),
        }
    }
}

// We contain different types without using trait objects.
let greetings: Vec<Language> = vec![English, Spanish];
```

There is also a handy [enum_dispatch] crate, which generates this boilerplate automatically in some cases. It has [illustrative benchmarks][15] about performance gains of using `enum` for dispatching.




## Reducing code bloat optimization

[Static dispatch][1] with type parameters has a downside of generating rather a lot of code (for each type), bloating binary size and potentially pessimizing execution cache usage. However, often generics aren’t really needed for speed, but for ergonomics.

The canonical solution of this problem is to factor out an inner method that contains all of the code minus the generic conversions, and leave the outer method as a shell. For example:
```rust
pub fn this<I: Into<String>>(i: I) -> usize {
    // do something really complicated with `i.into()`
    // potentially spanning multiple pages of code
}
```
becomes
```rust
#[inline]
pub fn this<I: Into<String>>(i: I) -> usize {
    _this_inner(i.into())
}
fn _this_inner(i: String) -> usize {
    // same code as above without the conversion
}
```
This ensures only the conversion gets monomorphized, leading to leaner code and compile-time performance wins.

There is a handy [momo] crate, which generates this boilerplate automatically in some cases. Read through its explanation article:
- [Llogiq: Momo · Get Back Some Compile Time From Monomorphization][16]




## Task

__Estimated time__: 1 day




Given the following `Storage` abstraction and `User` entity:
```rust
trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}
```

Implement `UserRepository` type with injectable `Storage` implementation, which can get, add, update and remove `User` in the injected `Storage`. Make two different implementations: one should use [dynamic dispatch][2] for `Storage` injecting, and the other one should use [static dispatch][1].
Prove your implementation correctness with tests.



## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is dispatch? When a function call represents a dispatch and when not?
- How does static dispatch work?
- How does dynamic dispatch work? Why is it required? Which limitations does it have in [Rust]? Why does it have them?
- When dynamic dispatch can be replaced with static dispatch? When not? What are the trade-offs?
- How can we reduce the size of compiler-generated code when using static dispatch?




[enum_dispatch]: https://docs.rs/enum_dispatch
[momo]: https://github.com/llogiq/momo
[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Static_dispatch
[2]: https://en.wikipedia.org/wiki/Dynamic_dispatch
[3]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
[4]: https://en.wikipedia.org/wiki/Type_erasure
[5]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects
[6]: https://doc.rust-lang.org/reference/items/traits.html#object-safety
[11]: https://blog.rust-lang.org/2015/05/11/traits.html
[12]: https://joshleeb.com/posts/rust-traits-and-trait-objects
[13]: https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust
[14]: https://en.wikipedia.org/wiki/Closed_set
[15]: https://docs.rs/enum_dispatch#the-benchmarks
[16]: https://llogiq.github.io/2019/05/18/momo.html
[17]: https://smallcultfollowing.com/babysteps/blog/2021/10/01/dyn-async-traits-part-2
[18]: https://lucumr.pocoo.org/2022/1/6/rust-extension-map
[19]: https://lucumr.pocoo.org/2022/1/7/as-any-hack
[20]: https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
