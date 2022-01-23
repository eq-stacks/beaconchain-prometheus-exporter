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

Then run using Docker:

```bash
$ docker run \
  -e ROOT_URL=https://prater.beaconcha.in \
  -e VALIDATOR_INDEX=271234 \
  -p 9184:9184 \
  bpe:dev
```

## Contributing

Issues and PRs welcome.

## License

MIT, Copyright (c) 2022 Equilibrium
