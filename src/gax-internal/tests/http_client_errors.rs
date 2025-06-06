// Copyright 2024 Google LLC
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

#[cfg(all(test, feature = "_internal_http_client"))]
mod test {
    use gax::options::*;
    use serde_json::json;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_error_with_status() -> Result<()> {
        use serde_json::Value;
        let (endpoint, _server) = echo_server::start().await?;

        let client = echo_server::builder(endpoint)
            .with_credentials(auth::credentials::testing::test_credentials())
            .build()
            .await?;

        let builder = client.builder(reqwest::Method::GET, "/error".into());
        let body = json!({});
        let response = client
            .execute::<Value, Value>(builder, Some(body), RequestOptions::default())
            .await;

        match response {
            Ok(v) => assert!(false, "expected an error got={v}"),
            Err(e) => {
                let inner = e.as_inner::<gax::error::ServiceError>().unwrap();
                assert_eq!(
                    inner.http_status_code().unwrap_or_default(),
                    axum::http::StatusCode::BAD_REQUEST.as_u16()
                );
                assert!(inner.headers().is_some(), "missing headers in {inner:?}");
                let headers = inner.headers().clone().unwrap();
                assert!(!headers.is_empty(), "empty headers in {inner:?}");
                let got = inner.status();
                let want = echo_server::make_status()?;
                assert_eq!(got, &want);
            }
        }

        Ok(())
    }
}
