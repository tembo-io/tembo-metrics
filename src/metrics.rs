#[derive(Clone)]
pub struct Metrics {
    pub registry: prometheus::Registry,
    pub cf_stack_quota: prometheus::IntGauge,
    pub cf_stack_usage: prometheus::IntGauge,
    pub iam_role_quota: prometheus::IntGauge,
    pub iam_role_usage: prometheus::IntGauge,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        let registry = prometheus::Registry::new();
        let cf_stack_quota = prometheus::IntGauge::new(
            "tembo_cloudformation_stack_quota",
            "AWS CloudFormation stack quota (L-0485CB21)",
        )
        .expect("Failed to create cf_stack_quota metric");
        let cf_stack_usage = prometheus::IntGauge::new(
            "tembo_cloudformation_stack_usage",
            "AWS CloudFormation stacks in use",
        )
        .expect("Failed to create cf_stack_usage metric");
        let iam_role_quota =
            prometheus::IntGauge::new("tembo_iam_role_quota", "AWS IAM role quota (L-FE177D64)")
                .expect("Failed to create iam_role_quota metric");
        let iam_role_usage =
            prometheus::IntGauge::new("tembo_iam_role_usage", "AWS IAM roles in use")
                .expect("Failed to create iam_role_usage metric");

        registry
            .register(Box::new(cf_stack_quota.clone()))
            .expect("Failed to register cf_stack_quota metric");
        registry
            .register(Box::new(cf_stack_usage.clone()))
            .expect("Failed to register cf_stack_usage metric");
        registry
            .register(Box::new(iam_role_quota.clone()))
            .expect("Failed to register cf_stack_usage metric");
        registry
            .register(Box::new(iam_role_usage.clone()))
            .expect("Failed to register cf_stack_usage metric");

        Self {
            registry,
            cf_stack_quota,
            cf_stack_usage,
            iam_role_quota,
            iam_role_usage,
        }
    }
}
