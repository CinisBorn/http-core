//! Request types, utilities and tools.

use std::collections::HashMap;

use crate::Method;

/// Supported HTTP versions. 
/// 
/// 
#[derive(Debug, PartialEq, Clone)]
pub enum HttpVersion {
    /// The HTTP/1.1 version.
    /// 
    /// Older version such as HTTP/0.9 or HTTP/1.0 won't 
    /// be supported in the future.
    V1,
    /// Unsupported HTTP version. 
    /// 
    /// This variant carries the unsupported 
    /// HTTP version.
    Unsupported(String),
}

impl HttpVersion {
    /// Creates a [`HttpVersion`] from a `String`.
    /// 
    /// `version` is case sensitive:
    /// ```no_run
    /// # use http_core::request::HttpVersion;
    /// assert_eq!(HttpVersion::from("HTTP/1.1"), HttpVersion::V1) // it works 
    /// ```
    /// If it's not possible create a `HttpVersion` from a `String`, the
    /// [`HttpVersion::Unsupported`] variant is returned.
    pub fn from(version: &str) -> Self {
        match version {
            "HTTP/1.1" => HttpVersion::V1,
            v => HttpVersion::Unsupported(v.to_string()),
        }
    }
}

#[derive(Debug)]
/// The request line.
/// 
/// It contains information about the HTTP version, 
/// method and target of [`Request`].
/// 
/// See more at
/// [RFC 9112](https://www.rfc-editor.org/rfc/rfc9112.html#name-message)
pub struct RequestLine {
    version: HttpVersion,
    method: Method,
    target: String,
}

impl RequestLine {
    /// Builds a new instance of `RequestLine`.
    pub fn build(version: HttpVersion, method: Method, target: &str) -> Self {
        Self {
            version,
            method,
            target: target.to_string(),
        }
    }

    /// Returns the HTTP method of [`Request`].
    /// 
    /// See more at [`Method`].
    pub fn method(&self) -> Method {
        self.method
    }

    /// Returns a reference to the target resource 
    /// of [`Request`].
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns a reference to the HTTP 
    /// version of [`Request`]
    pub fn version(&self) -> &HttpVersion {
        &self.version
    }
}

/// The client request. 
/// 
/// Contains information about request line, headers and 
/// message body.
/// 
/// See more about request line at [`RequestLine`].
#[derive(Debug)]
pub struct Request {
    start_line: RequestLine,
    headers: Option<HashMap<String, String>>,
    body: Option<Vec<u8>>,
}

impl Request {
    /// Builds a new `Request` instance.
    pub fn build(
        start_line: RequestLine,
        headers: Option<HashMap<String, String>>,
        body: Option<Vec<u8>>,
    ) -> Self {
        Self {
            start_line,
            headers,
            body,
        }
    }

    /// Returns a reference to the [`RequestLine`].
    pub fn request_line(&self) -> &RequestLine {
        &self.start_line
    }

    /// Returns a reference to the request [`Headers`].
    pub fn headers(&self) -> Option<&HashMap<String, String>> {
        self.headers.as_ref()
    }

    /// Returns a reference to the request message body.
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    /// Returns the `Connection` header of request if any.
    /// 
    /// This is an utility for help callers to reduce 
    /// boilerplate.
    pub fn connection(&self) -> Option<&str> {
        if let Some(headers) = self.headers() {
            return headers.get("Connection").map(|v| v.as_ref())            
        }

        None
    }
}
