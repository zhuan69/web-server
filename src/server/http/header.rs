use std::collections::HashMap;
use std::str::FromStr;

pub(super) type Headers = HashMap<HeaderKeys, String>;

#[derive(PartialEq, Hash, Eq)]
pub(super) enum HeaderKeys {
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,
    AcceptRanges,
    Age,
    Allow,
    Authorization,
    CacheControl,
    Connection,
    ContentEncoding,
    ContentLanguage,
    ContentLength,
    ContentLocation,
    ContentMD5,
    ContentRange,
    ContentType,
    Date,
    ETag,
    Expect,
    Expires,
    From,
    Host,
    IfMatch,
    IfModifiedSince,
    IfNonMatch,
    IfUnmodifiedSince,
    LastModified,
    Location,
    MaxForwards,
    Pragma,
    ProxyAuthenticate,
    ProxyAuthorization,
    Range,
    Referer,
    RetryAfter,
    Server,
    TE,
    Trailer,
    TransferEncoding,
    Upgrade,
    UserAgent,
    Vary,
    Via,
    Warning,
    WWWAuthenticate,
}

impl FromStr for HeaderKeys {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "accept" => Ok(HeaderKeys::Accept),
            "accept-charset" => Ok(HeaderKeys::AcceptCharset),
            "accept-encoding" => Ok(HeaderKeys::AcceptEncoding),
            "accept-language" => Ok(HeaderKeys::AcceptLanguage),
            "accept-ranges" => Ok(HeaderKeys::AcceptRanges),
            "age" => Ok(HeaderKeys::Age),
            "allow" => Ok(HeaderKeys::Allow),
            "authorization" => Ok(HeaderKeys::Authorization),
            "cache-control" => Ok(HeaderKeys::CacheControl),
            "connection" => Ok(HeaderKeys::Connection),
            "content-encoding" => Ok(HeaderKeys::ContentEncoding),
            "content-language" => Ok(HeaderKeys::ContentLanguage),
            "content-length" => Ok(HeaderKeys::ContentLength),
            "content-location" => Ok(HeaderKeys::ContentLocation),
            "content-md5" => Ok(HeaderKeys::ContentMD5),
            "content-range" => Ok(HeaderKeys::ContentRange),
            "content-type" => Ok(HeaderKeys::ContentType),
            "date" => Ok(HeaderKeys::Date),
            "etag" => Ok(HeaderKeys::ETag),
            "expect" => Ok(HeaderKeys::Expect),
            "expires" => Ok(HeaderKeys::Expires),
            "from" => Ok(HeaderKeys::From),
            "host" => Ok(HeaderKeys::Host),
            "if-match" => Ok(HeaderKeys::IfMatch),
            "if-modified-since" => Ok(HeaderKeys::IfModifiedSince),
            "if-non-match" => Ok(HeaderKeys::IfNonMatch),
            "if-unmodified-since" => Ok(HeaderKeys::IfUnmodifiedSince),
            "last-modified" => Ok(HeaderKeys::LastModified),
            "location" => Ok(HeaderKeys::Location),
            "max-forwards" => Ok(HeaderKeys::MaxForwards),
            "pragma" => Ok(HeaderKeys::Pragma),
            "proxy-authenticate" => Ok(HeaderKeys::ProxyAuthenticate),
            "proxy-authorization" => Ok(HeaderKeys::ProxyAuthorization),
            "range" => Ok(HeaderKeys::Range),
            "referer" => Ok(HeaderKeys::Referer),
            "retry-after" => Ok(HeaderKeys::RetryAfter),
            "server" => Ok(HeaderKeys::Server),
            "te" => Ok(HeaderKeys::TE),
            "trailer" => Ok(HeaderKeys::Trailer),
            "transfer-encoding" => Ok(HeaderKeys::TransferEncoding),
            "upgrade" => Ok(HeaderKeys::Upgrade),
            "user-agent" => Ok(HeaderKeys::UserAgent),
            "vary" => Ok(HeaderKeys::Vary),
            "via" => Ok(HeaderKeys::Via),
            "warning" => Ok(HeaderKeys::Warning),
            "www-authenticate" => Ok(HeaderKeys::WWWAuthenticate),
            _ => Err(String::from("Header Not Matched")),
        }
    }
}

pub(super) fn parse(headers: &mut Headers, key: String, value: String) {
    if let Ok(header_key) = key.parse::<HeaderKeys>() {
        if let Some(val) = headers.get_mut(&header_key) {
            *val = value;
        } else {
            headers.insert(header_key, value);
        }
    }
}

pub(super) fn get_item(headers: &Headers, key: HeaderKeys) -> String {
    headers.get(&key).unwrap().to_string()
}
