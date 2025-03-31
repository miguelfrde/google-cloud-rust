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

/// Implements a client for the Cloud Translation API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_translation_v3::client::TranslationService;
/// let client = TranslationService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// Provides natural language translation operations.
///
/// # Configuration
///
/// To configure `TranslationService` use the `with_*` methods in the type returned
/// by [builder()][TranslationService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://translate.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::translation_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::translation_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `TranslationService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `TranslationService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct TranslationService {
    inner: Arc<dyn super::stub::dynamic::TranslationService>,
}

impl TranslationService {
    /// Returns a builder for [TranslationService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_translation_v3::client::TranslationService;
    /// let client = TranslationService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::translation_service::ClientBuilder {
        gax::client_builder::internal::new_builder(
            super::builder::translation_service::client::Factory,
        )
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::TranslationService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::TranslationService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::TranslationService> {
        super::transport::TranslationService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::TranslationService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::TranslationService::new)
    }

    /// Translates input text and returns translated text.
    pub fn translate_text(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::TranslateText {
        super::builder::translation_service::TranslateText::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Romanize input text written in non-Latin scripts to Latin text.
    pub fn romanize_text(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::RomanizeText {
        super::builder::translation_service::RomanizeText::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Detects the language of text within a request.
    pub fn detect_language(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::DetectLanguage {
        super::builder::translation_service::DetectLanguage::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns a list of supported languages for translation.
    pub fn get_supported_languages(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::GetSupportedLanguages {
        super::builder::translation_service::GetSupportedLanguages::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Translates documents in synchronous mode.
    pub fn translate_document(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::TranslateDocument {
        super::builder::translation_service::TranslateDocument::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Translates a large volume of text in asynchronous batch mode.
    /// This function provides real-time output as the inputs are being processed.
    /// If caller cancels a request, the partial results (for an input file, it's
    /// all or nothing) may still be available on the specified output location.
    ///
    /// This call returns immediately and you can
    /// use google.longrunning.Operation.name to poll the status of the call.
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
    pub fn batch_translate_text(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::BatchTranslateText {
        super::builder::translation_service::BatchTranslateText::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Translates a large volume of document in asynchronous batch mode.
    /// This function provides real-time output as the inputs are being processed.
    /// If caller cancels a request, the partial results (for an input file, it's
    /// all or nothing) may still be available on the specified output location.
    ///
    /// This call returns immediately and you can use
    /// google.longrunning.Operation.name to poll the status of the call.
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
    pub fn batch_translate_document(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::BatchTranslateDocument {
        super::builder::translation_service::BatchTranslateDocument::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a glossary and returns the long-running operation. Returns
    /// NOT_FOUND, if the project doesn't exist.
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
    pub fn create_glossary(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::CreateGlossary {
        super::builder::translation_service::CreateGlossary::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a glossary. A LRO is used since the update can be async if the
    /// glossary's entry file is updated.
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
    pub fn update_glossary(
        &self,
        glossary: impl Into<crate::model::Glossary>,
    ) -> super::builder::translation_service::UpdateGlossary {
        super::builder::translation_service::UpdateGlossary::new(self.inner.clone())
            .set_glossary(glossary.into())
    }

    /// Lists glossaries in a project. Returns NOT_FOUND, if the project doesn't
    /// exist.
    pub fn list_glossaries(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListGlossaries {
        super::builder::translation_service::ListGlossaries::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a glossary. Returns NOT_FOUND, if the glossary doesn't
    /// exist.
    pub fn get_glossary(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::GetGlossary {
        super::builder::translation_service::GetGlossary::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a glossary, or cancels glossary construction
    /// if the glossary isn't created yet.
    /// Returns NOT_FOUND, if the glossary doesn't exist.
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
    pub fn delete_glossary(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::DeleteGlossary {
        super::builder::translation_service::DeleteGlossary::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a single glossary entry by the given id.
    pub fn get_glossary_entry(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::GetGlossaryEntry {
        super::builder::translation_service::GetGlossaryEntry::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List the entries for the glossary.
    pub fn list_glossary_entries(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListGlossaryEntries {
        super::builder::translation_service::ListGlossaryEntries::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a glossary entry.
    pub fn create_glossary_entry(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::CreateGlossaryEntry {
        super::builder::translation_service::CreateGlossaryEntry::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a glossary entry.
    pub fn update_glossary_entry(
        &self,
        glossary_entry: impl Into<crate::model::GlossaryEntry>,
    ) -> super::builder::translation_service::UpdateGlossaryEntry {
        super::builder::translation_service::UpdateGlossaryEntry::new(self.inner.clone())
            .set_glossary_entry(glossary_entry.into())
    }

    /// Deletes a single entry from the glossary
    pub fn delete_glossary_entry(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::DeleteGlossaryEntry {
        super::builder::translation_service::DeleteGlossaryEntry::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a Dataset.
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
    pub fn create_dataset(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::CreateDataset {
        super::builder::translation_service::CreateDataset::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a Dataset.
    pub fn get_dataset(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::GetDataset {
        super::builder::translation_service::GetDataset::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists datasets.
    pub fn list_datasets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListDatasets {
        super::builder::translation_service::ListDatasets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a dataset and all of its contents.
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
    pub fn delete_dataset(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::DeleteDataset {
        super::builder::translation_service::DeleteDataset::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates an Adaptive MT dataset.
    pub fn create_adaptive_mt_dataset(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::CreateAdaptiveMtDataset {
        super::builder::translation_service::CreateAdaptiveMtDataset::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes an Adaptive MT dataset, including all its entries and associated
    /// metadata.
    pub fn delete_adaptive_mt_dataset(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::DeleteAdaptiveMtDataset {
        super::builder::translation_service::DeleteAdaptiveMtDataset::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets the Adaptive MT dataset.
    pub fn get_adaptive_mt_dataset(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::GetAdaptiveMtDataset {
        super::builder::translation_service::GetAdaptiveMtDataset::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all Adaptive MT datasets for which the caller has read permission.
    pub fn list_adaptive_mt_datasets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListAdaptiveMtDatasets {
        super::builder::translation_service::ListAdaptiveMtDatasets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Translate text using Adaptive MT.
    pub fn adaptive_mt_translate(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::AdaptiveMtTranslate {
        super::builder::translation_service::AdaptiveMtTranslate::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets and AdaptiveMtFile
    pub fn get_adaptive_mt_file(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::GetAdaptiveMtFile {
        super::builder::translation_service::GetAdaptiveMtFile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes an AdaptiveMtFile along with its sentences.
    pub fn delete_adaptive_mt_file(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::DeleteAdaptiveMtFile {
        super::builder::translation_service::DeleteAdaptiveMtFile::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Imports an AdaptiveMtFile and adds all of its sentences into the
    /// AdaptiveMtDataset.
    pub fn import_adaptive_mt_file(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ImportAdaptiveMtFile {
        super::builder::translation_service::ImportAdaptiveMtFile::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists all AdaptiveMtFiles associated to an AdaptiveMtDataset.
    pub fn list_adaptive_mt_files(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListAdaptiveMtFiles {
        super::builder::translation_service::ListAdaptiveMtFiles::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists all AdaptiveMtSentences under a given file/dataset.
    pub fn list_adaptive_mt_sentences(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListAdaptiveMtSentences {
        super::builder::translation_service::ListAdaptiveMtSentences::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Import sentence pairs into translation Dataset.
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
    pub fn import_data(
        &self,
        dataset: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ImportData {
        super::builder::translation_service::ImportData::new(self.inner.clone())
            .set_dataset(dataset.into())
    }

    /// Exports dataset's data to the provided output location.
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
    pub fn export_data(
        &self,
        dataset: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ExportData {
        super::builder::translation_service::ExportData::new(self.inner.clone())
            .set_dataset(dataset.into())
    }

    /// Lists sentence pairs in the dataset.
    pub fn list_examples(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListExamples {
        super::builder::translation_service::ListExamples::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a Model.
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
    pub fn create_model(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::CreateModel {
        super::builder::translation_service::CreateModel::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists models.
    pub fn list_models(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListModels {
        super::builder::translation_service::ListModels::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a model.
    pub fn get_model(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::GetModel {
        super::builder::translation_service::GetModel::new(self.inner.clone()).set_name(name.into())
    }

    /// Deletes a model.
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
    pub fn delete_model(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::DeleteModel {
        super::builder::translation_service::DeleteModel::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListLocations {
        super::builder::translation_service::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::GetLocation {
        super::builder::translation_service::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::ListOperations {
        super::builder::translation_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::GetOperation {
        super::builder::translation_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::DeleteOperation {
        super::builder::translation_service::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::CancelOperation {
        super::builder::translation_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn wait_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::translation_service::WaitOperation {
        super::builder::translation_service::WaitOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
