use aws_sdk_iam::Client;

/// get_iam_role_count looks up all Roles in the account and returns the count as a i64
pub async fn get_iam_role_count(client: &Client) -> Result<i64, Box<dyn std::error::Error>> {
    let mut role_count = 0;
    let mut marker: Option<String> = None;

    loop {
        let mut request = client.list_roles();
        if let Some(token) = marker {
            request = request.marker(token);
        }

        let response = request.send().await?;

        let roles = response.roles();
        role_count += roles.len() as i64;

        match response.marker() {
            Some(token) => marker = Some(token.to_string()),
            None => break,
        }
    }

    Ok(role_count)
}
