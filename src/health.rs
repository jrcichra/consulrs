use crate::{
    api::{
        self,
        health::{
            common::{HealthServiceChecksInfo, HealthStateChecksInfo},
            requests::{
                ListNodesForServiceRequest, ListNodesForServiceRequestBuilder,
                ListServicesForStateRequest, ListServicesForStateRequestBuilder,
            },
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// List Service Instances for Service.
///
/// See [ListNodesForServiceRequest]
#[instrument(skip(client, opts), err)]
pub async fn list_nodes_for_service(
    client: &impl Client,
    service: &str,
    opts: Option<&mut ListNodesForServiceRequestBuilder>,
) -> Result<ApiResponse<Vec<HealthServiceChecksInfo>>, ClientError> {
    let mut t = ListNodesForServiceRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).service(service).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// List Checks in State
///
/// See [ListServicesForStateRequest]
#[instrument(skip(client, opts), err)]
pub async fn list_services_for_state(
    client: &impl Client,
    state: &str,
    opts: Option<&mut ListServicesForStateRequestBuilder>,
) -> Result<ApiResponse<Vec<HealthStateChecksInfo>>, ClientError> {
    let mut t = ListServicesForStateRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).state(state).build().unwrap();
    api::exec_with_result(client, endpoint).await
}
