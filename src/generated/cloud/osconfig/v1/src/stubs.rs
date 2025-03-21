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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::OsConfigService].
///
/// Application developers may need to implement this trait to mock
/// `client::OsConfigService`.  In other use-cases, application developers only
/// use `client::OsConfigService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait OsConfigService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::OsConfigService::execute_patch_job].
    fn execute_patch_job(
        &self,
        _req: crate::model::ExecutePatchJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PatchJob>> + Send {
        std::future::ready::<crate::Result<crate::model::PatchJob>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::get_patch_job].
    fn get_patch_job(
        &self,
        _req: crate::model::GetPatchJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PatchJob>> + Send {
        std::future::ready::<crate::Result<crate::model::PatchJob>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::cancel_patch_job].
    fn cancel_patch_job(
        &self,
        _req: crate::model::CancelPatchJobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PatchJob>> + Send {
        std::future::ready::<crate::Result<crate::model::PatchJob>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::list_patch_jobs].
    fn list_patch_jobs(
        &self,
        _req: crate::model::ListPatchJobsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListPatchJobsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListPatchJobsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::list_patch_job_instance_details].
    fn list_patch_job_instance_details(
        &self,
        _req: crate::model::ListPatchJobInstanceDetailsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListPatchJobInstanceDetailsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListPatchJobInstanceDetailsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OsConfigService::create_patch_deployment].
    fn create_patch_deployment(
        &self,
        _req: crate::model::CreatePatchDeploymentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PatchDeployment>> + Send
    {
        std::future::ready::<crate::Result<crate::model::PatchDeployment>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::get_patch_deployment].
    fn get_patch_deployment(
        &self,
        _req: crate::model::GetPatchDeploymentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PatchDeployment>> + Send
    {
        std::future::ready::<crate::Result<crate::model::PatchDeployment>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::list_patch_deployments].
    fn list_patch_deployments(
        &self,
        _req: crate::model::ListPatchDeploymentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListPatchDeploymentsResponse>>
    + Send {
        std::future::ready::<crate::Result<crate::model::ListPatchDeploymentsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OsConfigService::delete_patch_deployment].
    fn delete_patch_deployment(
        &self,
        _req: crate::model::DeletePatchDeploymentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::OsConfigService::update_patch_deployment].
    fn update_patch_deployment(
        &self,
        _req: crate::model::UpdatePatchDeploymentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PatchDeployment>> + Send
    {
        std::future::ready::<crate::Result<crate::model::PatchDeployment>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::pause_patch_deployment].
    fn pause_patch_deployment(
        &self,
        _req: crate::model::PausePatchDeploymentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PatchDeployment>> + Send
    {
        std::future::ready::<crate::Result<crate::model::PatchDeployment>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::resume_patch_deployment].
    fn resume_patch_deployment(
        &self,
        _req: crate::model::ResumePatchDeploymentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PatchDeployment>> + Send
    {
        std::future::ready::<crate::Result<crate::model::PatchDeployment>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigService::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }
}

/// Defines the trait used to implement [super::client::OsConfigZonalService].
///
/// Application developers may need to implement this trait to mock
/// `client::OsConfigZonalService`.  In other use-cases, application developers only
/// use `client::OsConfigZonalService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait OsConfigZonalService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::OsConfigZonalService::create_os_policy_assignment].
    fn create_os_policy_assignment(
        &self,
        _req: crate::model::CreateOSPolicyAssignmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigZonalService::update_os_policy_assignment].
    fn update_os_policy_assignment(
        &self,
        _req: crate::model::UpdateOSPolicyAssignmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigZonalService::get_os_policy_assignment].
    fn get_os_policy_assignment(
        &self,
        _req: crate::model::GetOSPolicyAssignmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::OSPolicyAssignment>> + Send
    {
        std::future::ready::<crate::Result<crate::model::OSPolicyAssignment>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigZonalService::list_os_policy_assignments].
    fn list_os_policy_assignments(
        &self,
        _req: crate::model::ListOSPolicyAssignmentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListOSPolicyAssignmentsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListOSPolicyAssignmentsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OsConfigZonalService::list_os_policy_assignment_revisions].
    fn list_os_policy_assignment_revisions(
        &self,
        _req: crate::model::ListOSPolicyAssignmentRevisionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListOSPolicyAssignmentRevisionsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListOSPolicyAssignmentRevisionsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::OsConfigZonalService::delete_os_policy_assignment].
    fn delete_os_policy_assignment(
        &self,
        _req: crate::model::DeleteOSPolicyAssignmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigZonalService::get_os_policy_assignment_report].
    fn get_os_policy_assignment_report(
        &self,
        _req: crate::model::GetOSPolicyAssignmentReportRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::OSPolicyAssignmentReport>> + Send
    {
        std::future::ready::<crate::Result<crate::model::OSPolicyAssignmentReport>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OsConfigZonalService::list_os_policy_assignment_reports].
    fn list_os_policy_assignment_reports(
        &self,
        _req: crate::model::ListOSPolicyAssignmentReportsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListOSPolicyAssignmentReportsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListOSPolicyAssignmentReportsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::OsConfigZonalService::get_inventory].
    fn get_inventory(
        &self,
        _req: crate::model::GetInventoryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Inventory>> + Send {
        std::future::ready::<crate::Result<crate::model::Inventory>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigZonalService::list_inventories].
    fn list_inventories(
        &self,
        _req: crate::model::ListInventoriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListInventoriesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListInventoriesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OsConfigZonalService::get_vulnerability_report].
    fn get_vulnerability_report(
        &self,
        _req: crate::model::GetVulnerabilityReportRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::VulnerabilityReport>> + Send
    {
        std::future::ready::<crate::Result<crate::model::VulnerabilityReport>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigZonalService::list_vulnerability_reports].
    fn list_vulnerability_reports(
        &self,
        _req: crate::model::ListVulnerabilityReportsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListVulnerabilityReportsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListVulnerabilityReportsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::OsConfigZonalService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::OsConfigZonalService::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
