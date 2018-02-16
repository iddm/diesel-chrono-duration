//! # diesel-chrono-duration
//!
//! This crate implements storage functionality for the `chrono::Duration` type.
//! It could be included into the `diesel` itself but its policy does not allow that.
//!
//! ## Installation
//!
//! Add the following dependency to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! diesel-chrono-duration = "0.1"
//! ```
//!
//! And add this to your root file:
//!
//! ```no_run
//! extern crate diesel_chrono_duration;
//! ```

extern crate chrono;
#[macro_use]
extern crate diesel;

use std::io::Write;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::BigInt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromSqlRow, AsExpression)]
#[sql_type = "BigInt"]
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
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        if let Some(num_nanoseconds) = self.0.num_nanoseconds() {
            ToSql::<BigInt, DB>::to_sql(&num_nanoseconds, out)
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
    fn from_sql(value: Option<&<DB as Backend>::RawValue>) -> deserialize::Result<Self> {
        let i64_value = <i64 as FromSql<BigInt, DB>>::from_sql(value)?;
        Ok(chrono::Duration::nanoseconds(i64_value).into())
    }
}
