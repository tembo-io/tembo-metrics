use crate::aws::{cloudformation, servicequotas};
use crate::state::State;
use actix_web::{get, web, HttpResponse, Responder};
use prometheus::{Encoder, TextEncoder};
use std::sync::Arc;
use tracing::{debug, error, info};

#[get("/metrics")]
pub async fn metrics(data: web::Data<Arc<State>>) -> impl Responder {
    let state = data.as_ref();

    info!("Serving metrics");
    debug!(
        "Current CF stack quota: {}",
        state.metrics.cf_stack_quota.get()
    );
    debug!(
        "Current CF stack usage: {}",
        state.metrics.cf_stack_usage.get()
    );

    let encoder = TextEncoder::new();
    let metric_families = state.metrics.registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}

pub async fn set_and_update_metrics(data: &State) -> Result<(), Box<dyn std::error::Error>> {
    // Get a reference to the inner State
    // let state = data.as_ref();

    // Get CloudFormation stack quota from Service Quotas API
    match servicequotas::get_cf_stack_quota(&data.sq_client).await {
        Ok(quota) => {
            data.metrics.cf_stack_quota.set(quota);
            info!("Updated CF stack quota metric: {}", quota);
            // Verify the update
            info!(
                "Verified CF stack quota metric: {}",
                data.metrics.cf_stack_quota.get()
            );
        }
        Err(e) => {
            error!("Failed to get CF stack quota: {}", e);
            return Err(e);
        }
    }

    // Get CloudFormation stack count
    match cloudformation::get_stack_count(&data.cfn_client).await {
        Ok(stack_count) => {
            data.metrics.cf_stack_usage.set(stack_count);
            info!("Updated CF stack usage metric: {}", stack_count);
            // Verify the update
            info!(
                "Verified CF stack usage metric: {}",
                data.metrics.cf_stack_usage.get()
            );
        }
        Err(e) => {
            error!("Failed to get CF stack count: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
