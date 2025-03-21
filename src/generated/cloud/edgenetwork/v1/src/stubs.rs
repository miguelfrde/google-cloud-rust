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
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::EdgeNetwork].
///
/// Application developers may need to implement this trait to mock
/// `client::EdgeNetwork`.  In other use-cases, application developers only
/// use `client::EdgeNetwork` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait EdgeNetwork: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::EdgeNetwork::initialize_zone].
    fn initialize_zone(
        &self,
        _req: crate::model::InitializeZoneRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::InitializeZoneResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::InitializeZoneResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeNetwork::list_zones].
    fn list_zones(
        &self,
        _req: crate::model::ListZonesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListZonesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListZonesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::get_zone].
    fn get_zone(
        &self,
        _req: crate::model::GetZoneRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Zone>> + Send {
        std::future::ready::<crate::Result<crate::model::Zone>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::EdgeNetwork::list_networks].
    fn list_networks(
        &self,
        _req: crate::model::ListNetworksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListNetworksResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListNetworksResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::get_network].
    fn get_network(
        &self,
        _req: crate::model::GetNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Network>> + Send {
        std::future::ready::<crate::Result<crate::model::Network>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::diagnose_network].
    fn diagnose_network(
        &self,
        _req: crate::model::DiagnoseNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::DiagnoseNetworkResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::DiagnoseNetworkResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeNetwork::create_network].
    fn create_network(
        &self,
        _req: crate::model::CreateNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::delete_network].
    fn delete_network(
        &self,
        _req: crate::model::DeleteNetworkRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::list_subnets].
    fn list_subnets(
        &self,
        _req: crate::model::ListSubnetsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSubnetsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListSubnetsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::get_subnet].
    fn get_subnet(
        &self,
        _req: crate::model::GetSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Subnet>> + Send {
        std::future::ready::<crate::Result<crate::model::Subnet>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::create_subnet].
    fn create_subnet(
        &self,
        _req: crate::model::CreateSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::update_subnet].
    fn update_subnet(
        &self,
        _req: crate::model::UpdateSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::delete_subnet].
    fn delete_subnet(
        &self,
        _req: crate::model::DeleteSubnetRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::list_interconnects].
    fn list_interconnects(
        &self,
        _req: crate::model::ListInterconnectsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListInterconnectsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListInterconnectsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeNetwork::get_interconnect].
    fn get_interconnect(
        &self,
        _req: crate::model::GetInterconnectRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Interconnect>> + Send {
        std::future::ready::<crate::Result<crate::model::Interconnect>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::diagnose_interconnect].
    fn diagnose_interconnect(
        &self,
        _req: crate::model::DiagnoseInterconnectRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::DiagnoseInterconnectResponse>>
    + Send {
        std::future::ready::<crate::Result<crate::model::DiagnoseInterconnectResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeNetwork::list_interconnect_attachments].
    fn list_interconnect_attachments(
        &self,
        _req: crate::model::ListInterconnectAttachmentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListInterconnectAttachmentsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListInterconnectAttachmentsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeNetwork::get_interconnect_attachment].
    fn get_interconnect_attachment(
        &self,
        _req: crate::model::GetInterconnectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::InterconnectAttachment>> + Send
    {
        std::future::ready::<crate::Result<crate::model::InterconnectAttachment>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeNetwork::create_interconnect_attachment].
    fn create_interconnect_attachment(
        &self,
        _req: crate::model::CreateInterconnectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::delete_interconnect_attachment].
    fn delete_interconnect_attachment(
        &self,
        _req: crate::model::DeleteInterconnectAttachmentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::list_routers].
    fn list_routers(
        &self,
        _req: crate::model::ListRoutersRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRoutersResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRoutersResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::get_router].
    fn get_router(
        &self,
        _req: crate::model::GetRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Router>> + Send {
        std::future::ready::<crate::Result<crate::model::Router>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::diagnose_router].
    fn diagnose_router(
        &self,
        _req: crate::model::DiagnoseRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::DiagnoseRouterResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::DiagnoseRouterResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeNetwork::create_router].
    fn create_router(
        &self,
        _req: crate::model::CreateRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::update_router].
    fn update_router(
        &self,
        _req: crate::model::UpdateRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::delete_router].
    fn delete_router(
        &self,
        _req: crate::model::DeleteRouterRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeNetwork::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
    + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::EdgeNetwork::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::EdgeNetwork::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::EdgeNetwork::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
