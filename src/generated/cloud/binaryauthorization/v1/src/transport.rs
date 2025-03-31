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

use crate::Result;
#[allow(unused_imports)]
use gax::error::Error;

/// Implements [BinauthzManagementServiceV1](super::stub::BinauthzManagementServiceV1) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct BinauthzManagementServiceV1 {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for BinauthzManagementServiceV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("BinauthzManagementServiceV1")
            .field("inner", &self.inner)
            .finish()
    }
}

impl BinauthzManagementServiceV1 {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::BinauthzManagementServiceV1 for BinauthzManagementServiceV1 {
    async fn get_policy(
        &self,
        req: crate::model::GetPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn update_policy(
        &self,
        req: crate::model::UpdatePolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PUT,
                format!(
                    "/v1/{}",
                    req.policy
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("policy"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req.policy), options).await
    }

    async fn create_attestor(
        &self,
        req: crate::model::CreateAttestorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Attestor> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/attestors", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("attestorId", &req.attestor_id)]);
        self.inner
            .execute(builder, Some(req.attestor), options)
            .await
    }

    async fn get_attestor(
        &self,
        req: crate::model::GetAttestorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Attestor> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn update_attestor(
        &self,
        req: crate::model::UpdateAttestorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Attestor> {
        let options = options.set_default_idempotency(reqwest::Method::PUT.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PUT,
                format!(
                    "/v1/{}",
                    req.attestor
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("attestor"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.attestor), options)
            .await
    }

    async fn list_attestors(
        &self,
        req: crate::model::ListAttestorsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAttestorsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/attestors", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }

    async fn delete_attestor(
        &self,
        req: crate::model::DeleteAttestorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::DELETE, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
            .map(|_: wkt::Empty| ())
    }
}

/// Implements [SystemPolicyV1](super::stub::SystemPolicyV1) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct SystemPolicyV1 {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for SystemPolicyV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("SystemPolicyV1")
            .field("inner", &self.inner)
            .finish()
    }
}

impl SystemPolicyV1 {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::SystemPolicyV1 for SystemPolicyV1 {
    async fn get_system_policy(
        &self,
        req: crate::model::GetSystemPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::http::NoBody>, options)
            .await
    }
}

/// Implements [ValidationHelperV1](super::stub::ValidationHelperV1) using a [gaxi::http::ReqwestClient].
#[derive(Clone)]
pub struct ValidationHelperV1 {
    inner: gaxi::http::ReqwestClient,
}

impl std::fmt::Debug for ValidationHelperV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("ValidationHelperV1")
            .field("inner", &self.inner)
            .finish()
    }
}

impl ValidationHelperV1 {
    pub async fn new(config: gaxi::options::ClientConfig) -> Result<Self> {
        let inner = gaxi::http::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stub::ValidationHelperV1 for ValidationHelperV1 {
    async fn validate_attestation_occurrence(
        &self,
        req: crate::model::ValidateAttestationOccurrenceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ValidateAttestationOccurrenceResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:validateAttestationOccurrence", req.attestor),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }
}
