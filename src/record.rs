use crate::header::Header;

/// WARC Record
///
/// A record consists of the version string, a list of headers and the actual content (in bytes)
pub struct Record {
    /// WARC version string (WARC/1.1)
    pub version: String,

    /// Record header fields
    pub header: Header,

    /// Record body content length
    pub content_length: u64,

    /// Record content block
    pub content: Vec<u8>,
}

impl Record {
    pub fn new(header: Header, content_length: u64) -> Self {
        let version = "WARC/1.1".to_string();
        Self {
            version,
            header,
            content_length,
            content: vec![],
        }
    }
}
