use buildstructor::builder;
use derive_more::From;
use http::header::{HeaderName, CONTENT_TYPE};
use http::{HeaderMap, HeaderValue};
use multimap::MultiMap;
use std::error::Error;

pub struct Collections {
    headers: HeaderMap,
}

#[builder]
impl Collections {
    #[builder]
    fn new(
        headers: MultiMap<IntoHeaderName, IntoHeaderValue>,
    ) -> Result<Collections, Box<dyn Error>> {
        let mut valid_headers = HeaderMap::new();
        for (key, values) in headers {
            let header_name: HeaderName = key.try_into()?;
            for value in values {
                let header_value: HeaderValue = value.try_into()?;
                valid_headers.insert(header_name.clone(), header_value);
            }
        }

        Ok(Self {
            headers: valid_headers,
        })
    }
}

#[derive(From, Eq, Hash, PartialEq)]
pub enum IntoHeaderName {
    String(String),
    HeaderName(HeaderName),
}

#[derive(From, Eq, Hash, PartialEq)]
pub enum IntoHeaderValue {
    String(String),
    HeaderValue(HeaderValue),
}

impl From<&str> for IntoHeaderName {
    fn from(name: &str) -> Self {
        IntoHeaderName::String(name.to_string())
    }
}

impl From<&str> for IntoHeaderValue {
    fn from(value: &str) -> Self {
        IntoHeaderValue::String(value.to_string())
    }
}

impl TryFrom<IntoHeaderName> for HeaderName {
    type Error = Box<dyn Error>;

    fn try_from(value: IntoHeaderName) -> Result<Self, Self::Error> {
        Ok(match value {
            IntoHeaderName::String(name) => HeaderName::try_from(name)?,
            IntoHeaderName::HeaderName(name) => name,
        })
    }
}

impl TryFrom<IntoHeaderValue> for HeaderValue {
    type Error = Box<dyn Error>;

    fn try_from(value: IntoHeaderValue) -> Result<Self, Self::Error> {
        Ok(match value {
            IntoHeaderValue::String(value) => HeaderValue::try_from(value)?,
            IntoHeaderValue::HeaderValue(value) => value,
        })
    }
}

fn main() {
    let collections = Collections::builder()
        .header("foo", "bar")
        .header(CONTENT_TYPE, "html")
        .header("bif".to_string(), "baz".to_string())
        .build()
        .unwrap();

    assert_eq!(
        collections.headers.get("foo").unwrap(),
        HeaderValue::from_static("bar")
    );
    assert_eq!(
        collections.headers.get(CONTENT_TYPE).unwrap(),
        HeaderValue::from_static("html")
    );
    assert_eq!(
        collections.headers.get("bif").unwrap(),
        HeaderValue::from_static("baz")
    );
}
