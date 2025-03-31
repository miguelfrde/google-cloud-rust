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

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Datastream API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_datastream_v1::client::Datastream;
/// let client = Datastream::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Datastream service
///
/// # Configuration
///
/// To configure `Datastream` use the `with_*` methods in the type returned
/// by [builder()][Datastream::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://datastream.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::datastream::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::datastream::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `Datastream` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Datastream` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Datastream {
    inner: Arc<dyn super::stub::dynamic::Datastream>,
}

impl Datastream {
    /// Returns a builder for [Datastream].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_datastream_v1::client::Datastream;
    /// let client = Datastream::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::datastream::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::datastream::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::Datastream + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    pub(crate) async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(config).await?;
        Ok(Self { inner })
    }

    async fn build_inner(
        conf: gaxi::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::Datastream>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::Datastream> {
        super::transport::Datastream::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::Datastream> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Datastream::new)
    }

    /// Use this method to list connection profiles created in a project and
    /// location.
    pub fn list_connection_profiles(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::ListConnectionProfiles {
        super::builder::datastream::ListConnectionProfiles::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Use this method to get details about a connection profile.
    pub fn get_connection_profile(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::GetConnectionProfile {
        super::builder::datastream::GetConnectionProfile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Use this method to create a connection profile in a project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_connection_profile(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::CreateConnectionProfile {
        super::builder::datastream::CreateConnectionProfile::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Use this method to update the parameters of a connection profile.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_connection_profile(
        &self,
        connection_profile: impl Into<crate::model::ConnectionProfile>,
    ) -> super::builder::datastream::UpdateConnectionProfile {
        super::builder::datastream::UpdateConnectionProfile::new(self.inner.clone())
            .set_connection_profile(connection_profile.into())
    }

    /// Use this method to delete a connection profile.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_connection_profile(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::DeleteConnectionProfile {
        super::builder::datastream::DeleteConnectionProfile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Use this method to discover a connection profile.
    /// The discover API call exposes the data objects and metadata belonging to
    /// the profile. Typically, a request returns children data objects of a
    /// parent data object that's optionally supplied in the request.
    pub fn discover_connection_profile(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::DiscoverConnectionProfile {
        super::builder::datastream::DiscoverConnectionProfile::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Use this method to list streams in a project and location.
    pub fn list_streams(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::ListStreams {
        super::builder::datastream::ListStreams::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Use this method to get details about a stream.
    pub fn get_stream(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::GetStream {
        super::builder::datastream::GetStream::new(self.inner.clone()).set_name(name.into())
    }

    /// Use this method to create a stream.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_stream(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::CreateStream {
        super::builder::datastream::CreateStream::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Use this method to update the configuration of a stream.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_stream(
        &self,
        stream: impl Into<crate::model::Stream>,
    ) -> super::builder::datastream::UpdateStream {
        super::builder::datastream::UpdateStream::new(self.inner.clone()).set_stream(stream.into())
    }

    /// Use this method to delete a stream.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_stream(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::DeleteStream {
        super::builder::datastream::DeleteStream::new(self.inner.clone()).set_name(name.into())
    }

    /// Use this method to start, resume or recover a stream with a non default CDC
    /// strategy.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn run_stream(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::RunStream {
        super::builder::datastream::RunStream::new(self.inner.clone()).set_name(name.into())
    }

    /// Use this method to get details about a stream object.
    pub fn get_stream_object(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::GetStreamObject {
        super::builder::datastream::GetStreamObject::new(self.inner.clone()).set_name(name.into())
    }

    /// Use this method to look up a stream object by its source object identifier.
    pub fn lookup_stream_object(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::LookupStreamObject {
        super::builder::datastream::LookupStreamObject::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Use this method to list the objects of a specific stream.
    pub fn list_stream_objects(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::ListStreamObjects {
        super::builder::datastream::ListStreamObjects::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Use this method to start a backfill job for the specified stream object.
    pub fn start_backfill_job(
        &self,
        object: impl Into<std::string::String>,
    ) -> super::builder::datastream::StartBackfillJob {
        super::builder::datastream::StartBackfillJob::new(self.inner.clone())
            .set_object(object.into())
    }

    /// Use this method to stop a backfill job for the specified stream object.
    pub fn stop_backfill_job(
        &self,
        object: impl Into<std::string::String>,
    ) -> super::builder::datastream::StopBackfillJob {
        super::builder::datastream::StopBackfillJob::new(self.inner.clone())
            .set_object(object.into())
    }

    /// The FetchStaticIps API call exposes the static IP addresses used by
    /// Datastream.
    pub fn fetch_static_ips(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::FetchStaticIps {
        super::builder::datastream::FetchStaticIps::new(self.inner.clone()).set_name(name.into())
    }

    /// Use this method to create a private connectivity configuration.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_private_connection(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::CreatePrivateConnection {
        super::builder::datastream::CreatePrivateConnection::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Use this method to get details about a private connectivity configuration.
    pub fn get_private_connection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::GetPrivateConnection {
        super::builder::datastream::GetPrivateConnection::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Use this method to list private connectivity configurations in a project
    /// and location.
    pub fn list_private_connections(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::ListPrivateConnections {
        super::builder::datastream::ListPrivateConnections::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Use this method to delete a private connectivity configuration.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_private_connection(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::DeletePrivateConnection {
        super::builder::datastream::DeletePrivateConnection::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Use this method to create a route for a private connectivity configuration
    /// in a project and location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_route(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::CreateRoute {
        super::builder::datastream::CreateRoute::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Use this method to get details about a route.
    pub fn get_route(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::GetRoute {
        super::builder::datastream::GetRoute::new(self.inner.clone()).set_name(name.into())
    }

    /// Use this method to list routes created for a private connectivity
    /// configuration in a project and location.
    pub fn list_routes(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::datastream::ListRoutes {
        super::builder::datastream::ListRoutes::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Use this method to delete a route.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_route(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::DeleteRoute {
        super::builder::datastream::DeleteRoute::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::ListLocations {
        super::builder::datastream::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::GetLocation {
        super::builder::datastream::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::ListOperations {
        super::builder::datastream::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::GetOperation {
        super::builder::datastream::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::DeleteOperation {
        super::builder::datastream::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastream::CancelOperation {
        super::builder::datastream::CancelOperation::new(self.inner.clone()).set_name(name.into())
    }
}
