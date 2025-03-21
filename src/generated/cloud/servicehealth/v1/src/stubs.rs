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

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::ServiceHealth].
///
/// Application developers may need to implement this trait to mock
/// `client::ServiceHealth`.  In other use-cases, application developers only
/// use `client::ServiceHealth` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait ServiceHealth: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::ServiceHealth::list_events].
    fn list_events(
        &self,
        _req: crate::model::ListEventsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListEventsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListEventsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceHealth::get_event].
    fn get_event(
        &self,
        _req: crate::model::GetEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Event>> + Send {
        std::future::ready::<crate::Result<crate::model::Event>>(Err(Error::other("unimplemented")))
    }

    /// Implements [super::client::ServiceHealth::list_organization_events].
    fn list_organization_events(
        &self,
        _req: crate::model::ListOrganizationEventsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListOrganizationEventsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListOrganizationEventsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ServiceHealth::get_organization_event].
    fn get_organization_event(
        &self,
        _req: crate::model::GetOrganizationEventRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::OrganizationEvent>> + Send
    {
        std::future::ready::<crate::Result<crate::model::OrganizationEvent>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceHealth::list_organization_impacts].
    fn list_organization_impacts(
        &self,
        _req: crate::model::ListOrganizationImpactsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<crate::model::ListOrganizationImpactsResponse>,
    > + Send {
        std::future::ready::<crate::Result<crate::model::ListOrganizationImpactsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [super::client::ServiceHealth::get_organization_impact].
    fn get_organization_impact(
        &self,
        _req: crate::model::GetOrganizationImpactRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::OrganizationImpact>> + Send
    {
        std::future::ready::<crate::Result<crate::model::OrganizationImpact>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [super::client::ServiceHealth::list_locations].
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

    /// Implements [super::client::ServiceHealth::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }
}
