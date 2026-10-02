# Production fixture-path boundary

`bash scripts/fixture-boundary-contract/run.sh` imports the actual production
middleware directly, then executes its HTTP tests. It checks retired simulation
names with invalid, missing-field and valid decision JSON, plus a normal approval
ID that must still reach the genuine handler chain. Dependency versions must match
the root lockfile; source fingerprints must remain unchanged during execution.

This is a focused boundary check, not full backend or browser certification. The
required native browser runner additionally authenticates the isolated SQL-fixture
owner against the newly built backend and asserts 404 for every retired fixture
route. No production feature or environment variable enables those routes.
