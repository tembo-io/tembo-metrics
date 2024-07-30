use crate::metrics::Metrics;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_cloudformation::config::Region as cf_region;
use aws_sdk_servicequotas::config::Region as sq_region;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

pub struct State {
    pub registry: prometheus::Registry,
    pub sq_client: aws_sdk_servicequotas::Client,
    pub cfn_client: aws_sdk_cloudformation::Client,
    pub metrics: Metrics,
    pub last_update: Mutex<std::time::Instant>,
    pub updating: AtomicBool,
    pub clients_ready: AtomicBool,
}

impl State {
    pub async fn new(region: String) -> Arc<Self> {
        let sq_region_provider = RegionProviderChain::first_try(sq_region::new(region.clone()));
        let sq_aws_config = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .region(sq_region_provider)
            .load()
            .await;

        let cf_region_provider = RegionProviderChain::first_try(cf_region::new(region.clone()));
        let cf_aws_config = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .region(cf_region_provider)
            .load()
            .await;

        let sq_client = aws_sdk_servicequotas::Client::new(&sq_aws_config);
        let cfn_client = aws_sdk_cloudformation::Client::new(&cf_aws_config);

        // Setup Prometheus registry
        let registry = prometheus::Registry::new();
        let cf_stack_quota = prometheus::IntGauge::new(
            "tembo_cloudformation_stack_quota",
            "AWS CloudFormation stack quota (L-0485CB21)",
        )
        .unwrap();
        let cf_stack_usage = prometheus::IntGauge::new(
            "tembo_cloudformation_stack_usage",
            "AWS CloudFormation stacks in use",
        )
        .unwrap();

        registry.register(Box::new(cf_stack_quota.clone())).unwrap();
        registry.register(Box::new(cf_stack_usage.clone())).unwrap();

        let state = Arc::new(Self {
            registry,
            sq_client,
            cfn_client,
            metrics: Metrics::new(),
            last_update: Mutex::new(std::time::Instant::now()),
            updating: AtomicBool::new(false),
            clients_ready: AtomicBool::new(false),
        });

        // Initialize clients
        if let Err(e) = Self::init_clients(&state).await {
            error!("Failed to initialize clients: {}", e);
        } else {
            state.clients_ready.store(true, Ordering::SeqCst);
            info!("AWS clients initialized successfully");
        }

        state
    }

    // init_clients tests the clients to ensure they are working
    async fn init_clients(state: &Self) -> Result<(), Box<dyn std::error::Error>> {
        // Test Service Quotas client
        let _ = state.sq_client.list_services().send().await?;

        // Test CloudFormation client
        let _ = state.cfn_client.list_stacks().send().await?;

        Ok(())
    }
}
