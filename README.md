# Beaconchai.in Prometheus Exporter
> Export validator metrics from https://beaconcha.in

This exporter currently exports four metrics:

- `attestation_effectiveness`
- `attestation_efficiency`
- `optimal_inclusion_distance`
- `validator_balance`

## Installation

This exporter is available as a Docker container at `eqlabs/beaconchain-prometheus-exporter`

## Usage

The exporter is configured via the following environment variables:

- `ROOT_URL` e.g. `https://beaconcha.in`
- `VALIDATOR_INDEX`

## Contributing

Issues and PRs welcome.

## License

MIT