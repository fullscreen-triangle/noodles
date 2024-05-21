//! FASTQ I/O.

pub mod reader;
mod writer;

pub use self::{reader::Reader, writer::Writer};
