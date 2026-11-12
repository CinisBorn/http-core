#![doc = include_str!("../README.md")]

pub mod request;
pub mod response;

/// Supported HTTP methods
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum Method {
    /// The GET method.
    GET,
    /// The POST method.
    POST,
    /// The DELETE method.
    DELETE,
    /// The PATCH method.
    PATCH,
    /// The CONNECT method.
    CONNECT,
    /// The OPTIONS method.
    OPTIONS,
    /// The PUT method.
    PUT,
    /// The HEAD method.
    HEAD
}

impl Method {
    /// Converts a [`&str`] into a [`Method`].
    /// 
    /// The `method` parameter is case-insensitive:
    /// ```no_run
    /// # use http_core::Method;
    /// assert_eq!(Method::to_method("GET"), Some(Method::GET));
    /// assert_eq!(Method::to_method("Get"), Some(Method::GET));
    /// assert_eq!(Method::to_method("get"), Some(Method::GET));
    /// assert_eq!(Method::to_method("GeT"), Some(Method::GET));
    /// // ...
    /// ```
    /// 
    /// Returns `None` if the provided value doesn't 
    /// match a supported method:
    /// 
    /// - `get`
    /// - `head`
    /// - `put`
    /// - `patch`
    /// - `delete`
    /// - `connect`
    /// - `post`
    /// - `options`
    /// 
    /// # Custom methods 
    /// 
    /// Custom methods are not supported in the present moment, although 
    /// this crate may support in the future.
    pub fn to_method(method: &str) -> Option<Method> {
        match method.to_uppercase().as_str() {
            "GET"     => Some(Method::GET),
            "HEAD"    => Some(Method::HEAD),
            "PUT"     => Some(Method::PUT),
            "PATCH"   => Some(Method::PATCH),
            "DELETE"  => Some(Method::DELETE),
            "CONNECT" => Some(Method::CONNECT),
            "POST"    => Some(Method::POST),
            "OPTIONS" => Some(Method::OPTIONS),
            _ => None,
        }
    }

    /// Converts a [`Method`] into [`&str`].
    /// 
    /// The returned value is in uppercase.
    pub fn to_str(&self) -> &str {
        match self {
            Method::CONNECT => "CONNECT",
            Method::DELETE  => "DELETE",
            Method::GET     => "GET",
            Method::HEAD    => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::PATCH   => "PATCH",
            Method::POST    => "POST",
            Method::PUT     => "PUT",
        }
    }
}
