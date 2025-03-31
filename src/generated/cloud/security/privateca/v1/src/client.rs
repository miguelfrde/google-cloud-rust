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

/// Implements a client for the Certificate Authority API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_security_privateca_v1::client::CertificateAuthorityService;
/// let client = CertificateAuthorityService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// [Certificate Authority
/// Service][google.cloud.security.privateca.v1.CertificateAuthorityService]
/// manages private certificate authorities and issued certificates.
///
/// [google.cloud.security.privateca.v1.CertificateAuthorityService]: crate::client::CertificateAuthorityService
///
/// # Configuration
///
/// To configure `CertificateAuthorityService` use the `with_*` methods in the type returned
/// by [builder()][CertificateAuthorityService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://privateca.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::certificate_authority_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::certificate_authority_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CertificateAuthorityService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CertificateAuthorityService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CertificateAuthorityService {
    inner: Arc<dyn super::stub::dynamic::CertificateAuthorityService>,
}

impl CertificateAuthorityService {
    /// Returns a builder for [CertificateAuthorityService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_security_privateca_v1::client::CertificateAuthorityService;
    /// let client = CertificateAuthorityService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::certificate_authority_service::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::certificate_authority_service::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::CertificateAuthorityService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::CertificateAuthorityService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::CertificateAuthorityService> {
        super::transport::CertificateAuthorityService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::CertificateAuthorityService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::CertificateAuthorityService::new)
    }

    /// Create a new [Certificate][google.cloud.security.privateca.v1.Certificate]
    /// in a given Project, Location from a particular
    /// [CaPool][google.cloud.security.privateca.v1.CaPool].
    ///
    /// [google.cloud.security.privateca.v1.CaPool]: crate::model::CaPool
    /// [google.cloud.security.privateca.v1.Certificate]: crate::model::Certificate
    pub fn create_certificate(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::CreateCertificate {
        super::builder::certificate_authority_service::CreateCertificate::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns a [Certificate][google.cloud.security.privateca.v1.Certificate].
    ///
    /// [google.cloud.security.privateca.v1.Certificate]: crate::model::Certificate
    pub fn get_certificate(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::GetCertificate {
        super::builder::certificate_authority_service::GetCertificate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists [Certificates][google.cloud.security.privateca.v1.Certificate].
    ///
    /// [google.cloud.security.privateca.v1.Certificate]: crate::model::Certificate
    pub fn list_certificates(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::ListCertificates {
        super::builder::certificate_authority_service::ListCertificates::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Revoke a [Certificate][google.cloud.security.privateca.v1.Certificate].
    ///
    /// [google.cloud.security.privateca.v1.Certificate]: crate::model::Certificate
    pub fn revoke_certificate(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::RevokeCertificate {
        super::builder::certificate_authority_service::RevokeCertificate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Update a [Certificate][google.cloud.security.privateca.v1.Certificate].
    /// Currently, the only field you can update is the
    /// [labels][google.cloud.security.privateca.v1.Certificate.labels] field.
    ///
    /// [google.cloud.security.privateca.v1.Certificate]: crate::model::Certificate
    /// [google.cloud.security.privateca.v1.Certificate.labels]: crate::model::Certificate::labels
    pub fn update_certificate(
        &self,
        certificate: impl Into<crate::model::Certificate>,
    ) -> super::builder::certificate_authority_service::UpdateCertificate {
        super::builder::certificate_authority_service::UpdateCertificate::new(self.inner.clone())
            .set_certificate(certificate.into())
    }

    /// Activate a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// that is in state
    /// [AWAITING_USER_ACTIVATION][google.cloud.security.privateca.v1.CertificateAuthority.State.AWAITING_USER_ACTIVATION]
    /// and is of type
    /// [SUBORDINATE][google.cloud.security.privateca.v1.CertificateAuthority.Type.SUBORDINATE].
    /// After the parent Certificate Authority signs a certificate signing request
    /// from
    /// [FetchCertificateAuthorityCsr][google.cloud.security.privateca.v1.CertificateAuthorityService.FetchCertificateAuthorityCsr],
    /// this method can complete the activation process.
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
    /// [google.cloud.security.privateca.v1.CertificateAuthority.State.AWAITING_USER_ACTIVATION]: crate::model::certificate_authority::state::AWAITING_USER_ACTIVATION
    /// [google.cloud.security.privateca.v1.CertificateAuthority.Type.SUBORDINATE]: crate::model::certificate_authority::r#type::SUBORDINATE
    /// [google.cloud.security.privateca.v1.CertificateAuthorityService.FetchCertificateAuthorityCsr]: crate::client::CertificateAuthorityService::fetch_certificate_authority_csr
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
    pub fn activate_certificate_authority(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::ActivateCertificateAuthority {
        super::builder::certificate_authority_service::ActivateCertificateAuthority::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Create a new
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in a given Project and Location.
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
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
    pub fn create_certificate_authority(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::CreateCertificateAuthority {
        super::builder::certificate_authority_service::CreateCertificateAuthority::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// Disable a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
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
    pub fn disable_certificate_authority(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::DisableCertificateAuthority {
        super::builder::certificate_authority_service::DisableCertificateAuthority::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Enable a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
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
    pub fn enable_certificate_authority(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::EnableCertificateAuthority {
        super::builder::certificate_authority_service::EnableCertificateAuthority::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Fetch a certificate signing request (CSR) from a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// that is in state
    /// [AWAITING_USER_ACTIVATION][google.cloud.security.privateca.v1.CertificateAuthority.State.AWAITING_USER_ACTIVATION]
    /// and is of type
    /// [SUBORDINATE][google.cloud.security.privateca.v1.CertificateAuthority.Type.SUBORDINATE].
    /// The CSR must then be signed by the desired parent Certificate Authority,
    /// which could be another
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// resource, or could be an on-prem certificate authority. See also
    /// [ActivateCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.ActivateCertificateAuthority].
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
    /// [google.cloud.security.privateca.v1.CertificateAuthority.State.AWAITING_USER_ACTIVATION]: crate::model::certificate_authority::state::AWAITING_USER_ACTIVATION
    /// [google.cloud.security.privateca.v1.CertificateAuthority.Type.SUBORDINATE]: crate::model::certificate_authority::r#type::SUBORDINATE
    /// [google.cloud.security.privateca.v1.CertificateAuthorityService.ActivateCertificateAuthority]: crate::client::CertificateAuthorityService::activate_certificate_authority
    pub fn fetch_certificate_authority_csr(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::FetchCertificateAuthorityCsr {
        super::builder::certificate_authority_service::FetchCertificateAuthorityCsr::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Returns a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
    pub fn get_certificate_authority(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::GetCertificateAuthority {
        super::builder::certificate_authority_service::GetCertificateAuthority::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Lists
    /// [CertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthority].
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
    pub fn list_certificate_authorities(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::ListCertificateAuthorities {
        super::builder::certificate_authority_service::ListCertificateAuthorities::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// Undelete a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// that has been deleted.
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
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
    pub fn undelete_certificate_authority(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::UndeleteCertificateAuthority {
        super::builder::certificate_authority_service::UndeleteCertificateAuthority::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Delete a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
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
    pub fn delete_certificate_authority(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::DeleteCertificateAuthority {
        super::builder::certificate_authority_service::DeleteCertificateAuthority::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Update a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
    ///
    /// [google.cloud.security.privateca.v1.CertificateAuthority]: crate::model::CertificateAuthority
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
    pub fn update_certificate_authority(
        &self,
        certificate_authority: impl Into<crate::model::CertificateAuthority>,
    ) -> super::builder::certificate_authority_service::UpdateCertificateAuthority {
        super::builder::certificate_authority_service::UpdateCertificateAuthority::new(
            self.inner.clone(),
        )
        .set_certificate_authority(certificate_authority.into())
    }

    /// Create a [CaPool][google.cloud.security.privateca.v1.CaPool].
    ///
    /// [google.cloud.security.privateca.v1.CaPool]: crate::model::CaPool
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
    pub fn create_ca_pool(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::CreateCaPool {
        super::builder::certificate_authority_service::CreateCaPool::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Update a [CaPool][google.cloud.security.privateca.v1.CaPool].
    ///
    /// [google.cloud.security.privateca.v1.CaPool]: crate::model::CaPool
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
    pub fn update_ca_pool(
        &self,
        ca_pool: impl Into<crate::model::CaPool>,
    ) -> super::builder::certificate_authority_service::UpdateCaPool {
        super::builder::certificate_authority_service::UpdateCaPool::new(self.inner.clone())
            .set_ca_pool(ca_pool.into())
    }

    /// Returns a [CaPool][google.cloud.security.privateca.v1.CaPool].
    ///
    /// [google.cloud.security.privateca.v1.CaPool]: crate::model::CaPool
    pub fn get_ca_pool(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::GetCaPool {
        super::builder::certificate_authority_service::GetCaPool::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists [CaPools][google.cloud.security.privateca.v1.CaPool].
    ///
    /// [google.cloud.security.privateca.v1.CaPool]: crate::model::CaPool
    pub fn list_ca_pools(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::ListCaPools {
        super::builder::certificate_authority_service::ListCaPools::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Delete a [CaPool][google.cloud.security.privateca.v1.CaPool].
    ///
    /// [google.cloud.security.privateca.v1.CaPool]: crate::model::CaPool
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
    pub fn delete_ca_pool(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::DeleteCaPool {
        super::builder::certificate_authority_service::DeleteCaPool::new(self.inner.clone())
            .set_name(name.into())
    }

    /// FetchCaCerts returns the current trust anchor for the
    /// [CaPool][google.cloud.security.privateca.v1.CaPool]. This will include CA
    /// certificate chains for all certificate authorities in the ENABLED,
    /// DISABLED, or STAGED states.
    ///
    /// [google.cloud.security.privateca.v1.CaPool]: crate::model::CaPool
    pub fn fetch_ca_certs(
        &self,
        ca_pool: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::FetchCaCerts {
        super::builder::certificate_authority_service::FetchCaCerts::new(self.inner.clone())
            .set_ca_pool(ca_pool.into())
    }

    /// Returns a
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList].
    ///
    /// [google.cloud.security.privateca.v1.CertificateRevocationList]: crate::model::CertificateRevocationList
    pub fn get_certificate_revocation_list(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::GetCertificateRevocationList {
        super::builder::certificate_authority_service::GetCertificateRevocationList::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Lists
    /// [CertificateRevocationLists][google.cloud.security.privateca.v1.CertificateRevocationList].
    ///
    /// [google.cloud.security.privateca.v1.CertificateRevocationList]: crate::model::CertificateRevocationList
    pub fn list_certificate_revocation_lists(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::ListCertificateRevocationLists {
        super::builder::certificate_authority_service::ListCertificateRevocationLists::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// Update a
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList].
    ///
    /// [google.cloud.security.privateca.v1.CertificateRevocationList]: crate::model::CertificateRevocationList
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
    pub fn update_certificate_revocation_list(
        &self,
        certificate_revocation_list: impl Into<crate::model::CertificateRevocationList>,
    ) -> super::builder::certificate_authority_service::UpdateCertificateRevocationList {
        super::builder::certificate_authority_service::UpdateCertificateRevocationList::new(
            self.inner.clone(),
        )
        .set_certificate_revocation_list(certificate_revocation_list.into())
    }

    /// Create a new
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
    /// in a given Project and Location.
    ///
    /// [google.cloud.security.privateca.v1.CertificateTemplate]: crate::model::CertificateTemplate
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
    pub fn create_certificate_template(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::CreateCertificateTemplate {
        super::builder::certificate_authority_service::CreateCertificateTemplate::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// DeleteCertificateTemplate deletes a
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate].
    ///
    /// [google.cloud.security.privateca.v1.CertificateTemplate]: crate::model::CertificateTemplate
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
    pub fn delete_certificate_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::DeleteCertificateTemplate {
        super::builder::certificate_authority_service::DeleteCertificateTemplate::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Returns a
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate].
    ///
    /// [google.cloud.security.privateca.v1.CertificateTemplate]: crate::model::CertificateTemplate
    pub fn get_certificate_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::GetCertificateTemplate {
        super::builder::certificate_authority_service::GetCertificateTemplate::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Lists
    /// [CertificateTemplates][google.cloud.security.privateca.v1.CertificateTemplate].
    ///
    /// [google.cloud.security.privateca.v1.CertificateTemplate]: crate::model::CertificateTemplate
    pub fn list_certificate_templates(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::ListCertificateTemplates {
        super::builder::certificate_authority_service::ListCertificateTemplates::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// Update a
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate].
    ///
    /// [google.cloud.security.privateca.v1.CertificateTemplate]: crate::model::CertificateTemplate
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
    pub fn update_certificate_template(
        &self,
        certificate_template: impl Into<crate::model::CertificateTemplate>,
    ) -> super::builder::certificate_authority_service::UpdateCertificateTemplate {
        super::builder::certificate_authority_service::UpdateCertificateTemplate::new(
            self.inner.clone(),
        )
        .set_certificate_template(certificate_template.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::ListLocations {
        super::builder::certificate_authority_service::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::GetLocation {
        super::builder::certificate_authority_service::GetLocation::new(self.inner.clone())
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
    ) -> super::builder::certificate_authority_service::SetIamPolicy {
        super::builder::certificate_authority_service::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::GetIamPolicy {
        super::builder::certificate_authority_service::GetIamPolicy::new(self.inner.clone())
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
    ) -> super::builder::certificate_authority_service::TestIamPermissions {
        super::builder::certificate_authority_service::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::ListOperations {
        super::builder::certificate_authority_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::GetOperation {
        super::builder::certificate_authority_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::DeleteOperation {
        super::builder::certificate_authority_service::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::certificate_authority_service::CancelOperation {
        super::builder::certificate_authority_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
