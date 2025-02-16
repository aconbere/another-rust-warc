use std::io::{copy, BufReader, Error, Read, Write};

use crate::header::Header;
use crate::record::Record;

pub fn write_version(w: &mut dyn Write, version: &str) -> Result<(), Error> {
    write!(w, "{}\r\n", version)?;
    Ok(())
}

/// writes a header block to w
///
/// Note: Does not support multiline header values
pub fn write_header(w: &mut dyn Write, headers: &Header) -> Result<(), Error> {
    for (k, v) in headers {
        write!(w, "{}: {}\r\n", k.clone().to_string(), v)?;
    }
    write!(w, "\r\n")?;
    Ok(())
}

pub fn write_body(
    w: &mut dyn Write,
    content_length: u64,
    body: &mut dyn Read,
) -> Result<(), Error> {
    let mut reader = BufReader::new(body.take(content_length));
    copy(&mut reader, w)?;
    write!(w, "\r\n\r\n")?;
    Ok(())
}

pub fn write_record(w: &mut dyn Write, record: &Record, body: &mut dyn Read) -> Result<(), Error> {
    write_version(w, &record.version)?;
    write_header(w, &record.header)?;
    write_body(w, record.content_length, body)?;
    Ok(())
}

pub fn write_record_without_body(w: &mut dyn Write, record: &Record) -> Result<(), Error> {
    write_version(w, &record.version)?;
    write_header(w, &record.header)?;
    Ok(())
}
