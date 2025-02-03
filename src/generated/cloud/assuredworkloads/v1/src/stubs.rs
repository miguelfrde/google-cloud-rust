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

/// Defines the trait used to implement [crate::client::AssuredWorkloadsService].
///
/// Application developers may need to implement this trait to mock
/// `client::AssuredWorkloadsService`.  In other use-cases, application developers only
/// use `client::AssuredWorkloadsService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait AssuredWorkloadsService: std::fmt::Debug + Send + Sync {
    /// Implements [crate::client::AssuredWorkloadsService::create_workload].
    fn create_workload(
        &self,
        _req: crate::model::CreateWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::AssuredWorkloadsService::update_workload].
    fn update_workload(
        &self,
        _req: crate::model::UpdateWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Workload>> + Send {
        std::future::ready::<crate::Result<crate::model::Workload>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::AssuredWorkloadsService::restrict_allowed_resources].
    fn restrict_allowed_resources(
        &self,
        _req: crate::model::RestrictAllowedResourcesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::RestrictAllowedResourcesResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::RestrictAllowedResourcesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::AssuredWorkloadsService::delete_workload].
    fn delete_workload(
        &self,
        _req: crate::model::DeleteWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::AssuredWorkloadsService::get_workload].
    fn get_workload(
        &self,
        _req: crate::model::GetWorkloadRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Workload>> + Send {
        std::future::ready::<crate::Result<crate::model::Workload>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::AssuredWorkloadsService::list_workloads].
    fn list_workloads(
        &self,
        _req: crate::model::ListWorkloadsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListWorkloadsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListWorkloadsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::AssuredWorkloadsService::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::AssuredWorkloadsService::get_operation].
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
