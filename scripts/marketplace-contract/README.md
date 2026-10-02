# Marketplace contract checks

Run `scripts/marketplace-contract/run.sh` to compile the maintained marketplace module and the actual native construction/listing/publication fragments against a small graph. Registry dependency versions and checksums must match the root lockfile. `prepare.py` extracts the production fragments and records their source hashes before each run; the provider implementation is included by path.

These tests cover authoritative provider reads, deletion and client restart, descriptor validation and acknowledgements, a local HTTP registry, and unavailable configuration. The tool adapter and its shared types are also compiled from maintained source, including unknown-outcome retry classification. They do not replace full workspace compilation, authenticated mounted server/browser acceptance, or real-provider verification. The loopback registry is an isolated test service and never contacts a live marketplace.
