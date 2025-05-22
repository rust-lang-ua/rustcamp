Task 2.3: Bound behavior, not data
==================================

Often, when we want to abstract over some type or behavior in [Rust] we are going from this:

```rust
struct UserService {
    repo: UserRepo,
}
```

to this:

```rust
struct UserService<R: UserRepo> {
    repo: R,
}
```

We specify `R: UserRepo` bound here as we want to restrict types in `repo` field to implement `UserRepo` behavior.

However, such restriction directly on a type leads to what is called "trait bounds pollution": we have to repeat this bound in every single `impl`, even in those ones, which has no relation to `UserRepo` behavior at all.

```rust
struct UserService<R: UserRepo> {
    repo: R,
}

impl<R> Display for UserService<R>
where
    R: Display + UserRepo, // <- We are not interested in UserRepo here,
{                          //    all we need is just Display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserService with repo {}", self.repo)
    }
}
```

In a complex codebase such pollution multiplies from different types and may become a nightmare at some point.

The solution to this problem would be to understand that a __trait represents a certain behavior__, and, in reality, __we need that behavior only when we're declaring one__. Type declaration has nothing about behavior, it's all about _data_. __It's functions and methods where behavior happens__. So, let's just expect certain behavior when we really need this:

```rust
struct UserService<R> {
    repo: R,
}

// Expect Display when we expressing Display behavior.
impl<R: Display> Display for UserService<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserService with repo {}", self.repo)
    }
}

// Expect UserRepo when we expressing actual UserService behavior,
// which deals with Users.
impl<R: UserRepo> UserService<R> {
    fn activate(&self, user: User) {
        // Changing User state in UserRepo...
    }
}
```

Placing trait bounds on `impl` blocks, methods and functions, rather than on types, _reduces the trait bounds pollution_, _lowers [coupling][1] of code parts_ and _makes generic code more clean, straightforward and ergonomic_.

## Lift unnecessary bounds

As a more general rule: __you should try to lift trait bounds as much as possible__ (especially in a library code), as it enlarges a variety of usages for a type.

Sometimes this requires to omit using `#[derive]` as this may impose unnecessary trait bound. For example:

```rust
#[derive(Clone)]
struct Loader<K, V> {
    state: Arc<Mutex<State<K, V>>>,
}

struct My;

let loader: Loader<My, My> = ..;
let copy = loader.clone(); // compile error as `My` doesn't impl `Clone`
```

This happens because `#[derive(Clone)]` applies `K: Clone` and `V: Clone` bounds in the derived code, despite the fact that they are not necessary at all, as [`Arc` always implements `Clone`][2] (also, consider `T: ?Sized` bound in the [linked implementation][2], which lifts implicit `T: Sized` bound, so allows to use `Arc::clone()` even for [unsized types][3] too).

By providing hand-baked implementation we are able to clone values of `Loader<My, My>` type without any problems:

```rust
struct Loader<K, V> {
    state: Arc<Mutex<State<K, V>>>,
}

// Manual implementation is used to omit applying unnecessary Clone bounds.
impl<K, V> Clone for Loader<K, V> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

let loader: Loader<My, My> = ..;
let copy = loader.clone(); // it compiles now!
```

## Task

__Estimated time__: 1 day

Refactor the code contained in [this task's crate](src/main.rs) to reduce trait bounds pollution as much as possible.

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- Which problems do trait bounds impose in [Rust] when are placed on a type definition?
- Why placing trait bounds on `impl` blocks is better?
- When cannot we do that and should use trait bounds on a type definition? When is it preferred?
- What are the problems with `std` derive macros regarding type parameters? How could they be solved?

[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Coupling_(computer_programming)
[2]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#impl-Clone
[3]: ../../1_concepts/1_7_sized
