use std::env;
use std::time::Duration;

use tokio::{task, time};

mod beaconchain;
use beaconchain::{Attestation, BeaconchainResponse, ResponseType};

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
                calculate_optimal_inclusion_distance(attestations.first().unwrap())
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

fn calculate_optimal_inclusion_distance(attestation: &Attestation) -> i64 {
    match attestation.inclusionslot {
        0 => 0,
        _ => (attestation.inclusionslot - attestation.attesterslot - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_optimal_inclusion_distance_when_inclusion_is_zero() {
        let input = r#"{ "data": [{ "attesterslot": 3024077, "inclusionslot": 0 }] }"#;
        let result = serde_json::from_str::<BeaconchainResponse>(&input).unwrap();

        if let ResponseType::Attestations(attestations) = result.data {
            let optimal_inclusion_distance = calculate_optimal_inclusion_distance(attestations.first().unwrap());

            assert_eq!(optimal_inclusion_distance, 0)
        }

    }
}