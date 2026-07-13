Task 4.3: API servers, clients and tools
========================================

Naturally, in 📰 [client-server][4]⏱0.5h applications, a client and a server negotiate with each other via some 📰 [API (application programming interface)][API]⏱0.75h, which often takes form of 📰 [RPC (remote procedure call)][RPC]⏱0.25h for better structuring and standardizing (due 📰 [IDL (interface definition language)][IDL]⏱0.25h usage).

📚 [Rust]⏱0.25h ecosystem provides support for all modern widely-used and adopted 📰 [RPC]⏱0.25h technologies, and even comes with its 📚 [own unique ones][`tarpc`]⏱0.25h.




## RESTful

Since 📰 [REST]⏱0.25h is rather an __architecture convention/style__ than a strict 📰 [specification][3]⏱0.75h for 📰 [RPC]⏱0.25h, and 📰 [REST]⏱0.25hful 📰 [API]⏱0.75hs are typically __loosely based on 📰 [HTTP]⏱1.5h methods__ directly, there is usually __no need in special frameworks__ in 📚 [Rust]⏱0.25h to implement a 📰 [REST]⏱0.25hful 📰 [API]⏱0.75h server or to request the one. Just any [HTTP server][101] or [HTTP client][231] will do.

This approach, however, __suffers from lacking 📰 [API]⏱0.75h schema__, and so, makes it hard to build a rich ecosystem around with ready-to-use tooling (or connect with existing ones). Fortunately, this is easily solved by using a concrete 📰 [RPC specification][3]⏱0.75h on top of 📰 [REST]⏱0.25h conventions, and following it strictly. 

For more information about 📰 [REST]⏱0.25h, read through the following articles:
- 📰 [Tyler Charboneau: What’s the Difference Between RPC and REST?][111]⏱0.25h


### OpenAPI

📰 [OpenAPI]⏱0.25h (former 📰 [Swagger]⏱0.25h) is a 📰 [specification][3]⏱0.75h for a 📰 [machine-readable][102]⏱0.25h 📰 [IDL (interface definition language)][IDL]⏱0.25h, allowing to describe, produce, consume and visualize 📰 [REST]⏱0.25hful web 📰 [API]⏱0.75hs. In a nutshell, 📰 [OpenAPI]⏱0.25h is a __kind of 📰 [REST]⏱0.25h-based 📰 [RPC]⏱0.25h__.

> The OpenAPI Specification (OAS) defines a standard, language-agnostic interface to HTTP APIs which allows both humans and computers to discover and understand the capabilities of the service without access to source code, documentation, or through network traffic inspection. When properly defined, a consumer can understand and interact with the remote service with a minimal amount of implementation logic.
>
> An OpenAPI definition can then be used by documentation generation tools to display the API, code generation tools to generate servers and clients in various programming languages, testing tools, and many other use cases.

In 📚 [Rust]⏱0.25h ecosystem, most 📰 [OpenAPI]⏱0.25h crates follow the __code-first approach__ (generating 📰 [OpenAPI]⏱0.25h schema from source code). The most notable crates for this are 📚 [`utoipa`]⏱0.25h, 📰 [`okapi`]⏱0.25h and 📚 [`apistos`]⏱0.25h.

For the opposite (generating source code from 📰 [OpenAPI]⏱0.25h schema) 📚 [Rust]⏱0.25h ecosystem lacks its own pure implementation, and the original 📰 [OpenAPI]⏱0.25h tool 📰 [`openapi-generator`]⏱1.75h should be used (powered by the 📚 [`swagger`]⏱0.25h crate).

For more familiarity with 📰 [OpenAPI]⏱0.25h and using it in 📚 [Rust]⏱0.25h, read through the following articles:
- 📰 [OpenAPI Initiative]⏱0.25h
- 🧭 [SwaggerHub Documentation: OpenAPI 3.0 Tutorial][122]⏱0.5h
- 📚 [Official `utoipa` crate docs][`utoipa`]⏱0.25h
- 📰 [Official `okapi` crate docs][`okapi`]⏱0.25h
- 📚 [Official `apistos` crate docs][`apistos`]⏱0.25h
- 📰 [Twilio Docs: Generate a Rust client for Twilio's API][121]⏱0.25h
- 📰 [Fabian Odenthal: Auto-Generating & Validating OpenAPI Docs in Rust: A Streamlined Approach with Utoipa and Schemathesis][123]⏱0.25h
- 📰 [Olly Dixon: Auto-generating API service using Rust, to TypeScript & Dart][124]
- 📰 [Joshua Mo: Working with OpenAPI using Rust][125]⏱0.25h




## GraphQL

📰 [GraphQL]⏱0.25h is a 📰 [flexible][200]⏱0.5h query language for 📰 [API]⏱0.75hs, allowing to request data partially and compose multiple nested requests as a single one, seasoned with a schema having an 📰 [expressive][201]⏱0.5h 📰 [type system][1]⏱1.25h (comparing to other 📰 [API]⏱0.75h schemas) and 📰 [very strong][202]⏱0.25h 📰 [introspection][2]⏱0.25h capabilities out-of-the-box.

One of the strongest parts of 📰 [GraphQL]⏱0.25h is its 📋 [whole ecosystem][203]⏱0.5h built around the language, allowing to auto-generate code from schema (or schema from code), have documentation directly from introspection, play interactively with any 📰 [API]⏱0.75hs in playgrounds, easily mock them, and much, much more. __Once you've built your 📰 [GraphQL]⏱0.25h schema, you have everything else ready-to-go.__

Another strong part of 📰 [GraphQL]⏱0.25h is that its __protocol is 📰 [transport][204]⏱0.25h-agnostic__, so the same schema and queries, used via 📰 [HTTP]⏱1.5h, are __easily reusable via 📰 [WebSocket]⏱1h__, allowing to 📰 [stream data][205]⏱0.5h with almost zero effort atop.

For more familiarity with 📰 [GraphQL]⏱0.25h, read through the following articles:
- 📰 [GraphQL docs: Introduction to GraphQL][206]⏱0.25h
- 🧭 [The Fullstack Tutorial for GraphQL][207]⏱0.25h


### Server

For implementing a 📰 [GraphQL]⏱0.25h server in 📚 [Rust]⏱0.25h, there are two major crates in its ecosystem: 📚 [`juniper`]⏱0.25h (provides more static guarantees) and 📚 [`async-graphql`]⏱0.25h (more feature-rich). Both __manifest code-to-schema approach__ (writing 📚 [Rust]⏱0.25h code and later generating a 📰 [GraphQL]⏱0.25h schema from it), because 📚 [Rust]⏱0.25h type system is far more expressive than the 📰 [GraphQL]⏱0.25h one.

📚 [`juniper-from-schema`]⏱0.25h crate, however, tries to take it in opposite direction, and to some degree successfully __provides schema-to-code approach__ (generating 📚 [Rust]⏱0.25h code using 📚 [`juniper`]⏱0.25h from a provided 📰 [GraphQL]⏱0.25h schema).

For more familiarity with implementing 📰 [GraphQL]⏱0.25h server in 📚 [Rust]⏱0.25h, read through the following articles:
- 📚 [Official `juniper` crate docs][`juniper`]⏱0.25h
- 📰 [Juniper Book]⏱0.25h
- 📚 [Official `juniper-from-schema` crate docs][`juniper-from-schema`]⏱0.25h
- 📚 [Official `async-graphql` crate docs][`async-graphql`]⏱0.25h
- 📰 [Async-graphql Book]⏱0.25h


### Client

For making request to existing 📰 [GraphQL]⏱0.25h[GraphQL]⏱0.25h 📰 [API]⏱0.75hs, you don't necessarily need a special crate in 📚 [Rust]⏱0.25h for trivial cases, just [any HTTP client][231] is capable to send a 📰 [simple query/mutation request][232]⏱0.25h.

However, if more static guarantees is needed, then the 📰 [`graphql-client`]⏱0.25h crate may be used, providing the __query-to-code approach__ (📚 [Rust]⏱0.25h code is generated from 📰 [GraphQL]⏱0.25h files defining queries).

📚 [`cynic`]⏱0.25h crate takes the __opposite code-to-query approach__ of generating a 📰 [GraphQL]⏱0.25h query out of 📚 [Rust]⏱0.25h code and validating it statically against a provided 📰 [GraphQL]⏱0.25h schema.

For more familiarity with making 📰 [GraphQL]⏱0.25h requests in 📚 [Rust]⏱0.25h, read through the following articles:
- 📰 [Official `graphql-client` crate description][`graphql-client`]⏱0.25h
- 📚 [Official `cynic` crate docs][`cynic`]⏱0.25h
- [Official `cynic` crate guide](https://cynic-rs.dev)









## Task

__Estimated time__: 1 day




Rework [the task from the previous task](../4_2_http/README.md#task) in a 📰 ["thick client" paradigm][41]⏱0.25h:
- Server represents a 📰 [REST]⏱0.25hful 📰 [API]⏱0.75h with separate endpoints for each operation.
- 📰 [CLI]⏱1.5h client parses commands by itself and makes accurate requests to the server 📰 [REST]⏱0.25hful 📰 [API]⏱0.75h.

It should be possible to perform all the operations via 📰 [cURL]⏱0.25h (or any other 📰 [HTTP]⏱1.5h/📰 [API]⏱0.75h client) directly on the 📰 [REST]⏱0.25hful 📰 [API]⏱0.75h server, without using the 📰 [CLI]⏱1.5h client.

Additionally, implement generation of 📰 [OpenAPI]⏱0.25h schema out of you server 📰 [REST]⏱0.25hful 📰 [API]⏱0.75h code, and generate 📰 [HTML]⏱2h documentation from the generated 📰 [OpenAPI]⏱0.25h schema.

Avoid architecture 📰 [over-engineering][42]⏱0.25h for this task, just use simple, straightforward and obvious solutions.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
1. What is API? What is RPC? How do they relate?
2. What does "code-first" approach mean? What does "schema-first" approach mean? Which advantages and disadvantages do they have?
3. What does REST paradigm mean? What are essentials of RESTful API? Which strengths does it have? What does it lack?  
4. What is OpenAPI? What is Swagger? How do they relate? Why are they beneficial for RESTful API?
5. What is GraphQL? Which are strong sides of this technology? What problems does it bring in practice? 




[`apistos`]: https://docs.rs/apistos
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
[124]: https://www.polydelic.com/media/autogenerating-a-rust-api-to-typescript-and-dart
[125]: https://www.shuttle.rs/blog/2024/04/04/using-openapi-rust
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
