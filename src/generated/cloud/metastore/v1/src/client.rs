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

/// Implements a client for the Dataproc Metastore API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_metastore_v1::client::DataprocMetastore;
/// let client = DataprocMetastore::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Configures and manages metastore services.
/// Metastore services are fully managed, highly available, autoscaled,
/// autohealing, OSS-native deployments of technical metadata management
/// software. Each metastore service exposes a network endpoint through which
/// metadata queries are served. Metadata queries can originate from a variety
/// of sources, including Apache Hive, Apache Presto, and Apache Spark.
///
/// The Dataproc Metastore API defines the following resource model:
///
/// * The service works with a collection of Google Cloud projects, named:
///   `/projects/*`
///
/// * Each project has a collection of available locations, named: `/locations/*`
///   (a location must refer to a Google Cloud `region`)
///
/// * Each location has a collection of services, named: `/services/*`
///
/// * Dataproc Metastore services are resources with names of the form:
///
/// * `/projects/{project_number}/locations/{location_id}/services/{service_id}`.
///
///
/// # Configuration
///
/// To configure `DataprocMetastore` use the `with_*` methods in the type returned
/// by [builder()][DataprocMetastore::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://metastore.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::dataproc_metastore::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::dataproc_metastore::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `DataprocMetastore` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `DataprocMetastore` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct DataprocMetastore {
    inner: Arc<dyn super::stub::dynamic::DataprocMetastore>,
}

impl DataprocMetastore {
    /// Returns a builder for [DataprocMetastore].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_metastore_v1::client::DataprocMetastore;
    /// let client = DataprocMetastore::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::dataproc_metastore::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::dataproc_metastore::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::DataprocMetastore + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::DataprocMetastore>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::DataprocMetastore> {
        super::transport::DataprocMetastore::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::DataprocMetastore> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::DataprocMetastore::new)
    }

    /// Lists services in a project and location.
    pub fn list_services(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::ListServices {
        super::builder::dataproc_metastore::ListServices::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a single service.
    pub fn get_service(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::GetService {
        super::builder::dataproc_metastore::GetService::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a metastore service in a project and location.
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
    pub fn create_service(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::CreateService {
        super::builder::dataproc_metastore::CreateService::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single service.
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
    pub fn update_service(
        &self,
        service: impl Into<crate::model::Service>,
    ) -> super::builder::dataproc_metastore::UpdateService {
        super::builder::dataproc_metastore::UpdateService::new(self.inner.clone())
            .set_service(service.into())
    }

    /// Deletes a single service.
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
    pub fn delete_service(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::DeleteService {
        super::builder::dataproc_metastore::DeleteService::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists imports in a service.
    pub fn list_metadata_imports(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::ListMetadataImports {
        super::builder::dataproc_metastore::ListMetadataImports::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single import.
    pub fn get_metadata_import(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::GetMetadataImport {
        super::builder::dataproc_metastore::GetMetadataImport::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new MetadataImport in a given project and location.
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
    pub fn create_metadata_import(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::CreateMetadataImport {
        super::builder::dataproc_metastore::CreateMetadataImport::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a single import.
    /// Only the description field of MetadataImport is supported to be updated.
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
    pub fn update_metadata_import(
        &self,
        metadata_import: impl Into<crate::model::MetadataImport>,
    ) -> super::builder::dataproc_metastore::UpdateMetadataImport {
        super::builder::dataproc_metastore::UpdateMetadataImport::new(self.inner.clone())
            .set_metadata_import(metadata_import.into())
    }

    /// Exports metadata from a service.
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
    pub fn export_metadata(
        &self,
        service: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::ExportMetadata {
        super::builder::dataproc_metastore::ExportMetadata::new(self.inner.clone())
            .set_service(service.into())
    }

    /// Restores a service from a backup.
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
    pub fn restore_service(
        &self,
        service: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::RestoreService {
        super::builder::dataproc_metastore::RestoreService::new(self.inner.clone())
            .set_service(service.into())
    }

    /// Lists backups in a service.
    pub fn list_backups(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::ListBackups {
        super::builder::dataproc_metastore::ListBackups::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single backup.
    pub fn get_backup(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::GetBackup {
        super::builder::dataproc_metastore::GetBackup::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new backup in a given project and location.
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
    pub fn create_backup(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::CreateBackup {
        super::builder::dataproc_metastore::CreateBackup::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a single backup.
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
    pub fn delete_backup(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::DeleteBackup {
        super::builder::dataproc_metastore::DeleteBackup::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Query DPMS metadata.
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
    pub fn query_metadata(
        &self,
        service: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::QueryMetadata {
        super::builder::dataproc_metastore::QueryMetadata::new(self.inner.clone())
            .set_service(service.into())
    }

    /// Move a table to another database.
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
    pub fn move_table_to_database(
        &self,
        service: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::MoveTableToDatabase {
        super::builder::dataproc_metastore::MoveTableToDatabase::new(self.inner.clone())
            .set_service(service.into())
    }

    /// Alter metadata resource location. The metadata resource can be a database,
    /// table, or partition. This functionality only updates the parent directory
    /// for the respective metadata resource and does not transfer any existing
    /// data to the new location.
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
    pub fn alter_metadata_resource_location(
        &self,
        service: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::AlterMetadataResourceLocation {
        super::builder::dataproc_metastore::AlterMetadataResourceLocation::new(self.inner.clone())
            .set_service(service.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::ListLocations {
        super::builder::dataproc_metastore::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::GetLocation {
        super::builder::dataproc_metastore::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::SetIamPolicy {
        super::builder::dataproc_metastore::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::GetIamPolicy {
        super::builder::dataproc_metastore::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::TestIamPermissions {
        super::builder::dataproc_metastore::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::ListOperations {
        super::builder::dataproc_metastore::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::GetOperation {
        super::builder::dataproc_metastore::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::DeleteOperation {
        super::builder::dataproc_metastore::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore::CancelOperation {
        super::builder::dataproc_metastore::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}

/// Implements a client for the Dataproc Metastore API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_metastore_v1::client::DataprocMetastoreFederation;
/// let client = DataprocMetastoreFederation::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Configures and manages metastore federation services.
/// Dataproc Metastore Federation Service allows federating a collection of
/// backend metastores like BigQuery, Dataplex Lakes, and other Dataproc
/// Metastores. The Federation Service exposes a gRPC URL through which metadata
/// from the backend metastores are served at query time.
///
/// The Dataproc Metastore Federation API defines the following resource model:
///
/// * The service works with a collection of Google Cloud projects.
/// * Each project has a collection of available locations.
/// * Each location has a collection of federations.
/// * Dataproc Metastore Federations are resources with names of the
///   form:
///   `projects/{project_number}/locations/{location_id}/federations/{federation_id}`.
///
/// # Configuration
///
/// To configure `DataprocMetastoreFederation` use the `with_*` methods in the type returned
/// by [builder()][DataprocMetastoreFederation::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://metastore.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::dataproc_metastore_federation::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::dataproc_metastore_federation::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `DataprocMetastoreFederation` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `DataprocMetastoreFederation` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct DataprocMetastoreFederation {
    inner: Arc<dyn super::stub::dynamic::DataprocMetastoreFederation>,
}

impl DataprocMetastoreFederation {
    /// Returns a builder for [DataprocMetastoreFederation].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_metastore_v1::client::DataprocMetastoreFederation;
    /// let client = DataprocMetastoreFederation::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::dataproc_metastore_federation::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::dataproc_metastore_federation::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::DataprocMetastoreFederation + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::DataprocMetastoreFederation>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::DataprocMetastoreFederation> {
        super::transport::DataprocMetastoreFederation::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::DataprocMetastoreFederation> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::DataprocMetastoreFederation::new)
    }

    /// Lists federations in a project and location.
    pub fn list_federations(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::ListFederations {
        super::builder::dataproc_metastore_federation::ListFederations::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a single federation.
    pub fn get_federation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::GetFederation {
        super::builder::dataproc_metastore_federation::GetFederation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a metastore federation in a project and location.
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
    pub fn create_federation(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::CreateFederation {
        super::builder::dataproc_metastore_federation::CreateFederation::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the fields of a federation.
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
    pub fn update_federation(
        &self,
        federation: impl Into<crate::model::Federation>,
    ) -> super::builder::dataproc_metastore_federation::UpdateFederation {
        super::builder::dataproc_metastore_federation::UpdateFederation::new(self.inner.clone())
            .set_federation(federation.into())
    }

    /// Deletes a single federation.
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
    pub fn delete_federation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::DeleteFederation {
        super::builder::dataproc_metastore_federation::DeleteFederation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::ListLocations {
        super::builder::dataproc_metastore_federation::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::GetLocation {
        super::builder::dataproc_metastore_federation::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::SetIamPolicy {
        super::builder::dataproc_metastore_federation::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::GetIamPolicy {
        super::builder::dataproc_metastore_federation::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::TestIamPermissions {
        super::builder::dataproc_metastore_federation::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::ListOperations {
        super::builder::dataproc_metastore_federation::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::GetOperation {
        super::builder::dataproc_metastore_federation::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::DeleteOperation {
        super::builder::dataproc_metastore_federation::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::dataproc_metastore_federation::CancelOperation {
        super::builder::dataproc_metastore_federation::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
