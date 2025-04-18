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

//! Implements the common features of all gRPC-based client.

use gax::Result;
use gax::error::Error;

#[doc(hidden)]
pub type InnerClient = tonic::client::Grpc<tonic::transport::Channel>;

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Client {
    inner: InnerClient,
    cred: auth::credentials::Credential,
}

impl Client {
    /// Create a new client.
    pub async fn new(config: crate::options::ClientConfig, default_endpoint: &str) -> Result<Self> {
        let cred = Self::make_credentials(&config).await?;
        let inner = Self::make_inner(config.endpoint, default_endpoint).await?;
        Ok(Self { inner, cred })
    }

    /// Sends a request.
    pub async fn execute<Request, Response>(
        &self,
        method: tonic::GrpcMethod<'static>,
        path: http::uri::PathAndQuery,
        request: Request,
        _options: gax::options::RequestOptions,
        api_client_header: &'static str,
        request_params: &'static str,
    ) -> Result<Response>
    where
        Request: prost::Message + 'static,
        Response: prost::Message + std::default::Default + 'static,
    {
        let headers = self.make_headers(api_client_header, request_params).await?;

        let mut inner = self.inner.clone();
        use gax::error::Error;
        let mut extensions = tonic::Extensions::new();
        extensions.insert(method);
        let metadata = tonic::metadata::MetadataMap::from_headers(headers);
        let request = tonic::Request::from_parts(metadata, extensions, request);
        let codec = tonic::codec::ProstCodec::default();
        inner.ready().await.map_err(Error::rpc)?;
        let response: tonic::Response<Response> = inner
            .unary(request, path, codec)
            .await
            .map_err(Error::rpc)?;
        let response = response.into_inner();
        Ok(response)
    }

    async fn make_inner(endpoint: Option<String>, default_endpoint: &str) -> Result<InnerClient> {
        let endpoint = tonic::transport::Endpoint::from_shared(
            endpoint.unwrap_or_else(|| default_endpoint.to_string()),
        )
        .map_err(Error::other)?;
        let conn = endpoint.connect().await.map_err(Error::io)?;
        Ok(tonic::client::Grpc::new(conn))
    }

    async fn make_credentials(
        config: &crate::options::ClientConfig,
    ) -> Result<auth::credentials::Credential> {
        if let Some(c) = config.cred.clone() {
            return Ok(c);
        }
        auth::credentials::create_access_token_credential()
            .await
            .map_err(Error::authentication)
    }

    async fn make_headers(
        &self,
        api_client_header: &'static str,
        request_params: &str,
    ) -> Result<http::header::HeaderMap> {
        let mut headers = self
            .cred
            .get_headers()
            .await
            .map_err(Error::authentication)?;
        headers.push((
            http::header::HeaderName::from_static("x-goog-api-client"),
            http::header::HeaderValue::from_static(api_client_header),
        ));
        headers.push((
            http::header::HeaderName::from_static("x-goog-request-params"),
            http::header::HeaderValue::from_str(request_params).map_err(Error::other)?,
        ));
        Ok(http::header::HeaderMap::from_iter(headers))
    }
}
