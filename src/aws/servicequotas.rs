use aws_sdk_servicequotas::Client;

/// get_cf_stack_quota returns the Cloudformation Stack quota as i64
/// L-0485CB21 is the quota code for the Cloudformation Stack Count
pub async fn get_cf_stack_quota(client: &Client) -> Result<i64, Box<dyn std::error::Error>> {
    let resp = client
        .get_service_quota()
        .service_code("cloudformation")
        .quota_code("L-0485CB21")
        .send()
        .await?;

    let quota = resp
        .quota
        .and_then(|q| q.value)
        .ok_or("Failed to get Cloudformation Stack quote")?;

    Ok(quota as i64)
}
