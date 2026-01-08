// Copyright 2023 The RocketMQ Rust Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cheetah_string::CheetahString;
use rocketmq_macros::RequestHeaderCodecV2;
use serde::Deserialize;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, RequestHeaderCodecV2)]
#[serde(rename_all = "camelCase")]
pub struct CleanBrokerDataRequestHeader {
    cluster_name: Option<CheetahString>,
    #[required]
    broker_name: CheetahString,
    broker_controller_ids_to_clean: Option<CheetahString>,
    #[required]
    is_clean_living_broker: bool,
    invoke_time: SystemTime,
}
impl Default for CleanBrokerDataRequestHeader {
    fn default() -> Self {
        Self {
            cluster_name: None,
            broker_name: Default::default(),
            broker_controller_ids_to_clean: None,
            is_clean_living_broker: false,
            invoke_time: SystemTime::now(),
        }
    }
}
