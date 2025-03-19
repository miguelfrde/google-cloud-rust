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

/// Implements [AdvisoryNotificationsService](super::stubs::AdvisoryNotificationsService) using a [gaxi::ReqwestClient].
#[derive(Clone)]
pub struct AdvisoryNotificationsService {
    inner: gaxi::ReqwestClient,
}

impl std::fmt::Debug for AdvisoryNotificationsService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("AdvisoryNotificationsService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl AdvisoryNotificationsService {
    pub async fn new(config: gaxi::ClientConfig) -> Result<Self> {
        let inner = gaxi::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::AdvisoryNotificationsService for AdvisoryNotificationsService {
    async fn list_notifications(
        &self,
        req: crate::model::ListNotificationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListNotificationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/notifications", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("view", &req.view.value())]);
        let builder = builder.query(&[("languageCode", &req.language_code)]);
        self.inner
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn get_notification(
        &self,
        req: crate::model::GetNotificationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Notification> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("languageCode", &req.language_code)]);
        self.inner
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn get_settings(
        &self,
        req: crate::model::GetSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Settings> {
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
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn update_settings(
        &self,
        req: crate::model::UpdateSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Settings> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.settings
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("settings"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.settings), options)
            .await
    }
}
