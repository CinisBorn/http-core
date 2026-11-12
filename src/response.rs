//! Contains necessary types, utilities and tools for HTTP Response handling.

use crate::Method;
use std::collections::HashMap;

/// The response status code.
/// 
/// Contains variants representing HTTP status code. This `enum` is 
/// not exhaustive and it may support new status code in the future.
#[derive(Debug, Clone)]
pub enum StatusCode {
    /// The HTTP status code `200`.
    /// 
    /// Represents a successful request.
    Ok,
    /// The HTTP status code `204`.
    /// 
    /// Represents a successful request, but there 
    /// is not content send back in the response 
    /// body.
    NoContent,
    /// The HTTP status code `304`.
    /// 
    /// requested resource has not changed since 
    /// the version the client already has.
    NotModified,
    /// The HTTP status code `400`.
    /// 
    /// Represents an error from client-side.
    BadRequest,
    /// The HTTP status code `404`.
    /// 
    /// The requested resource was not found.
    NotFound,
    /// The HTTP status code `405`.
    /// 
    /// The server knows the method, but the target 
    /// resource doesn't support this method.
    MethodNotAllowed(Vec<Method>),
    /// The HTTP status code `408`.
    /// 
    /// The request expired. The connection will be closed.
    RequestTimeout,
    /// The HTTP status code `411` 
    /// 
    /// The request contains a message body, but neither the 
    /// `Transfer-Encoding` nor `Content-Length` header is present.
    LengthRequired,
    /// The HTTP status code `413`.
    /// 
    /// The message body is too large.
    ContentTooLarge,
    /// The HTTP status code `431`.
    /// 
    /// There are two scenarions where this error can be returned:
    /// 
    /// - The request header size is too large.
    /// - The header is numerous.
    RequestHeaderFieldsTooLarge,
    /// The HTTP status code `414`.
    /// 
    /// The request uri is too large/long.
    UriTooLong,
    /// HTTP status code `505`.
    /// 
    /// The HTTP version is not supported by server.
    HttpVersionNotSupported,
    /// The HTTP status code `500`.
    /// 
    /// A server-side error occuried. 
    InternalServerError,
    /// The HTTP status code `501`..  
    /// 
    /// The message body encoding is not supported. 
    NotImplemented,
    /// The HTTP status code `503`.
    /// 
    /// The server is busy.
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
/// A HTTP response.
/// 
/// The response contains information about the state of resource, request 
/// and so on...
/// 
/// A `Response` is initialized with: [`Response::default`]. The caller may
/// set a `body`, `header` and [`StatusCode`].
/// 
/// By default, the response status code is [`StatusCode::Ok`] 
/// with no header and no message body.
pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: StatusCode::Ok,
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
