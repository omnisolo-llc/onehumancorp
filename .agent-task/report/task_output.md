issue_title: "🛡️ Sentinel: [blocked no-work finding: F13]"
issue_description: |
  **Verified trace limitations:**
  - Role constraint: Sentinel (Maintainer L7)
  - Selected target: F13 (API key, consumer plan and native-client subscription are distinct)
  - Current state: F13 is already fully implemented.

  **Executed test commands:**
  - `grep -rn -A 5 -B 5 "proxy" src/server/` (Passed)
  - `cat src/server/harness/middleware/provider_facade.rs | awk '/fn check_request/,/^}/'` (Passed)
  - `cat src/server/harness/middleware/provider_facade.rs | awk 'NR>=350 && NR<=380'` (Passed)
  - `cat src/server/harness/middleware/provider_facade.rs | awk 'NR>=380 && NR<=410'` (Passed)
  - `grep -rn -A 5 -B 5 "ByokApi" src/server/` (Passed)
  - `cat src/server/harness/middleware/usage_ledger.rs | awk '/pub enum PayerMode/,/^}/'` (Passed)
  - `cat src/server/harness/middleware/usage_meter.rs | awk '/pub fn from_env/,/^}/'` (Passed)
  - `cat src/server/harness/tests/provider_facade.rs | grep -A 20 -n "async fn facade_rejects_native_subscription_proxying"` (Passed)
  - `make lint` (Failed: next: not found)
  - `npm install && make lint` (Output truncated)
  - `npm install lucide-react && npm run build:web` (Output truncated)
  - `make test` (Timed out)
  - `cargo test -p server_harness --lib middleware::provider_facade` (Output truncated)

  The `provider_facade.rs` already explicitly blocks `NativeSubscription` payer modes, specifically checking `if meter.scope.payer == super::usage_ledger::PayerMode::NativeSubscription` and returning a `FORBIDDEN` error. It also restricts `ByokApi` mode strictly to `api.openai.com` and `api.anthropic.com` origins, fetching the key dynamically from the secure `ConnectionVault`. The test `facade_rejects_native_subscription_proxying` verifies this behavior.
  Furthermore, `usage_meter.rs` restricts parsing the `OMNISOLO_USAGE_PAYER` to `managed_api` or `byok_api`, returning an error for `native_subscription`.

  Since the logic is already implemented, this task requires no code changes.
