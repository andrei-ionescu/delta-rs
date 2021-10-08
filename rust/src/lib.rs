//! Native Delta Lake implementation in Rust
//!
//! # Usage
//!
//! Load a Delta Table by path:
//!
//! ```rust
//! async {
//!   let table = deltalake::open_table("./tests/data/simple_table").await.unwrap();
//!   let files = table.get_files();
//! };
//! ```
//!
//! Load a specific version of Delta Table by path then filter files by partitions:
//!
//! ```rust
//! async {
//!   let table = deltalake::open_table_with_version("./tests/data/simple_table", 0).await.unwrap();
//!   let files = table.get_files_by_partitions(&[deltalake::PartitionFilter {
//!       key: "month",
//!       value: deltalake::PartitionValue::Equal("12"),
//!   }]);
//! };
//! ```
//!
//! Load a specific version of Delta Table by path and datetime:
//!
//! ```rust
//! async {
//!   let table = deltalake::open_table_with_ds(
//!       "./tests/data/simple_table",
//!       "2020-05-02T23:47:31-07:00",
//!   ).await.unwrap();
//!   let files = table.get_files();
//! };
//! ```
//!
//! # Optional cargo package features
//!
//! - `s3` - enable the S3 storage backend to work with Delta Tables in AWS S3.
//! - `glue` - enable the Glue data catalog to work with Delta Tables with AWS Glue.
//! - `azure` - enable the Azure storage backend to work with Delta Tables in Azure Data Lake Storage Gen2 accounts.
//! - `datafusion-ext` - enable the `datafusion::datasource::TableProvider` trait implementation for Delta Tables, allowing them to be queried using [DataFusion](https://github.com/apache/arrow-datafusion).

#![deny(warnings)]
#![deny(missing_docs)]

extern crate log;

#[cfg(all(feature = "arrow", feature = "arrow2"))]
compile_error!("Feature arrow and arrow2 are mutually exclusive and cannot be enabled together");

#[cfg(all(feature = "parquet", feature = "parquet2"))]
compile_error!(
    "Feature parquet and parquet2 are mutually exclusive and cannot be enabled together"
);

#[cfg(feature = "arrow")]
pub use arrow;
#[cfg(feature = "parquet")]
extern crate parquet;
#[cfg(feature = "arrow")]
pub mod delta_arrow;

#[cfg(all(feature = "arrow", feature = "parquet"))]
pub mod writer;

#[cfg(all(feature = "arrow", feature = "parquet"))]
pub mod checkpoints;

#[cfg(feature = "arrow2")]
pub use arrow2;
#[cfg(feature = "parquet2")]
pub use parquet2;

extern crate chrono;
extern crate lazy_static;
extern crate regex;
extern crate serde;
extern crate thiserror;

pub mod action;
pub mod data_catalog;
mod delta;
pub mod delta_config;
pub mod partitions;
mod schema;
pub mod storage;
mod table_state;

#[cfg(feature = "datafusion-ext")]
pub mod delta_datafusion;

#[cfg(feature = "rust-dataframe-ext")]
mod delta_dataframe;

pub use self::data_catalog::{get_data_catalog, DataCatalog, DataCatalogError};
pub use self::delta::*;
pub use self::partitions::*;
pub use self::schema::*;
pub use self::storage::{
    get_backend_for_uri, get_backend_for_uri_with_options, parse_uri, StorageBackend, StorageError,
    Uri, UriError,
};

#[cfg(feature = "s3")]
pub use self::storage::s3::{dynamodb_lock::dynamo_lock_options, s3_storage_options};
