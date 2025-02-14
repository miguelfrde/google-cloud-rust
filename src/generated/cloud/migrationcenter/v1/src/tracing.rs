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

/// Implements a [MigrationCenter](crate::stubs::MigrationCenter) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct MigrationCenter<T>
where
    T: crate::stubs::MigrationCenter + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> MigrationCenter<T>
where
    T: crate::stubs::MigrationCenter + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::MigrationCenter for MigrationCenter<T>
where
    T: crate::stubs::MigrationCenter + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_assets(
        &self,
        req: crate::model::ListAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAssetsResponse> {
        self.inner.list_assets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_asset(
        &self,
        req: crate::model::GetAssetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Asset> {
        self.inner.get_asset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_asset(
        &self,
        req: crate::model::UpdateAssetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Asset> {
        self.inner.update_asset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_update_assets(
        &self,
        req: crate::model::BatchUpdateAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchUpdateAssetsResponse> {
        self.inner.batch_update_assets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_asset(
        &self,
        req: crate::model::DeleteAssetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_asset(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn batch_delete_assets(
        &self,
        req: crate::model::BatchDeleteAssetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.batch_delete_assets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn report_asset_frames(
        &self,
        req: crate::model::ReportAssetFramesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ReportAssetFramesResponse> {
        self.inner.report_asset_frames(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn aggregate_assets_values(
        &self,
        req: crate::model::AggregateAssetsValuesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AggregateAssetsValuesResponse> {
        self.inner.aggregate_assets_values(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_import_job(
        &self,
        req: crate::model::CreateImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_import_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_import_jobs(
        &self,
        req: crate::model::ListImportJobsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListImportJobsResponse> {
        self.inner.list_import_jobs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_import_job(
        &self,
        req: crate::model::GetImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ImportJob> {
        self.inner.get_import_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_import_job(
        &self,
        req: crate::model::DeleteImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_import_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_import_job(
        &self,
        req: crate::model::UpdateImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_import_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn validate_import_job(
        &self,
        req: crate::model::ValidateImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.validate_import_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn run_import_job(
        &self,
        req: crate::model::RunImportJobRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.run_import_job(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_import_data_file(
        &self,
        req: crate::model::GetImportDataFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ImportDataFile> {
        self.inner.get_import_data_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_import_data_files(
        &self,
        req: crate::model::ListImportDataFilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListImportDataFilesResponse> {
        self.inner.list_import_data_files(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_import_data_file(
        &self,
        req: crate::model::CreateImportDataFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_import_data_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_import_data_file(
        &self,
        req: crate::model::DeleteImportDataFileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_import_data_file(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_groups(
        &self,
        req: crate::model::ListGroupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListGroupsResponse> {
        self.inner.list_groups(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_group(
        &self,
        req: crate::model::GetGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Group> {
        self.inner.get_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_group(
        &self,
        req: crate::model::CreateGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_group(
        &self,
        req: crate::model::UpdateGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_group(
        &self,
        req: crate::model::DeleteGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn add_assets_to_group(
        &self,
        req: crate::model::AddAssetsToGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.add_assets_to_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn remove_assets_from_group(
        &self,
        req: crate::model::RemoveAssetsFromGroupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.remove_assets_from_group(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_error_frames(
        &self,
        req: crate::model::ListErrorFramesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListErrorFramesResponse> {
        self.inner.list_error_frames(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_error_frame(
        &self,
        req: crate::model::GetErrorFrameRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ErrorFrame> {
        self.inner.get_error_frame(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_sources(
        &self,
        req: crate::model::ListSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSourcesResponse> {
        self.inner.list_sources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_source(
        &self,
        req: crate::model::GetSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Source> {
        self.inner.get_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_source(
        &self,
        req: crate::model::CreateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_source(
        &self,
        req: crate::model::UpdateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_source(
        &self,
        req: crate::model::DeleteSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_preference_sets(
        &self,
        req: crate::model::ListPreferenceSetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListPreferenceSetsResponse> {
        self.inner.list_preference_sets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_preference_set(
        &self,
        req: crate::model::GetPreferenceSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::PreferenceSet> {
        self.inner.get_preference_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_preference_set(
        &self,
        req: crate::model::CreatePreferenceSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_preference_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_preference_set(
        &self,
        req: crate::model::UpdatePreferenceSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_preference_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_preference_set(
        &self,
        req: crate::model::DeletePreferenceSetRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_preference_set(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_settings(
        &self,
        req: crate::model::GetSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Settings> {
        self.inner.get_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_settings(
        &self,
        req: crate::model::UpdateSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_settings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_report_config(
        &self,
        req: crate::model::CreateReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_report_config(
        &self,
        req: crate::model::GetReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ReportConfig> {
        self.inner.get_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_report_configs(
        &self,
        req: crate::model::ListReportConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListReportConfigsResponse> {
        self.inner.list_report_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_report_config(
        &self,
        req: crate::model::DeleteReportConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_report_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_report(
        &self,
        req: crate::model::CreateReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_report(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_report(
        &self,
        req: crate::model::GetReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Report> {
        self.inner.get_report(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_reports(
        &self,
        req: crate::model::ListReportsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListReportsResponse> {
        self.inner.list_reports(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_report(
        &self,
        req: crate::model::DeleteReportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_report(req, options).await
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
