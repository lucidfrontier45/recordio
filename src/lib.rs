#![allow(clippy::non_ascii_literal)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

pub(crate) mod backend;
pub(crate) mod common;
pub(crate) mod read;
pub(crate) mod write;

pub use common::DataFormat;
pub use read::{
    read_record_from_file, read_record_from_reader, read_records_from_file,
    read_records_from_reader,
};
pub use write::{
    write_record_to_file, write_record_to_writer, write_records_to_file, write_records_to_writer,
};
