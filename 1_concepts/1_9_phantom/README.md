Task 1.9: Phantom types
=======================

Because [Rust] has a rich type system, a programming logic and semantics are mostly expressed in types rather than in data/values, which is known as a "programming with types" concept. Often, this leads to situations where you need to express some type relations without having any values of those types. Here is where [phantom types][5] come in: they carry some semantics on type level, which invariants are checked by compiler, and are totally compiled out in runtime.

> A phantom type parameter is simply a type parameter which is never used.

However, in [Rust], this often causes the compiler to complain, and the solution is to add a "dummy" use by way of [`PhantomData`].

This is a quite common practice when you're writing a highly abstracted generics code. A real-world example (and somewhat scary) would be:

```rust
trait CommandGateway<C: Command> {
    type Result;
    
    fn command(&self, cmd: C) -> Self::Result;
}

// Here we need to abstract over some types
// to be able to use them in CommandGateway implementation.
pub struct Snapshotter<Repo, AggEv, Err> {
    repo: Repo,
    _aggregate_event: PhantomData<AggEv>,
    _error: PhantomData<Err>,
}

impl<Cmd, Repo, AggEv, Err> CommandGateway<Cmd>
    for Snapshotter<Repo, AggEv, Err>
where
    Cmd: Command + 'static,
    Cmd::Aggregate: VersionedAggregate
        + EventMessageSourced<
            AggEv,
            EventMeta<<Cmd::Aggregate as Aggregate>::Id>,
        >,
    Repo: AggregateRepository<Cmd::Aggregate>
        + EventStore<
            Cmd::Aggregate,
            Event = AggEv,
            EventMeta = EventMeta<<Cmd::Aggregate as Aggregate>::Id>,
        > + Clone
        + 'static,
    AggEv: AggregateEvent + 'static,
    Err: From<
            SnapshotterError<
                <Repo as AggregateRepository<Cmd::Aggregate>>::Error,
                <Repo as EventStore<Cmd::Aggregate>>::Error,
            >,
        > + 'static,
{
    type Result = DynFuture<Option<Cmd::Aggregate>, Err>;
    
    fn command(&self, cmd: Cmd) -> Self::Result {
        let repo = self.repo.clone();
        let fut = repo
            .load(cmd.aggregate_id().unwrap())
            .map_err(AggregateLoadingFailed)
            .map(|agg| agg.unwrap_or_else(Cmd::Aggregate::initial_state))
            .and_then(move |agg| {
                repo.read_events(
                    cmd.aggregate_id().unwrap(),
                    Some(agg.version()),
                )
                .map_err(EventsReadingFailed)
                .fold((agg, false), |(mut agg, _), ev| {
                    agg.apply_event_message(&ev);
                    Ok((agg, true))
                })
                .map(move |(agg, changed)| (agg, changed, repo))
            })
            .and_then(|(agg, has_changed, repo)| {
                if has_changed {
                    Either::A(
                        repo.store(&agg)
                            .map_err(AggregateStoringFailed)
                            .map(move |_| Some(agg)),
                    )
                } else {
                    Either::B(future::ok(None))
                }
            });
        Box::new(fut.map_err(Err::from))
    }
}
```

For better understanding [`PhantomData`] purpose, design, limitations and use cases, read through the following articles:

- [Official `PhantomData` docs][`PhantomData`]
- [Rust By Example: 14.9. Phantom type parameters][1]
- [Rustonomicon: 3.10. PhantomData][2]
- [Reddit: Why PhantomData][3]
- [RIP Tutorial: Using PhantomData as a Type Marker][4]
- [Aayushya Vajpayee: Write Cleaner, More Maintainable Rust Code with PhantomData][11]
- [Sergey Potapov: Phantom Types in Rust][6]

## Transparency

[`PhantomData`] is transparent for [auto traits][7], which means, for example, that `PhantomData<usize>` is `Send` and `Sized`, while `PhantomData<dyn Any>` is neither `Send` nor `Sized`.

In some situations this allows us to provide the exact semantics we need for a type (like [invariance][8] for [a lifetime][9], for example).

In other situations we don't actually care about semantics of the phantom type parameter at all. Moreover, we don't want the substituted type to change [auto traits][7] implementations of the whole type in any way, preserving only the semantics of the actual contained data, as this may impose ergonomic problems to us:

```rust
struct Nonce<Of>(PhantomData<Of>, usize);

// This compiles OK, as `Nonce<()>` is `Send`.
let nonce: Nonce<()> = Nonce(PhantomData, 1);
thread::spawn(move || {
    println!("{nonce:?}");
});

// This doesn't compile, as `Nonce<Rc<()>>` is not `Send`.
let nonce: Nonce<Rc<()>> = Nonce(PhantomData, 2);
thread::spawn(move || {
    println!("{nonce:?}");
});

// This doesn't compile, as `dyn Any` is not `Sized`.
let nonce: Nonce<dyn Any> = Nonce(PhantomData, 3);
```

To omit such problems, let's just form the correct type inside [`PhantomData`], so we always have the desired [auto traits][7] implementations despite the substituted type:

```rust
struct Nonce<Of: ?Sized>(PhantomData<AtomicPtr<Box<Of>>>, usize);

// This compiles OK now, despite `Rc<()>` is not `Send`.
let nonce: Nonce<Rc<()>> = Nonce(PhantomData, 2);
thread::spawn(move || {
    println!("{nonce:?}");
});

// This compiles OK now, as any `?Sized` type is allowed.
let nonce: Nonce<dyn Any> = Nonce(PhantomData, 3);
```

## Custom phantom type

Interesting enough, despite the [`PhantomData`] being a [lang item][10], it's still possible to define a custom type without using the original [`PhantomData`], but behaving like the one. This is demonstrated quite fairly by the [`ghost`] crate.

```rust
use ghost::phantom;

#[phantom]
#[derive(Copy, Clone, Default, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Crazy<'a, V: 'a, T> where &'a V: IntoIterator<Item = T>;

fn main() {
    let _ = Crazy::<'static, Vec<String>, &'static String>;

    // Lifetime elision.
    let crazy = Crazy::<Vec<String>, &String>;
    println!("{:?}", crazy);
}
```

For more detailed explanation, read through the following articles:

- [Official `ghost` crate docs][`ghost`]

## Task

__Estimated time__: 1 day

Implement a `Fact<T>` type which returns some random fact about `T` type that `Fact<T>` is implemented for.

```rust
let f: Fact<Vec<T>> = Fact::new();
println!("Fact about Vec: {}", f.fact());
println!("Fact about Vec: {}", f.fact());
```

```rust
Fact about Vec: Vec is heap-allocated.
Fact about Vec: Vec may re-allocate on growing.
```

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- Why does [`PhantomData`] exist in [Rust]? Which problems does it solve?
- How does [`PhantomData`]'s transparency work in practice?
- What alternatives of [`PhantomData`] do exist? When is it meaningful to use them?

[`ghost`]: https://docs.rs/ghost
[`PhantomData`]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/rust-by-example/generics/phantom.html
[2]: https://doc.rust-lang.org/nomicon/phantom-data.html
[3]: https://www.reddit.com/r/rust/comments/8oqj14/why_phantomdata
[4]: https://riptutorial.com/rust/example/24109/using-phantomdata-as-a-type-marker
[5]: https://stackoverflow.com/questions/28247543/motivation-behind-phantom-types
[6]: https://www.greyblake.com/blog/phantom-types-in-rust
[7]: https://doc.rust-lang.org/stable/reference/special-types-and-traits.html#auto-traits
[8]: https://docs.rs/variance/0.1.3/src/variance/lib.rs.html#16
[9]: https://docs.rs/variance/0.1.3/src/variance/lib.rs.html#92
[10]: https://manishearth.github.io/blog/2017/01/11/rust-tidbits-what-is-a-lang-item
[11]: https://aayushyavajpayee.substack.com/p/coming-soon
