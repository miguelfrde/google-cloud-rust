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

/// Implements a [Datastream](crate::stubs::Datastream) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Datastream<T>
where
    T: crate::stubs::Datastream + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Datastream<T>
where
    T: crate::stubs::Datastream + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::Datastream for Datastream<T>
where
    T: crate::stubs::Datastream + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_connection_profiles(
        &self,
        req: crate::model::ListConnectionProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListConnectionProfilesResponse> {
        self.inner.list_connection_profiles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_connection_profile(
        &self,
        req: crate::model::GetConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ConnectionProfile> {
        self.inner.get_connection_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_connection_profile(
        &self,
        req: crate::model::CreateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_connection_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_connection_profile(
        &self,
        req: crate::model::UpdateConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_connection_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_connection_profile(
        &self,
        req: crate::model::DeleteConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_connection_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn discover_connection_profile(
        &self,
        req: crate::model::DiscoverConnectionProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DiscoverConnectionProfileResponse> {
        self.inner.discover_connection_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_streams(
        &self,
        req: crate::model::ListStreamsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListStreamsResponse> {
        self.inner.list_streams(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_stream(
        &self,
        req: crate::model::GetStreamRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Stream> {
        self.inner.get_stream(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_stream(
        &self,
        req: crate::model::CreateStreamRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_stream(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_stream(
        &self,
        req: crate::model::UpdateStreamRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_stream(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_stream(
        &self,
        req: crate::model::DeleteStreamRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_stream(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn run_stream(
        &self,
        req: crate::model::RunStreamRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.run_stream(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_stream_object(
        &self,
        req: crate::model::GetStreamObjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StreamObject> {
        self.inner.get_stream_object(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn lookup_stream_object(
        &self,
        req: crate::model::LookupStreamObjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StreamObject> {
        self.inner.lookup_stream_object(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_stream_objects(
        &self,
        req: crate::model::ListStreamObjectsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListStreamObjectsResponse> {
        self.inner.list_stream_objects(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn start_backfill_job(
        &self,
        req: crate::model::StartBackfillJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StartBackfillJobResponse> {
        self.inner.start_backfill_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn stop_backfill_job(
        &self,
        req: crate::model::StopBackfillJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StopBackfillJobResponse> {
        self.inner.stop_backfill_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn fetch_static_ips(
        &self,
        req: crate::model::FetchStaticIpsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::FetchStaticIpsResponse> {
        self.inner.fetch_static_ips(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_private_connection(
        &self,
        req: crate::model::CreatePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_private_connection(
        &self,
        req: crate::model::GetPrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PrivateConnection> {
        self.inner.get_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_private_connections(
        &self,
        req: crate::model::ListPrivateConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPrivateConnectionsResponse> {
        self.inner.list_private_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_private_connection(
        &self,
        req: crate::model::DeletePrivateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_private_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_route(
        &self,
        req: crate::model::CreateRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_route(
        &self,
        req: crate::model::GetRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Route> {
        self.inner.get_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_routes(
        &self,
        req: crate::model::ListRoutesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRoutesResponse> {
        self.inner.list_routes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_route(
        &self,
        req: crate::model::DeleteRouteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_route(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
