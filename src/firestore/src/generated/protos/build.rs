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

fn main() {
    #[cfg(feature = "_generate-protos")]
    {
        let root = std::env::var("GOOGLEAPIS_ROOT")
            .expect("GOOGLEAPIS_ROOT must be set");
        let files = &[
            "google/firestore/v1/aggregation_result.proto",
            "google/firestore/v1/bloom_filter.proto",
            "google/firestore/v1/common.proto",
            "google/firestore/v1/document.proto",
            "google/firestore/v1/firestore.proto",
            "google/firestore/v1/query.proto",
            "google/firestore/v1/query_profile.proto",
            "google/firestore/v1/write.proto",
        ];
        let includes = &[
            &root
        ];
        let mut config = tonic_build::Config::new();
        config.bytes(&["."]);
        config.disable_comments(&["."]);
        config.enable_type_names();
        config.type_name_domain(&["."], "type.googleapis.com");
        config.out_dir(".");
        tonic_build::configure()
            .bytes(&["."])
            .disable_comments("google.firestore.v1.Firestore")
            .disable_comments("google.firestore.v1.Firestore.GetDocument")
            .disable_comments("google.firestore.v1.Firestore.ListDocuments")
            .disable_comments("google.firestore.v1.Firestore.UpdateDocument")
            .disable_comments("google.firestore.v1.Firestore.DeleteDocument")
            .disable_comments("google.firestore.v1.Firestore.BatchGetDocuments")
            .disable_comments("google.firestore.v1.Firestore.BeginTransaction")
            .disable_comments("google.firestore.v1.Firestore.Commit")
            .disable_comments("google.firestore.v1.Firestore.Rollback")
            .disable_comments("google.firestore.v1.Firestore.RunQuery")
            .disable_comments("google.firestore.v1.Firestore.RunAggregationQuery")
            .disable_comments("google.firestore.v1.Firestore.PartitionQuery")
            .disable_comments("google.firestore.v1.Firestore.Write")
            .disable_comments("google.firestore.v1.Firestore.Listen")
            .disable_comments("google.firestore.v1.Firestore.ListCollectionIds")
            .disable_comments("google.firestore.v1.Firestore.BatchWrite")
            .disable_comments("google.firestore.v1.Firestore.CreateDocument")
            .build_server(false)
            .out_dir(".")
            .compile_protos_with_config(config, files, includes)
            .unwrap();
    }
}
