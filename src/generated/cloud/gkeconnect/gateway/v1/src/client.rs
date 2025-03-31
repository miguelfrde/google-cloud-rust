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

/// Implements a client for the Connect Gateway API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_gkeconnect_gateway_v1::client::GatewayControl;
/// let client = GatewayControl::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// GatewayControl is the control plane API for Connect Gateway.
///
/// # Configuration
///
/// To configure `GatewayControl` use the `with_*` methods in the type returned
/// by [builder()][GatewayControl::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://connectgateway.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::gateway_control::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::gateway_control::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `GatewayControl` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `GatewayControl` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct GatewayControl {
    inner: Arc<dyn super::stub::dynamic::GatewayControl>,
}

impl GatewayControl {
    /// Returns a builder for [GatewayControl].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_gkeconnect_gateway_v1::client::GatewayControl;
    /// let client = GatewayControl::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::gateway_control::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::gateway_control::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::GatewayControl + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::GatewayControl>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::GatewayControl> {
        super::transport::GatewayControl::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::GatewayControl> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::GatewayControl::new)
    }

    /// GenerateCredentials provides connection information that allows a user to
    /// access the specified membership using Connect Gateway.
    pub fn generate_credentials(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::gateway_control::GenerateCredentials {
        super::builder::gateway_control::GenerateCredentials::new(self.inner.clone())
            .set_name(name.into())
    }
}
