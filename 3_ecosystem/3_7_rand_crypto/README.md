Task 3.7: Randomness and cryptography
=====================================

## Randomness

For random values generation [Rust] ecosystem has the [`rand`] crate, providing __unified interface__ and numerous random values __generator implementations with various statistical quality and performance guarantees__.

[The Rust Rand Book] not only explains how to use [`rand`] crate primitives, but also makes a good intro to the [basics of random values generation problem][1] and [how it's solved in a modern world][2]. Read through it to understand what primitives you should use for different situations:

- when performance is a goal;
- when cryptographical security and good statically quality is a goal;
- what is good for general purpose.

One of the most common cases when you need to deal with generating random values is a generation of universally unique identifiers (such as [UUID]). Fortunately, [Rust] has the [`uuid`] crate already, which implements [all versions of UUID specification][3].

More reading:

- [Aleksey Kladov: On Random Numbers][20]
- [Orhun Parmaksız: Zero-dependency random number generation in Rust][17]

## Encryption and signing

While at the moment [Rust] doesn't have The Cryptographic Library, its ecosystem contains a bunch of well implemented (and still maturing) crates for different purposes.

### [`ring`]

[`ring`] library implements a core set of cryptographic operations exposed via an easy-to-use (and hard-to-misuse) API. It started as a subset of famous [BoringSSL] library (_"ring"_ is a substring of "Bo_ring_SSL"), so inherits some its code and regularly merges changes from it.

[`ring`] is focused on a general-purpose cryptography. If you need just raw cryptography primitives - that is the way to go. Use it when you need to create:

- digital signature;
- simply encrypt plain data;
- key derivation;
- and so on...

If you need more high-level implementations (like WebPKI [X.509] certificate validation, or cryptographic protocols like [TLS], [SSH]) consider to use other crates (which are often built on top of [`ring`]).

### [dalek]

While [`ring`] is focused on providing general-purpose cryptography primitives, [dalek] crates provide only few, but are focused to implement the best theoretical primitives.

If you're going to build something that uses just some high-end cryptographic primitives (like using [Curve25519] for signing and verification) you should give [dalek] a try.

### [AWS] Libcrypto

[`aws-lc-rs`] is a [`ring`]-compatible crypto library using the cryptographic operations provided by [AWS-LC].

The motivation [provided by authors][18] is quite self-explanatory:
> [Rust] developers increasingly need to deploy applications that meet US and Canadian government cryptographic requirements. We evaluated how to deliver [FIPS] validated cryptography in idiomatic and performant [Rust], built around our [AWS-LC] offering. We found that the popular [`ring`] library fulfilled much of the cryptographic needs in the [Rust] community, but it did not meet the needs of developers with [FIPS] requirements. Our intention is to contribute a drop-in replacement for [`ring`] that provides [FIPS] support and is compatible with the [`ring`] API. [Rust] developers with prescribed cryptographic requirements can seamlessly integrate [`aws-lc-rs`] into their applications and deploy them into [AWS] Regions.

More reading:

- [Sean McGrai: Introducing AWS Libcrypto for Rust, an Open Source Cryptographic Library for Rust][19]

## Hashing

### Raw hash functions

The basic collection of raw [cryptographic hash functions][11] is introduced in [RustCrypto/hashes] crates collection.

__DO NOT use them for password hashing!__ Consider to use some password hashing algorithm instead ([Argon2], [bcrypt], [scrypt] or [PBKDF2]).

### Password hashing

There is the similar [RustCrypto/password-hashing] crates' collection for password hashing.

However, it lacks implementation for [Argon2] and [bcrypt] algorithms, so those [should be found][12] and chosen on your choice. For [Argon2] the [`rust-argon2`] crate seems to be the most mature one at the moment.

## Constant-time comparison

For [constant-time comparison][13] in [Rust] consider to use [`subtle`] crate from [dalek].

## TLS / SSL

For [TLS] usage [Rust] ecosystem currently has two common solutions:

### [`native-tls`]

[`native-tls`] crate is an abstraction over platform-specific [TLS] implementations. It uses [SChannel] on Windows (via [`schannel`] crate), Secure Transport on OSX (via [`security-framework`] crate), [OpenSSL] on all other platforms (via [`openssl`] crate), and provides a unified interface for using these libraries.

While this solution requires external non-[Rust] libraries to be present, it's a stable solution based on production-grade [TLS] implementations.

### [`rustls`]

[`rustls`] crate is a pure-[Rust] implementation of [TLS]. It's built on top of [`ring`] and [`webpki`] crates.

Despite the fact it's quite a feature rich solution, it [lacks good support for old and legacy cryptography][14] and has no stable version yet. Consider to use it when the legacy is non-concern for you.

## More reading

- [Sylvain Kerkour: Overview of the Rust cryptography ecosystem][15] (Tue, Aug 24, 2021)
- [Cheatsheat on cyphering, hashing adn security][16]

## Task

__Estimated time__: 1 day

Implement the following functions:

1. `generate_password()`: generates random password of given length and symbols set;
2. `select_rand_val()`: retrieves random element from a given slice;
3. `new_access_token()`: generates unique cryptographically secure random value in `a-zA-Z0-9` symbols set and has exactly `64` symbols.
4. `get_file_hash()`: returns SHA-3 hash of a file specified by its path.
5. `hash_password()`: returns [Argon2] password hash for a given password.

## Questions

After completing everything above, you should be able to answer (and understand why) the following questions:

- What is the main trade-off of generating random numbers? How is it applied in practice?
- What is symmetric cryptography? What is asymmetric cryptography? Which benefits does each one have?
- What is signing in asymmetric cryptography? What is encryption in asymmetric cryptography? How do they work given the same private and public keys?
- What is hash function? What is password hashing? Why is it not enough to use just a raw hash function for password hashing?
- What is constant-time comparison? When and why it should be used?
- Which are options of using [TLS] in [Rust]? Which advantages and disadvantages does each one have?

[`aws-lc-rs`]: https://docs.rs/aws-lc-rs
[`native-tls`]: https://docs.rs/native-tls
[`openssl`]: https://docs.rs/openssl
[`rand`]: https://docs.rs/rand
[`ring`]: https://docs.rs/ring
[`rust-argon2`]: https://docs.rs/rust-argon2
[`rustls`]: https://docs.rs/rustls
[`schannel`]: https://docs.rs/schannel
[`security-framework`]: https://docs.rs/security-framework
[`subtle`]: https://docs.rs/subtle
[`uuid`]: https://docs.rs/uuid
[`webpki`]: https://docs.rs/webpki
[Argon2]: https://en.wikipedia.org/wiki/Argon2
[AWS]: https://aws.amazon.com
[AWS-LC]: https://github.com/awslabs/aws-lc
[bcrypt]: https://en.wikipedia.org/wiki/Bcrypt
[BoringSSL]: https://github.com/google/boringssl
[Curve25519]: https://en.wikipedia.org/wiki/Curve25519
[dalek]: https://doc.dalek.rs
[FIPS]: https://en.wikipedia.org/wiki/Federal_Information_Processing_Standards
[OpenSSL]: https://en.wikipedia.org/wiki/OpenSSL
[PBKDF2]: https://en.wikipedia.org/wiki/PBKDF2
[Rust]: https://www.rust-lang.org
[RustCrypto/hashes]: https://github.com/RustCrypto/hashes
[RustCrypto/password-hashing]: https://github.com/RustCrypto/password-hashing
[SChannel]: https://en.wikipedia.org/wiki/Security_Support_Provider_Interface
[scrypt]: https://en.wikipedia.org/wiki/Scrypt
[SSH]: https://en.wikipedia.org/wiki/Secure_Shell
[The Rust Rand Book]: https://rust-random.github.io/book
[TLS]: https://en.wikipedia.org/wiki/Transport_Layer_Security
[UUID]: https://en.wikipedia.org/wiki/Universally_unique_identifier
[X.509]: https://en.wikipedia.org/wiki/X.509

[1]: https://rust-random.github.io/book/guide-data.html
[2]: https://rust-random.github.io/book/guide-gen.html
[3]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Versions
[11]: https://en.wikipedia.org/wiki/Cryptographic_hash_function
[12]: https://crates.io/search?q=argon2
[13]: https://codahale.com/a-lesson-in-timing-attacks
[14]: https://docs.rs/rustls/#non-features
[15]: https://kerkour.com/blog/rust-cryptography-ecosystem
[16]: https://cheatsheetseries.owasp.org/index.html
[17]: https://blog.orhun.dev/zero-deps-random-in-rust
[18]: https://github.com/awslabs/aws-lc-rs#motivation
[19]: https://aws.amazon.com/blogs/opensource/introducing-aws-libcrypto-for-rust-an-open-source-cryptographic-library-for-rust
[20]: https://matklad.github.io/2023/01/04/on-random-numbers.html
