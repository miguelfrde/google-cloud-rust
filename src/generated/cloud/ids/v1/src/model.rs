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
extern crate async_trait;
extern crate bytes;
extern crate gax;
extern crate lazy_static;
extern crate longrunning;
extern crate lro;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate std;
extern crate tracing;
extern crate wkt;

/// Endpoint describes a single IDS endpoint. It defines a forwarding rule to
/// which packets can be sent for IDS inspection.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Endpoint {
    /// Output only. The name of the endpoint.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// Output only. The create time timestamp.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// Output only. The update time timestamp.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub update_time: std::option::Option<wkt::Timestamp>,

    /// The labels of the endpoint.
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<std::string::String, std::string::String>,

    /// Required. The fully qualified URL of the network to which the IDS Endpoint is
    /// attached.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub network: std::string::String,

    /// Output only. The fully qualified URL of the endpoint's ILB Forwarding Rule.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub endpoint_forwarding_rule: std::string::String,

    /// Output only. The IP address of the IDS Endpoint's ILB.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub endpoint_ip: std::string::String,

    /// User-provided description of the endpoint
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub description: std::string::String,

    /// Required. Lowest threat severity that this endpoint will alert on.
    pub severity: crate::model::endpoint::Severity,

    /// Output only. Current state of the endpoint.
    pub state: crate::model::endpoint::State,

    /// Whether the endpoint should report traffic logs in addition to threat logs.
    pub traffic_logs: bool,
}

impl Endpoint {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::Endpoint::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [create_time][crate::model::Endpoint::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [update_time][crate::model::Endpoint::update_time].
    pub fn set_update_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.update_time = v.into();
        self
    }

    /// Sets the value of [network][crate::model::Endpoint::network].
    pub fn set_network<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.network = v.into();
        self
    }

    /// Sets the value of [endpoint_forwarding_rule][crate::model::Endpoint::endpoint_forwarding_rule].
    pub fn set_endpoint_forwarding_rule<T: std::convert::Into<std::string::String>>(
        mut self,
        v: T,
    ) -> Self {
        self.endpoint_forwarding_rule = v.into();
        self
    }

    /// Sets the value of [endpoint_ip][crate::model::Endpoint::endpoint_ip].
    pub fn set_endpoint_ip<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.endpoint_ip = v.into();
        self
    }

    /// Sets the value of [description][crate::model::Endpoint::description].
    pub fn set_description<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.description = v.into();
        self
    }

    /// Sets the value of [severity][crate::model::Endpoint::severity].
    pub fn set_severity<T: std::convert::Into<crate::model::endpoint::Severity>>(
        mut self,
        v: T,
    ) -> Self {
        self.severity = v.into();
        self
    }

    /// Sets the value of [state][crate::model::Endpoint::state].
    pub fn set_state<T: std::convert::Into<crate::model::endpoint::State>>(mut self, v: T) -> Self {
        self.state = v.into();
        self
    }

    /// Sets the value of [traffic_logs][crate::model::Endpoint::traffic_logs].
    pub fn set_traffic_logs<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.traffic_logs = v.into();
        self
    }

    /// Sets the value of [labels][crate::model::Endpoint::labels].
    pub fn set_labels<T, K, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (K, V)>,
        K: std::convert::Into<std::string::String>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.labels = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
        self
    }
}

impl wkt::message::Message for Endpoint {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.ids.v1.Endpoint"
    }
}

/// Defines additional types related to Endpoint
pub mod endpoint {
    #[allow(unused_imports)]
    use super::*;

    /// Threat severity levels.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Severity(std::borrow::Cow<'static, str>);

    impl Severity {
        /// Creates a new Severity instance.
        pub const fn new(v: &'static str) -> Self {
            Self(std::borrow::Cow::Borrowed(v))
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [Severity](Severity)
    pub mod severity {
        use super::Severity;

        /// Not set.
        pub const SEVERITY_UNSPECIFIED: Severity = Severity::new("SEVERITY_UNSPECIFIED");

        /// Informational alerts.
        pub const INFORMATIONAL: Severity = Severity::new("INFORMATIONAL");

        /// Low severity alerts.
        pub const LOW: Severity = Severity::new("LOW");

        /// Medium severity alerts.
        pub const MEDIUM: Severity = Severity::new("MEDIUM");

        /// High severity alerts.
        pub const HIGH: Severity = Severity::new("HIGH");

        /// Critical severity alerts.
        pub const CRITICAL: Severity = Severity::new("CRITICAL");
    }

    impl std::convert::From<std::string::String> for Severity {
        fn from(value: std::string::String) -> Self {
            Self(std::borrow::Cow::Owned(value))
        }
    }

    impl std::default::Default for Severity {
        fn default() -> Self {
            severity::SEVERITY_UNSPECIFIED
        }
    }

    /// Endpoint state
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct State(std::borrow::Cow<'static, str>);

    impl State {
        /// Creates a new State instance.
        pub const fn new(v: &'static str) -> Self {
            Self(std::borrow::Cow::Borrowed(v))
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [State](State)
    pub mod state {
        use super::State;

        /// Not set.
        pub const STATE_UNSPECIFIED: State = State::new("STATE_UNSPECIFIED");

        /// Being created.
        pub const CREATING: State = State::new("CREATING");

        /// Active and ready for traffic.
        pub const READY: State = State::new("READY");

        /// Being deleted.
        pub const DELETING: State = State::new("DELETING");
    }

    impl std::convert::From<std::string::String> for State {
        fn from(value: std::string::String) -> Self {
            Self(std::borrow::Cow::Owned(value))
        }
    }

    impl std::default::Default for State {
        fn default() -> Self {
            state::STATE_UNSPECIFIED
        }
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListEndpointsRequest {
    /// Required. The parent, which owns this collection of endpoints.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Optional. The maximum number of endpoints to return. The service may return fewer
    /// than this value.
    pub page_size: i32,

    /// Optional. A page token, received from a previous `ListEndpoints` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListEndpoints` must
    /// match the call that provided the page token.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub page_token: std::string::String,

    /// Optional. The filter expression, following the syntax outlined in
    /// <https://google.aip.dev/160>.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub filter: std::string::String,

    /// Optional. One or more fields to compare and use to sort the output.
    /// See <https://google.aip.dev/132#ordering>.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub order_by: std::string::String,
}

impl ListEndpointsRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::ListEndpointsRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [page_size][crate::model::ListEndpointsRequest::page_size].
    pub fn set_page_size<T: std::convert::Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of [page_token][crate::model::ListEndpointsRequest::page_token].
    pub fn set_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of [filter][crate::model::ListEndpointsRequest::filter].
    pub fn set_filter<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }

    /// Sets the value of [order_by][crate::model::ListEndpointsRequest::order_by].
    pub fn set_order_by<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.order_by = v.into();
        self
    }
}

impl wkt::message::Message for ListEndpointsRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.ids.v1.ListEndpointsRequest"
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListEndpointsResponse {
    /// The list of endpoints response.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub endpoints: std::vec::Vec<crate::model::Endpoint>,

    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub next_page_token: std::string::String,

    /// Locations that could not be reached.
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    pub unreachable: std::vec::Vec<std::string::String>,
}

impl ListEndpointsResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [next_page_token][crate::model::ListEndpointsResponse::next_page_token].
    pub fn set_next_page_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of [endpoints][crate::model::ListEndpointsResponse::endpoints].
    pub fn set_endpoints<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<crate::model::Endpoint>,
    {
        use std::iter::Iterator;
        self.endpoints = v.into_iter().map(|i| i.into()).collect();
        self
    }

    /// Sets the value of [unreachable][crate::model::ListEndpointsResponse::unreachable].
    pub fn set_unreachable<T, V>(mut self, v: T) -> Self
    where
        T: std::iter::IntoIterator<Item = V>,
        V: std::convert::Into<std::string::String>,
    {
        use std::iter::Iterator;
        self.unreachable = v.into_iter().map(|i| i.into()).collect();
        self
    }
}

impl wkt::message::Message for ListEndpointsResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.ids.v1.ListEndpointsResponse"
    }
}

#[cfg(feature = "unstable-stream")]
impl gax::paginator::PageableResponse for ListEndpointsResponse {
    type PageItem = crate::model::Endpoint;

    fn items(self) -> std::vec::Vec<Self::PageItem> {
        self.endpoints
    }

    fn next_page_token(&self) -> std::string::String {
        gax::paginator::extract_token(&self.next_page_token)
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetEndpointRequest {
    /// Required. The name of the endpoint to retrieve.
    /// Format: `projects/{project}/locations/{location}/endpoints/{endpoint}`
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,
}

impl GetEndpointRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::GetEndpointRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

impl wkt::message::Message for GetEndpointRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.ids.v1.GetEndpointRequest"
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateEndpointRequest {
    /// Required. The endpoint's parent.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub parent: std::string::String,

    /// Required. The endpoint identifier. This will be part of the endpoint's
    /// resource name.
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    /// Values that do not match this pattern will trigger an INVALID_ARGUMENT
    /// error.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub endpoint_id: std::string::String,

    /// Required. The endpoint to create.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub endpoint: std::option::Option<crate::model::Endpoint>,

    /// An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub request_id: std::string::String,
}

impl CreateEndpointRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [parent][crate::model::CreateEndpointRequest::parent].
    pub fn set_parent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of [endpoint_id][crate::model::CreateEndpointRequest::endpoint_id].
    pub fn set_endpoint_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.endpoint_id = v.into();
        self
    }

    /// Sets the value of [endpoint][crate::model::CreateEndpointRequest::endpoint].
    pub fn set_endpoint<T: std::convert::Into<std::option::Option<crate::model::Endpoint>>>(
        mut self,
        v: T,
    ) -> Self {
        self.endpoint = v.into();
        self
    }

    /// Sets the value of [request_id][crate::model::CreateEndpointRequest::request_id].
    pub fn set_request_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.request_id = v.into();
        self
    }
}

impl wkt::message::Message for CreateEndpointRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.ids.v1.CreateEndpointRequest"
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteEndpointRequest {
    /// Required. The name of the endpoint to delete.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub name: std::string::String,

    /// An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub request_id: std::string::String,
}

impl DeleteEndpointRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [name][crate::model::DeleteEndpointRequest::name].
    pub fn set_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of [request_id][crate::model::DeleteEndpointRequest::request_id].
    pub fn set_request_id<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.request_id = v.into();
        self
    }
}

impl wkt::message::Message for DeleteEndpointRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.ids.v1.DeleteEndpointRequest"
    }
}

/// Represents the metadata of the long-running operation.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub create_time: std::option::Option<wkt::Timestamp>,

    /// Output only. The time the operation finished running.
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub end_time: std::option::Option<wkt::Timestamp>,

    /// Output only. Server-defined resource path for the target of the operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub target: std::string::String,

    /// Output only. Name of the verb executed by the operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub verb: std::string::String,

    /// Output only. Human-readable status of the operation, if any.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub status_message: std::string::String,

    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,
    /// corresponding to `Code.CANCELLED`.
    ///
    /// [google.rpc.Status.code]: rpc::model::Status::code
    pub requested_cancellation: bool,

    /// Output only. API version used to start the operation.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub api_version: std::string::String,
}

impl OperationMetadata {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [create_time][crate::model::OperationMetadata::create_time].
    pub fn set_create_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of [end_time][crate::model::OperationMetadata::end_time].
    pub fn set_end_time<T: std::convert::Into<std::option::Option<wkt::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.end_time = v.into();
        self
    }

    /// Sets the value of [target][crate::model::OperationMetadata::target].
    pub fn set_target<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.target = v.into();
        self
    }

    /// Sets the value of [verb][crate::model::OperationMetadata::verb].
    pub fn set_verb<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.verb = v.into();
        self
    }

    /// Sets the value of [status_message][crate::model::OperationMetadata::status_message].
    pub fn set_status_message<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.status_message = v.into();
        self
    }

    /// Sets the value of [requested_cancellation][crate::model::OperationMetadata::requested_cancellation].
    pub fn set_requested_cancellation<T: std::convert::Into<bool>>(mut self, v: T) -> Self {
        self.requested_cancellation = v.into();
        self
    }

    /// Sets the value of [api_version][crate::model::OperationMetadata::api_version].
    pub fn set_api_version<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.api_version = v.into();
        self
    }
}

impl wkt::message::Message for OperationMetadata {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.ids.v1.OperationMetadata"
    }
}
