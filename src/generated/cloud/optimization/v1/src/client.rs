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

/// Implements a client for the Cloud Optimization API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_optimization_v1::client::FleetRouting;
/// let client = FleetRouting::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service for optimizing vehicle tours.
///
/// Validity of certain types of fields:
///
/// * `google.protobuf.Timestamp`
///   * Times are in Unix time: seconds since 1970-01-01T00:00:00+00:00.
///   * seconds must be in [0, 253402300799],
///     i.e. in [1970-01-01T00:00:00+00:00, 9999-12-31T23:59:59+00:00].
///   * nanos must be unset or set to 0.
/// * `google.protobuf.Duration`
///   * seconds must be in [0, 253402300799],
///     i.e. in [1970-01-01T00:00:00+00:00, 9999-12-31T23:59:59+00:00].
///   * nanos must be unset or set to 0.
/// * `google.type.LatLng`
///   * latitude must be in [-90.0, 90.0].
///   * longitude must be in [-180.0, 180.0].
///   * at least one of latitude and longitude must be non-zero.
///
/// # Configuration
///
/// To configure `FleetRouting` use the `with_*` methods in the type returned
/// by [builder()][FleetRouting::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://cloudoptimization.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::fleet_routing::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::fleet_routing::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `FleetRouting` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `FleetRouting` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct FleetRouting {
    inner: Arc<dyn super::stub::dynamic::FleetRouting>,
}

impl FleetRouting {
    /// Returns a builder for [FleetRouting].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_optimization_v1::client::FleetRouting;
    /// let client = FleetRouting::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::fleet_routing::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::fleet_routing::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::FleetRouting + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::FleetRouting>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::FleetRouting> {
        super::transport::FleetRouting::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::FleetRouting> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::FleetRouting::new)
    }

    /// Sends an `OptimizeToursRequest` containing a `ShipmentModel` and returns an
    /// `OptimizeToursResponse` containing `ShipmentRoute`s, which are a set of
    /// routes to be performed by vehicles minimizing the overall cost.
    ///
    /// A `ShipmentModel` model consists mainly of `Shipment`s that need to be
    /// carried out and `Vehicle`s that can be used to transport the `Shipment`s.
    /// The `ShipmentRoute`s assign `Shipment`s to `Vehicle`s. More specifically,
    /// they assign a series of `Visit`s to each vehicle, where a `Visit`
    /// corresponds to a `VisitRequest`, which is a pickup or delivery for a
    /// `Shipment`.
    ///
    /// The goal is to provide an assignment of `ShipmentRoute`s to `Vehicle`s that
    /// minimizes the total cost where cost has many components defined in the
    /// `ShipmentModel`.
    pub fn optimize_tours(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::fleet_routing::OptimizeTours {
        super::builder::fleet_routing::OptimizeTours::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Optimizes vehicle tours for one or more `OptimizeToursRequest`
    /// messages as a batch.
    ///
    /// This method is a Long Running Operation (LRO). The inputs for optimization
    /// (`OptimizeToursRequest` messages) and outputs (`OptimizeToursResponse`
    /// messages) are read/written from/to Cloud Storage in user-specified
    /// format. Like the `OptimizeTours` method, each `OptimizeToursRequest`
    /// contains a `ShipmentModel` and returns an `OptimizeToursResponse`
    /// containing `ShipmentRoute`s, which are a set of routes to be performed by
    /// vehicles minimizing the overall cost.
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
    pub fn batch_optimize_tours(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::fleet_routing::BatchOptimizeTours {
        super::builder::fleet_routing::BatchOptimizeTours::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::fleet_routing::GetOperation {
        super::builder::fleet_routing::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
