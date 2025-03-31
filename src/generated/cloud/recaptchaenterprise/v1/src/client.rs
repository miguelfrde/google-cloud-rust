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

/// Implements a client for the reCAPTCHA Enterprise API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_recaptchaenterprise_v1::client::RecaptchaEnterpriseService;
/// let client = RecaptchaEnterpriseService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Service to determine the likelihood an event is legitimate.
///
/// # Configuration
///
/// To configure `RecaptchaEnterpriseService` use the `with_*` methods in the type returned
/// by [builder()][RecaptchaEnterpriseService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://recaptchaenterprise.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::recaptcha_enterprise_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::recaptcha_enterprise_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `RecaptchaEnterpriseService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `RecaptchaEnterpriseService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct RecaptchaEnterpriseService {
    inner: Arc<dyn super::stub::dynamic::RecaptchaEnterpriseService>,
}

impl RecaptchaEnterpriseService {
    /// Returns a builder for [RecaptchaEnterpriseService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_recaptchaenterprise_v1::client::RecaptchaEnterpriseService;
    /// let client = RecaptchaEnterpriseService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::recaptcha_enterprise_service::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::recaptcha_enterprise_service::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::RecaptchaEnterpriseService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::RecaptchaEnterpriseService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::RecaptchaEnterpriseService> {
        super::transport::RecaptchaEnterpriseService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::RecaptchaEnterpriseService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::RecaptchaEnterpriseService::new)
    }

    /// Creates an Assessment of the likelihood an event is legitimate.
    pub fn create_assessment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::CreateAssessment {
        super::builder::recaptcha_enterprise_service::CreateAssessment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Annotates a previously created Assessment to provide additional information
    /// on whether the event turned out to be authentic or fraudulent.
    pub fn annotate_assessment(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::AnnotateAssessment {
        super::builder::recaptcha_enterprise_service::AnnotateAssessment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new reCAPTCHA Enterprise key.
    pub fn create_key(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::CreateKey {
        super::builder::recaptcha_enterprise_service::CreateKey::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns the list of all keys that belong to a project.
    pub fn list_keys(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::ListKeys {
        super::builder::recaptcha_enterprise_service::ListKeys::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns the secret key related to the specified public key.
    /// You must use the legacy secret key only in a 3rd party integration with
    /// legacy reCAPTCHA.
    pub fn retrieve_legacy_secret_key(
        &self,
        key: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::RetrieveLegacySecretKey {
        super::builder::recaptcha_enterprise_service::RetrieveLegacySecretKey::new(
            self.inner.clone(),
        )
        .set_key(key.into())
    }

    /// Returns the specified key.
    pub fn get_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::GetKey {
        super::builder::recaptcha_enterprise_service::GetKey::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the specified key.
    pub fn update_key(
        &self,
        key: impl Into<crate::model::Key>,
    ) -> super::builder::recaptcha_enterprise_service::UpdateKey {
        super::builder::recaptcha_enterprise_service::UpdateKey::new(self.inner.clone())
            .set_key(key.into())
    }

    /// Deletes the specified key.
    pub fn delete_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::DeleteKey {
        super::builder::recaptcha_enterprise_service::DeleteKey::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Migrates an existing key from reCAPTCHA to reCAPTCHA Enterprise.
    /// Once a key is migrated, it can be used from either product. SiteVerify
    /// requests are billed as CreateAssessment calls. You must be
    /// authenticated as one of the current owners of the reCAPTCHA Key, and
    /// your user must have the reCAPTCHA Enterprise Admin IAM role in the
    /// destination project.
    pub fn migrate_key(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::MigrateKey {
        super::builder::recaptcha_enterprise_service::MigrateKey::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Adds an IP override to a key. The following restrictions hold:
    ///
    /// * The maximum number of IP overrides per key is 100.
    /// * For any conflict (such as IP already exists or IP part of an existing
    ///   IP range), an error is returned.
    pub fn add_ip_override(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::AddIpOverride {
        super::builder::recaptcha_enterprise_service::AddIpOverride::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Removes an IP override from a key. The following restrictions hold:
    ///
    /// * If the IP isn't found in an existing IP override, a `NOT_FOUND` error
    ///   is returned.
    /// * If the IP is found in an existing IP override, but the
    ///   override type does not match, a `NOT_FOUND` error is returned.
    pub fn remove_ip_override(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::RemoveIpOverride {
        super::builder::recaptcha_enterprise_service::RemoveIpOverride::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all IP overrides for a key.
    pub fn list_ip_overrides(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::ListIpOverrides {
        super::builder::recaptcha_enterprise_service::ListIpOverrides::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get some aggregated metrics for a Key. This data can be used to build
    /// dashboards.
    pub fn get_metrics(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::GetMetrics {
        super::builder::recaptcha_enterprise_service::GetMetrics::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new FirewallPolicy, specifying conditions at which reCAPTCHA
    /// Enterprise actions can be executed.
    /// A project may have a maximum of 1000 policies.
    pub fn create_firewall_policy(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::CreateFirewallPolicy {
        super::builder::recaptcha_enterprise_service::CreateFirewallPolicy::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns the list of all firewall policies that belong to a project.
    pub fn list_firewall_policies(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::ListFirewallPolicies {
        super::builder::recaptcha_enterprise_service::ListFirewallPolicies::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns the specified firewall policy.
    pub fn get_firewall_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::GetFirewallPolicy {
        super::builder::recaptcha_enterprise_service::GetFirewallPolicy::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the specified firewall policy.
    pub fn update_firewall_policy(
        &self,
        firewall_policy: impl Into<crate::model::FirewallPolicy>,
    ) -> super::builder::recaptcha_enterprise_service::UpdateFirewallPolicy {
        super::builder::recaptcha_enterprise_service::UpdateFirewallPolicy::new(self.inner.clone())
            .set_firewall_policy(firewall_policy.into())
    }

    /// Deletes the specified firewall policy.
    pub fn delete_firewall_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::DeleteFirewallPolicy {
        super::builder::recaptcha_enterprise_service::DeleteFirewallPolicy::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Reorders all firewall policies.
    pub fn reorder_firewall_policies(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::ReorderFirewallPolicies {
        super::builder::recaptcha_enterprise_service::ReorderFirewallPolicies::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// List groups of related accounts.
    pub fn list_related_account_groups(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::ListRelatedAccountGroups {
        super::builder::recaptcha_enterprise_service::ListRelatedAccountGroups::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// Get memberships in a group of related accounts.
    pub fn list_related_account_group_memberships(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::ListRelatedAccountGroupMemberships {
        super::builder::recaptcha_enterprise_service::ListRelatedAccountGroupMemberships::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// Search group memberships related to a given account.
    pub fn search_related_account_group_memberships(
        &self,
        project: impl Into<std::string::String>,
    ) -> super::builder::recaptcha_enterprise_service::SearchRelatedAccountGroupMemberships {
        super::builder::recaptcha_enterprise_service::SearchRelatedAccountGroupMemberships::new(
            self.inner.clone(),
        )
        .set_project(project.into())
    }
}
