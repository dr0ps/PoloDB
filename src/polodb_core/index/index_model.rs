// Copyright 2024 Vincent Chan
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexModel {
    #[serde(rename = "key")]
    pub keys: Document,

    /// The options for the index.
    #[serde(flatten)]
    pub options: Option<IndexOptions>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexOptions {

    /// Specifies a name outside the default generated name.
    pub name: Option<String>,

    /// Forces the index to be unique so the collection will not accept documents where the index
    /// key value matches an existing value in the index. The default value is false.
    pub unique: Option<bool>,

}
