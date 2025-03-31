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

/// Implements a client for the Cloud Datastore API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_datastore_admin_v1::client::DatastoreAdmin;
/// let client = DatastoreAdmin::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Google Cloud Datastore Admin API
///
/// The Datastore Admin API provides several admin services for Cloud Datastore.
///
/// Concepts: Project, namespace, kind, and entity as defined in the Google Cloud
/// Datastore API.
///
/// Operation: An Operation represents work being performed in the background.
///
/// EntityFilter: Allows specifying a subset of entities in a project. This is
/// specified as a combination of kinds and namespaces (either or both of which
/// may be all).
///
/// Export/Import Service:
///
/// - The Export/Import service provides the ability to copy all or a subset of
///   entities to/from Google Cloud Storage.
/// - Exported data may be imported into Cloud Datastore for any Google Cloud
///   Platform project. It is not restricted to the export source project. It is
///   possible to export from one project and then import into another.
/// - Exported data can also be loaded into Google BigQuery for analysis.
/// - Exports and imports are performed asynchronously. An Operation resource is
///   created for each export/import. The state (including any errors encountered)
///   of the export/import may be queried via the Operation resource.
///
/// Index Service:
///
/// - The index service manages Cloud Datastore composite indexes.
/// - Index creation and deletion are performed asynchronously.
///   An Operation resource is created for each such asynchronous operation.
///   The state of the operation (including any errors encountered)
///   may be queried via the Operation resource.
///
/// Operation Service:
///
/// - The Operations collection provides a record of actions performed for the
///   specified project (including any operations in progress). Operations are not
///   created directly but through calls on other collections or resources.
/// - An operation that is not yet done may be cancelled. The request to cancel
///   is asynchronous and the operation may continue to run for some time after the
///   request to cancel is made.
/// - An operation that is done may be deleted so that it is no longer listed as
///   part of the Operation collection.
/// - ListOperations returns all pending operations, but not completed
///   operations.
/// - Operations are created by service DatastoreAdmin, but are accessed via
///   service google.longrunning.Operations.
///
/// # Configuration
///
/// To configure `DatastoreAdmin` use the `with_*` methods in the type returned
/// by [builder()][DatastoreAdmin::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://datastore.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::datastore_admin::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::datastore_admin::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `DatastoreAdmin` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `DatastoreAdmin` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct DatastoreAdmin {
    inner: Arc<dyn super::stub::dynamic::DatastoreAdmin>,
}

impl DatastoreAdmin {
    /// Returns a builder for [DatastoreAdmin].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_datastore_admin_v1::client::DatastoreAdmin;
    /// let client = DatastoreAdmin::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::datastore_admin::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::datastore_admin::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::DatastoreAdmin + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::DatastoreAdmin>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::DatastoreAdmin> {
        super::transport::DatastoreAdmin::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::DatastoreAdmin> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::DatastoreAdmin::new)
    }

    /// Exports a copy of all or a subset of entities from Google Cloud Datastore
    /// to another storage system, such as Google Cloud Storage. Recent updates to
    /// entities may not be reflected in the export. The export occurs in the
    /// background and its progress can be monitored and managed via the
    /// Operation resource that is created. The output of an export may only be
    /// used once the associated operation is done. If an export operation is
    /// cancelled before completion it may leave partial data behind in Google
    /// Cloud Storage.
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
    pub fn export_entities(
        &self,
        project_id: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::ExportEntities {
        super::builder::datastore_admin::ExportEntities::new(self.inner.clone())
            .set_project_id(project_id.into())
    }

    /// Imports entities into Google Cloud Datastore. Existing entities with the
    /// same key are overwritten. The import occurs in the background and its
    /// progress can be monitored and managed via the Operation resource that is
    /// created. If an ImportEntities operation is cancelled, it is possible
    /// that a subset of the data has already been imported to Cloud Datastore.
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
    pub fn import_entities(
        &self,
        project_id: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::ImportEntities {
        super::builder::datastore_admin::ImportEntities::new(self.inner.clone())
            .set_project_id(project_id.into())
    }

    /// Creates the specified index.
    /// A newly created index's initial state is `CREATING`. On completion of the
    /// returned [google.longrunning.Operation][google.longrunning.Operation], the
    /// state will be `READY`. If the index already exists, the call will return an
    /// `ALREADY_EXISTS` status.
    ///
    /// During index creation, the process could result in an error, in which
    /// case the index will move to the `ERROR` state. The process can be recovered
    /// by fixing the data that caused the error, removing the index with
    /// [delete][google.datastore.admin.v1.DatastoreAdmin.DeleteIndex], then
    /// re-creating the index with [create]
    /// [google.datastore.admin.v1.DatastoreAdmin.CreateIndex].
    ///
    /// Indexes with a single property cannot be created.
    ///
    /// [google.datastore.admin.v1.DatastoreAdmin.DeleteIndex]: crate::client::DatastoreAdmin::delete_index
    /// [google.longrunning.Operation]: longrunning::model::Operation
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
    pub fn create_index(
        &self,
        project_id: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::CreateIndex {
        super::builder::datastore_admin::CreateIndex::new(self.inner.clone())
            .set_project_id(project_id.into())
    }

    /// Deletes an existing index.
    /// An index can only be deleted if it is in a `READY` or `ERROR` state. On
    /// successful execution of the request, the index will be in a `DELETING`
    /// [state][google.datastore.admin.v1.Index.State]. And on completion of the
    /// returned [google.longrunning.Operation][google.longrunning.Operation], the
    /// index will be removed.
    ///
    /// During index deletion, the process could result in an error, in which
    /// case the index will move to the `ERROR` state. The process can be recovered
    /// by fixing the data that caused the error, followed by calling
    /// [delete][google.datastore.admin.v1.DatastoreAdmin.DeleteIndex] again.
    ///
    /// [google.datastore.admin.v1.DatastoreAdmin.DeleteIndex]: crate::client::DatastoreAdmin::delete_index
    /// [google.datastore.admin.v1.Index.State]: crate::model::index::State
    /// [google.longrunning.Operation]: longrunning::model::Operation
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
    pub fn delete_index(
        &self,
        project_id: impl Into<std::string::String>,
        index_id: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::DeleteIndex {
        super::builder::datastore_admin::DeleteIndex::new(self.inner.clone())
            .set_project_id(project_id.into())
            .set_index_id(index_id.into())
    }

    /// Gets an index.
    pub fn get_index(
        &self,
        project_id: impl Into<std::string::String>,
        index_id: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::GetIndex {
        super::builder::datastore_admin::GetIndex::new(self.inner.clone())
            .set_project_id(project_id.into())
            .set_index_id(index_id.into())
    }

    /// Lists the indexes that match the specified filters.  Datastore uses an
    /// eventually consistent query to fetch the list of indexes and may
    /// occasionally return stale results.
    pub fn list_indexes(
        &self,
        project_id: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::ListIndexes {
        super::builder::datastore_admin::ListIndexes::new(self.inner.clone())
            .set_project_id(project_id.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::ListOperations {
        super::builder::datastore_admin::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::GetOperation {
        super::builder::datastore_admin::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::DeleteOperation {
        super::builder::datastore_admin::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::datastore_admin::CancelOperation {
        super::builder::datastore_admin::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
