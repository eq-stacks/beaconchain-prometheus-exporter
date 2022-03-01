use prometheus_exporter::prometheus;
use prometheus_exporter::prometheus::{register_gauge_vec, register_int_gauge_vec};

pub(crate) struct Metrics {
    pub attestation_effectiveness: prometheus::GaugeVec,
    pub attestation_efficiency: prometheus::GaugeVec,
    pub optimal_inclusion_distance: prometheus::IntGaugeVec,
    pub validator_balance: prometheus::IntGaugeVec,
}

impl Metrics {
    pub fn init() -> Metrics {
        Metrics {
            attestation_efficiency: register_gauge_vec!(
                "attestation_efficiency",
                "Efficiency of attestation... more details TBD",
                &["validator_index"]
            )
            .expect("cannot create gauge attestation_efficiency"),
            attestation_effectiveness: register_gauge_vec!(
                "attestation_effectiveness",
                "Effectiveness of attestation... more details TBD",
                &["validator_index"]
            )
            .expect("cannot create gauge attestation_effectiveness"),
            optimal_inclusion_distance: register_int_gauge_vec!(
                "optimal_inclusion_distance",
                "Difference between the inclusion slot and the earliest slot possible. 0 is best.",
                &["validator_index"]
            )
            .expect("cannot create gauge optimal_inclusion_distance"),
            validator_balance: register_int_gauge_vec!(
                "validator_balance",
                "Current balance of the validator in ETH",
                &["validator_index"]
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
        let validator_index = "0x123";

        assert_eq!(
            metrics
                .attestation_efficiency
                .with_label_values(&["validator_index", validator_index])
                .get(),
            0.0
        );
        assert_eq!(
            metrics
                .attestation_effectiveness
                .with_label_values(&["validator_index", validator_index])
                .get(),
            0.0
        );
        assert_eq!(
            metrics
                .optimal_inclusion_distance
                .with_label_values(&["validator_index", validator_index])
                .get(),
            0
        );
        assert_eq!(
            metrics
                .validator_balance
                .with_label_values(&["validator_index", validator_index])
                .get(),
            0
        );

        metrics
            .attestation_efficiency
            .with_label_values(&["validator_index", validator_index])
            .set(1.234);
        metrics
            .attestation_effectiveness
            .with_label_values(&["validator_index", validator_index])
            .set(0.88823);
        metrics
            .optimal_inclusion_distance
            .with_label_values(&["validator_index", validator_index])
            .set(2);
        metrics
            .validator_balance
            .with_label_values(&["validator_index", validator_index])
            .set(3204440232);

        assert_eq!(
            metrics
                .attestation_efficiency
                .with_label_values(&["validator_index", validator_index])
                .get(),
            1.234
        );
        assert_eq!(
            metrics
                .attestation_effectiveness
                .with_label_values(&["validator_index", validator_index])
                .get(),
            0.88823
        );
        assert_eq!(
            metrics
                .optimal_inclusion_distance
                .with_label_values(&["validator_index", validator_index])
                .get(),
            2
        );
        assert_eq!(
            metrics
                .validator_balance
                .with_label_values(&["validator_index", validator_index])
                .get(),
            3204440232
        )
    }
}
