Step 2.5: Exhaustivity
======================

Exhaustiveness checking in [pattern matching][1] is a very useful tool, allowing to spot certain bugs at compile-time by cheking whether all the combinations of some values where covered and considered in a source code. Being applied correctly, it increases the [fearless refactoring][2] quality of a source code, eliminating possibilities for "forgot to change" bugs to subtly sneak into the codebase whenever it's extended.




## Enums

The most canonical and iconic example of exhaustiveness checking is using an enum in a `match` expression. The point here is to __[omit][5] using [`_` (wildcard pattern)][4] or match-anything bindings__, as such `match` expressions won't break in compile-time when something new is added.

For example, this is a very bad code:
```rust
fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        _ => Permissions::All, // anybody else is administrator 
    }
}
```
If, for some reason, a new `Role::Guest` is added, __with very high probability this code won't be changed accordingly__, introducing a security bug, by granting `Permissions::All` to any guest. This mainly happens, because the code itself doesn't signal back in any way that it should be reconsidered.

By leveraging exhaustivity, the code can be altered in the way __it breaks at compile-time whenever a new `Role` variant is added__:
```rust
fn grant_permissions(role: &Role) -> Permissions {
    match role {
        Role::Reporter => Permissions::Read,
        Role::Developer => Permissions::Read & Permissions::Edit,
        Role::Admin => Permissions::All, 
    }
}
```
```
error[E0004]: non-exhaustive patterns: `&Role::Guest` not covered
  --> src/lib.rs:16:11
   |
16 |     match role {
   |           ^^^^ pattern `&Role::Guest` not covered
   |
note: `Role` defined here
  --> src/lib.rs:2:5
   |
1  | enum Role {
   |      ----
2  |     Guest,
   |     ^^^^^ not covered
```




## Structs

While enums exhaustiveness is quite an obvious idea, due to extensive usage of `match` expressions in a regular code, the structs exhaustiveness, on the other hand, is not, while being as much useful. Exhaustivity for structs is achieved by __using [destructuring][6] without [`..` syntax (multiple fields ignoring)][7]__.

For example, having the following code:
```rust
struct Address {
    country: Country,
    city: City,
    street: Street,
    zip: Zip,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.country)?;
        writeln!(f, "{}", self.city)?;
        writeln!(f, "{}", self.street)?;
        write!(f, "{}", self.zip)
    }
}
```
It's super __easy to forget changing the `Display` implementation when a new `state` field is added__.

So, altering the code with __exhaustive destructuring allows to omit such a subtle bug, by breaking in compile-time__:
```rust
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            country,
            city,
            street,
            zip,
        } = self;
        writeln!(f, "{country}")?;
        writeln!(f, "{city}")?;
        writeln!(f, "{street}")?;
        write!(f, "{zip}")
    }
}
```
```
error[E0027]: pattern does not mention field `state`
  --> src/lib.rs:19:13
   |
19 |           let Self {
   |  _____________^
20 | |             country,
21 | |             city,
22 | |             street,
23 | |             zip,
24 | |         } = self;
   | |_________^ missing field `state`
   |
help: include the missing field in the pattern
   |
23 |             zip, state } = self;
   |                ~~~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
   |
23 |             zip, .. } = self;
   |    
```

Another real-world use-cases of maintaining invariants covering all struct fields via exhaustiveness checking are illustrated in the following articles:
- [Ashley Mannix: How we organize a complex Rust codebase][8]




## `#[non_exhaustive]`

Until now, it has been illustrated how __exhaustiveness checking can future-proof a user code__ (the one which uses API of some type, not declares), by making it to __break whenever the used [API] is extended__ and should be reconsidered.

__`#[non_exhaustive]` attribute__, interestedly, __serves the very same purpose of [future-proofing][12]__ a source code, but in a totally opposite manner: it's __used in a library code__ (the one which declares [API] of some type for usage) to preserve backwards compatibility __for omitting breaking any user code whenever the used [API] is extended__.

> Within the defining crate, `non_exhaustive` has no effect.

> Outside of the defining crate, types annotated with `non_exhaustive` have limitations that preserve backwards compatibility when new fields or variants are added.
> 
> Non-exhaustive types cannot be constructed outside of the defining crate:
> - Non-exhaustive variants (`struct` or `enum` variant) cannot be constructed with a `StructExpression` (including with functional update syntax).
> - `enum` instances can be constructed.

> There are limitations when matching on non-exhaustive types outside of the defining crate:
> - When pattern matching on a non-exhaustive variant (`struct` or `enum` variant), a `StructPattern` must be used which must include a `...` Tuple variant constructor visibility is lowered to `min($vis, pub(crate))`.
> - When pattern matching on a non-exhaustive `enum`, matching on a variant does not contribute towards the exhaustiveness of the arms.

> It's also not allowed to cast non-exhaustive types from foreign crates.

> Non-exhaustive types are always considered inhabited in downstream crates.

Despite being opposite qualities, both exhaustivity and non-exhaustivity are intended for [future-proofing][12] a codebase, thus cannot be applied blindly everywhere, but rather wisely, where it may really has sense. That's why it's __very important__ to understand their __use-cases and implicability__ very well.

For better understanding `#[non_exhaustive]` attribute purpose, design, limitations and use cases, read through the following articles:
- [Rust Reference: 7.6. The `non_exhaustive` attribute][9]
- [Rust RFC 2008: `non_exhaustive`][10]
- [Turreta: Using `#[non_exhaustive]` for Non-exhaustive Rust Structs][11]




## Task

__Estimated time__: 1 day




Refactor the code contained in [this step's crate](src/lib.rs), so the bugs introduced there will be uncovered at compile-time, and fix them appropriately.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- How can exhaustiveness checking be useful in [Rust] code for enums and structs? When should we use it, when not?
- How does `#[non_exhaustive]` attribute work in [Rust]? What are its use-cases? When should it be used, when not?




[API]: https://en.wikipedia.org/wiki/API
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/book/ch18-00-patterns.html
[2]: https://news.ycombinator.com/item?id=27553775
[3]: https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html#match-arms
[4]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern
[5]: https://rust-lang.github.io/rust-clippy/master/index.html#wildcard_enum_match_arm
[6]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values
[7]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-remaining-parts-of-a-value-with-
[8]: https://blog.datalust.co/rust-at-datalust-how-we-organize-a-complex-rust-codebase#maintaininginvariantsthatcoverallstructfields
[9]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
[10]: https://rust-lang.github.io/rfcs/2008-non-exhaustive.html
[11]: https://turreta.com/blog/2019/12/21/using-non_exhaustive-for-non-exhaustive-rust-structs
[12]: https://en.wikipedia.org/wiki/Future-proof
