


Rust language
* [async](https://doc.rust-lang.org/std/keyword.async.html)
* [await](https://doc.rust-lang.org/std/keyword.await.html)

(std lib core, language can assume its there, compiler generates an implementation of the trait)

Rust standard library
* [std::future::Future](https://doc.rust-lang.org/std/future/trait.Future.html) Trait
* [std::task](https://doc.rust-lang.org/std/task/index.html) - Context, Waker, etc.

Futures crate (mostly removed )
-- utilities for async Rust
-- re-export of Futures trait
-- Stream (will move into Rust std library)


Used by tokio - marked as optional?
* [futures-core](https://docs.rs/futures-core/0.3.1/futures_core/)
--> to include Stream, because later will be just Rust