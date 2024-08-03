# Tembo Metrics

Tembo Metrics is a Rust-based Prometheus exporter that monitors AWS CloudFormation stack usage and quotas. It provides real-time metrics to help you keep track of your CloudFormation resources and avoid hitting service limits.

## Features

- Monitors AWS CloudFormation stack usage and quotas
- Exposes Prometheus-compatible metrics
- Configurable through command-line arguments
- Includes health check endpoints for liveness and readiness probes
- Deployable as a Docker container or standalone binary

## Metrics

Tembo Metrics currently exposes the following Prometheus metrics:

1. `tembo_cloudformation_stack_quota`: AWS CloudFormation stack quota (L-0485CB21)

   - Type: Gauge
   - Description: The maximum number of CloudFormation stacks allowed in the account/region

2. `tembo_cloudformation_stack_usage`: AWS CloudFormation stacks in use

   - Type: Gauge
   - Description: The current number of CloudFormation stacks in the account/region

3. `tembo_iam_role_quota`: AWS IAM Role quota (L-FE177D64)

   - Type: Gauge
   - Description: The maximum number of AWS IAM Roles in the account/region

4. `tembo_iam_role_usage`: AWS IAM Roles in use
   - Type: Gauge
   - Description: The current number of AWS IAM Roles in the account/region

Example metrics output:

```
HELP tembo_cloudformation_stack_quota AWS CloudFormation stack quota (L-0485CB21)
TYPE tembo_cloudformation_stack_quota gauge
tembo_cloudformation_stack_quota 2000
HELP tembo_cloudformation_stack_usage AWS CloudFormation stacks in use
TYPE tembo_cloudformation_stack_usage gauge
tembo_cloudformation_stack_usage 605
# HELP tembo_iam_role_quota AWS IAM role quota (L-FE177D64)
# TYPE tembo_iam_role_quota gauge
tembo_iam_role_quota 1000
# HELP tembo_iam_role_usage AWS IAM roles in use
# TYPE tembo_iam_role_usage gauge
tembo_iam_role_usage 716
```

## Installation

### Prerequisites

- Rust 1.76 or later
- AWS credentials configured (either through environment variables or AWS CLI configuration)

### Building from source

1. Clone the repository:

```
git clone https://github.com/tembo-io/tembo-metrics.git
cd tembo-metrics
```

2. Build the project:

```
cargo build --release
```

3. The binary will be available at `target/release/tembo-metrics`

### Docker

A Dockerfile is provided to build and run Tembo Metrics in a container:

1. Build the Docker image:

```
docker buildx build -f Dockerfile . -t tembo-metrics
```

2. Run the container:

```
docker run -p 8080:8080 -e AWS_ACCESS_KEY_ID=your_access_key -e AWS_SECRET_ACCESS_KEY=your_secret_key tembo-metrics
```

## Usage

Run Tembo Metrics with the following command-line arguments:

```
tembo-metrics [OPTIONS]
Options:
--log-level <LEVEL>       Set the log level (default: info)
--region <REGION>         Set the AWS region (default: us-east-1)
--server-host <HOST>      Set the server host (default: 127.0.0.1)
--server-port <PORT>      Set the server port (default: 8080)
```

Example:

```
tembo-metrics --log-level debug --region us-west-2 --server-port 9090
```

## API Endpoints

- `/metrics`: Exposes Prometheus metrics
- `/healthz`: Liveness probe endpoint
- `/readyz`: Readiness probe endpoint

## Development

### Running tests

To run the test suite:

```
cargo test
```

### Linting

To run the linter:

```
cargo clippy
```

## Deployment

Tembo Metrics can be deployed in various environments. A Helm chart is provided in the `charts` directory for Kubernetes deployments.

For detailed information on deploying with Helm, please refer to the [Helm chart README](./charts/tembo-metrics/README.md).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

## Support
