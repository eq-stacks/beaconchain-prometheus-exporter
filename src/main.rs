use std::env;
use std::time::Duration;

use log::info;
use tokio::{task, time};

mod beaconchain;
use beaconchain::{BeaconchainResponse, ResponseType};

mod metrics;
use metrics::Metrics;

async fn scrape_metrics(metrics: &Metrics) -> Result<(), Box<dyn std::error::Error>> {
    let root_url: String = env::var("ROOT_URL").expect("Please set ROOT_URL env var");
    let validator_index: String =
        env::var("VALIDATOR_INDEX").expect("Please set VALIDATOR_INDEX env var");
    let validator_url = format!("{}/api/v1/validator/{}", root_url, validator_index);

    let urls = vec![
        format!("{}/performance", validator_url),
        format!("{}/attestationefficiency", validator_url),
        format!("{}/attestationeffectiveness", validator_url),
        format!("{}/attestations", validator_url),
    ];

    for url in urls.iter() {
        let resp = reqwest::get(url)
            .await?
            .json::<BeaconchainResponse>()
            .await?;

        match resp.data {
            ResponseType::AttestationEfficiency {
                attestation_efficiency,
            } => metrics.attestation_efficiency.set(attestation_efficiency),
            ResponseType::AttestationEffectiveness {
                attestation_effectiveness,
            } => metrics
                .attestation_effectiveness
                .set(attestation_effectiveness),
            ResponseType::Performance { balance } => metrics.validator_balance.set(balance as i64),
            ResponseType::Attestations(attestations) => metrics.optimal_inclusion_distance.set(
                attestations.first().unwrap().inclusionslot
                    - attestations.first().unwrap().attesterslot
                    - 1,
            ),
        };
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    prometheus_exporter::start("0.0.0.0:9184".parse().expect("failed to parse binding"))
        .expect("failed to start prometheus exporter");

    let forever = task::spawn(async {
        let metrics = Metrics::init();
        let mut interval = time::interval(Duration::from_millis(24000));
        loop {
            interval.tick().await;
            scrape_metrics(&metrics).await.unwrap();
        }
    });

    forever.await?
}
