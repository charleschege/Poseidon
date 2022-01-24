use crate::PoseidonResult;
use json::JsonValue;

#[derive(Debug, Clone)]
pub struct RpcClient<'a> {
    url: &'a str,
    headers: Vec<(String, String)>,
    body: JsonValue,
}

impl<'a> RpcClient<'a> {
    pub fn new(url: &'a str) -> Self {
        RpcClient {
            url,
            headers: vec![("Content-Type".to_owned(), "application/json".to_owned())],
            body: JsonValue::Null,
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.push((key.to_owned(), value.to_owned()));

        self
    }

    pub fn add_body(&mut self, body: JsonValue) -> &mut Self {
        self.body = body;

        self
    }

    pub fn send_sync(&self) -> PoseidonResult<minreq::Response> {
        let mut request = minreq::post(self.url).with_body(self.body.to_string().as_str());

        for header in &self.headers {
            request = request.with_header(&header.0, &header.1);
        }

        Ok(request.send()?)
    }

    #[cfg(feature = "smol_async_io")]
    pub fn send(&self) -> PoseidonResult<minreq::Response> {
        let mut request = minreq::post(self.url);

        for header in &self.headers {
            request = request.with_header(&header.0, &header.1);
        }

        Ok(request.send()?)
    }
}
