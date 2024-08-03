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

/// get_iam_role_quota returns the IAM Role quota as i64
/// L-FE177D64 is the quota code for the IAM Role Per Account quota
pub async fn get_iam_role_quota(client: &Client) -> Result<i64, Box<dyn std::error::Error>> {
    let resp = client
        .get_service_quota()
        .service_code("iam")
        .quota_code("L-FE177D64")
        .send()
        .await?;

    let quota = resp
        .quota
        .and_then(|q| q.value)
        .ok_or("Failed to get IAM Role quote")?;

    Ok(quota as i64)
}
