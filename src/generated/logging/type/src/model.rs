// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]
#![no_implicit_prelude]
extern crate bytes;
extern crate serde;
extern crate serde_with;
extern crate std;
extern crate wkt;

/// A common proto for logging HTTP requests. Only contains semantics
/// defined by the HTTP specification. Product-specific logging
/// information MUST be defined in a separate message.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct HttpRequest {
    /// The request method. Examples: `"GET"`, `"HEAD"`, `"PUT"`, `"POST"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub request_method: std::string::String,

    /// The scheme (http, https), the host name, the path and the query
    /// portion of the URL that was requested.
    /// Example: ``http://example.com/some/info?color=red``.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub request_url: std::string::String,

    /// The size of the HTTP request message in bytes, including the request
    /// headers and the request body.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub request_size: i64,

    /// The response code indicating the status of response.
    /// Examples: 200, 404.
    pub status: i32,

    /// The size of the HTTP response message sent back to the client, in bytes,
    /// including the response headers and the response body.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub response_size: i64,

    /// The user agent sent by the client. Example:
    /// `"Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET
    /// CLR 1.0.3705)"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub user_agent: std::string::String,

    /// The IP address (IPv4 or IPv6) of the client that issued the HTTP
    /// request. This field can include port information. Examples:
    /// `"192.168.1.1"`, `"10.0.0.1:80"`, `"FE80::0202:B3FF:FE1E:8329"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub remote_ip: std::string::String,

    /// The IP address (IPv4 or IPv6) of the origin server that the request was
    /// sent to. This field can include port information. Examples:
    /// `"192.168.1.1"`, `"10.0.0.1:80"`, `"FE80::0202:B3FF:FE1E:8329"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub server_ip: std::string::String,

    /// The referer URL of the request, as defined in
    /// [HTTP/1.1 Header Field
    /// Definitions](https://datatracker.ietf.org/doc/html/rfc2616#section-14.36).
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub referer: std::string::String,

    /// The request processing latency on the server, from the time the request was
    /// received until the response was sent.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub latency: std::option::Option<wkt::Duration>,

    /// Whether or not a cache lookup was attempted.
    pub cache_lookup: bool,

    /// Whether or not an entity was served from cache
    /// (with or without validation).
    pub cache_hit: bool,

    /// Whether or not the response was validated with the origin server before
    /// being served from cache. This field is only meaningful if `cache_hit` is
    /// True.
    pub cache_validated_with_origin_server: bool,

    /// The number of HTTP response bytes inserted into cache. Set only when a
    /// cache fill was attempted.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub cache_fill_bytes: i64,

    /// Protocol used for the request. Examples: "HTTP/1.1", "HTTP/2", "websocket"
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub protocol: std::string::String,
}

impl HttpRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [request_method][crate::model::HttpRequest::request_method].
    pub fn set_request_method<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.request_method = v.into();
        self
    }

    /// Sets the value of [request_url][crate::model::HttpRequest::request_url].
    pub fn set_request_url<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.request_url = v.into();
        self
    }

    /// Sets the value of [request_size][crate::model::HttpRequest::request_size].
    pub fn set_request_size<T: std::convert::Into<i64>>(mut self, v: T) -> Self {
        self.request_size = v.into();
        self
    }

    /// Sets the value of [status][crate::model::HttpRequest::status].
    pub fn set_status<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.status = v.into();
        self
    }

    /// Sets the value of [response_size][crate::model::HttpRequest::response_size].
    pub fn set_response_size<T: std::convert::Into<i64>>(mut self, v: T) -> Self {
        self.response_size = v.into();
        self
    }

    /// Sets the value of [user_agent][crate::model::HttpRequest::user_agent].
    pub fn set_user_agent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.user_agent = v.into();
        self
    }

    /// Sets the value of [remote_ip][crate::model::HttpRequest::remote_ip].
    pub fn set_remote_ip<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.remote_ip = v.into();
        self
    }

    /// Sets the value of [server_ip][crate::model::HttpRequest::server_ip].
    pub fn set_server_ip<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.server_ip = v.into();
        self
    }

    /// Sets the value of [referer][crate::model::HttpRequest::referer].
    pub fn set_referer<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.referer = v.into();
        self
    }

    /// Sets the value of [latency][crate::model::HttpRequest::latency].
    pub fn set_latency<T: std::convert::Into<std::option::Option<wkt::Duration>>>(
        mut self,
        v: T,
    ) -> Self {
        self.latency = v.into();
        self
    }

    /// Sets the value of [cache_lookup][crate::model::HttpRequest::cache_lookup].
    pub fn set_cache_lookup<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.cache_lookup = v.into();
        self
    }

    /// Sets the value of [cache_hit][crate::model::HttpRequest::cache_hit].
    pub fn set_cache_hit<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.cache_hit = v.into();
        self
    }

    /// Sets the value of [cache_validated_with_origin_server][crate::model::HttpRequest::cache_validated_with_origin_server].
    pub fn set_cache_validated_with_origin_server<T: std::convert::Into<bool>>(
        mut self,
        v: T,
    ) -> Self {
        self.cache_validated_with_origin_server = v.into();
        self
    }

    /// Sets the value of [cache_fill_bytes][crate::model::HttpRequest::cache_fill_bytes].
    pub fn set_cache_fill_bytes<T: std::convert::Into<i64>>(mut self, v: T) -> Self {
        self.cache_fill_bytes = v.into();
        self
    }

    /// Sets the value of [protocol][crate::model::HttpRequest::protocol].
    pub fn set_protocol<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.protocol = v.into();
        self
    }
}

impl wkt::message::Message for HttpRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.logging.type.HttpRequest"
    }
}

/// The severity of the event described in a log entry, expressed as one of the
/// standard severity levels listed below.  For your reference, the levels are
/// assigned the listed numeric values. The effect of using numeric values other
/// than those listed is undefined.
///
/// You can filter for log entries by severity.  For example, the following
/// filter expression will match log entries with severities `INFO`, `NOTICE`,
/// and `WARNING`:
///
/// ```norust
/// severity > DEBUG AND severity <= WARNING
/// ```
///
/// If you are writing log entries, you should map other severity encodings to
/// one of these standard levels. For example, you might map all of Java's FINE,
/// FINER, and FINEST levels to `LogSeverity.DEBUG`. You can preserve the
/// original severity level in the log entry payload if you wish.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LogSeverity(std::borrow::Cow<'static, str>);

impl LogSeverity {
    /// Creates a new LogSeverity instance.
    pub const fn new(v: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(v))
    }

    /// Gets the enum value.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Useful constants to work with [LogSeverity](LogSeverity)
pub mod log_severity {
    use super::LogSeverity;

    /// (0) The log entry has no assigned severity level.
    pub const DEFAULT: LogSeverity = LogSeverity::new("DEFAULT");

    /// (100) Debug or trace information.
    pub const DEBUG: LogSeverity = LogSeverity::new("DEBUG");

    /// (200) Routine information, such as ongoing status or performance.
    pub const INFO: LogSeverity = LogSeverity::new("INFO");

    /// (300) Normal but significant events, such as start up, shut down, or
    /// a configuration change.
    pub const NOTICE: LogSeverity = LogSeverity::new("NOTICE");

    /// (400) Warning events might cause problems.
    pub const WARNING: LogSeverity = LogSeverity::new("WARNING");

    /// (500) Error events are likely to cause problems.
    pub const ERROR: LogSeverity = LogSeverity::new("ERROR");

    /// (600) Critical events cause more severe problems or outages.
    pub const CRITICAL: LogSeverity = LogSeverity::new("CRITICAL");

    /// (700) A person must take an action immediately.
    pub const ALERT: LogSeverity = LogSeverity::new("ALERT");

    /// (800) One or more systems are unusable.
    pub const EMERGENCY: LogSeverity = LogSeverity::new("EMERGENCY");
}

impl std::convert::From<std::string::String> for LogSeverity {
    fn from(value: std::string::String) -> Self {
        Self(std::borrow::Cow::Owned(value))
    }
}

impl std::default::Default for LogSeverity {
    fn default() -> Self {
        log_severity::DEFAULT
    }
}
