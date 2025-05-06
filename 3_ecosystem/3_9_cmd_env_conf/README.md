Task 3.9: Command-line arguments, environment variables and configs
===================================================================

## CLI

[Rust] provides a _simple_ [`std::env::Arg`] iterator to access command-line arguments passed to a program.

However, most of the time you require more advanced tool for that, which provides `--help` and `--version` flags out-of-the-box and a _convenient way to setup and describe custom options/flags to build your own_ [CLI (command-line interface)][CLI]. For such cases there is the well-known [`clap`] crate in [Rust] ecosystem.

It has the [`derive` Cargo feature][6] (formerly, [`structopt`] crate) allowing to define [CLI] in a _declarative and clean way_.

For better understanding and familiarity with [CLI] tools in [Rust] ecosystem, read through the following articles:

- [Rust Book: 12.1. Accepting Command Line Arguments][1]
- [Official `std::env::Arg` docs][`std::env::Arg`]
- [Official `clap` crate docs][`clap`]
- [Pavlo Myroniuk: Rust Clap recipes][9]

## Environment variables

[Rust] provides common primitives in [`std::env`] for working with [environment variables][2] as strings.

However, most of the time you want to operate with typed data, not with raw strings. Similarly to [`clap`] for CLI, there is the [`envy`] crate in [Rust] ecosystem, which uses [`serde`] as facade and allows to read data from [environment variables][2] in a _declarative and clean way_ (due to [serde attributes][4] support).

It's worth mentioning, that [`clap`] crate is [able to parse from environment variables][7] too, which is super handy when it comes to backing your [CLI] with [environment variables][2].

Finally, [`dotenv`] crate should be mentioned. It sets [environment variables][2] basing on [`.env` file][8] contents, which is widely used convention to simplify environment configuration and to omit declaring all the required environment variables by hand each time when running some program. This one is especially _useful in development_ (consider also [`rs-env`] and [`direnv`] for better development experience).

For better understanding and familiarity with [environment variables][2] tools in [Rust] ecosystem, read through the following articles:

- [Rust Book: 12.5. Working with Environment Variables][3]
- [Official `std::env` docs][`std::env`]
- [Official `envy` crate docs][`envy`]
- [Official `dotenv` crate docs][`dotenv`]

## Configuration

For dealing with configurations there is the well-known [`config`] crate in [Rust] ecosystem, which simplifies creation and usage of hierarchical typed configuration structures in a [12-factor] way.

> `Config` lets you set a set of default parameters and then extend them via merging in configuration from a variety of sources:
>
> - Environment variables
> - String literals in well-known formats
> - Another `Config` instance
> - Files: TOML, JSON, YAML, INI, RON, JSON5 and custom ones defined with `Format` trait
> - Manual, programmatic override (via a `.set` method on the `Config` instance)
>
> Additionally, `Config` supports:
>
> - Live watching and re-reading of configuration files
> - Deep access into the merged configuration via a path syntax
> - Deserialization via `serde` of the configuration or any subset defined via a path

For better understanding and familiarity with [`config`] crate design, concepts, usage, and features, read through the following articles:

- [Official `config` crate docs][`config`]
- [`config` crate examples][5]

## Task

__Estimated time__: 1 day

Write a simple program which prints out its actual configuration. Configuration should be implemented as a typed hierarchical structure, which is able to parse from a specified file and/or environment variables.

The following priority should be applied (in ascending order) when merging:

1. Default values declared directly in [Rust] sources;
2. Values read from TOML file;
3. Values set by environment variables with `CONF_` prefix.

[CLI] of the program should look like:

```rust
$ cargo run -- --help
Prints its configuration to STDOUT

Usage: task_3_9 [OPTIONS]

Options:
  -d, --debug        Enables debug mode
  -c, --conf <CONF>  Path to configuration file [env: CONF_FILE=]  [default: config.toml]
  -h, --help         Print help
  -V, --version      Print version
```

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- What are the benefits of having strongly-type configuration?
- Why environment variables are useful for configuring an application? What is the main use-case for them?
- How is [`config`] crate really useful? Why should we it and cannot just deserialize a file into structs via [`serde`]?

[`clap`]: https://docs.rs/clap
[`config`]: https://docs.rs/config
[`direnv`]: https://direnv.net
[`dotenv`]: https://docs.rs/dotenv
[`envy`]: https://docs.rs/envy
[`rs-env`]: https://github.com/sysid/rs-env
[`serde`]: https://docs.rs/serde
[`std::env`]: https://doc.rust-lang.org/std/env/index.html
[`std::env::Arg`]: https://doc.rust-lang.org/std/env/struct.Args.html
[`structopt`]: https://docs.rs/structopt
[12-factor]: https://12factor.net/config
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[Rust]: https://www.rust-lang.org

[1]: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
[2]: https://en.wikipedia.org/wiki/Environment_variable
[3]: https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html
[4]: https://serde.rs/attributes.html#field-attributes
[5]: https://github.com/mehcode/config-rs/tree/master/examples
[6]: https://docs.rs/clap/latest/clap#example
[7]: https://docs.rs/clap/latest/clap/parser/enum.ValueSource.html#variant.EnvVariable
[8]: https://github.com/bkeepers/dotenv#usage
[9]: https://tbt.qkation.com/posts/clap-recipes
