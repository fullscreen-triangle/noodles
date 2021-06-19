//! Builds and writes a CRAM index from a CRAM file.
//!
//! This writes the output to stdout rather than `<src>.crai`.
//!
//! The output is similar to the output of `samtools index <src>`.

use std::{env, io};

use noodles_cram::{self as cram, crai};

fn main() -> io::Result<()> {
    let src = env::args().nth(1).expect("missing src");

    let index = cram::index(src)?;

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = crai::Writer::new(handle);

    writer.write_index(&index)?;

    Ok(())
}
