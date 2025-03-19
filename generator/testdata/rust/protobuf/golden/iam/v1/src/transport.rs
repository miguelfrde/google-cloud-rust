// Copyright 2024 Google LLC
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

/// Implements [IAMPolicy](super::stubs::IAMPolicy) using a [gaxi::ReqwestClient].
#[derive(Clone)]
pub struct IAMPolicy {
    inner: gaxi::ReqwestClient,
}

impl std::fmt::Debug for IAMPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("IAMPolicy")
            .field("inner", &self.inner)
            .finish()
    }
}

impl IAMPolicy {
    pub async fn new(config: gaxi::ClientConfig) -> Result<Self> {
        let inner = gaxi::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::IAMPolicy for IAMPolicy {
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:setIamPolicy"
                        , req.resource
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req)
            ,
            options,
        ).await
    }

    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:getIamPolicy"
                        , req.resource
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req)
            ,
            options,
        ).await
    }

    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}:testIamPermissions"
                        , req.resource
                )
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header("x-goog-api-client", reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER));
        self.inner.execute(
            builder,
            Some(req)
            ,
            options,
        ).await
    }

}

