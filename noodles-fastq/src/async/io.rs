//! Async FASTQ I/O.

mod reader;
mod writer;

pub use self::{reader::Reader, writer::Writer};
