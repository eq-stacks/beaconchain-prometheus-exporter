use prometheus_exporter::prometheus;
use prometheus_exporter::prometheus::{register_gauge, register_int_gauge};

pub(crate) struct Metrics {
    pub attestation_effectiveness: prometheus::Gauge,
    pub attestation_efficiency: prometheus::Gauge,
    pub optimal_inclusion_distance: prometheus::IntGauge,
    pub validator_balance: prometheus::IntGauge,
}

impl Metrics {
    pub fn init() -> Metrics {
        Metrics {
            attestation_efficiency: register_gauge!(
                "attestation_efficiency",
                "Efficiency of attestation... more details TBD"
            )
            .expect("cannot create gauge attestation_efficiency"),
            attestation_effectiveness: register_gauge!(
                "attestation_effectiveness",
                "Effectiveness of attestation... more details TBD"
            )
            .expect("cannot create gauge attestation_effectiveness"),
            optimal_inclusion_distance: register_int_gauge!(
                "optimal_inclusion_distance",
                "Difference between the inclusion slot and the earliest slot possible. 0 is best."
            )
            .expect("cannot create gauge optimal_inclusion_distance"),
            validator_balance: register_int_gauge!(
                "validator_balance",
                "Current balance of the validator in ETH"
            )
            .expect("can not create gauge validator_balance"),
        }
    }
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_metrics_init() {
    let metrics = Metrics::init();

    assert_eq!(metrics.attestation_efficiency.get(), 0.0);
    assert_eq!(metrics.attestation_effectiveness.get(), 0.0);
    assert_eq!(metrics.optimal_inclusion_distance.get(), 0);
    assert_eq!(metrics.validator_balance.get(), 0);

    metrics.attestation_efficiency.set(1.234);
    metrics.attestation_effectiveness.set(0.88823);
    metrics.optimal_inclusion_distance.set(2);
    metrics.validator_balance.set(3204440232);

    assert_eq!(metrics.attestation_efficiency.get(), 1.234);
    assert_eq!(metrics.attestation_effectiveness.get(), 0.88823);
    assert_eq!(metrics.optimal_inclusion_distance.get(), 2);
    assert_eq!(metrics.validator_balance.get(), 3204440232)
  }
}