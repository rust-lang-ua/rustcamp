Task 4.2: HTTP servers and clients
==================================

The current situation regarding 📰 [HTTP]⏱1.5h in 🏠 [Rust] ecosystem can be grasped quite well in 📋 [the "Web programming" section of "Awesome Rust"][1] and in the 📰 ["Web Frameworks"][2], 📰 ["HTTP Clients"][3] and 📰 ["Lower Web-Stack" topics of "Are we web yet?"][4]. Of course, most of them use [async I/O][5].




## Low-level

There are few core crates, providing general-purpose 📰 [HTTP]⏱1.5h implementation, powering the whole variety of 📰 [web frameworks][21]⏱0.5h and 📰 [HTTP]⏱1.5h clients in 🏠 [Rust] ecosystem.

The most prominent and mature one is, of course, the 📚 [`hyper`] crate (built using 📚 [`tokio`]). Almost all 📰 [web frameworks][21]⏱0.5h of 🏠 [Rust] ecosystem are built on top of it.

The main alternatives are:
- 📚 [`async-h1`], powering the 📚 [`async-std`] ecosystem for 📰 [HTTP]⏱1.5h.
- 📚 [`actix-http`], powering the 📚 [`actix-web`] ecosystem.

For more details, read through the following articles:
- 📚 [Official `hyper` crate docs][`hyper`]
- 📚 [Official `async-h1` crate docs][`async-h1`]
- 📚 [Official `actix-http` crate docs][`actix-http`]




## Server

While 📚 [`hyper`] provides its own server implementation, using it directly can feel quite low-level and unergonomic, due to its nature. Naturally, there are 📰 [numerous web frameworks][2] built on top of 📚 [`hyper`], which provide high-level, ergonomic and friendly interface. The most notable are:
- 📚 [`axum`] - a 📰 [web application framework][21]⏱0.5h that focuses on ergonomics and modularity, and provides macro-free request routing (yet ergonomic and declarative), simple and predictive error-handling, and leverages full advantage of the 📚 [`tower`] and 📚 [`tower-http`] ecosystem of 📰 [middleware][22]⏱0.25h, services, and utilities.
- 📚 [`warp`] - a super-easy, composable, 📰 [web server framework][21]⏱0.5h for warp speeds, built around the "everything is a 📚 [`Filter`]" concept.
- 📚 [`rocket`] - a 📰 [web framework][21]⏱0.5h, aims to be fast, easy, and flexible while offering guaranteed safety and security where it can, and, importantly, aiming to be fun (accomplishing this by ensuring that you write as little code as needed to accomplish your task).
- 📚 [`poem`] - a full-featured and easy-to-use 📰 [web framework][21]⏱0.5h, focusing on providing all the capabilities (like 📰 [i18n]⏱0.5h) out-of-the-box.
- 📚 [`salvo`] - a powerful and simple 📰 [web server framework][21]⏱0.5h, adopting 📰 [HTTP/3]⏱0.5h implementation.

For those who prefer 📚 [`async-std`] ecosystem, the definitive choice (and the single one, at the moment) is the 📚 [`tide`] crate.

All the 📰 [web frameworks][21]⏱0.5h above inherit the 📰 [work-stealing][23]⏱0.25h from the asynchronous runtime they're run on, and so, require the proper synchronization (being 📚 [`Send`]) from user-provided 📰 [HTTP]⏱1.5h request handlers, which may introduce an unnecessary or undesired overhead. That's why __📚 [`actix-web`] crate was designed__ and implemented specifically with this consideration in mind (__to avoid 📰 [work-stealing][23]⏱0.25h__), being built on top of 📚 [`actix-rt`] crate (leveraging 📰 [thread-per-core][24]⏱0.5h model), and thus, not requiring any synchronization in its request handlers (allowing `!Send` 📚 [`Future`]s). Also, 📚 [`actix-web`], at the time, was the first mature and production-ready 📰 [web framework][21]⏱0.5h in 🏠 [Rust] ecosystem, possessing a 📰 [top of "TechEmpower Web Framework Benchmarks"][25].

For better understanding and familiarity with 📰 [HTTP]⏱1.5h servers in 🏠 [Rust], read through the following articles:
- 📚 [Official `actix-web` crate docs][`actix-web`]
- [Official `actix-web` crate guides: Server](https://actix.rs/docs/server)
- 📚 [Official `axum` crate docs][`axum`]
- 📚 [Official `warp` crate docs][`warp`]
- 📚 [Official `rocket` crate docs][`rocket`]
- 📚 [Official `poem` crate docs][`poem`]
- [Official `salvo` book](https://salvo.rs/book)
- 📚 [Official `tide` crate docs][`tide`]
- 🧭 [Official `hyper` crate guides: Server][26]⏱0.25h




## Client

Similarly to a server, while 📚 [`hyper`] provides its own client implementation, using it directly can feel quite low-level and unergonomic. So, the "default choice" 📰 [HTTP]⏱1.5h client (and mostly used) in 🏠 [Rust] ecosystem is the 📚 [`reqwest`] crate, built on top of 📚 [`hyper`].

📚 [`isahc`] crate, as an alternative, is a runtime-agnostic wrapper (with major focus on being practical and ergonomic) around the famous 📰 [cURL] library.

For simple and trivial scenarios, __where an asynchronous runtime is redundant__ and/or low overhead is preferred, the viable alternative is the 📚 [`ureq`] crate.

For 📚 [`async-std`] ecosystem, the main crate is 📚 [`surf`], which is, however, not restricted to 📚 [`async-std`] only, and is able to use alternative backends: 📰 [cURL] (via 📚 [`isahc`]), 📚 [`hyper`], 📰 [WASM]⏱1.25h (via 📰 [browser's `window.fetch` API][32]).

For 📚 [`actix-web`] ecosystem, the meaningful option would be the 📚 [`awc`] crate, which supports 📰 [WebSocket]⏱1h connections out-of-the-box (while most other 📰 [HTTP]⏱1.5h clients lacks that).

For better understanding and familiarity with 📰 [HTTP]⏱1.5h clients in 🏠 [Rust], read through the following articles:
- 📚 [Official `reqwest` crate docs][`reqwest`]
- 📰 [Joshua Mo: Writing a Web Scraper in Rust using Reqwest][33]⏱0.25h
- 📚 [Official `isahc` crate docs][`isahc`]
- 📚 [Official `ureq` crate docs][`ureq`]
- 📚 [Official `surf` crate docs][`surf`]
- 📚 [Official `awc` crate docs][`awc`]
- 🧭 [Official `hyper` crate guides: Client][31]⏱0.25h




## WebSocket

Many 📰 [HTTP]⏱1.5h clients and servers in 🏠 [Rust] lack a built-in 📰 [WebSocket]⏱1h implementation. Therefore, the 📚 [`tungstenite`] crate was created, providing a barebone and agnostic 📰 [WebSocket]⏱1h implementation. Crates, like 📚 [`async-tungstenite`] and 📚 [`tokio-tungstenite`], provide the actual ready-for-use client/server implementation for the desired ecosystem and asynchronous runtime.

For 📚 [`actix-web`] ecosystem, the idiomatic solution is the 📚 [`actix-web-actors::ws`] module, providing implementation in a form of 📰 [actor][41]⏱1.25h (via 📚 [`actix`]).

For better understanding and familiarity with 📰 [WebSocket]⏱1h implementations in 🏠 [Rust], read through the following articles:
- 📚 [Official `tungstenite` crate docs][`tungstenite`]
- 📚 [Official `async-tungstenite` crate docs][`async-tungstenite`]
- 📚 [Official `tokio-tungstenite` crate docs][`tokio-tungstenite`]
- 📚 [Official `actix-web-actors::ws` module docs][`actix-web-actors::ws`]




## Task

__Estimated time__: 1 day




Rework [the task from the previous task](../4_1_db/README.md#task) in a 📰 [client-server architecture][51]. It should consist of a 📰 [CLI] client and a server 📰 [daemon][52]⏱0.25h, and utilize the 📰 ["thin client" approach][53]⏱0.5h:
- 📰 [CLI] client does nothing except sending commands "as is" to the server and rendering its responses.
- Server 📰 [daemon][52]⏱0.25h, having a single 📰 [HTTP]⏱1.5h endpoint, does all the parsing and executing of commands sent by the 📰 [CLI] client.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
1. What is HTTP? What does HTTP/2 imply? What does HTTP/3 imply?
2. How do work-stealing and thread-per-core paradigms affect programming a web server in practice? Which one is better and when? When does this question (choosing) become meaningful, in practice?
3. What are common crates for making HTTP requests in 🏠 [Rust]? Which trade-offs do they have?
4. What is WebSocket? How is it used and when? How does it work, in a nutshell?




[`actix`]: https://docs.rs/actix
[`actix-http`]: https://docs.rs/actix-http
[`actix-rt`]: https://docs.rs/actix-rt
[`actix-web`]: https://docs.rs/actix-web
[`actix-web-actors::ws`]: https://docs.rs/actix-web-actors/latest/actix_web_actors/ws/index.html
[`async-h1`]: https://docs.rs/async-h1
[`async-std`]: https://docs.rs/async-std
[`async-tungstenite`]: https://docs.rs/crate/async-tungstenite
[`awc`]: https://docs.rs/awc
[`axum`]: https://docs.rs/axum
[`Filter`]: https://docs.rs/warp/latest/warp/trait.Filter.html
[`Future`]: https://doc.rust-lang.org/stable/std/future/trait.Future.html
[`hyper`]: https://docs.rs/hyper
[`isahc`]: https://docs.rs/isahc
[`poem`]: https://docs.rs/poem
[`reqwest`]: https://docs.rs/reqwest
[`rocket`]: https://docs.rs/rocket
[`salvo`]: https://docs.rs/salvo
[`surf`]: https://docs.rs/surf
[`tower`]: https://docs.rs/tower
[`tower-http`]: https://docs.rs/tower-http
[`tungstenite`]: https://docs.rs/crate/tungstenite
[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`tide`]: https://docs.rs/tide
[`tokio`]: https://docs.rs/tokio
[`tokio-tungstenite`]: https://docs.rs/crate/tokio-tungstenite
[`ureq`]: https://docs.rs/ureq
[`warp`]: https://docs.rs/warp
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[cURL]: https://en.wikipedia.org/wiki/CURL
[HTTP]: https://en.wikipedia.org/wiki/HTTP
[HTTP/3]: https://en.wikipedia.org/wiki/HTTP/3
[i18n]: https://en.wikipedia.org/wiki/Internationalization_and_localization
[Rust]: https://www.rust-lang.org
[WASM]: https://en.wikipedia.org/wiki/WebAssembly
[WebSocket]: https://en.wikipedia.org/wiki/WebSocket

[1]: https://github.com/rust-unofficial/awesome-rust#web-programming
[2]: https://www.arewewebyet.org/topics/frameworks
[3]: https://www.arewewebyet.org/topics/http-clients
[4]: https://www.arewewebyet.org/topics/lower-web-stack
[5]: ../../3_ecosystem/3_11_async
[21]: https://en.wikipedia.org/wiki/Web_framework
[22]: https://en.wikipedia.org/wiki/Middleware
[23]: https://en.wikipedia.org/wiki/Work_stealing
[24]: https://www.datadoghq.com/blog/engineering/introducing-glommio
[25]: https://www.techempower.com/benchmarks#hw=ph&test=plaintext&section=data-r18
[26]: https://hyper.rs/guides/server/hello-world
[31]: https://hyper.rs/guides/client/basic
[32]: https://developer.mozilla.org/docs/Web/API/Fetch_API
[33]: https://www.shuttle.rs/blog/2023/09/13/web-scraping-rust-reqwest
[41]: https://en.wikipedia.org/wiki/Actor_model
[51]: https://en.wikipedia.org/wiki/Client%E2%80%93server_model
[52]: https://en.wikipedia.org/wiki/Daemon_(computing)
[53]: https://en.wikipedia.org/wiki/Thin_client
