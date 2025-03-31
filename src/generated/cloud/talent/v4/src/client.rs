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

/// Implements a client for the Cloud Talent Solution API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_talent_v4::client::CompanyService;
/// let client = CompanyService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service that handles company management, including CRUD and enumeration.
///
/// # Configuration
///
/// To configure `CompanyService` use the `with_*` methods in the type returned
/// by [builder()][CompanyService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://jobs.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::company_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::company_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `CompanyService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CompanyService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CompanyService {
    inner: Arc<dyn super::stub::dynamic::CompanyService>,
}

impl CompanyService {
    /// Returns a builder for [CompanyService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_talent_v4::client::CompanyService;
    /// let client = CompanyService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::company_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::company_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::CompanyService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::CompanyService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::CompanyService> {
        super::transport::CompanyService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::CompanyService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::CompanyService::new)
    }

    /// Creates a new company entity.
    pub fn create_company(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::company_service::CreateCompany {
        super::builder::company_service::CreateCompany::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves specified company.
    pub fn get_company(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::company_service::GetCompany {
        super::builder::company_service::GetCompany::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates specified company.
    pub fn update_company(
        &self,
        company: impl Into<crate::model::Company>,
    ) -> super::builder::company_service::UpdateCompany {
        super::builder::company_service::UpdateCompany::new(self.inner.clone())
            .set_company(company.into())
    }

    /// Deletes specified company.
    /// Prerequisite: The company has no jobs associated with it.
    pub fn delete_company(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::company_service::DeleteCompany {
        super::builder::company_service::DeleteCompany::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists all companies associated with the project.
    pub fn list_companies(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::company_service::ListCompanies {
        super::builder::company_service::ListCompanies::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::company_service::GetOperation {
        super::builder::company_service::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Cloud Talent Solution API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_talent_v4::client::Completion;
/// let client = Completion::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service handles auto completion.
///
/// # Configuration
///
/// To configure `Completion` use the `with_*` methods in the type returned
/// by [builder()][Completion::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://jobs.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::completion::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::completion::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `Completion` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Completion` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Completion {
    inner: Arc<dyn super::stub::dynamic::Completion>,
}

impl Completion {
    /// Returns a builder for [Completion].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_talent_v4::client::Completion;
    /// let client = Completion::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::completion::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::completion::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::Completion + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::Completion>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::Completion> {
        super::transport::Completion::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::Completion> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Completion::new)
    }

    /// Completes the specified prefix with keyword suggestions.
    /// Intended for use by a job search auto-complete search box.
    pub fn complete_query(
        &self,
        tenant: impl Into<std::string::String>,
    ) -> super::builder::completion::CompleteQuery {
        super::builder::completion::CompleteQuery::new(self.inner.clone()).set_tenant(tenant.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::completion::GetOperation {
        super::builder::completion::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Cloud Talent Solution API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_talent_v4::client::EventService;
/// let client = EventService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service handles client event report.
///
/// # Configuration
///
/// To configure `EventService` use the `with_*` methods in the type returned
/// by [builder()][EventService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://jobs.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::event_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::event_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `EventService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `EventService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct EventService {
    inner: Arc<dyn super::stub::dynamic::EventService>,
}

impl EventService {
    /// Returns a builder for [EventService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_talent_v4::client::EventService;
    /// let client = EventService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::event_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::event_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::EventService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::EventService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::EventService> {
        super::transport::EventService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::EventService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::EventService::new)
    }

    /// Report events issued when end user interacts with customer's application
    /// that uses Cloud Talent Solution. You may inspect the created events in
    /// [self service
    /// tools](https://console.cloud.google.com/talent-solution/overview).
    /// [Learn
    /// more](https://cloud.google.com/talent-solution/docs/management-tools)
    /// about self service tools.
    pub fn create_client_event(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::event_service::CreateClientEvent {
        super::builder::event_service::CreateClientEvent::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::event_service::GetOperation {
        super::builder::event_service::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Cloud Talent Solution API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_talent_v4::client::JobService;
/// let client = JobService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service handles job management, including job CRUD, enumeration and search.
///
/// # Configuration
///
/// To configure `JobService` use the `with_*` methods in the type returned
/// by [builder()][JobService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://jobs.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::job_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::job_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `JobService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `JobService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct JobService {
    inner: Arc<dyn super::stub::dynamic::JobService>,
}

impl JobService {
    /// Returns a builder for [JobService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_talent_v4::client::JobService;
    /// let client = JobService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::job_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::job_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::JobService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::JobService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::JobService> {
        super::transport::JobService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::JobService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::JobService::new)
    }

    /// Creates a new job.
    ///
    /// Typically, the job becomes searchable within 10 seconds, but it may take
    /// up to 5 minutes.
    pub fn create_job(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::job_service::CreateJob {
        super::builder::job_service::CreateJob::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Begins executing a batch create jobs operation.
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
    pub fn batch_create_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::job_service::BatchCreateJobs {
        super::builder::job_service::BatchCreateJobs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves the specified job, whose status is OPEN or recently EXPIRED
    /// within the last 90 days.
    pub fn get_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::job_service::GetJob {
        super::builder::job_service::GetJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates specified job.
    ///
    /// Typically, updated contents become visible in search results within 10
    /// seconds, but it may take up to 5 minutes.
    pub fn update_job(
        &self,
        job: impl Into<crate::model::Job>,
    ) -> super::builder::job_service::UpdateJob {
        super::builder::job_service::UpdateJob::new(self.inner.clone()).set_job(job.into())
    }

    /// Begins executing a batch update jobs operation.
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
    pub fn batch_update_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::job_service::BatchUpdateJobs {
        super::builder::job_service::BatchUpdateJobs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes the specified job.
    ///
    /// Typically, the job becomes unsearchable within 10 seconds, but it may take
    /// up to 5 minutes.
    pub fn delete_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::job_service::DeleteJob {
        super::builder::job_service::DeleteJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Begins executing a batch delete jobs operation.
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
    pub fn batch_delete_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::job_service::BatchDeleteJobs {
        super::builder::job_service::BatchDeleteJobs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists jobs by filter.
    pub fn list_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::job_service::ListJobs {
        super::builder::job_service::ListJobs::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Searches for jobs using the provided
    /// [SearchJobsRequest][google.cloud.talent.v4.SearchJobsRequest].
    ///
    /// This call constrains the
    /// [visibility][google.cloud.talent.v4.Job.visibility] of jobs present in the
    /// database, and only returns jobs that the caller has permission to search
    /// against.
    ///
    /// [google.cloud.talent.v4.Job.visibility]: crate::model::Job::visibility
    /// [google.cloud.talent.v4.SearchJobsRequest]: crate::model::SearchJobsRequest
    pub fn search_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::job_service::SearchJobs {
        super::builder::job_service::SearchJobs::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Searches for jobs using the provided
    /// [SearchJobsRequest][google.cloud.talent.v4.SearchJobsRequest].
    ///
    /// This API call is intended for the use case of targeting passive job
    /// seekers (for example, job seekers who have signed up to receive email
    /// alerts about potential job opportunities), it has different algorithmic
    /// adjustments that are designed to specifically target passive job seekers.
    ///
    /// This call constrains the
    /// [visibility][google.cloud.talent.v4.Job.visibility] of jobs present in the
    /// database, and only returns jobs the caller has permission to search
    /// against.
    ///
    /// [google.cloud.talent.v4.Job.visibility]: crate::model::Job::visibility
    /// [google.cloud.talent.v4.SearchJobsRequest]: crate::model::SearchJobsRequest
    pub fn search_jobs_for_alert(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::job_service::SearchJobsForAlert {
        super::builder::job_service::SearchJobsForAlert::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::job_service::GetOperation {
        super::builder::job_service::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Cloud Talent Solution API.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// # use google_cloud_talent_v4::client::TenantService;
/// let client = TenantService::builder().build().await?;
/// // use `client` to make requests to the {Codec.APITitle}}.
/// # gax::Result::<()>::Ok(()) });
/// ```
///
/// # Service Description
///
/// A service that handles tenant management, including CRUD and enumeration.
///
/// # Configuration
///
/// To configure `TenantService` use the `with_*` methods in the type returned
/// by [builder()][TenantService::builder]. The default configuration should
/// work for most applications. Common configuration changes include
///
/// * [with_endpoint()]: by default this client uses the global default endpoint
///   (`https://jobs.googleapis.com`). Applications using regional
///   endpoints or running in restricted networks (e.g. a network configured
//    with [Private Google Access with VPC Service Controls]) may want to
///   override this default.
/// * [with_credentials()]: by default this client uses
///   [Application Default Credentials]. Applications using custom
///   authentication may need to override this default.
///
/// [with_endpoint()]: super::builder::tenant_service::ClientBuilder::with_endpoint
/// [with_credentials()]: super::builder::tenant_service::ClientBuilder::credentials
/// [Private Google Access with VPC Service Controls]: https://cloud.google.com/vpc-service-controls/docs/private-connectivity
/// [Application Default Credentials]: https://cloud.google.com/docs/authentication#adc
///
/// # Pooling and Cloning
///
/// `TenantService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `TenantService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct TenantService {
    inner: Arc<dyn super::stub::dynamic::TenantService>,
}

impl TenantService {
    /// Returns a builder for [TenantService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_talent_v4::client::TenantService;
    /// let client = TenantService::builder().build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub fn builder() -> super::builder::tenant_service::ClientBuilder {
        gax::client_builder::internal::new_builder(super::builder::tenant_service::client::Factory)
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is in tests mocking the
    /// client's behavior.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::TenantService + 'static,
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
    ) -> Result<Arc<dyn super::stub::dynamic::TenantService>> {
        if gaxi::options::tracing_enabled(&conf) {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::TenantService> {
        super::transport::TenantService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gaxi::options::ClientConfig,
    ) -> Result<impl super::stub::TenantService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::TenantService::new)
    }

    /// Creates a new tenant entity.
    pub fn create_tenant(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::tenant_service::CreateTenant {
        super::builder::tenant_service::CreateTenant::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves specified tenant.
    pub fn get_tenant(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::tenant_service::GetTenant {
        super::builder::tenant_service::GetTenant::new(self.inner.clone()).set_name(name.into())
    }

    /// Updates specified tenant.
    pub fn update_tenant(
        &self,
        tenant: impl Into<crate::model::Tenant>,
    ) -> super::builder::tenant_service::UpdateTenant {
        super::builder::tenant_service::UpdateTenant::new(self.inner.clone())
            .set_tenant(tenant.into())
    }

    /// Deletes specified tenant.
    pub fn delete_tenant(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::tenant_service::DeleteTenant {
        super::builder::tenant_service::DeleteTenant::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists all tenants associated with the project.
    pub fn list_tenants(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::tenant_service::ListTenants {
        super::builder::tenant_service::ListTenants::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::tenant_service::GetOperation {
        super::builder::tenant_service::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
