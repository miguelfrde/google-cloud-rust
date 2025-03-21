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

/// Defines the trait used to implement [super::client::PolicyBindings].
///
/// Application developers may need to implement this trait to mock
/// `client::PolicyBindings`.  In other use-cases, application developers only
/// use `client::PolicyBindings` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait PolicyBindings: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::PolicyBindings::create_policy_binding].
    fn create_policy_binding(
        &self,
        _req: crate::model::CreatePolicyBindingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyBindings::get_policy_binding].
    fn get_policy_binding(
        &self,
        _req: crate::model::GetPolicyBindingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PolicyBinding>> + Send {
        std::future::ready::<crate::Result<crate::model::PolicyBinding>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyBindings::update_policy_binding].
    fn update_policy_binding(
        &self,
        _req: crate::model::UpdatePolicyBindingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyBindings::delete_policy_binding].
    fn delete_policy_binding(
        &self,
        _req: crate::model::DeletePolicyBindingRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PolicyBindings::list_policy_bindings].
    fn list_policy_bindings(
        &self,
        _req: crate::model::ListPolicyBindingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListPolicyBindingsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListPolicyBindingsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PolicyBindings::search_target_policy_bindings].
    fn search_target_policy_bindings(
        &self,
        _req: crate::model::SearchTargetPolicyBindingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::SearchTargetPolicyBindingsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::SearchTargetPolicyBindingsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PolicyBindings::get_operation].
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

/// Defines the trait used to implement [super::client::PrincipalAccessBoundaryPolicies].
///
/// Application developers may need to implement this trait to mock
/// `client::PrincipalAccessBoundaryPolicies`.  In other use-cases, application developers only
/// use `client::PrincipalAccessBoundaryPolicies` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait PrincipalAccessBoundaryPolicies: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::PrincipalAccessBoundaryPolicies::create_principal_access_boundary_policy].
    fn create_principal_access_boundary_policy(
        &self,
        _req: crate::model::CreatePrincipalAccessBoundaryPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PrincipalAccessBoundaryPolicies::get_principal_access_boundary_policy].
    fn get_principal_access_boundary_policy(
        &self,
        _req: crate::model::GetPrincipalAccessBoundaryPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::PrincipalAccessBoundaryPolicy>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::PrincipalAccessBoundaryPolicy>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::PrincipalAccessBoundaryPolicies::update_principal_access_boundary_policy].
    fn update_principal_access_boundary_policy(
        &self,
        _req: crate::model::UpdatePrincipalAccessBoundaryPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PrincipalAccessBoundaryPolicies::delete_principal_access_boundary_policy].
    fn delete_principal_access_boundary_policy(
        &self,
        _req: crate::model::DeletePrincipalAccessBoundaryPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::PrincipalAccessBoundaryPolicies::list_principal_access_boundary_policies].
    fn list_principal_access_boundary_policies(
        &self,
        _req: crate::model::ListPrincipalAccessBoundaryPoliciesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListPrincipalAccessBoundaryPoliciesResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListPrincipalAccessBoundaryPoliciesResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::PrincipalAccessBoundaryPolicies::search_principal_access_boundary_policy_bindings].
    fn search_principal_access_boundary_policy_bindings(
        &self,
        _req: crate::model::SearchPrincipalAccessBoundaryPolicyBindingsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::SearchPrincipalAccessBoundaryPolicyBindingsResponse>,
    > + Send {
        std::future::ready::<
            crate::Result<crate::model::SearchPrincipalAccessBoundaryPolicyBindingsResponse>,
        >(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::PrincipalAccessBoundaryPolicies::get_operation].
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
