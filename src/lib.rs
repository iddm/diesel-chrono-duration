//! # diesel-chrono-duration
//!
//! This crate implements storage functionality for the `chrono::Duration` type.
//! It could be included into the `diesel` itself but its policy does not allow that.
//!
//! ## Usage
//!
//! Just use the [`ChronoDurationProxy`](https://docs.rs/diesel-chrono-duration) type instead of vanilla [`chrono::Duration`](https://docs.rs/chrono/0.4.6/chrono/struct.Duration.html).

#![warn(missing_docs)]

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::query_builder::bind_collector::RawBytesBindCollector;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::BigInt;
use diesel::{AsExpression, FromSqlRow};

/// A proxy type for which the diesel traits are implemented. Use this type whenever
/// you want to operate with `chrono::Duration`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromSqlRow, AsExpression)]
#[diesel(sql_type = BigInt)]
pub struct ChronoDurationProxy(pub chrono::Duration);

impl From<chrono::Duration> for ChronoDurationProxy {
    fn from(duration: chrono::Duration) -> ChronoDurationProxy {
        ChronoDurationProxy(duration)
    }
}

impl AsRef<chrono::Duration> for ChronoDurationProxy {
    fn as_ref(&self) -> &chrono::Duration {
        &self.0
    }
}

impl std::ops::Deref for ChronoDurationProxy {
    type Target = chrono::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<DB> ToSql<BigInt, DB> for ChronoDurationProxy
where
    i64: ToSql<BigInt, DB>,
    for<'c> DB: Backend + Backend<BindCollector<'c> = RawBytesBindCollector<DB>>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        if let Some(num_nanoseconds) = self.0.num_nanoseconds() {
            //<i64 as ToSql<BigInt, DB>>::to_sql(&num_nanoseconds, &mut out.reborrow())
            num_nanoseconds.to_sql(&mut out.reborrow())
        } else {
            Err(format!("{:?} as nanoseconds is too large to fit in an i64", self).into())
        }
    }
}

impl<DB> FromSql<BigInt, DB> for ChronoDurationProxy
where
    i64: FromSql<BigInt, DB>,
    DB: Backend,
{
    fn from_sql(value: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let i64_value = <i64 as FromSql<BigInt, DB>>::from_sql(value)?;
        Ok(chrono::Duration::nanoseconds(i64_value).into())
    }
}
