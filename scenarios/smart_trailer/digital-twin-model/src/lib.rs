// Copyright (c) Microsoft Corporation.
// Licensed under the Apache License, Version 2.0.
// SPDX-License-Identifier: Apache-2.0

pub mod trailer_v1;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "$model")]
    pub model: String,
}
