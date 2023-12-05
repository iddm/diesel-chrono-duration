# diesel-chrono-duration
[![](https://meritbadge.herokuapp.com/diesel-chrono-duration)](https://crates.io/crates/diesel-chrono-duration) [![](https://travis-ci.org/iddm/diesel-chrono-duration.svg?branch=master)](https://travis-ci.org/iddm/diesel-chrono-duration) ![CI](https://github.com/iddm/diesel-chrono-duration/workflows/CI/badge.svg) [![](https://docs.rs/diesel-chrono-duration/badge.svg)](https://docs.rs/diesel-chrono-duration)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


This crate adds support for the [`chrono::Duration`](https://docs.rs/chrono/0.4.0/chrono/struct.Duration.html) type into the [`diesel` ORM](http://diesel.rs/).

## Why

Diesel crate has a policy of including only such type implementations which can be represented as is in the SQL types. `chrono::Duration` does not have a direct 1-to-1 mapping in both the `SQLite` and `PostgreSQL`.

## How

The `chrono::Duration` type stores its value as `i64` number. This is exactly 8 bytes and such types are `BigInteger` and `BigInt`. So, the `ToSql` and `FromSql` traits implementation simply uses `chrono::Duration`'s inner
`i64` value.

## Usage

To implement this we added `ChronoDurationProxy` type which is just a strong type as defined as:

```rust
pub struct ChronoDurationProxy(pub chrono::Duration);
```

In your table model you use it instead of `chrono::Duration`:

```rust
extern crate diesel_chrono_duration;

use diesel_chrono_duration::ChronoDurationProxy;

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "sometable"]
pub struct SomeTable {
    /// The ID of the record in the table.
    pub id: i32,
    /// Some duration
    pub duration: ChronoDurationProxy,
}
```

Later, when you want to use it's value as `chrono::Duration` you have these options:

- `*duration` returns a reference to the inner `chrono::Duration` object. It is done by the `Deref` trait.
- `&duration` also returns a reference to the inner `chrono::Duration` object. It is done by the `AsRef` trait.
- `duration.0` as in usual rust.

## Contribute

The project is very simple and small but all contributions are "please make a contribution". Thanks in advance.

## License

This project is [licensed under the MIT license](https://github.com/iddm/diesel-chrono-duration/blob/master/LICENSE).
