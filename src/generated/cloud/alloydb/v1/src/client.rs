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

/// Implements a client for the AlloyDB API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_alloydb_v1::client::AlloyDBAdmin;
/// let client = AlloyDBAdmin::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service describing handlers for resources
///
/// # Configuration
///
/// To configure `AlloyDBAdmin` use the `with_*` methods in the type returned
/// by [builder()][AlloyDBAdmin::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://alloydb.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::alloy_db_admin::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::alloy_db_admin::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `AlloyDBAdmin` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AlloyDBAdmin` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct AlloyDBAdmin {
    inner: Arc<dyn super::stub::dynamic::AlloyDBAdmin>,
}

impl AlloyDBAdmin {
    /// Returns a builder for [AlloyDBAdmin].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_alloydb_v1::client::AlloyDBAdmin;
    /// let client = AlloyDBAdmin::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::alloy_db_admin::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::alloy_db_admin::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::AlloyDBAdmin + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::AlloyDBAdmin>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::AlloyDBAdmin> {
        super::transport::AlloyDBAdmin::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::AlloyDBAdmin> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::AlloyDBAdmin::new)
    }

    /// Lists Clusters in a given project and location.
    pub fn list_clusters(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::ListClusters {
        super::builder::alloy_db_admin::ListClusters::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Cluster.
    pub fn get_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::GetCluster {
        super::builder::alloy_db_admin::GetCluster::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new Cluster in a given project and location.
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
    pub fn create_cluster(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::CreateCluster {
        super::builder::alloy_db_admin::CreateCluster::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single Cluster.
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
    pub fn update_cluster(
        &self,
        cluster: impl Into<crate::model::Cluster>,
    ) -> super::builder::alloy_db_admin::UpdateCluster {
        super::builder::alloy_db_admin::UpdateCluster::new(self.inner.clone())
            .set_cluster(cluster.into())
    }

    /// Deletes a single Cluster.
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
    pub fn delete_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::DeleteCluster {
        super::builder::alloy_db_admin::DeleteCluster::new(self.inner.clone()).set_name(name.into())
    }

    /// Promotes a SECONDARY cluster. This turns down replication
    /// from the PRIMARY cluster and promotes a secondary cluster
    /// into its own standalone cluster.
    /// Imperative only.
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
    pub fn promote_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::PromoteCluster {
        super::builder::alloy_db_admin::PromoteCluster::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Switches the roles of PRIMARY and SECONDARY clusters without any data loss.
    /// This promotes the SECONDARY cluster to PRIMARY and sets up the original
    /// PRIMARY cluster to replicate from this newly promoted cluster.
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
    pub fn switchover_cluster(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::SwitchoverCluster {
        super::builder::alloy_db_admin::SwitchoverCluster::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new Cluster in a given project and location, with a volume
    /// restored from the provided source, either a backup ID or a point-in-time
    /// and a source cluster.
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
    pub fn restore_cluster(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::RestoreCluster {
        super::builder::alloy_db_admin::RestoreCluster::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a cluster of type SECONDARY in the given location using
    /// the primary cluster as the source.
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
    pub fn create_secondary_cluster(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::CreateSecondaryCluster {
        super::builder::alloy_db_admin::CreateSecondaryCluster::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists Instances in a given project and location.
    pub fn list_instances(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::ListInstances {
        super::builder::alloy_db_admin::ListInstances::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Instance.
    pub fn get_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::GetInstance {
        super::builder::alloy_db_admin::GetInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new Instance in a given project and location.
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
    ) -> super::builder::alloy_db_admin::CreateInstance {
        super::builder::alloy_db_admin::CreateInstance::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a new SECONDARY Instance in a given project and location.
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
    pub fn create_secondary_instance(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::CreateSecondaryInstance {
        super::builder::alloy_db_admin::CreateSecondaryInstance::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates new instances under the given project, location and cluster.
    /// There can be only one primary instance in a cluster. If the primary
    /// instance exists in the cluster as well as this request, then API will
    /// throw an error.
    /// The primary instance should exist before any read pool instance is
    /// created. If the primary instance is a part of the request payload, then
    /// the API will take care of creating instances in the correct order.
    /// This method is here to support Google-internal use cases, and is not meant
    /// for external customers to consume. Please do not start relying on it; its
    /// behavior is subject to change without notice.
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
    pub fn batch_create_instances(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::BatchCreateInstances {
        super::builder::alloy_db_admin::BatchCreateInstances::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single Instance.
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
    ) -> super::builder::alloy_db_admin::UpdateInstance {
        super::builder::alloy_db_admin::UpdateInstance::new(self.inner.clone())
            .set_instance(instance.into())
    }

    /// Deletes a single Instance.
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
    ) -> super::builder::alloy_db_admin::DeleteInstance {
        super::builder::alloy_db_admin::DeleteInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Forces a Failover for a highly available instance.
    /// Failover promotes the HA standby instance as the new primary.
    /// Imperative only.
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
    pub fn failover_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::FailoverInstance {
        super::builder::alloy_db_admin::FailoverInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Injects fault in an instance.
    /// Imperative only.
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
    pub fn inject_fault(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::InjectFault {
        super::builder::alloy_db_admin::InjectFault::new(self.inner.clone()).set_name(name.into())
    }

    /// Restart an Instance in a cluster.
    /// Imperative only.
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
    pub fn restart_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::RestartInstance {
        super::builder::alloy_db_admin::RestartInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Executes a SQL statement in a database inside an AlloyDB instance.
    pub fn execute_sql(
        &self,
        instance: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::ExecuteSql {
        super::builder::alloy_db_admin::ExecuteSql::new(self.inner.clone())
            .set_instance(instance.into())
    }

    /// Lists Backups in a given project and location.
    pub fn list_backups(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::ListBackups {
        super::builder::alloy_db_admin::ListBackups::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Backup.
    pub fn get_backup(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::GetBackup {
        super::builder::alloy_db_admin::GetBackup::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new Backup in a given project and location.
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
    ) -> super::builder::alloy_db_admin::CreateBackup {
        super::builder::alloy_db_admin::CreateBackup::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single Backup.
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
    pub fn update_backup(
        &self,
        backup: impl Into<crate::model::Backup>,
    ) -> super::builder::alloy_db_admin::UpdateBackup {
        super::builder::alloy_db_admin::UpdateBackup::new(self.inner.clone())
            .set_backup(backup.into())
    }

    /// Deletes a single Backup.
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
    ) -> super::builder::alloy_db_admin::DeleteBackup {
        super::builder::alloy_db_admin::DeleteBackup::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists SupportedDatabaseFlags for a given project and location.
    pub fn list_supported_database_flags(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::ListSupportedDatabaseFlags {
        super::builder::alloy_db_admin::ListSupportedDatabaseFlags::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Generate a client certificate signed by a Cluster CA.
    /// The sole purpose of this endpoint is to support AlloyDB connectors and the
    /// Auth Proxy client. The endpoint's behavior is subject to change without
    /// notice, so do not rely on its behavior remaining constant. Future changes
    /// will not break AlloyDB connectors or the Auth Proxy client.
    pub fn generate_client_certificate(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::GenerateClientCertificate {
        super::builder::alloy_db_admin::GenerateClientCertificate::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get instance metadata used for a connection.
    pub fn get_connection_info(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::GetConnectionInfo {
        super::builder::alloy_db_admin::GetConnectionInfo::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists Users in a given project and location.
    pub fn list_users(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::ListUsers {
        super::builder::alloy_db_admin::ListUsers::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details of a single User.
    pub fn get_user(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::GetUser {
        super::builder::alloy_db_admin::GetUser::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new User in a given project, location, and cluster.
    pub fn create_user(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::CreateUser {
        super::builder::alloy_db_admin::CreateUser::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single User.
    pub fn update_user(
        &self,
        user: impl Into<crate::model::User>,
    ) -> super::builder::alloy_db_admin::UpdateUser {
        super::builder::alloy_db_admin::UpdateUser::new(self.inner.clone()).set_user(user.into())
    }

    /// Deletes a single User.
    pub fn delete_user(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::DeleteUser {
        super::builder::alloy_db_admin::DeleteUser::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists Databases in a given project and location.
    pub fn list_databases(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::ListDatabases {
        super::builder::alloy_db_admin::ListDatabases::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::ListLocations {
        super::builder::alloy_db_admin::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::GetLocation {
        super::builder::alloy_db_admin::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::ListOperations {
        super::builder::alloy_db_admin::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::GetOperation {
        super::builder::alloy_db_admin::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::DeleteOperation {
        super::builder::alloy_db_admin::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::alloy_db_admin::CancelOperation {
        super::builder::alloy_db_admin::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
