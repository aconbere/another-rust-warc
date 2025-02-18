use std::collections::HashMap;
use std::fmt;

use uuid::Uuid;

/// Header is a map from field-name to field-value
pub type Header = HashMap<FieldNames, String>;

pub struct RecordID {
    id: Uuid,
}

impl RecordID {
    pub fn new() -> Self {
        return Self { id: Uuid::new_v4() };
    }
}

impl fmt::Display for RecordID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.id.urn())
    }
}

#[derive(PartialEq)]
pub enum RecordTypes {
    General,
    WarcInfo,
    Response,
    Resource,
    Request,
    Metadata,
    Revisit,
    Conversion,
    Continuation,
}

impl RecordTypes {
    pub fn to_string(&self) -> String {
        match self {
            RecordTypes::General => "general".to_string(),
            RecordTypes::WarcInfo => "warcinfo".to_string(),
            RecordTypes::Response => "response".to_string(),
            RecordTypes::Resource => "resource".to_string(),
            RecordTypes::Request => "request".to_string(),
            RecordTypes::Metadata => "metadata".to_string(),
            RecordTypes::Revisit => "revisit".to_string(),
            RecordTypes::Conversion => "conversion".to_string(),
            RecordTypes::Continuation => "continuation".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Result<RecordTypes, String> {
        match s {
            "general" => Ok(RecordTypes::General),
            "warcinfo" => Ok(RecordTypes::WarcInfo),
            "response" => Ok(RecordTypes::Response),
            "resource" => Ok(RecordTypes::Resource),
            "request" => Ok(RecordTypes::Request),
            "metadata" => Ok(RecordTypes::Metadata),
            "revisit" => Ok(RecordTypes::Revisit),
            "conversion" => Ok(RecordTypes::Conversion),
            "continuation" => Ok(RecordTypes::Continuation),
            _ => Err(format!("no record type matches: {}", s)),
        }
    }
}

/// FieldName
///
/// Represents a WARC header field-name. The common cases all have enum values, other field names
/// can use the `Custom` enum.
#[derive(Eq, Hash, PartialEq, Clone)]
pub enum FieldNames {
    RecordID,
    ContentLength,
    Date,
    Type,
    ContentType,
    ConcurrentTo,
    BlockDigest,
    PayloadDigest,
    IPAddress,
    RefersTo,
    RefersToTargetURI,
    RefersToDate,
    TargetURI,
    Truncated,
    WarcinfoID,
    FileName,
    Profile,
    IdentifiedPayloadType,
    SegmentNumber,
    SegmentOriginID,
    SegmentTotalLength,
    Custom(String),
}

impl FieldNames {
    /// from_string will match to the correct enum value or use the
    /// `New` enum for custom field names
    pub fn from_string(s: &str) -> FieldNames {
        let s = s.to_ascii_lowercase();

        match s.as_str() {
            "warc-record-id" => FieldNames::RecordID,
            "content-length" => FieldNames::ContentLength,
            "warc-date" => FieldNames::Date,
            "warc-type" => FieldNames::Type,
            "content-type" => FieldNames::ContentType,
            "warc-concurrent-to" => FieldNames::ConcurrentTo,
            "warc-block-digest" => FieldNames::BlockDigest,
            "warc-payload-digest" => FieldNames::PayloadDigest,
            "warc-ip-address" => FieldNames::IPAddress,
            "warc-refers-to" => FieldNames::RefersTo,
            "warc-refers-to-target-uri" => FieldNames::RefersToTargetURI,
            "warc-refers-to-date" => FieldNames::RefersToDate,
            "warc-target-uri" => FieldNames::TargetURI,
            "warc-truncated" => FieldNames::Truncated,
            "warc-warcinfo-id" => FieldNames::WarcinfoID,
            "warc-filename" => FieldNames::FileName,
            "warc-profile" => FieldNames::Profile,
            "warc-identified-payload-type" => FieldNames::IdentifiedPayloadType,
            "warc-segment-number" => FieldNames::SegmentNumber,
            "warc-segment-origin-id" => FieldNames::SegmentOriginID,
            "warc-segment-total-length" => FieldNames::SegmentTotalLength,
            _ => FieldNames::Custom(s),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FieldNames::RecordID => "WARC-Record-ID".to_string(),
            FieldNames::ContentLength => "Content-Length".to_string(),
            FieldNames::Date => "WARC-Date".to_string(),
            FieldNames::Type => "WARC-type".to_string(),
            FieldNames::ContentType => "Content-Type".to_string(),
            FieldNames::ConcurrentTo => "WARC-Concurrent-To".to_string(),
            FieldNames::BlockDigest => "WARC-Block-Digest".to_string(),
            FieldNames::PayloadDigest => "WARC-Payload-Digest".to_string(),
            FieldNames::IPAddress => "WARC-IP-Address".to_string(),
            FieldNames::RefersTo => "WARC-Refers-To".to_string(),
            FieldNames::RefersToTargetURI => "WARC-Refers-To-Target-URI".to_string(),
            FieldNames::RefersToDate => "WARC-Refers-To-Date".to_string(),
            FieldNames::TargetURI => "WARC-Target-URI".to_string(),
            FieldNames::Truncated => "WARC-Truncated".to_string(),
            FieldNames::WarcinfoID => "WARC-Warcinfo-ID".to_string(),
            FieldNames::FileName => "WARC-Filename".to_string(),
            FieldNames::Profile => "WARC-Profile".to_string(),
            FieldNames::IdentifiedPayloadType => "WARC-Identified-Payload-Type".to_string(),
            FieldNames::SegmentNumber => "WARC-Segment-Number".to_string(),
            FieldNames::SegmentOriginID => "WARC-Segment-Origin-ID".to_string(),
            FieldNames::SegmentTotalLength => "WARC-Segment-Total-Length".to_string(),
            FieldNames::Custom(s) => s.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_downcases_input() {
        assert!(FieldNames::from_string("warc-record-id") == FieldNames::RecordID);
        assert!(FieldNames::from_string("WARC-record-id") == FieldNames::RecordID);
        assert!(FieldNames::from_string("WARC-Record-Id") == FieldNames::RecordID);
        assert!(
            FieldNames::from_string("custom-field-name")
                == FieldNames::Custom("custom-field-name".to_string())
        );
    }

    #[test]
    fn test_to_string() {
        assert!(FieldNames::RecordID.to_string() == "WARC-Record-ID".to_string());
    }
}
