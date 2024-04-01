use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::Debug;

use crate::api::{
    catalog::common::Node, check::common::HealthCheck, service::common::AgentService,
};

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct HealthServiceChecksInfo {
    pub node: Node,
    pub service: AgentService,
    pub checks: Vec<HealthCheck>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct HealthStateChecksInfo {
    pub node: Option<String>,
    #[serde(rename = "CheckID")]
    pub check_id: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub output: Option<String>,
    #[serde(rename = "ServiceID")]
    pub service_id: Option<String>,
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceTags")]
    pub service_tags: Option<Vec<String>>,
    pub namespace: Option<String>,
}
