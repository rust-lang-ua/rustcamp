Task 4.4: RPC
========================================

[gRPC] is a widely-adopted high performance [RPC] framework, having a __strict schema__, powered with pluggable support for load balancing, tracing, health checking and authentication, built on top of [HTTP/2] (and so, having a __mandatory encryption__), and __heavily using code-from-schema generation__.

For more familiarity with [gRPC], read through the following articles:

- [gRPC docs: Introduction to gRPC][301]
- [gRPC docs: Core concepts, architecture and lifecycle][302]

### Server and client

For implementing a [gRPC] server in [Rust], there are two main production-ready crates in its ecosystem: [`tonic`] (pure [Rust] implementation, based on [`tokio`]) and [`grpcio`] (wrapper around [gRPC core][311] implementation).

In [gRPC] ecosystem, usually, implementing a [gRPC] client doesn't differ much from implementing a server, since both are auto-generated from the same `.proto` schema. So, for [Rust], the same [`tonic`] and [`grpcio`] crates do the job when it comes to making [gRPC] requests.

For more familiarity with using [gRPC] in [Rust], read through the following articles:

- [Official `tonic` crate docs][`tonic`]
- [Official `grpcio` crate docs][`grpcio`]

## Task

__Estimated time__: 1 day

Rework the application [from the previous task](../4_3_api/README.md#task) by introducing [gRPC] as an [API] between the ["thick client"][41] and the server, while preserving the [REST]ful [API]:

- Server communicates with the client via [gRPC].
- [CLI] client parses commands by itself and makes accurate requests to the server via [gRPC].
- it should be still possible to perform all the operations via [cURL] (or any other [HTTP]/[API] client) directly on the [REST]ful [API] server.

Try to keep your solution as simple and straightforward as possible.

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- What is gRPC? What is the difference between RPC and gRPC?
- What are the strengths of gRPC? Which are good use-cases for it, and which are not? Why?

[`grpcio`]: https://docs.rs/crate/grpcio
[`tarpc`]: https://docs.rs/tarpc
[`tonic`]: https://docs.rs/tonic
[`tokio`]: https://docs.rs/tokio
[API]: https://en.wikipedia.org/wiki/API
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[cURL]: https://en.wikipedia.org/wiki/CURL
[gRPC]: https://grpc.io
[HTTP]: https://en.wikipedia.org/wiki/HTTP
[HTTP/2]: https://en.wikipedia.org/wiki/HTTP/2
[REST]: https://en.wikipedia.org/wiki/Representational_state_transfer
[RPC]: https://en.wikipedia.org/wiki/Remote_procedure_call
[Rust]: https://www.rust-lang.org

[301]: https://grpc.io/docs/what-is-grpc/introduction
[302]: https://grpc.io/docs/what-is-grpc/core-concepts
[311]: https://github.com/grpc/grpc
[41]: https://en.wikipedia.org/wiki/Rich_client
