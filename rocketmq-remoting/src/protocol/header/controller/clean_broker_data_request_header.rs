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

/// Request header for cleaning broker data from the controller.
///
/// This request is used when a broker is permanently removed from the cluster
/// or during maintenance operations to clean up stale broker metadata.
#[derive(Clone, Debug, Serialize, Deserialize, RequestHeaderCodecV2)]
#[serde(rename_all = "camelCase")]
pub struct CleanBrokerDataRequestHeader {
    /// The cluster name
    pub cluster_name: Option<CheetahString>,

    /// The broker group name to clean
    pub broker_name: Option<CheetahString>,

    /// Optional: Comma-separated list of specific broker IDs to clean.
    /// If not provided, all brokers in the group are considered for cleanup.
    pub broker_controller_ids_to_clean: Option<CheetahString>,

    /// Whether to allow cleaning data for brokers that are still alive.
    /// Default is false for safety.
    pub clean_living_broker: Option<bool>,
}

impl Default for CleanBrokerDataRequestHeader {
    fn default() -> Self {
        Self {
            cluster_name: None,
            broker_name: None,
            broker_controller_ids_to_clean: None,
            clean_living_broker: Some(false),
        }
    }
}

impl CleanBrokerDataRequestHeader {
    /// Create a new CleanBrokerDataRequestHeader
    pub fn new(
        cluster_name: impl Into<CheetahString>,
        broker_name: impl Into<CheetahString>,
    ) -> Self {
        Self {
            cluster_name: Some(cluster_name.into()),
            broker_name: Some(broker_name.into()),
            broker_controller_ids_to_clean: None,
            clean_living_broker: Some(false),
        }
    }

    /// Set the broker controller IDs to clean
    pub fn with_broker_controller_ids(mut self, ids: impl Into<CheetahString>) -> Self {
        self.broker_controller_ids_to_clean = Some(ids.into());
        self
    }

    /// Set whether to clean living brokers
    pub fn with_clean_living_broker(mut self, clean_living: bool) -> Self {
        self.clean_living_broker = Some(clean_living);
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::protocol::command_custom_header::CommandCustomHeader;
    use crate::protocol::command_custom_header::FromMap;

    #[test]
    fn clean_broker_data_request_header_serializes_correctly() {
        let header = CleanBrokerDataRequestHeader {
            cluster_name: Some(CheetahString::from_static_str("test_cluster")),
            broker_name: Some(CheetahString::from_static_str("test_broker")),
            broker_controller_ids_to_clean: Some(CheetahString::from_static_str("1,2,3")),
            clean_living_broker: Some(false),
        };
        let map = header.to_map().unwrap();
        assert_eq!(
            map.get(&CheetahString::from_static_str("clusterName")).unwrap(),
            "test_cluster"
        );
        assert_eq!(
            map.get(&CheetahString::from_static_str("brokerName")).unwrap(),
            "test_broker"
        );
        assert_eq!(
            map.get(&CheetahString::from_static_str("brokerControllerIdsToClean")).unwrap(),
            "1,2,3"
        );
        assert_eq!(
            map.get(&CheetahString::from_static_str("cleanLivingBroker")).unwrap(),
            "false"
        );
    }

    #[test]
    fn clean_broker_data_request_header_deserializes_correctly() {
        let mut map = HashMap::new();
        map.insert(
            CheetahString::from_static_str("clusterName"),
            CheetahString::from_static_str("test_cluster"),
        );
        map.insert(
            CheetahString::from_static_str("brokerName"),
            CheetahString::from_static_str("test_broker"),
        );
        map.insert(
            CheetahString::from_static_str("brokerControllerIdsToClean"),
            CheetahString::from_static_str("1,2,3"),
        );
        map.insert(
            CheetahString::from_static_str("cleanLivingBroker"),
            CheetahString::from_static_str("false"),
        );

        let header = CleanBrokerDataRequestHeader::from_map(map).unwrap();
        assert_eq!(header.cluster_name.unwrap(), "test_cluster");
        assert_eq!(header.broker_name.unwrap(), "test_broker");
        assert_eq!(header.broker_controller_ids_to_clean.unwrap(), "1,2,3");
        assert_eq!(header.clean_living_broker.unwrap(), false);
    }

    #[test]
    fn clean_broker_data_request_header_default() {
        let header = CleanBrokerDataRequestHeader::default();
        assert!(header.cluster_name.is_none());
        assert!(header.broker_name.is_none());
        assert!(header.broker_controller_ids_to_clean.is_none());
        assert_eq!(header.clean_living_broker, Some(false));
    }

    #[test]
    fn clean_broker_data_request_header_builder() {
        let header = CleanBrokerDataRequestHeader::new("cluster1", "broker1")
            .with_broker_controller_ids("1,2")
            .with_clean_living_broker(true);

        assert_eq!(header.cluster_name.unwrap(), "cluster1");
        assert_eq!(header.broker_name.unwrap(), "broker1");
        assert_eq!(header.broker_controller_ids_to_clean.unwrap(), "1,2");
        assert_eq!(header.clean_living_broker, Some(true));
    }
}
