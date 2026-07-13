Task 3.1: Testing and mocking
=============================

📚 [Rust] testing ecosystem 📋 [is not huge, but has grown quite well][1], providing some interesting libraries and solutions.




## Built-in testing capabilities

📚 [Rust] provides quite good built-in testing capabilities, which are very well described in the following articles:
- 📚 [Rust Book: 11. Writing Automated Tests][2]
- 📚 [Rust By Example: 21. Testing][3]
- 📚 [Rust By Example: 12.3. Tests][4]




## BDD style

📰 [BDD (behavior-driven development)][BDD] testing style implies that _test cases represent a program specification_, while _tests themselves prove the specification correctness_.

While 📚 [Rust] ecosystem has 📚 [some BDD testing style crates][11] (the most mature one is 📚 [`cucumber`] crate), it's not a requirement to use them to follow the 📰 [BDD] style (as they may be too complex for some trivial cases, like 📰 [unit testing][12]). There is nothing preventing you from following 📰 [BDD] style in usual 📚 [Rust] tests. So, instead of:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash() {
        let h = hash("some_string");
        
        assert_eq!(h.len(), 64);
        assert!(!h.contains("z"));
    }
}
```
You're always free to write it more meaningfully:
```rust
#[cfg(test)]
mod hash_spec {
    use super::*;
    
    #[test]
    fn has_64_symbols_len() {
        assert_eq!(hash("some_string").len(), 64);
    }
    
    #[test]
    fn contains_hex_chars_only() {
        assert!(!hash("some_string").contains("z"));
    }
}
```
This makes tests more granular (and so, more meaningful test failures) and testing intentions become more understandable for readers.




## Mocking

📚 [Rust] ecosystem has 📋 [enough solutions][1] for 📰 [mocking][41], some of them are quite mature.

The most interested one is 📚 [`mockiato`] crate at the moment, as is quite ergonomic in use and supports stable 📚 [Rust]. 📚 [`unimock`] crate works in the very similar way, but supports supertraits, as uses the single `Unimock` type for mocking. 📚 [`faux`] and 📚 [`mry`] crates are focused on struct mocking (instead of traits).

Additionally, 📚 [`mockito`] and 📚 [`wiremock`] crates should be mentioned as a quite useful one for HTTP testing.

The most powerful, however, is 📚 [`mockall`] crate. See 📰 [this overview][43] for more details.

For better overview and familiarity with 📰 [mocking][41] in 📚 [Rust], read through the following articles:
- 📰 [Alan Somers: Rust Mock Shootout!][43]
- 📰 [Oduah Chigozie: Mocking in Rust: Mockall and alternatives][45]
- 📚 [Official `mockall` crate docs][`mockall`]
- 📚 [Official `mockiato` crate docs][`mockiato`]
- 📚 [Official `unimock` crate docs][`unimock`]
- 🧭 [Audun Halland: How to write a type-level mock library in Rust][44]




## Property testing

📰 [Property testing][21] is another testing paradigm for considering. In a nutshell, it can be explained in the following way:

> _Property testing_ is a system of testing code by checking that certain properties of its output or behaviour are fulfilled for all inputs. These inputs are generated automatically, and, critically, when a failing input is found, the input is automatically reduced to a _minimal_ test case.

📚 [Rust] ecosystem has quite good 📚 [`proptest`] and 📚 [`quickcheck`] crates, which provide tools and primitives for 📰 [property testing][21].

For better understanding and familiarity with 📰 [property testing][21] in 📚 [Rust], read through the following articles:
- 📚 [`proptest` crate description][`proptest`]
- 📚 [`quickcheck` crate description][`quickcheck`]
- 📰 [Proptest Book][22]




## Fuzzing

📰 [Fuzzing][31] is another testing technique, which involves providing invalid, unexpected, or random data as inputs to a computer program. It 📰 [really helps][32] to spot program crashes and memory leaks in edge cases.

📚 [Rust] ecosystem has 📚 [several tools][33] for 📰 [fuzzing][31] at the moment. Most known are:
- 📚 [`cargo-fuzz`] is a command-line wrapper for using 📰 [`libFuzzer`].
- 📰 [afl.rs] allows to run 📰 [AFL (american fuzzy lop)][AFL] on code written in 📚 [Rust].
- 📚 [`honggfuzz`] is a security oriented fuzzer with powerful analysis options, which supports evolutionary, feedback-driven fuzzing based on code coverage (software- and hardware-based).

For better understanding and familiarity with 📰 [fuzzing][31] in 📚 [Rust], read through the following articles:
- 📰 [Rust Fuzz Book][34]
- 📚 [Official `cargo-fuzz` crate docs][`cargo-fuzz`]
- 📚 [Official `honggfuzz` crate docs][`honggfuzz`]
- 📰 [Adrian Taylor: Comparative fuzzing parallel Rust tools][35]




## More reading

- 🧭 [Aleksey Kladov: How to Test][63]
- 📰 [Joshua Mo: Everything you need to know about testing in Rust][64]

## Integrated tests

- 🎥 [Integrated tests are scam (J.B. Rainsberg)][61]
- 📰 [nextest][62]

## CLI Testing
- [assert_cmd](https://crates.io/crates/assert_cmd) - Easy command initialization and assertions.
- [assert_fs](https://crates.io/crates/assert_fs) - Filesystem fixtures and assertions for testing.
- [predicates](https://crates.io/crates/predicates) - Composable first-order predicate functions.
- [rexpect](https://crates.io/crates/rexpect) - For interactive CLI testing.
- [CLI testing](https://rust-cli.github.io/book/tutorial/testing.html)
- [example with rexpect](https://www.rustadventure.dev/building-a-digital-garden-cli/clap-v4/testing-interactive-clis-with-rexpect)
- [one more example](https://out-of-cheese-error.netlify.app/the-way)


## Task

__Estimated time__: 1 day




For the implementation of a small 📚 [guessing game][51] in [this task's crate](src/main.rs) provide all possible tests you're able to write.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
1. What is TDD style? What is BDD style? Where is the essential accent of BDD?
2. What is mocking? When is it useful?
3. What is property-based testing? How does it achieve its goals?
4. What is fuzzing? How does it differ from property testing?




[`cargo-fuzz`]: https://docs.rs/cargo-fuzz
[`cucumber`]: https://docs.rs/cucumber
[`faux`]: https://docs.rs/faux
[`honggfuzz`]: https://docs.rs/honggfuzz
[`libFuzzer`]: https://llvm.org/docs/LibFuzzer.html
[`mockall`]: https://docs.rs/mockall
[`mockiato`]: https://docs.rs/mockiato
[`mockito`]: https://docs.rs/mockito
[`mry`]: https://docs.rs/mry
[`proptest`]: https://docs.rs/proptest
[`quickcheck`]: https://docs.rs/quickcheck
[`unimock`]: https://docs.rs/unimock
[`wiremock`]: https://docs.rs/wiremock
[AFL]: http://lcamtuf.coredump.cx/afl
[afl.rs]: https://github.com/rust-fuzz/afl.rs
[BDD]: https://en.wikipedia.org/wiki/Behavior-driven_development
[Rust]: https://www.rust-lang.org

[1]: https://github.com/rust-unofficial/awesome-rust#testing
[2]: https://doc.rust-lang.org/book/ch11-00-testing.html
[3]: https://doc.rust-lang.org/rust-by-example/testing.html
[4]: https://doc.rust-lang.org/rust-by-example/cargo/test.html
[11]: https://crates.io/search?q=bdd
[12]: https://en.wikipedia.org/wiki/Unit_testing
[21]: https://en.wikipedia.org/wiki/Property_testing
[22]: https://altsysrq.github.io/proptest-book/intro.html
[31]: https://en.wikipedia.org/wiki/Fuzzing
[32]: https://github.com/rust-fuzz/trophy-case
[33]: https://crates.io/search?q=fuzzing
[34]: https://rust-fuzz.github.io/book/cargo-fuzz.html
[35]: https://medium.com/@adetaylor/comparative-fuzzing-parallel-rust-tools-fac5ce9c9c2d
[41]: https://en.wikipedia.org/wiki/Mock_object
[43]: https://asomers.github.io/mock_shootout
[44]: https://audunhalland.github.io/blog/how-to-write-a-type-level-mock-library-in-rust
[45]: https://blog.logrocket.com/mocking-rust-mockall-alternatives
[51]: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
[61]: https://www.youtube.com/watch?v=VDfX44fZoMc
[62]: https://nexte.st/
[63]: https://matklad.github.io/2021/05/31/how-to-test.html
[64]: https://www.shuttle.rs/blog/2024/03/21/testing-in-rust
