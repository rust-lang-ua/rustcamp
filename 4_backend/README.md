Chapter 4: Backend ecosystem
=========================

These tasks describe common crates and tools in [Rust] ecosystem required for web backend development.

> ❗️Before completing this task you should complete all its sub-tasks.

After doing them you should be able to answer the following questions:

- How should I interact with databases in [Rust] application and why? How can I organize migrations for my project?
- What should I use for [HTTP] server implementation in [Rust], when and why? What about [WebSocket] connections?
- What are options for making [HTTP] request (including [WebSocket] ones)?
- What is [RPC]? Name several the most adopted technologies, their advantages and disadvantages, explain which one could be used under which circumstances, and what and where is their best fit?

## Task

__Estimated time__: 3 days

Write a simple [GraphQL] API server with the following data model:

- `User` has `id` (unique), `name` (unique) and `friends` (list of other `User`s) fields.
- `User` is able to authenticate with its `password`.

API requirements:

- Ability to register users.
- Ability to authenticate users.
- Ability to retrieve a single user and all its friends (with their friends) (should require authorization).
- Ability to add some user to friends list and remove from there (should require authorization).

Web frameworks, tools and database choices are up to you. Keep things simple to fit into the dedicated time.

If you have enough time after implementing base requirements, consider to add the following to your solution:

- Provide migrations for database schema (if possible).
- Add comprehensive documentation to your code and [API], and generate it in [HTML] form.
- Cover your implementation with unit and E2E tests.
- Implement [GraphQL] query [depth limiting][21].
- Use [dataloading][22] to optimize interaction with database in [GraphQL] resolvers.

[API]: https://en.wikipedia.org/wiki/API
[GraphQL]: https://graphql.org/learn
[HTML]: https://en.wikipedia.org/wiki/HTML
[HTTP]: https://en.wikipedia.org/wiki/HTTP
[RPC]: https://en.wikipedia.org/wiki/Remote_procedure_call
[Rust]: https://www.rust-lang.org
[WebSocket]: https://en.wikipedia.org/wiki/WebSocket

[21]: https://escape.tech/blog/cyclic-queries-and-depth-limit
[22]: https://medium.com/the-marcy-lab-school/how-to-use-dataloader-js-9727c527efd0
