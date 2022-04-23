use buildstructor::builder;
use http::header::HeaderName;
use http::HeaderValue;
use std::error::Error;
#[derive(Debug)]
pub struct Request<T> {
    inner: http::Request<T>,
}

#[builder]
impl<T> Request<T> {
    pub fn fake_new<K, V>(
        headers: Vec<(K, V)>,
        uri: Option<http::Uri>,
        method: Option<http::Method>,
        body: T,
    ) -> http::Result<Request<T>>
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        let mut builder = http::request::Builder::new()
            .method(method.unwrap_or_default())
            .uri(uri.unwrap_or_default());
        for (key, value) in headers {
            builder = builder.header(key, value);
        }
        let req = builder.body(body)?;

        Ok(Self { inner: req })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = Request::fake_builder()
        .header(("a".to_string(), "b".to_string()))
        .header(("a".to_string(), "b".to_string()))
        .body("")
        .build()
        .unwrap();

    Ok(())
}
