use crate::aws::{cloudformation, iam, servicequotas};
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
    update_cf_stack_quota(data).await?;
    update_cf_stack_usage(data).await?;
    update_iam_role_quota(data).await?;
    update_iam_role_usage(data).await?;
    Ok(())
}

async fn update_cf_stack_quota(data: &State) -> Result<(), Box<dyn std::error::Error>> {
    match servicequotas::get_cf_stack_quota(&data.sq_client).await {
        Ok(quota) => {
            data.metrics.cf_stack_quota.set(quota);
            info!("Updated CF stack quota metric: {}", quota);
            info!(
                "Verified CF stack quota metric: {}",
                data.metrics.cf_stack_quota.get()
            );
            Ok(())
        }
        Err(e) => {
            error!("Failed to get CF stack quota: {}", e);
            Err(e)
        }
    }
}

async fn update_cf_stack_usage(data: &State) -> Result<(), Box<dyn std::error::Error>> {
    match cloudformation::get_stack_count(&data.cfn_client).await {
        Ok(stack_count) => {
            data.metrics.cf_stack_usage.set(stack_count);
            info!("Updated CF stack usage metric: {}", stack_count);
            info!(
                "Verified CF stack usage metric: {}",
                data.metrics.cf_stack_usage.get()
            );
            Ok(())
        }
        Err(e) => {
            error!("Failed to get CF stack count: {}", e);
            Err(e)
        }
    }
}

async fn update_iam_role_quota(data: &State) -> Result<(), Box<dyn std::error::Error>> {
    match servicequotas::get_iam_role_quota(&data.sq_client).await {
        Ok(quota) => {
            data.metrics.iam_role_quota.set(quota);
            info!("Updated IAM role quota metric: {}", quota);
            info!(
                "Verified IAM role quota metric: {}",
                data.metrics.iam_role_quota.get()
            );
            Ok(())
        }
        Err(e) => {
            error!("Failed to get IAM role quota: {}", e);
            Err(e)
        }
    }
}

async fn update_iam_role_usage(data: &State) -> Result<(), Box<dyn std::error::Error>> {
    match iam::get_iam_role_count(&data.iam_client).await {
        Ok(role_count) => {
            data.metrics.iam_role_usage.set(role_count);
            info!("Updated IAM role usage metric: {}", role_count);
            info!(
                "Verified IAM role usage metric: {}",
                data.metrics.iam_role_usage.get()
            );
            Ok(())
        }
        Err(e) => {
            error!("Failed to get IAM role count: {}", e);
            Err(e)
        }
    }
}
