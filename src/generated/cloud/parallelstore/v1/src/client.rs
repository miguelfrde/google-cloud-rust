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

/// Implements a client for the Parallelstore API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_parallelstore_v1::client::Parallelstore;
/// let client = Parallelstore::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service describing handlers for resources
/// Configures and manages parallelstore resources.
///
/// Parallelstore service.
///
/// The `parallelstore.googleapis.com` service implements the parallelstore API
/// and defines the following resource model for managing instances:
///
/// * The service works with a collection of cloud projects, named: `/projects/*`
/// * Each project has a collection of available locations, named: `/locations/*`
/// * Each location has a collection of instances named `/instances/*`.
/// * Parallelstore instances are resources of the form:
///   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`
///
/// Note that location_id must be a Google Cloud `zone`; for example:
///
/// * `projects/12345/locations/us-central1-c/instances/my-parallelstore-share`
///
/// # Configuration
///
/// To configure `Parallelstore` use the `with_*` methods in the type returned
/// by [builder()][Parallelstore::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://parallelstore.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::parallelstore::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::parallelstore::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `Parallelstore` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Parallelstore` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Parallelstore {
    inner: Arc<dyn super::stub::dynamic::Parallelstore>,
}

impl Parallelstore {
    /// Returns a builder for [Parallelstore].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_parallelstore_v1::client::Parallelstore;
    /// let client = Parallelstore::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::parallelstore::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::parallelstore::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::Parallelstore + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::Parallelstore>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::Parallelstore> {
        super::transport::Parallelstore::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::Parallelstore> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Parallelstore::new)
    }

    /// Lists all instances in a given project and location.
    pub fn list_instances(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::ListInstances {
        super::builder::parallelstore::ListInstances::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single instance.
    pub fn get_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::GetInstance {
        super::builder::parallelstore::GetInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a Parallelstore instance in a given project and location.
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
    pub fn create_instance(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::CreateInstance {
        super::builder::parallelstore::CreateInstance::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single instance.
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
    pub fn update_instance(
        &self,
        instance: impl Into<crate::model::Instance>,
    ) -> super::builder::parallelstore::UpdateInstance {
        super::builder::parallelstore::UpdateInstance::new(self.inner.clone())
            .set_instance(instance.into())
    }

    /// Deletes a single instance.
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
    pub fn delete_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::DeleteInstance {
        super::builder::parallelstore::DeleteInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Copies data from Cloud Storage to Parallelstore.
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
    pub fn import_data(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::ImportData {
        super::builder::parallelstore::ImportData::new(self.inner.clone()).set_name(name.into())
    }

    /// Copies data from Parallelstore to Cloud Storage.
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
    pub fn export_data(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::ExportData {
        super::builder::parallelstore::ExportData::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::ListLocations {
        super::builder::parallelstore::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::GetLocation {
        super::builder::parallelstore::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::ListOperations {
        super::builder::parallelstore::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::GetOperation {
        super::builder::parallelstore::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::DeleteOperation {
        super::builder::parallelstore::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::parallelstore::CancelOperation {
        super::builder::parallelstore::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
