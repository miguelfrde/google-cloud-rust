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

/// Implements a client for the Cloud Asset API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_asset_v1::client::AssetService;
/// let client = AssetService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Asset service definition.
///
/// # Configuration
///
/// To configure `AssetService` use the `with_*` methods in the type returned
/// by [builder()][AssetService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://cloudasset.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::asset_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::asset_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `AssetService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AssetService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct AssetService {
    inner: Arc<dyn super::stub::dynamic::AssetService>,
}

impl AssetService {
    /// Returns a builder for [AssetService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_asset_v1::client::AssetService;
    /// let client = AssetService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::asset_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::asset_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::AssetService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::AssetService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::AssetService> {
        super::transport::AssetService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::AssetService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::AssetService::new)
    }

    /// Exports assets with time and resource types to a given Cloud Storage
    /// location/BigQuery table. For Cloud Storage location destinations, the
    /// output format is newline-delimited JSON. Each line represents a
    /// [google.cloud.asset.v1.Asset][google.cloud.asset.v1.Asset] in the JSON
    /// format; for BigQuery table destinations, the output table stores the fields
    /// in asset Protobuf as columns. This API implements the
    /// [google.longrunning.Operation][google.longrunning.Operation] API, which
    /// allows you to keep track of the export. We recommend intervals of at least
    /// 2 seconds with exponential retry to poll the export operation result. For
    /// regular-size resource parent, the export operation usually finishes within
    /// 5 minutes.
    ///
    /// [google.cloud.asset.v1.Asset]: crate::model::Asset
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
    pub fn export_assets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::asset_service::ExportAssets {
        super::builder::asset_service::ExportAssets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists assets with time and resource types and returns paged results in
    /// response.
    pub fn list_assets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::asset_service::ListAssets {
        super::builder::asset_service::ListAssets::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Batch gets the update history of assets that overlap a time window.
    /// For IAM_POLICY content, this API outputs history when the asset and its
    /// attached IAM POLICY both exist. This can create gaps in the output history.
    /// Otherwise, this API outputs history with asset in both non-delete or
    /// deleted status.
    /// If a specified asset does not exist, this API returns an INVALID_ARGUMENT
    /// error.
    pub fn batch_get_assets_history(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::asset_service::BatchGetAssetsHistory {
        super::builder::asset_service::BatchGetAssetsHistory::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a feed in a parent project/folder/organization to listen to its
    /// asset updates.
    pub fn create_feed(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::asset_service::CreateFeed {
        super::builder::asset_service::CreateFeed::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets details about an asset feed.
    pub fn get_feed(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::asset_service::GetFeed {
        super::builder::asset_service::GetFeed::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists all asset feeds in a parent project/folder/organization.
    pub fn list_feeds(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::asset_service::ListFeeds {
        super::builder::asset_service::ListFeeds::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Updates an asset feed configuration.
    pub fn update_feed(
        &self,
        feed: impl Into<crate::model::Feed>,
    ) -> super::builder::asset_service::UpdateFeed {
        super::builder::asset_service::UpdateFeed::new(self.inner.clone()).set_feed(feed.into())
    }

    /// Deletes an asset feed.
    pub fn delete_feed(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::asset_service::DeleteFeed {
        super::builder::asset_service::DeleteFeed::new(self.inner.clone()).set_name(name.into())
    }

    /// Searches all Google Cloud resources within the specified scope, such as a
    /// project, folder, or organization. The caller must be granted the
    /// `cloudasset.assets.searchAllResources` permission on the desired scope,
    /// otherwise the request will be rejected.
    pub fn search_all_resources(
        &self,
        scope: impl Into<std::string::String>,
    ) -> super::builder::asset_service::SearchAllResources {
        super::builder::asset_service::SearchAllResources::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Searches all IAM policies within the specified scope, such as a project,
    /// folder, or organization. The caller must be granted the
    /// `cloudasset.assets.searchAllIamPolicies` permission on the desired scope,
    /// otherwise the request will be rejected.
    pub fn search_all_iam_policies(
        &self,
        scope: impl Into<std::string::String>,
    ) -> super::builder::asset_service::SearchAllIamPolicies {
        super::builder::asset_service::SearchAllIamPolicies::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Analyzes IAM policies to answer which identities have what accesses on
    /// which resources.
    pub fn analyze_iam_policy(
        &self,
        analysis_query: impl Into<crate::model::IamPolicyAnalysisQuery>,
    ) -> super::builder::asset_service::AnalyzeIamPolicy {
        super::builder::asset_service::AnalyzeIamPolicy::new(self.inner.clone())
            .set_analysis_query(analysis_query.into())
    }

    /// Analyzes IAM policies asynchronously to answer which identities have what
    /// accesses on which resources, and writes the analysis results to a Google
    /// Cloud Storage or a BigQuery destination. For Cloud Storage destination, the
    /// output format is the JSON format that represents a
    /// [AnalyzeIamPolicyResponse][google.cloud.asset.v1.AnalyzeIamPolicyResponse].
    /// This method implements the
    /// [google.longrunning.Operation][google.longrunning.Operation], which allows
    /// you to track the operation status. We recommend intervals of at least 2
    /// seconds with exponential backoff retry to poll the operation result. The
    /// metadata contains the metadata for the long-running operation.
    ///
    /// [google.cloud.asset.v1.AnalyzeIamPolicyResponse]: crate::model::AnalyzeIamPolicyResponse
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
    pub fn analyze_iam_policy_longrunning(
        &self,
        analysis_query: impl Into<crate::model::IamPolicyAnalysisQuery>,
    ) -> super::builder::asset_service::AnalyzeIamPolicyLongrunning {
        super::builder::asset_service::AnalyzeIamPolicyLongrunning::new(self.inner.clone())
            .set_analysis_query(analysis_query.into())
    }

    /// Analyze moving a resource to a specified destination without kicking off
    /// the actual move. The analysis is best effort depending on the user's
    /// permissions of viewing different hierarchical policies and configurations.
    /// The policies and configuration are subject to change before the actual
    /// resource migration takes place.
    pub fn analyze_move(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::asset_service::AnalyzeMove {
        super::builder::asset_service::AnalyzeMove::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Issue a job that queries assets using a SQL statement compatible with
    /// [BigQuery SQL](https://cloud.google.com/bigquery/docs/introduction-sql).
    ///
    /// If the query execution finishes within timeout and there's no pagination,
    /// the full query results will be returned in the `QueryAssetsResponse`.
    ///
    /// Otherwise, full query results can be obtained by issuing extra requests
    /// with the `job_reference` from the a previous `QueryAssets` call.
    ///
    /// Note, the query result has approximately 10 GB limitation enforced by
    /// [BigQuery](https://cloud.google.com/bigquery/docs/best-practices-performance-output).
    /// Queries return larger results will result in errors.
    pub fn query_assets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::asset_service::QueryAssets {
        super::builder::asset_service::QueryAssets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a saved query in a parent project/folder/organization.
    pub fn create_saved_query(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::asset_service::CreateSavedQuery {
        super::builder::asset_service::CreateSavedQuery::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details about a saved query.
    pub fn get_saved_query(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::asset_service::GetSavedQuery {
        super::builder::asset_service::GetSavedQuery::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists all saved queries in a parent project/folder/organization.
    pub fn list_saved_queries(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::asset_service::ListSavedQueries {
        super::builder::asset_service::ListSavedQueries::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a saved query.
    pub fn update_saved_query(
        &self,
        saved_query: impl Into<crate::model::SavedQuery>,
    ) -> super::builder::asset_service::UpdateSavedQuery {
        super::builder::asset_service::UpdateSavedQuery::new(self.inner.clone())
            .set_saved_query(saved_query.into())
    }

    /// Deletes a saved query.
    pub fn delete_saved_query(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::asset_service::DeleteSavedQuery {
        super::builder::asset_service::DeleteSavedQuery::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets effective IAM policies for a batch of resources.
    pub fn batch_get_effective_iam_policies(
        &self,
        scope: impl Into<std::string::String>,
    ) -> super::builder::asset_service::BatchGetEffectiveIamPolicies {
        super::builder::asset_service::BatchGetEffectiveIamPolicies::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Analyzes organization policies under a scope.
    pub fn analyze_org_policies(
        &self,
        scope: impl Into<std::string::String>,
    ) -> super::builder::asset_service::AnalyzeOrgPolicies {
        super::builder::asset_service::AnalyzeOrgPolicies::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Analyzes organization policies governed containers (projects, folders or
    /// organization) under a scope.
    pub fn analyze_org_policy_governed_containers(
        &self,
        scope: impl Into<std::string::String>,
    ) -> super::builder::asset_service::AnalyzeOrgPolicyGovernedContainers {
        super::builder::asset_service::AnalyzeOrgPolicyGovernedContainers::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Analyzes organization policies governed assets (Google Cloud resources or
    /// policies) under a scope. This RPC supports custom constraints and the
    /// following canned constraints:
    ///
    /// * constraints/ainotebooks.accessMode
    /// * constraints/ainotebooks.disableFileDownloads
    /// * constraints/ainotebooks.disableRootAccess
    /// * constraints/ainotebooks.disableTerminal
    /// * constraints/ainotebooks.environmentOptions
    /// * constraints/ainotebooks.requireAutoUpgradeSchedule
    /// * constraints/ainotebooks.restrictVpcNetworks
    /// * constraints/compute.disableGuestAttributesAccess
    /// * constraints/compute.disableInstanceDataAccessApis
    /// * constraints/compute.disableNestedVirtualization
    /// * constraints/compute.disableSerialPortAccess
    /// * constraints/compute.disableSerialPortLogging
    /// * constraints/compute.disableVpcExternalIpv6
    /// * constraints/compute.requireOsLogin
    /// * constraints/compute.requireShieldedVm
    /// * constraints/compute.restrictLoadBalancerCreationForTypes
    /// * constraints/compute.restrictProtocolForwardingCreationForTypes
    /// * constraints/compute.restrictXpnProjectLienRemoval
    /// * constraints/compute.setNewProjectDefaultToZonalDNSOnly
    /// * constraints/compute.skipDefaultNetworkCreation
    /// * constraints/compute.trustedImageProjects
    /// * constraints/compute.vmCanIpForward
    /// * constraints/compute.vmExternalIpAccess
    /// * constraints/gcp.detailedAuditLoggingMode
    /// * constraints/gcp.resourceLocations
    /// * constraints/iam.allowedPolicyMemberDomains
    /// * constraints/iam.automaticIamGrantsForDefaultServiceAccounts
    /// * constraints/iam.disableServiceAccountCreation
    /// * constraints/iam.disableServiceAccountKeyCreation
    /// * constraints/iam.disableServiceAccountKeyUpload
    /// * constraints/iam.restrictCrossProjectServiceAccountLienRemoval
    /// * constraints/iam.serviceAccountKeyExpiryHours
    /// * constraints/resourcemanager.accessBoundaries
    /// * constraints/resourcemanager.allowedExportDestinations
    /// * constraints/sql.restrictAuthorizedNetworks
    /// * constraints/sql.restrictNoncompliantDiagnosticDataAccess
    /// * constraints/sql.restrictNoncompliantResourceCreation
    /// * constraints/sql.restrictPublicIp
    /// * constraints/storage.publicAccessPrevention
    /// * constraints/storage.restrictAuthTypes
    /// * constraints/storage.uniformBucketLevelAccess
    ///
    /// This RPC only returns either resources of types [supported by search
    /// APIs](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
    /// or IAM policies.
    pub fn analyze_org_policy_governed_assets(
        &self,
        scope: impl Into<std::string::String>,
    ) -> super::builder::asset_service::AnalyzeOrgPolicyGovernedAssets {
        super::builder::asset_service::AnalyzeOrgPolicyGovernedAssets::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::asset_service::GetOperation {
        super::builder::asset_service::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
