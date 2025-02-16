Another-Rust-Warc
=========

A fork of [rust-warc](https://github.com/orottier/rust-warc) adding writing support.

## Why another rust warc crate?

There are two main available warc libraries for rust; [warc](https://github.com/jedireza/warc) and [rust-warc](https://github.com/orottier/rust-warc). Both libraries have flaws for my personal usage. The former lacks support for streaming bodies (a feature I wanted for downloading larger bodies) and the former only has reader support. To move forward I forked rust-warc which I appreciated its simple and easily understood codebase and added write support.

## Example

```rust
use std::fs::File;
use std::path::Path;

use another_rust_warc::header::{FieldNames, Header, RecordID, RecordTypes};
use another_rust_warc::record::Record;
use another_rust_warc::writer::write_record;
use anyhow::{Result, anyhow};
use chrono::prelude::*;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;

fn fetch(url: &str, output: &Path) -> Result<()> {
    let client = Client::new();

    let request = client
        .get(uri_str)
        .header(USER_AGENT, USER_AGENT_STR)
        .build()?;

    let mut response =
        client.execute(request.try_clone().ok_or(anyhow!("could not clone body"))?)?;

    let mut out_file = File::create(output)?;

    let date = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    let content_length = response
        .content_length()
        .ok_or(anyhow!("no valid content length header on response"))?;

    let remote_addr = response
        .remote_addr()
        .ok_or(anyhow!("no valid remote address header in response"))?
        .to_string();

    let mut header = Header::new();
    header.insert(FieldNames::RecordID, RecordID::new().to_string());
    header.insert(FieldNames::Type, RecordTypes::Response.to_string());
    header.insert(FieldNames::Date, date);
    header.insert(FieldNames::IPAddress, remote_addr);
    header.insert(FieldNames::ContentLength, content_length.to_string());
    let record = Record::new(header, content_length);

    // Writes the warc record
    write_record(writer, &record, response)?;
}
```
