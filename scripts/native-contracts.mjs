// Native replacement for the former sh_test/py_test build targets. Keep these
// checks in CI: Cargo and Vitest alone cannot discover shell/security contracts.
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
// Node build/discovery tests are owned by test:scripts (part of make test).
// Keep this runner focused on checks Cargo and Node test discovery cannot find.
const checks = [
  ['python3', 'scripts/ci_rust_test.py'],
  ['python3', '.github/scripts/check_checkout_paths_test.py'],
  ['bash', '.github/scripts/check_repo_hygiene_test.sh'],
  ['bash', '.github/scripts/check_repo_hygiene.sh'],
  ['bash', 'deploy/tests/deploy_artifacts_test.sh'],
  ['bash', 'deploy/tests/e2e_ci_execution_contract_test.sh', 'deploy/tests/kind_e2e_test.sh', 'deploy/tests/docker_compose_e2e_test.sh', 'deploy/helm/omnisolo/templates/backend-deployment.yaml', 'deploy/helm/omnisolo/values.yaml', 'deploy/docker/server-init/bootstrap-admin.sh', 'deploy/docker-compose.yml'],
  ['bash', 'deploy/tests/operational_api_contract_test.sh', 'deploy/scripts/omnisolo-agent-wizard.sh', 'deploy/scripts/omnisolo-seed-data.sh'],
  ['bash', 'deploy/tests/tls_certificates_test.sh', 'deploy/tests/support/generate_test_tls.sh'],
  ['bash', 'scripts/api_versioning_docs_test.sh'],
  ['bash', 'src/server/production_feature_contract_test.sh'],
  ['bash', 'src/server/catalog_auth_contract_test.sh'],
  ['bash', 'src/server/pii_leakage_check.sh', 'src/server'],
  ['bash', 'src/server/migrations/sqlx_migration_contract_test.sh'],
  ['bash', 'src/server/migrations/inbox_schema_contract_test.sh'],
  ['bash', 'src/server/persistence/backend_neutrality_test.sh'],
  ['python3', '.github/scripts/check_postgres_security_ci_test.py'],
  ['python3', '.github/scripts/check_postgres_security_ci.py'],
];
let failures = 0;
for (const [executable, ...args] of checks) {
  console.log(`\nContract: ${args.join(' ')}`);
  const result = spawnSync(executable, args, { cwd: root, shell: false, stdio: 'inherit', timeout: 180_000 });
  if (result.error || result.signal || result.status !== 0) {
    failures++;
    console.error(`FAILED (${result.error?.message || result.signal || result.status})`);
  }
}
console.log(`\nNative contracts: ${checks.length - failures}/${checks.length} passed; ${failures} failed.`);
process.exitCode = failures ? 1 : 0;
