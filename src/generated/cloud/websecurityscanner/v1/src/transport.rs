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

/// Implements [WebSecurityScanner](super::stubs::WebSecurityScanner) using a [gaxi::ReqwestClient].
#[derive(Clone)]
pub struct WebSecurityScanner {
    inner: gaxi::ReqwestClient,
}

impl std::fmt::Debug for WebSecurityScanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("WebSecurityScanner")
            .field("inner", &self.inner)
            .finish()
    }
}

impl WebSecurityScanner {
    pub async fn new(config: gaxi::ClientConfig) -> Result<Self> {
        let inner = gaxi::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl super::stubs::WebSecurityScanner for WebSecurityScanner {
    async fn create_scan_config(
        &self,
        req: crate::model::CreateScanConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ScanConfig> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/{}/scanConfigs", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, Some(req.scan_config), options)
            .await
    }

    async fn delete_scan_config(
        &self,
        req: crate::model::DeleteScanConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
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
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn get_scan_config(
        &self,
        req: crate::model::GetScanConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ScanConfig> {
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

    async fn list_scan_configs(
        &self,
        req: crate::model::ListScanConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListScanConfigsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/scanConfigs", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        self.inner
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn update_scan_config(
        &self,
        req: crate::model::UpdateScanConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ScanConfig> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/{}",
                    req.scan_config
                        .as_ref()
                        .ok_or_else(|| gaxi::path_parameter::missing("scan_config"))?
                        .name
                ),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .update_mask
            .as_ref()
            .map(|p| serde_json::to_value(p).map_err(Error::serde))
            .transpose()?
            .into_iter()
            .fold(builder, |builder, v| {
                use gaxi::query_parameter::QueryParameter;
                v.add(builder, "updateMask")
            });
        self.inner
            .execute(builder, Some(req.scan_config), options)
            .await
    }

    async fn start_scan_run(
        &self,
        req: crate::model::StartScanRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ScanRun> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:start", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn get_scan_run(
        &self,
        req: crate::model::GetScanRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ScanRun> {
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

    async fn list_scan_runs(
        &self,
        req: crate::model::ListScanRunsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListScanRunsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/scanRuns", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        self.inner
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn stop_scan_run(
        &self,
        req: crate::model::StopScanRunRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ScanRun> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::POST, format!("/v1/{}:stop", req.name))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn list_crawled_urls(
        &self,
        req: crate::model::ListCrawledUrlsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListCrawledUrlsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/crawledUrls", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        self.inner
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn get_finding(
        &self,
        req: crate::model::GetFindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Finding> {
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

    async fn list_findings(
        &self,
        req: crate::model::ListFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListFindingsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(reqwest::Method::GET, format!("/v1/{}/findings", req.parent))
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("filter", &req.filter)]);
        let builder = builder.query(&[("pageToken", &req.page_token)]);
        let builder = builder.query(&[("pageSize", &req.page_size)]);
        self.inner
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }

    async fn list_finding_type_stats(
        &self,
        req: crate::model::ListFindingTypeStatsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListFindingTypeStatsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/{}/findingTypeStats", req.parent),
            )
            .query(&[("$alt", "json;enum-encoding=int")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gaxi::NoBody>, options)
            .await
    }
}
