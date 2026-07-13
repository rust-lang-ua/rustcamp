Task 4.1: Databases, connection pools and ORMs
==============================================

The current situation with databases integration in 📚 [Rust]⏱0.25h ecosystem is illustrated quite well in 📋 [this "Awesome Rust" section][1]⏱2.25h and in 📰 ["Database" topic of "Are we web yet?"][2]⏱0.25h: the majority of the drivers are implemented fully in 📚 [Rust]⏱0.25h, and only few wrap existing libraries, and of course, most of them use [async I/O][3].




## Connection pool

The important concept to understand is a 📰 [connection pool][11]⏱0.25h pattern. It's widely adopted in situations where a program represents a long-running application (like 📰 [daemons][12]⏱0.25h or 📰 [servers][13]⏱0.5h). The key point is that __instead of creating a new connection to database every time__ we need to interact with, we'd __rather pre-create a 📰 [pool][14]⏱0.5h of such connections and reuse them__. As connection creation is quite an expensive operation, applying this pattern leads to huge performance improvements.

Fortunately, 📚 [Rust]⏱0.25h ecosystem provides generic implementations of database-agnostic 📋 [connection pool][1]⏱2.25h in both flavours: synchronous and asynchronous.

For better understanding 📋 [connection pooling][1]⏱2.25h, read through the following articles:
- 📰 [Charlie Custer: What is Connection Pooling, and Why Should You Care][15]⏱0.25h


### Synchronous

For synchronous connections there is the 📚 [`r2d2`]⏱0.25h crate (the pioneer among such crates, existed far before [async I/O][3] has landed in 📚 [Rust]⏱0.25h). You can easily adopt it for your specific use-case (or database) just by implementing 📚 [its traits][22]⏱0.25h. Obviously, there are 📚 [implementations for common drivers][21]⏱0.25h already.

For more details, read through the following articles:
- 📚 [Official `r2d2` crate docs][`r2d2`]⏱0.25h


### Asynchronous

For asynchronous connections there are much more options in 📚 [Rust]⏱0.25h ecosystem, due to historical reasons and bigger competitiveness (as the result of bigger [async I/O][3] popularity).

The very first one, historically, was the 📚 [`bb8`]⏱0.25h crate. It mirrors the 📚 [`r2d2`]⏱0.25h crate for asynchronous connections (📚 [`tokio`]⏱0.25h only), and originally was based on it. Similarly, there are 📚 [implemented bridges for common drivers][23]⏱0.25h already.

📚 [`deadpool`]⏱0.25h is an 📚 [alternative and very mature][25]⏱0.25h implementation of the 📰 [connection pool][11]⏱0.25h pattern, supporting both 📚 [`tokio`]⏱0.25h and 📚 [`async-std`]⏱0.25h, provided with 📚 [its own large ecosystem][24]⏱0.25h.

Another alternative implementation is the 📚 [`mobc`]⏱0.25h crate, yet inspired by 📚 [`deadpool`]⏱0.25h and 📚 [`r2d2`]⏱0.25h crates. Similarly, supports both 📚 [`tokio`]⏱0.25h and 📚 [`async-std`]⏱0.25h and provides some 📚 [bridges for common drivers][26]⏱0.25h.

📰 [`qp`]⏱0.25h (Quick Pool) is a very simple and 📰 [limited][29]⏱0.25h implementation of the 📰 [connection pool][11]⏱0.25h pattern, 📰 [utilizing lock-free primitives][27]⏱0.25h and 📰 [focused on being performant][28]⏱0.25h.

For more details, read through the following articles:
- 📚 [Official `bb8` crate docs][`bb8`]⏱0.25h
- 📚 [Official `deadpool` crate docs][`deadpool`]⏱0.25h
- 📚 [Official `mobc` crate docs][`mobc`]⏱0.25h
- 📰 [Official `qp` crate docs][`qp`]⏱0.25h




## Query builder

Query builder is effectively a __📰 [builder pattern][81]⏱0.25h applied for building 📰 [SQL]⏱1h__ (or other 📰 [data query languages][82]⏱0.5h) queries, and __allowing to write them as a regular 📚 [Rust]⏱0.25h code__ (and so, 📰 [using an embedded DSL instead of external DSL][83]⏱0.75h).

The canonical implementation of this pattern in 📚 [Rust]⏱0.25h ecosystem is represented by 📚 [`sea-query`]⏱0.25h and 📚 [`sql_query_builder`]⏱0.25h crates.

📚 [`barrel`]⏱0.25h crate, on the other hand, allows to write 📰 [schema migrations][61]⏱0.25h, rather than querying data.

For more details, read through the following articles:
- 📚 [Official `sea-query` crate docs][`sea-query`]⏱0.25h
- 📚 [Official `sql_query_builder` crate docs][`sql_query_builder`]⏱0.25h
- 📚 [Official `barrel` crate docs][`barrel`]⏱0.25h


### Non-📰 [DSL]⏱0.75h toolkit

📚 [`sqlx`]⏱0.25h crate, while being a feature-rich toolkit for 📰 [SQL]⏱1h, takes a 📰 [completely opposite approach][91]⏱1h here: it focuses on writing pure 📰 [SQL]⏱1h queries (no custom 📰 [DSL]⏱0.75h, no [query building](#query-builder)), which are statically checked to be correct at compile-time.

For better understanding 📚 [`sqlx`]⏱0.25h design, concepts, usage, and features, read through the following articles:
- 📚 [Official `sqlx` crate docs][`sqlx`]⏱0.25h




## ORM

Regarding the 📰 [ORM pattern][41]⏱0.25h, there are 📰 [multiple][42]⏱0.25h feature-rich and mature implementation in 📚 [Rust]⏱0.25h ecosystem at the moment. Every one has its own unique design, advantages and disadvantages.

The very first 📰 [ORM][41]⏱0.25h created in 📚 [Rust]⏱0.25h was the 📚 [`diesel`]⏱0.25h crate. Even now, it supports 📰 [only synchronous][43]⏱0.25h connections (as was created before [async I/O][3] has landed in 📚 [Rust]⏱0.25h). However, still may be used with asynchronous connections, thankfully to the 📚 [`diesel-async`]⏱0.25h extension.

📚 [`sea-orm`]⏱0.25h (built on top of 📚 [`sea-query`]⏱0.25h) is an alternative feature-rich and 📚 [mature][46]⏱0.25h implementation of the [ORM] pattern in 📚 [Rust]⏱0.25h, focused on 📰 [dynamic querying to avoid complexity of static checks ("fighting the ORM")][47]⏱0.25h.

📚 [`ormx`]⏱0.25h is a lightweight extension of the 📚 [`sqlx`]⏱0.25h crate, aimed to provide it with 📰 [ORM][41]⏱0.25h-like features.

📚 [`rustorm`]⏱0.25h is a very simple and 📰 [SQL]⏱1h-centered 📰 [ORM][41]⏱0.25h, focused on easing conversions of database types to their appropriate 📚 [Rust]⏱0.25h types.

For better understanding 📰 [ORMs][41]⏱0.25h design, concepts, usage, and features, read through the following articles:
- 📚 [Official `diesel` crate docs][`diesel`]⏱0.25h
- 🧭 [Official `diesel` crate guides][44]⏱0.25h
- 📚 [Official `sea-orm` crate docs][`sea-orm`]⏱0.25h
- 📰 [Official `sea-orm` crate guides][45]⏱0.25h
- 📚 [Official `ormx` crate docs][`ormx`]⏱0.25h
- 📚 [Official `rustorm` crate docs][`rustorm`]⏱0.25h




## Migrations

For 📰 [database migrations][61]⏱0.25h there are 📰 [multiple tools][62]⏱0.25h in 📚 [Rust]⏱0.25h ecosystem.

For 📚 [`diesel`]⏱0.25h users, the obvious choice is the 📚 [`diesel_migrations`]⏱0.25h crate (which may be used directly via 📚 [`diesel_cli`]⏱0.25h). Though, doesn't require the 📚 [`diesel`]⏱0.25h itself to be used, and may be used as a fully separate tool.

For 📚 [`sqlx`]⏱0.25h users, similarly, the 📚 [`sqlx-cli`]⏱0.25h tool 📰 [provides migrations][64]⏱0.25h out-of-the-box, while also may be used 📚 [directly in the application code][65]⏱0.25h.

📚 [`refinery`]⏱0.25h and 📚 [`migrant`]⏱0.25h are another standalone 📚 [Rust]⏱0.25h tools for 📰 [migrations][61]⏱0.25h, allowing both 📰 [CLI]⏱1.5h and 📚 ["in-application-code"][66]⏱0.25h usage. The interesting part about the 📚 [`refinery`]⏱0.25h crate is that it also allows to write "in-application-code" 📰 [migrations][61]⏱0.25h via the 📚 [`barrel`]⏱0.25h schema migration builder.

For being familiar with 📰 [migrations][61]⏱0.25h tools, their similarities and differences, read through the following articles:
- 📚 [Official `diesel_migrations` crate docs][`diesel_migrations`]⏱0.25h
- 📚 [Official `diesel_cli` crate docs][`diesel_cli`]⏱0.25h
- 🧭 [Official `diesel` crate guides: Getting Started][63]⏱0.75h
- 📚 [Official `sqlx` crate docs: Macro `sqlx::migrate`][65]⏱0.25h
- 📚 [Official `refinery` crate docs][`refinery`]⏱0.25h
- 📚 [Official `migrant` crate docs][`migrant`]⏱0.25h




## Task

__Estimated time__: 1 day




Create an 📰 [SQL]⏱1h database (📰 [PostgreSQL]⏱0.25h, 📰 [MySQL]⏱0.25h or 📰 [SQLite]⏱0.25h, on your choice) consisting of the following tables:
- `users`: `id`, `name` and any other fields on your choice; 
- `roles`: 📰 [`slug`][201]⏱0.25h as a primary key, `name` and `permissions` (the concrete format on your choice) fields;
- `users_roles`: `users.id` to `roles.slug` many-to-many relationship.

Write a simple 📰 [CLI]⏱1.5h application which allows to 📰 [CRUD]⏱0.25h data in your database tables in the following ways:
- create and delete `users` and `roles` (a `user` must always have an assigned `role`);
- update fields of a single `user` or a `role`;
- assign or unassign a `role` to/from a `user`;
- list all `roles` or a single `role` by its `slug`;
- list all `users` or a single `user` by its `id` (a `user` should be displayed with all the `roles` assigned to him).

Consider to ensure 📰 [data consistency][202]⏱0.25h in your database as much as possible.




## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:
1. What is connection pool pattern? How does it work? Which problems does it solve?
2. What is ORM pattern? How does it differ from query building? What benefits do they give?
3. Why writing raw SQL queries could be meaningful? Which are use-cases for it and when is it preferred over ORMs?
4. What are migrations? Why should we use them? How do they work? 
5. Which kinds of migrations do exist? What are their advantages and disadvantages? When and which kind is preferred?  




[`async-std`]: https://docs.rs/async-std
[`barrel`]: https://docs.rs/barrel
[`bb8`]: https://docs.rs/bb8
[`deadpool`]: https://docs.rs/deadpool
[`diesel`]: https://docs.rs/diesel
[`diesel_cli`]: https://docs.rs/diesel_cli
[`diesel_migrations`]: https://docs.rs/diesel_migrations
[`diesel-async`]: https://docs.rs/diesel-async
[`migrant`]: https://docs.rs/migrant
[`mobc`]: https://docs.rs/mobc
[`ormx`]: https://docs.rs/ormx
[`qp`]: https://github.com/Astro36/qp
[`r2d2`]: https://docs.rs/r2d2
[`refinery`]: https://docs.rs/refinery
[`rustorm`]: https://docs.rs/crate/rustorm
[`sea-orm`]: https://docs.rs/sea-orm
[`sea-query`]: https://docs.rs/sea-query
[`sql_query_builder`]: https://docs.rs/sql_query_builder
[`sqlx`]: https://docs.rs/crate/sqlx
[`sqlx-cli`]: https://docs.rs/crate/sqlx-cli
[`tokio`]: https://docs.rs/tokio
[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[CRUD]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
[DSL]: https://en.wikipedia.org/wiki/Domain-specific_language
[MySQL]: https://www.mysql.com
[PostgreSQL]: https://www.postgresql.org
[Rust]: https://www.rust-lang.org
[SQL]: https://en.wikipedia.org/wiki/SQL
[SQLite]: https://www.sqlite.org

[1]: https://github.com/rust-unofficial/awesome-rust#database-1
[2]: https://www.arewewebyet.org/topics/database
[3]: ../../3_ecosystem/3_11_async
[11]: https://en.wikipedia.org/wiki/Connection_pool
[12]: https://en.wikipedia.org/wiki/Daemon_(computing)
[13]: https://en.wikipedia.org/wiki/Server_(computing)
[14]: https://en.wikipedia.org/wiki/Object_pool_pattern
[15]: https://www.cockroachlabs.com/blog/what-is-connection-pooling
[21]: https://crates.io/search?q=r2d2
[22]: https://docs.rs/r2d2#traits
[23]: https://crates.io/search?q=bb8
[24]: https://crates.io/search?q=deadpool
[25]: https://docs.rs/deadpool#reasons-for-yet-another-connection-pool
[26]: https://crates.io/search?q=mobc
[27]: https://github.com/Astro36/qp#bb8-vs-qp
[28]: https://github.com/Astro36/qp#performance-comparison
[29]: https://github.com/Astro36/qp#dbcp
[41]: https://en.wikipedia.org/wiki/Object-relational_mapping
[42]: https://www.arewewebyet.org/topics/database#orms
[43]: https://github.com/diesel-rs/diesel/issues/399
[44]: https://diesel.rs/guides
[45]: https://www.sea-ql.org/SeaORM/docs/index
[46]: https://docs.rs/sea-orm#whos-using-seaorm
[47]: https://www.sea-ql.org/SeaORM/docs/internal-design/diesel#programming-paradigm
[61]: https://en.wikipedia.org/wiki/Schema_migration
[62]: https://www.arewewebyet.org/topics/database#tooling
[63]: https://diesel.rs/guides/getting-started
[64]: https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#create-and-run-migrations
[65]: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
[66]: https://docs.rs/refinery/latest/refinery/macro.embed_migrations.html
[81]: https://en.wikipedia.org/wiki/Builder_pattern
[82]: https://en.wikipedia.org/wiki/Query_language
[83]: https://en.wikipedia.org/wiki/Domain-specific_language#External_and_Embedded_Domain_Specific_Languages
[91]: https://github.com/launchbadge/sqlx#sqlx-is-not-an-orm
[201]: https://en.wikipedia.org/wiki/Clean_URL#Slug 
[202]: https://en.wikipedia.org/wiki/Consistency_(database_systems)
