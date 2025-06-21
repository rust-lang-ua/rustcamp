Task 2.2: Swapping values with `mem::replace`
=============================================

As [Rust] implies [move semantics][1] by default and quite strict [borrowing rules][2], often, there are situations (especially, with large `struct`s and `enum`s) where mutating value in-place or values swapping may not be allowed by borrow checker, which is quite confusing and leads to doing needless clones (so providing redudant performance costs). For example:
```rust
impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        // error: cannot move out of dereference of `&mut`-pointer
        let buf = self.buf;
        self.buf = Vec::new();
        buf
    }
}
```

A neat and need-to-know trick in such situations is to use [`mem::replace`] (or [`mem::swap`]). It allows to swap two values of the same type without moving things around, partial destructuring and references mess. So, the example above is simply turns into:
```rust
impl<T> Buffer<T> {
    fn get_and_reset(&mut self) -> Vec<T> {
        mem::replace(&mut self.buf, Vec::new())
    }
}
```

For better understanding [`mem::replace`], [`mem::swap`] and [`mem::take`] purpose, design, limitations and use cases, read through the following articles:
- [Official `mem::replace` docs][`mem::replace`]
- [Official `mem::swap` docs][`mem::swap`]
- [Official `mem::take` docs][`mem::take`]
- [Karol Kuczmarski: Moving out of a container in Rust][4]
- [Ferrous Systems: Using `mem::take` to reduce heap allocations][6]

Some examples of useful applying these functions are described below.




## Keeping owned values in changed enums

This situation has detailed explanation in the following article:
- [Rust Design Patterns: `mem::replace` to keep owned values in changed enums][3]

> The borrow checker won't allow us to take out `name` of the enum (because _something_ must be there). We could of course `.clone()` name and put the clone into our `MyEnum::B`, but that would be an instance of the "Clone to satisfy the borrow checker" antipattern. Anyway, we can avoid the extra allocation by changing `e` with only a mutable borrow.
> 
> `mem::replace` lets us swap out the value, replacing it with something else. In this case, we put in an empty `String`, which does not need to allocate. As a result, we get the original `name` _as an owned value_. We can then wrap this in another enum.

```rust
enum MyEnum {
    A { name: String },
    B { name: String },
}

fn swizzle(e: &mut MyEnum) {
    use self::MyEnum::*;
    *e = match *e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { ref mut name } => B { name: mem::replace(name, String::new()) },
        B { ref mut name } => A { name: mem::replace(name, String::new()) },
    }
}
```

> Look ma, no allocation! Also you may feel like Indiana Jones while doing it.




## Mutating embedded collection

Consider the following situation:
```rust
struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}

impl Names {
    fn apply_exclusions(&mut self) {
        self.exclusions.drain(..).for_each(|name| {
            self.remove_name(&name);
        })
    }
    
    fn remove_name(&mut self, name: &str) {
        self.names.remove(name);
    }
}
```
which does not compile due to 2 mutable borrows:
```rust
error[E0500]: closure requires unique access to `*self` but it is already borrowed
  --> src/lib.rs:10:44
   |
10 |         self.exclusions.drain(..).for_each(|name| {
   |         ------------------------- -------- ^^^^^^ closure construction occurs here
   |         |                         |
   |         |                         first borrow later used by call
   |         borrow occurs here
11 |             self.remove_name(&name);
   |             ---- second borrow occurs due to use of `*self` in closure
```

Using [`mem::take`] here allows us to avoid the problem with 2 mutable borrows at almost no cost (`Vec::default()` is no-op), by swapping out the value in a temporary variable:
```rust
impl Names {
    fn apply_exclusions(&mut self) {
        let mut exclusions = mem::take(&mut self.exclusions);
        exclusions.drain(..).for_each(|name| {
            self.remove_name(&name);
        });
    }

    fn remove_name(&mut self, name: &str) {
        self.names.remove(name);
    }
}
```

It's worth mentioning, that this problem became much less common after [disjoint capture in closures had been introduced in 2021 Rust edition][5]. For illustration, the `self.name` mutation is intentionally separated into its own method, so we can lock the whole `&mut self`. If we simplify the code straightforwardly, it just compiles fine, due to mutable borrows are disjoint: 
```rust
struct Names {
    exclusions: Vec<String>,
    names: HashSet<String>,
}

impl Names {
    fn apply_exclusions(&mut self) {
        self.exclusions.drain(..).for_each(|name| {
            self.names.remove(&name);
        })
    }
}
```




## Task

__Estimated time__: 1 day




Improve and optimize the code contained in [this task's crate](src/main.rs) to cut off redudant performance costs.
Add tests.



## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is the reason of [`mem::replace`] existing in [Rust]? What does it give to us? Why cannot we solve the same problems without it?
- Provide some meaningful examples of using [`mem::replace`] in [Rust].




[`mem::replace`]: https://doc.rust-lang.org/std/mem/fn.replace.html
[`mem::swap`]: https://doc.rust-lang.org/std/mem/fn.swap.html
[`mem::take`]: https://doc.rust-lang.org/std/mem/fn.take.html
[Rust]: https://www.rust-lang.org

[1]: https://stackoverflow.com/a/30290070/1828012
[2]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references
[3]: https://rust-unofficial.github.io/patterns/idioms/mem-replace.html
[4]: http://xion.io/post/code/rust-move-out-of-container.html
[5]: https://doc.rust-lang.org/edition-guide/rust-2021/disjoint-capture-in-closures.html
[6]: https://ferrous-systems.com/blog/rustls-borrow-checker-p1
