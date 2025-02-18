use std::io::BufRead;

use crate::header::{FieldNames, Header};
use crate::record::Record;

// trim a string in place (no (re)allocations)
fn rtrim(s: &mut String) {
    s.truncate(s.trim_end().len());
}

fn parse_record(mut read: impl BufRead) -> Result<Record, ReaderError> {
    let mut version = String::new();

    if let Err(io) = read.read_line(&mut version) {
        return Err(ReaderError::IO(io));
    }

    if version.is_empty() {
        return Err(ReaderError::EOF);
    }

    rtrim(&mut version);

    if !version.starts_with("WARC/1.") {
        let err = format!("Unknown WARC version: {}", version);
        return Err(ReaderError::Malformed(err));
    }

    let mut header = Header::with_capacity(16); // no allocations if <= 16 header fields

    /* Header values can be multiple lines if the key is preceded by a space or tab character
     * the continuation value sums up lines until the next header key is encountered.
     */
    let mut continuation: Option<(FieldNames, String)> = None;
    loop {
        let mut line_buf = String::new();

        read.read_line(&mut line_buf)
            .map_err(|io| ReaderError::IO(io))?;

        if &line_buf == "\r\n" {
            break;
        }

        rtrim(&mut line_buf);

        if line_buf.starts_with(' ') || line_buf.starts_with('\t') {
            if let Some((_, value)) = &mut continuation {
                value.push('\n');
                value.push_str(line_buf.trim());
            } else {
                return Err(ReaderError::Malformed(String::from("Invalid header block")));
            }
        } else {
            if let Some((key, value)) = std::mem::replace(&mut continuation, None) {
                header.insert(key, value);
            }

            if let Some(semi) = line_buf.find(':') {
                let value = line_buf.split_off(semi + 1).trim().to_string();
                line_buf.pop(); // eat colon
                rtrim(&mut line_buf);

                continuation = Some((FieldNames::from_string(&line_buf), value));
            } else {
                return Err(ReaderError::Malformed(String::from("Invalid header field")));
            }
        }
    }

    // insert leftover continuation
    if let Some((key, value)) = continuation {
        header.insert(key, value);
    }

    let content_length = {
        let content_len_header =
            header
                .get(&FieldNames::ContentLength)
                .ok_or(ReaderError::Malformed(String::from(
                    "Content-Length is missing",
                )))?;
        content_len_header
            .parse::<u64>()
            .or(Err(ReaderError::Malformed(String::from(
                "Content-Length is not a number",
            ))))?
    };

    let mut content = vec![0; content_length as usize];
    read.read_exact(&mut content)
        .map_err(|io| ReaderError::IO(io))?;

    let mut linefeed = [0u8; 4];
    read.read_exact(&mut linefeed)
        .map_err(|io| ReaderError::IO(io))?;

    if linefeed != [b'\r', b'\n', b'\r', b'\n'] {
        return Err(ReaderError::Malformed(String::from(
            "No double linefeed after record content",
        )));
    }

    Ok(Record {
        version,
        header,
        content,
        content_length,
    })
}

/// WARC Processing error
#[derive(Debug)]
pub enum ReaderError {
    Malformed(String),
    IO(std::io::Error),
    EOF,
}

/// WARC reader instance
///
/// The Reader serves as an iterator for [Records](Record) (or [errors](ReaderError))
pub struct Reader<R> {
    read: R,
    valid_state: bool,
}

impl<R: BufRead> Reader<R> {
    /// Create a new Reader from a [BufRead] input
    pub fn new(read: R) -> Self {
        Self {
            read,
            valid_state: true,
        }
    }
}

impl<R: BufRead> Iterator for Reader<R> {
    type Item = Result<Record, ReaderError>;

    fn next(&mut self) -> Option<Result<Record, ReaderError>> {
        if !self.valid_state {
            return None;
        }

        match parse_record(&mut self.read) {
            Ok(item) => Some(Ok(item)),
            Err(ReaderError::EOF) => None,
            Err(e) => {
                self.valid_state = false;
                Some(Err(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() {
        let data = &include_bytes!("../data/warc.in")[..];
        let mut warc = Reader::new(data);

        let item: Option<Result<Record, ReaderError>> = warc.next();
        assert!(item.is_some());

        // count remaining items
        assert_eq!(warc.count(), 2);
    }

    #[test]
    fn reader_parses_many_records() {
        let data = &include_bytes!("../data/warc.in")[..];

        let mut warc = Reader::new(data);

        let item = warc.next();
        assert!(item.is_some());
        let item = item.unwrap();
        assert!(item.is_ok());
        let item = item.unwrap();
        assert_eq!(item.header.get(&FieldNames::Type), Some(&"warcinfo".into()));

        let item = warc.next();
        assert!(item.is_some());
        let item = item.unwrap();
        assert!(item.is_ok());
        let item = item.unwrap();
        assert_eq!(item.header.get(&FieldNames::Type), Some(&"request".into()));

        let item = warc.next();
        assert!(item.is_some());
        let item = item.unwrap();
        assert!(item.is_err()); // incomplete record
    }

    #[test]
    fn test_parse_record() {
        let mut data = &include_bytes!("../data/test.warc")[..];

        let item = parse_record(&mut data).unwrap();

        assert_eq!(item.version, "WARC/1.1");

        // header names are case insensitive
        assert_eq!(
            item.header.get(&FieldNames::ContentType),
            Some(&"text/plain".into())
        );
        // and may span multiple lines
        assert_eq!(
            item.header.get(&FieldNames::RecordID),
            Some(&"multiline\nuuid value".into())
        );

        assert_eq!(item.content, "test".as_bytes());
    }
}
