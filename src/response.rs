//! Contains necessary types, utilities and tools for HTTP Response handling.

use crate::Method;
use std::collections::HashMap;

/// The response code status.
#[derive(Debug, Clone)]
pub enum StatusCode {
    /// Represents the HTTP status code: `200`.
    Ok,
    /// Represents the HTTP status code: `204`.
    NoContent,
    /// Represents the HTTP status code: `304`.
    NotModified,
    /// Represents the HTTP status code: `400`.
    BadRequest,
    /// Represents the HTTP status code: `404`.
    NotFound,
    /// Represents the HTTP status code: `405`.
    MethodNotAllowed(Vec<Method>),
    /// The request exceeded the time duration specified by the server.
    /// 
    /// Represents the HTTP status code: `408`.
    RequestTimeout,
    /// The request contains a message body, but neither the `Transfer-Encoding`  nor 
    /// `Content-Length` header is present.
    /// 
    /// Represents the HTTP status code: `411` 
    LengthRequired,
    /// The message body exceeded the max size specified by the server.
    /// 
    /// Represents the HTTP status code: `413`.
    ContentTooLarge,
    /// The header request exceeded the max number of fields.
    /// 
    /// Represents the HTTP status code: `431`.
    RequestHeaderFieldsTooLarge,
    /// The request uri exceeded the max size.
    /// 
    /// Represents the HTTP status code: `414`.
    UriTooLong,
    /// Represents the HTTP status code: `505`.
    HttpVersionNotSupported,
    /// Represents the HTTP status code: `500`.
    InternalServerError,
    /// The message body encoding is not supported. 
    /// 
    /// Represents the HTTP status code `501`..  
    NotImplemented,
    /// Represents the HTTP status code: `503` .
    ServiceUnavailable,
}

impl StatusCode {
    /// gets the numeric code from phrasal code.
    pub fn code(&self) -> String {
        match self {
            Self::Ok => 200.to_string(),
            Self::NoContent => 204.to_string(),
            Self::NotModified => 304.to_string(),
            Self::BadRequest => 400.to_string(),
            Self::NotFound => 404.to_string(),
            Self::MethodNotAllowed(_) => 405.to_string(),
            Self::RequestTimeout => 408.to_string(),
            Self::LengthRequired => 411.to_string(),
            Self::ContentTooLarge => 413.to_string(),
            Self::UriTooLong => 414.to_string(),
            Self::RequestHeaderFieldsTooLarge => 431.to_string(),
            Self::InternalServerError => 500.to_string(),
            Self::NotImplemented => 501.to_string(),
            Self::ServiceUnavailable => 503.to_string(),
            Self::HttpVersionNotSupported => 505.to_string()
        }
    }
}

#[derive(Debug)]
/// Represents a HTTP Response
pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: StatusCode::Ok, // temporary default value
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl ResponseData for Response {
    fn set_headers(&mut self, value: HashMap<String, String>) {
        self.headers = value;
    }

    fn set_body(&mut self, value: Option<&[u8]>) {
        self.body = value.map(|l| l.to_vec());
    }

    fn set_status(&mut self, value: StatusCode) {
        self.status = value;
    }

    fn headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    fn body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }

    fn status(&self) -> &StatusCode {
        &self.status
    }

    fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

/// Defines fundamental data handling behavior to [`Response`]
pub trait ResponseData {
    /// Sets the headers of HTTP Response.
    ///
    /// ```no_run
    /// # use std::collections::HashMap;
    /// # use http_core::response::Response;
    /// # use crate::http_core::response::ResponseData;
    /// let mut res = Response::new();
    /// res.set_headers(HashMap::new());
    /// ```
    /// You can define a pre-defined set of headers:
    /// ```no_run
    /// # use std::collections::HashMap;
    /// # use http_core::response::Response;
    /// # use crate::http_core::response::ResponseData;
    /// let mut res = Response::new();
    /// let mut headers = HashMap::new();
    ///
    /// headers.insert(
    ///     "Content-Type".to_string(),
    ///     "application/json".to_string()
    /// );
    ///
    /// res.set_headers(headers);
    /// ```
    fn set_headers(&mut self, value: HashMap<String, String>);
    /// Sets the [`Response`] body.
    /// ```no_run
    /// # use http_core::response::Response;
    /// # use crate::http_core::response::ResponseData;
    /// let mut response = Response::new();
    /// response.set_body(Some(b"Hello World!"))
    /// ```
    fn set_body(&mut self, value: Option<&[u8]>);
    /// Sets the [`Response`] status code.
    /// ```no_run
    /// # use http_core::response::StatusCode;
    /// # use http_core::response::Response;
    /// # use crate::http_core::response::ResponseData;
    /// let mut response = Response::new();
    /// response.set_status(StatusCode::Ok);
    /// ```
    fn set_status(&mut self, value: StatusCode);
    /// Gets a mutable reference to the response headers.
    fn headers_mut(&mut self) -> &mut HashMap<String, String>;
    /// Gets a referece to the [`Response`] body.
    fn body(&self) -> Option<&Vec<u8>>;
    /// Gets a reference to the [`Response`] status code.
    fn status(&self) -> &StatusCode;
    /// Gets a reference to the [`Response`] headers.
    fn headers(&self) -> &HashMap<String, String>;
}
