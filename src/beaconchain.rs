/// See: https://beaconcha.in/api/v1/docs/index.html
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseType {
    AttestationEfficiency { attestation_efficiency: f64 },
    AttestationEffectiveness { attestation_effectiveness: f64 },
    Performance { balance: u64 },
    Attestations(Vec<Attestation>),
}

#[derive(Deserialize, Debug)]
pub struct BeaconchainResponse {
    pub data: ResponseType,
}

#[derive(Deserialize, Debug)]
pub struct Attestation {
    pub attesterslot: i64,
    pub inclusionslot: i64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_performance() {
        let input = r#"{ "data": { "balance": 33154517070 } }"#;
        serde_json::from_str::<BeaconchainResponse>(&input).unwrap();
    }

    #[test]
    fn test_deserialize_attestation_effectiveness() {
        let input = r#"{ "data": { "attestation_effectiveness": 0.889923 } }"#;
        serde_json::from_str::<BeaconchainResponse>(&input).unwrap();
    }

    #[test]
    fn test_deserialize_attestation_efficiency() {
        let input = r#"{ "data": { "attestation_efficiency": 1.23345 } }"#;
        serde_json::from_str::<BeaconchainResponse>(&input).unwrap();
    }

    #[test]
    fn test_deserialize_attestations() {
        let input = r#"{ "data": [{ "attesterslot": 10101, "inclusionslot": 10102 }] }"#;
        serde_json::from_str::<BeaconchainResponse>(&input).unwrap();
    }
}
