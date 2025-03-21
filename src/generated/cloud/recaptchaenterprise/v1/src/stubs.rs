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

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::RecaptchaEnterpriseService].
///
/// Application developers may need to implement this trait to mock
/// `client::RecaptchaEnterpriseService`.  In other use-cases, application developers only
/// use `client::RecaptchaEnterpriseService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait RecaptchaEnterpriseService: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::RecaptchaEnterpriseService::create_assessment].
    fn create_assessment(
        &self,
        _req: crate::model::CreateAssessmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Assessment>> + Send {
        std::future::ready::<crate::Result<crate::model::Assessment>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::annotate_assessment].
    fn annotate_assessment(
        &self,
        _req: crate::model::AnnotateAssessmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AnnotateAssessmentResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AnnotateAssessmentResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::create_key].
    fn create_key(
        &self,
        _req: crate::model::CreateKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Key>> + Send {
        std::future::ready::<crate::Result<crate::model::Key>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::list_keys].
    fn list_keys(
        &self,
        _req: crate::model::ListKeysRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListKeysResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListKeysResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::retrieve_legacy_secret_key].
    fn retrieve_legacy_secret_key(
        &self,
        _req: crate::model::RetrieveLegacySecretKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::RetrieveLegacySecretKeyResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::RetrieveLegacySecretKeyResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::get_key].
    fn get_key(
        &self,
        _req: crate::model::GetKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Key>> + Send {
        std::future::ready::<crate::Result<crate::model::Key>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::update_key].
    fn update_key(
        &self,
        _req: crate::model::UpdateKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Key>> + Send {
        std::future::ready::<crate::Result<crate::model::Key>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::delete_key].
    fn delete_key(
        &self,
        _req: crate::model::DeleteKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::migrate_key].
    fn migrate_key(
        &self,
        _req: crate::model::MigrateKeyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Key>> + Send {
        std::future::ready::<crate::Result<crate::model::Key>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::add_ip_override].
    fn add_ip_override(
        &self,
        _req: crate::model::AddIpOverrideRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::AddIpOverrideResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::AddIpOverrideResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::remove_ip_override].
    fn remove_ip_override(
        &self,
        _req: crate::model::RemoveIpOverrideRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::RemoveIpOverrideResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::RemoveIpOverrideResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::list_ip_overrides].
    fn list_ip_overrides(
        &self,
        _req: crate::model::ListIpOverridesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListIpOverridesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListIpOverridesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::get_metrics].
    fn get_metrics(
        &self,
        _req: crate::model::GetMetricsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Metrics>> + Send {
        std::future::ready::<crate::Result<crate::model::Metrics>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::create_firewall_policy].
    fn create_firewall_policy(
        &self,
        _req: crate::model::CreateFirewallPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FirewallPolicy>> + Send {
        std::future::ready::<crate::Result<crate::model::FirewallPolicy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::list_firewall_policies].
    fn list_firewall_policies(
        &self,
        _req: crate::model::ListFirewallPoliciesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListFirewallPoliciesResponse>>
    + Send {
        std::future::ready::<crate::Result<crate::model::ListFirewallPoliciesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::get_firewall_policy].
    fn get_firewall_policy(
        &self,
        _req: crate::model::GetFirewallPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FirewallPolicy>> + Send {
        std::future::ready::<crate::Result<crate::model::FirewallPolicy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::update_firewall_policy].
    fn update_firewall_policy(
        &self,
        _req: crate::model::UpdateFirewallPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::FirewallPolicy>> + Send {
        std::future::ready::<crate::Result<crate::model::FirewallPolicy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::delete_firewall_policy].
    fn delete_firewall_policy(
        &self,
        _req: crate::model::DeleteFirewallPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::reorder_firewall_policies].
    fn reorder_firewall_policies(
        &self,
        _req: crate::model::ReorderFirewallPoliciesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ReorderFirewallPoliciesResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ReorderFirewallPoliciesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::list_related_account_groups].
    fn list_related_account_groups(
        &self,
        _req: crate::model::ListRelatedAccountGroupsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListRelatedAccountGroupsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListRelatedAccountGroupsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::RecaptchaEnterpriseService::list_related_account_group_memberships].
    fn list_related_account_group_memberships(
        &self,
        _req: crate::model::ListRelatedAccountGroupMembershipsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListRelatedAccountGroupMembershipsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListRelatedAccountGroupMembershipsResponse>>(
            Err(Error::other("unimplemented")),
        )
    }

    /// Implements [super::client::RecaptchaEnterpriseService::search_related_account_group_memberships].
    fn search_related_account_group_memberships(
        &self,
        _req: crate::model::SearchRelatedAccountGroupMembershipsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::SearchRelatedAccountGroupMembershipsResponse>,
    > + Send {
        std::future::ready::<
            crate::Result<crate::model::SearchRelatedAccountGroupMembershipsResponse>,
        >(Err(Error::other("unimplemented")))
    }
}
