Step 4.3: API servers, clients and tools
========================================

Naturally, in [client-server][4] applications, a client and a server negotiate with each other via some [API (application programming interface)][API], which often takes form of [RPC (remote procedure call)][RPC] for better structuring and standardizing (due [IDL (interface definition language)][IDL] usage).

[Rust] ecosystem provides support for all modern widely-used and adopted [RPC] technologies, and even comes with its [own unique ones][`tarpc`].




## RESTful

Since [REST] is rather an __architecture convention/style__ than a strict [specification][3] for [RPC], and [REST]ful [API]s are typically __loosely based on [HTTP] methods__ directly, there is usually __no need in special frameworks__ in [Rust] to implement a [REST]ful [API] server or to request the one. Just any [HTTP server][101] or [HTTP client][231] will do.

This approach, however, __suffers from lacking [API] schema__, and so, makes it hard to build a rich ecosystem around with ready-to-use tooling (or connect with existing ones). Fortunately, this is easily solved by using a concrete [RPC specification][3] on top of [REST] conventions, and following it strictly. 

For more information about [REST], read through the following articles:
- [Tyler Charboneau: What’s the Difference Between RPC and REST?][111]


### OpenAPI

[OpenAPI] (former [Swagger]) is a [specification][3] for a [machine-readable][102] [IDL (interface definition language)][IDL], allowing to describe, produce, consume and visualize [REST]ful web [API]s. In a nutshell, [OpenAPI] is a __kind of [REST]-based [RPC]__.

> The OpenAPI Specification (OAS) defines a standard, language-agnostic interface to HTTP APIs which allows both humans and computers to discover and understand the capabilities of the service without access to source code, documentation, or through network traffic inspection. When properly defined, a consumer can understand and interact with the remote service with a minimal amount of implementation logic.
>
> An OpenAPI definition can then be used by documentation generation tools to display the API, code generation tools to generate servers and clients in various programming languages, testing tools, and many other use cases.

In [Rust] ecosystem, most [OpenAPI] crates follow the __code-first approach__ (generating [OpenAPI] schema from source code). The most notable crates for this are [`utoipa`] and [`okapi`].

For the opposite (generating source code from [OpenAPI] schema) [Rust] ecosystem lacks its own pure implementation, and the original [OpenAPI] tool [`openapi-generator`] should be used (powered by the [`swagger`] crate).

For more familiarity with [OpenAPI] and using it in [Rust], read through the following articles:
- [OpenAPI Initiative]
- [SwaggerHub Documentation: OpenAPI 3.0 Tutorial][122]
- [Official `utoipa` crate docs][`cynic`]
- [Official `okapi` crate docs][`okapi`]
- [Twilio Docs: Generate a Rust client for Twilio's API][121]
- [Fabian Odenthal: Auto-Generating & Validating OpenAPI Docs in Rust: A Streamlined Approach with Utoipa and Schemathesis][123]




## GraphQL

[GraphQL] is a [flexible][200] query language for [API]s, allowing to request data partially and compose multiple nested requests as a single one, seasoned with a schema having an [expressive][201] [type system][1] (comparing to other [API] schemas) and [very strong][202] [introspection][2] capabilities out-of-the-box.

One of the strongest parts of [GraphQL] is its [whole ecosystem][203] built around the language, allowing to auto-generate code from schema (or schema from code), have documentation directly from introspection, play interactively with any [API]s in playgrounds, easily mock them, and much, much more. __Once you've built your [GraphQL] schema, you have everything else ready-to-go.__

Another strong part of [GraphQL] is that its __protocol is [transport][204]-agnostic__, so the same schema and queries, used via [HTTP], are __easily reusable via [WebSocket]__, allowing to [stream data][205] with almost zero effort atop.

For more familiarity with [GraphQL], read through the following articles:
- [GraphQL docs: Introduction to GraphQL][206]
- [The Fullstack Tutorial for GraphQL][207]


### Server

For implementing a [GraphQL] server in [Rust], there are two major crates in its ecosystem: [`juniper`] (provides more static guarantees) and [`async-graphql`] (more feature-rich). Both __manifest code-to-schema approach__ (writing [Rust] code and later generating a [GraphQL] schema from it), because [Rust] type system is far more expressive than the [GraphQL] one.

[`juniper-from-schema`] crate, however, tries to take it in opposite direction, and to some degree successfully __provides schema-to-code approach__ (generating [Rust] code using [`juniper`] from a provided [GraphQL] schema).

For more familiarity with implementing [GraphQL] server in [Rust], read through the following articles:
- [Official `juniper` crate docs][`juniper`]
- [Juniper Book]
- [Official `juniper-from-schema` crate docs][`juniper-from-schema`]
- [Official `async-graphql` crate docs][`async-graphql`]
- [Async-graphql Book]


### Client

For making request to existing [GraphQL][GraphQL] [API]s, you don't necessarily need a special crate in [Rust] for trivial cases, just [any HTTP client][231] is capable to send a [simple query/mutation request][232].

However, if more static guarantees is needed, then the [`graphql-client`] crate may be used, providing the __query-to-code approach__ ([Rust] code is generated from [GraphQL] files defining queries).

[`cynic`] crate takes the __opposite code-to-query approach__ of generating a [GraphQL] query out of [Rust] code and validating it statically against a provided [GraphQL] schema.

For more familiarity with making [GraphQL] requests in [Rust], read through the following articles:
- [Official `graphql-client` crate description][`graphql-client`]
- [Official `cynic` crate docs][`cynic`]
- [Official `cynic` crate guide](https://cynic-rs.dev)




## gRPC

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




Rework [the task from the previous step](../4_2_http/README.md#task) in a ["thick client" paradigm][41]:
- Server represents a [REST]ful [API] with separate endpoints for each operation.
- [CLI] client parses commands by itself and makes accurate requests to the server [REST]ful [API].

It should be possible to perform all the operations via [cURL] (or any other [HTTP]/[API] client) directly on the [REST]ful [API] server, without using the [CLI] client.

Additionally, implement generation of [OpenAPI] schema out of you server [REST]ful [API] code, and generate [HTML] documentation from the generated [OpenAPI] schema.

Avoid architecture [over-engineering][42] for this task, just use simple, straightforward and obvious solutions.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
- What is API? What is RPC? How do they relate?
- What does "code-first" approach mean? What does "schema-first" approach mean? Which advantages and disadvantages do they have?
- What does REST paradigm mean? What are essentials of RESTful API? Which strengths does it have? What does it lack?  
- What is OpenAPI? What is Swagger? How do they relate? Why are they beneficial for RESTful API?
- What is GraphQL? Which are strong sides of this technology? What problems does it bring in practice? 
- What is gRPC? What are its strengths? Which are good use-cases for it, and which are not? Why? 




[`async-graphql`]: https://docs.rs/async-graphql
[`cynic`]: https://docs.rs/cynic
[`graphql-client`]: https://github.com/graphql-rust/graphql-client
[`grpcio`]: https://docs.rs/crate/grpcio
[`juniper`]: https://docs.rs/juniper
[`juniper-from-schema`]: https://docs.rs/juniper-from-schema
[`okapi`]: https://github.com/GREsau/okapi
[`openapi-generator`]: https://github.com/OpenAPITools/openapi-generator
[`swagger`]: https://docs.rs/swagger
[`tarpc`]: https://docs.rs/tarpc
[`tonic`]: https://docs.rs/tonic
[`tokio`]: https://docs.rs/tokio
[`utoipa`]: https://docs.rs/utoipa
[API]: https://en.wikipedia.org/wiki/API
[Async-graphql Book]: https://async-graphql.github.io/async-graphql/en
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[cURL]: https://en.wikipedia.org/wiki/CURL
[GraphQL]: https://graphql.org
[gRPC]: https://grpc.io
[HTML]: https://en.wikipedia.org/wiki/HTML
[HTTP]: https://en.wikipedia.org/wiki/HTTP
[HTTP/2]: https://en.wikipedia.org/wiki/HTTP/2
[IDL]: https://en.wikipedia.org/wiki/Interface_description_language
[Juniper Book]: https://graphql-rust.github.io/juniper/master
[OpenAPI]: https://en.wikipedia.org/wiki/OpenAPI_Specification
[OpenAPI Initiative]: https://learn.openapis.org
[REST]: https://en.wikipedia.org/wiki/Representational_state_transfer
[RPC]: https://en.wikipedia.org/wiki/Remote_procedure_call
[Rust]: https://www.rust-lang.org
[Swagger]: https://en.wikipedia.org/wiki/Swagger_(software)
[WebSocket]: https://en.wikipedia.org/wiki/WebSocket

[1]: https://en.wikipedia.org/wiki/Type_system
[2]: https://en.wikipedia.org/wiki/Type_introspection
[3]: https://en.wikipedia.org/wiki/Specification_(technical_standard)
[4]: https://en.wikipedia.org/wiki/Client%E2%80%93server_model
[101]: ../4_2_http/README.md#server
[102]: https://en.wikipedia.org/wiki/Machine-readable_medium_and_data
[111]: https://nordicapis.com/whats-the-difference-between-rpc-and-rest
[121]: https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api
[122]: https://support.smartbear.com/swaggerhub/docs/tutorials/openapi-3-tutorial.html
[123]: https://identeco.de/en/blog/generating_and_validating_openapi_docs_in_rust
[200]: https://graphql.org/learn/queries
[201]: https://graphql.org/learn/schema
[202]: https://graphql.org/learn/introspection
[203]: https://github.com/chentsulin/awesome-graphql#tools
[204]: https://en.wikipedia.org/wiki/Transport_layer
[205]: https://www.apollographql.com/docs/react/data/subscriptions
[206]: https://graphql.org/learn
[207]: https://www.howtographql.com
[231]: ../4_2_http/README.md#client
[232]: https://graphql.org/learn/serving-over-http
[301]: https://grpc.io/docs/what-is-grpc/introduction
[302]: https://grpc.io/docs/what-is-grpc/core-concepts
[311]: https://github.com/grpc/grpc
[41]: https://en.wikipedia.org/wiki/Rich_client
[42]: https://en.wikipedia.org/wiki/Overengineering
