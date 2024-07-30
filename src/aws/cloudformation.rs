use aws_sdk_cloudformation::types::StackStatus;
use aws_sdk_cloudformation::Client;

pub async fn get_stack_count(client: &Client) -> Result<i64, Box<dyn std::error::Error>> {
    let mut stack_count = 0;
    let mut next_token: Option<String> = None;

    loop {
        let mut request = client.list_stacks();
        if let Some(token) = next_token {
            request = request.next_token(token);
        }

        let response = request.send().await?;

        let summaries = response.stack_summaries();
        for summary in summaries {
            // Count stacks that are not in DELETE_COMPLETE status
            if summary.stack_status() != Some(&StackStatus::DeleteComplete) {
                stack_count += 1;
            }
        }

        match response.next_token() {
            Some(token) => next_token = Some(token.to_string()),
            None => break,
        }
    }

    Ok(stack_count)
}
