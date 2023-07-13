Step 3.4: Regular expressions and custom parsers
================================================



## Regular expressions

To operate with [regular expressions][1] there is the [`regex`] crate in [Rust] ecosystem, which is kinda a default choice to go with in most cases.

> A Rust library for parsing, compiling, and executing regular expressions. Its syntax is similar to Perl-style regular expressions, but lacks a few features like look around and backreferences. In exchange, all searches execute in linear time with respect to the size of the regular expression and search text. Much of the syntax and implementation is inspired by [RE2].

If you need additional features (like look around and backreferences), consider using:
- [`fancy-regex`] crate, building additional functionality on top of the [`regex`] crate.
- [`pcre2`] crate, providing a safe high level Rust binding to [PCRE2] library.
- [`hyperscan`] crate, wrapping a [Hyperscan] library.


### Compile only once

Important to know, that in [Rust] __regular expression needs to be compiled before we can use it__. The compilation is not cheap. So, the following code introduces a performance problem:
```rust
fn is_email(email: &str) -> bool {
    let re = Regex::new(".+@.+").unwrap();  // compiles every time the function is called
    re.is_match(email)
}
```

To omit unnecessary performance penalty we should __compile regular expression once and reuse its compilation result__. This is easily achieved by using the [`once_cell`] crate both in global and/or local scopes:
```rust
static REGEX_EMAIL: Regex = once_cell::sync::Lazy::new(|| {
    Regex::new(".+@.+").unwrap()
}); // compiles once on the first use

fn is_email(email: &str) -> bool {
    REGEX_EMAIL.is_match(email)
}
```

This may feel different with how [regular expressions][1] are used in other programming languages, because some of them implicitly cache compilation results and/or do not expose compilation API at all (like [PHP]). But if your background is a language like [Go] or [Java], this concept should be familiar to you.




## Custom parsers

If regular expressions are [not powerful enough][2] for your parsing problem, then you are ended up with writing your own parser. [Rust] ecosystem has [numerous][3] crates to help with that:
- [Parser combinators][4]:
    - [`nom`] crate, nearly the most performant among others, and especially good for parsing binary stuff (byte/bit-oriented).
    - [`chumsky`] crate, focusing on high-quality errors and ergonomics over performance.
    - [`combine`] crate, inspired by the [Parsec] library in [Haskell].
    - [`pom`] crate, providing [PEG][5] parser combinators created using operator overloading without macros.
    - [`chomp`] crate, a fast [monadic][13]-style [parser combinator][4] library.
- [Parser generators][12]:
    - [`peg`] crate, a simple yet flexible [parser generator][12] that makes it easy to write robust parsers, based on the [Parsing Expression Grammar][5] formalism.
    - [`pest`] crate, with a focus on accessibility, correctness, and performance, using [PEG (parsing expression grammar)][5] as an input and deriving parser's code for it.
    - [`lalrpop`] crate, generating [LR(1) parser][6] code from custom grammar files.
    - [`parsel`] crate, a library for generating parsers directly from syntax tree node types.

For better understanding parsing problem and approaches, along with some examples, read through the following articles:
- [Laurence Tratt: Which Parsing Approach?][9]
- [Richard L. Apodaca: A Beginner's Guide to Parsing in Rust][10]
- [Eshan Singh: Practical Parsing in Rust with nom][14]
- [Nazmul Idris: Guide to parsing with nom][18]
- [Brian Kung: Building a CEDICT parser in Rust with Nom][11]
- [The Nom Guide (Nominomicon)][19]
- [Aleksey Kladov: Resilient LL Parsing Tutorial][15]
- [Aleksey Kladov: Simple but Powerful Pratt Parsing][16]
- [Aleksey Kladov: From Pratt to Dijkstra][17]




## Task

__Estimated time__: 1 day




Given the following [Rust `fmt` syntax grammar][7]:
> ```
> format_string := text [ maybe_format text ] *
> maybe_format := '{' '{' | '}' '}' | format
> format := '{' [ argument ] [ ':' format_spec ] [ ws ] * '}'
> argument := integer | identifier
>
> format_spec := [[fill]align][sign]['#']['0'][width]['.' precision]type
> fill := character
> align := '<' | '^' | '>'
> sign := '+' | '-'
> width := count
> precision := count | '*'
> type := '' | '?' | 'x?' | 'X?' | identifier
> count := parameter | integer
> parameter := argument '$'
> ```
> In the above grammar,
> - `text` must not contain any `'{'` or `'}'` characters,
> - `ws` is any character for which [`char::is_whitespace`](https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace) returns `true`, has no semantic meaning and is completely optional,
> - `integer` is a decimal integer that may contain leading zeroes and must fit into an `usize` and
> - `identifier` is an `IDENTIFIER_OR_KEYWORD` (not an `IDENTIFIER`) as defined by the [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html).

Implement a parser to parse `sign`, `width` and `precision` from a given input (assumed to be a `format_spec`).

Provide implementations in two flavours: [`regex`]-based and via building a custom parser.

Prove your implementation correctness with tests.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- How does [`regex`] crate achieve linear time complexity? In what price?
- How to avoid regular expression recompilation in [Rust]? Why is it important?
- Which are the common kinds of libraries for writing custom parses in [Rust]? Which benefits does each one have?
- What advantages does libraries give for writing a custom parser? Are they mandatory? When does it make sense to avoid using a library for implementing a parser?




[`chomp`]: https://docs.rs/chomp
[`chumsky`]: https://docs.rs/chumsky
[`combine`]: https://docs.rs/combine
[`fancy-regex`]: https://docs.rs/fancy-regex
[`hyperscan`]: https://docs.rs/hyperscan
[`lalrpop`]: https://docs.rs/lalrpop
[`nom`]: https://docs.rs/nom
[`once_cell`]: https://docs.rs/once_cell
[`parsel`]: https://docs.rs/parsel
[`peg`]: https://docs.rs/peg
[`pest`]: https://docs.rs/pest
[`pcre2`]: https://docs.rs/pcre2
[`pom`]: https://docs.rs/pom
[`regex`]: https://docs.rs/regex
[Go]: https://golang.org
[Haskell]: https://www.haskell.org
[Hyperscan]: https://github.com/intel/hyperscan
[Java]: https://www.java.com
[Parsec]: https://hackage.haskell.org/package/parsec
[PCRE2]: https://www.pcre.org
[PHP]: https://php.net
[RE2]: https://github.com/google/re2
[Rust]: https://www.rust-lang.org

[1]: https://en.wikipedia.org/wiki/Regular_expression
[2]: https://stackoverflow.com/questions/1732348/regex-match-open-tags-except-xhtml-self-contained-tags
[3]: https://github.com/rust-unofficial/awesome-rust#parsing
[4]: https://en.wikipedia.org/wiki/Parser_combinator
[5]: https://en.wikipedia.org/wiki/Parsing_expression_grammar
[6]: https://en.wikipedia.org/wiki/Canonical_LR_parser
[7]: https://doc.rust-lang.org/std/fmt/index.html#syntax
[8]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[9]: https://tratt.net/laurie/blog/entries/which_parsing_approach.html
[10]: https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust
[11]: https://briankung.dev/2021/12/07/building-a-cedict-parser-in-rust-with-nom
[12]: https://en.wikipedia.org/wiki/Parser_generator
[13]: https://en.wikipedia.org/wiki/Monad_(functional_programming)
[14]: https://naiveai.hashnode.dev/practical-parsing-nom
[15]: https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html
[16]: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
[17]: https://matklad.github.io/2020/04/15/from-pratt-to-dijkstra.html
[18]: https://developerlife.com/2023/02/20/guide-to-nom-parsing
[19]: https://tfpk.github.io/nominomicon/introduction.html
