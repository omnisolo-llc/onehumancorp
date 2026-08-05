
> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.



**Tech Stack:** Rust/Cargo, Bazel/Bzlmod, Docker Compose, Helm/Kubernetes YAML, Prometheus YAML, Bash, Next.js catalog data, Markdown.

**Prerequisites:** Start from commit `98ecdc439` or a descendant containing `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`. Work directly on `main` as explicitly requested. Do not modify native inbox behavior in this plan.

---

## File Structure

- Modify root Rust/Bazel manifest inputs: `Cargo.toml`, `Cargo.lock`, `MODULE.bazel.lock`, `src/server/integrations/mod.rs`, `src/ui/tauri/BUILD.bazel`.
- Modify Compose/monitoring: `deploy/docker-compose.yml`, `deploy/docker-compose.e2e.yml`, `deploy/docker/prometheus/prometheus.yml`, `deploy/docker/prometheus/prometheus-agent.yml`.
- Modify Helm/deploy graph: `deploy/helm/ohc/values.yaml`, backend/HPA/network-policy/ServiceMonitor templates, `deploy/BUILD.bazel`, and `deploy/tests/kind_e2e_test.sh`.
- Modify active product/docs references and annotate three historical reports as superseded.
- Modify `.github/workflows/ci.yml`: run the residue guard in CI.
- Modify `docs/reports/production_agent_optimization_report.md`: record removal evidence without claiming unrun external checks.


**Files:**

- [ ] **Step 1: Write the failing tracked-file guard**

Create the executable script with this complete content:

```bash
#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

active_roots=(
  Cargo.toml
  README.md
  src
  deploy
  docs/business
  docs/technical/developer
  docs/technical/reports
)

mapfile -t tracked < <(git ls-files -- "${active_roots[@]}" | awk -v guard="$guard_path" '$0 != guard')
if ((${#tracked[@]} == 0)); then
  echo "chat platform residue scan failed: no tracked active files were discovered" >&2
  exit 2
fi

  printf '%s\n' "$matches" >&2
  exit 1
elif [[ $? -ne 1 ]]; then
  echo "chat platform residue scanner failed" >&2
  exit 2
fi

historical=(
  docs/research/ohc_tool_integration_research_report.md
  docs/reports/tool_integration_research_report_q3.md
  docs/research/triage_report_bazel.md
)
for path in "${historical[@]}"; do
  git ls-files --error-unmatch "$path" >/dev/null
  rg -q '^> Superseded architecture: .*native omnichannel' "$path" || {
    echo "missing native-architecture superseded marker: $path" >&2
    exit 1
  }
done
```

- [ ] **Step 2: Mark it executable and verify RED**

Run:

```bash
```


- [ ] **Step 3: Verify the guard cannot pass on an empty inventory**

Run this disposable Git behavior test:

```bash
tmp="$(mktemp -d)"
git -C "$tmp" init -q
if (cd "$tmp" && bash guard.sh); then exit 1; fi
rm -rf "$tmp"
```

Expected: the copied guard exits nonzero because no tracked active files exist.

- [ ] **Step 4: Commit the red contract**

```bash
```

### Task 2: Remove the Unused Rust Integration Crate

**Files:**
- Modify: `Cargo.toml:50,197-198`
- Modify: `src/server/integrations/mod.rs:3-5`
- Modify: `src/ui/tauri/BUILD.bazel:215`
- Modify: `Cargo.lock`
- Modify: `MODULE.bazel.lock`

- [ ] **Step 1: Add a failing Rust graph assertion**

Run before editing:

```bash
else
  exit 1
fi
```

Expected: print the RED message and exit 0, proving the package is currently present.

- [ ] **Step 2: Delete the crate and remove exact manifest edges**

Apply these removals:

```diff
-#[cfg(ohc_bazel)]
-#[cfg(not(ohc_bazel))]
```

Delete the five files listed above. Preserve every adjacent workspace member, dependency, re-export, and Tauri manifest entry.

- [ ] **Step 3: Regenerate Cargo/Bazel lock state**

Run:

```bash
cargo metadata --format-version=1 --no-deps >/tmp/ohc-cargo-metadata.json
bazel sync --only=crates
```


- [ ] **Step 4: Verify the Rust graph and focused builds**

Run:

```bash
cargo check -p ohc-mono
bazel test //src/server/integrations:server_integrations_unit_test --test_output=errors
```

Expected: both residue checks find no match; Cargo check and Bazel target PASS.

- [ ] **Step 5: Commit Rust removal**

```bash
git add Cargo.toml Cargo.lock MODULE.bazel.lock src/server/integrations/mod.rs src/ui/tauri/BUILD.bazel
```

### Task 3: Remove Docker Compose and Prometheus Services

**Files:**
- Modify: `deploy/docker-compose.yml:61-69,158,199-278`
- Modify: `deploy/docker-compose.e2e.yml:20`
- Modify: `deploy/docker/prometheus/prometheus.yml:17-19`
- Modify: `deploy/docker/prometheus/prometheus-agent.yml:18-20`

- [ ] **Step 1: Capture the failing Compose/Prometheus assertions**

Run:

```bash
```


- [ ] **Step 2: Remove complete Compose service and database/env blocks**

Delete from `deploy/docker-compose.yml`:

```yaml
```

Change the database initializer exactly to:

```yaml
      POSTGRES_MULTIPLE_DATABASES: ohc
```


- [ ] **Step 3: Remove Prometheus scrape jobs**


- [ ] **Step 4: Validate Compose and Prometheus residue**

Run:

```bash
docker compose -f deploy/docker-compose.yml config >/tmp/ohc-compose.yaml
docker compose -f deploy/docker-compose.e2e.yml config >/tmp/ohc-compose-e2e.yaml
```

Expected: both Compose files parse; no rendered service or tracked monitoring reference remains.

- [ ] **Step 5: Commit Compose removal**

```bash
git add deploy/docker-compose.yml deploy/docker-compose.e2e.yml deploy/docker/prometheus
```

### Task 4: Remove Helm Resources and Network Access

**Files:**
- Modify: `deploy/helm/ohc/values.yaml:23-46`
- Modify: `deploy/helm/ohc/templates/backend-deployment.yaml:66-77`
- Modify: `deploy/helm/ohc/templates/hpa.yaml:93-137`
- Modify: `deploy/helm/ohc/templates/network-policy.yaml:160-186,321,373`
- Modify: `deploy/helm/ohc/templates/servicemonitor.yaml:48-62`
- Modify: `deploy/BUILD.bazel:242-243,330-331`
- Modify: `deploy/tests/kind_e2e_test.sh:135`

- [ ] **Step 1: Capture failing rendered Helm assertions**

Run:

```bash
helm template ohc deploy/helm/ohc >/tmp/ohc-before.yaml
```


- [ ] **Step 2: Delete dedicated resources and values/env configuration**


```yaml
              value: "true"
              valueFrom:
                secretKeyRef:
                  key: adminPassword
            {{- end }}
```

- [ ] **Step 3: Remove scaling, monitoring, and network selectors**


- [ ] **Step 4: Validate Helm, rendered policy, Bazel package, and Kind contract**

Run:

```bash
helm lint deploy/helm/ohc
helm template ohc deploy/helm/ohc >/tmp/ohc-after.yaml
bazel test //deploy:deploy_artifacts_test --test_output=errors
```


- [ ] **Step 5: Commit Helm removal**

```bash
git add deploy/BUILD.bazel deploy/helm/ohc deploy/tests/kind_e2e_test.sh
```

### Task 5: Replace Active Product and Documentation References

**Files:**
- Modify: `src/ui/next/src/app/agents/catalog.ts:61`
- Test: `src/ui/next/src/app/agents/catalog.test.ts`
- Modify: `README.md:151`
- Modify: `docs/technical/developer/developer-guide.md:204`
- Modify: `docs/business/cost-blueprint.md:17-23`
- Modify: `docs/business/growth_strategy_audit.md:21`
- Modify: `docs/research/[customer_success]_invisible_ambassador_agent.md:10`
- Modify: `docs/technical/reports/cpp-migration-evaluation.md:19`
- Modify: `docs/research/ohc_tool_integration_research_report.md:1`
- Modify: `docs/reports/tool_integration_research_report_q3.md:1`
- Modify: `docs/research/triage_report_bazel.md:1`
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1: Write the failing native connector/catalog expectation**

Create `src/ui/next/src/app/agents/catalog.test.ts`:

```ts
import { describe, expect, it } from "vitest";
import { experts } from "./catalog";

describe("Customer Ambassador connectors", () => {
  it("advertises only native/direct omnichannel connectors", () => {
    const ambassador = experts.find((agent) => agent.name === "Customer Ambassador");
    expect(ambassador?.connectors).toEqual([
      "Native Omnichannel Inbox",
      "Instagram DMs",
      "WhatsApp",
      "SMS",
      "Email",
    ]);
  });
});
```

Run: `cd src/ui/next && pnpm exec vitest run src/app/agents/catalog.test.ts`

Expected: FAIL because the current connector list contains the external platform and lacks the native/direct set.

- [ ] **Step 2: Replace the catalog value**

Change the Customer Ambassador connector list to this exact value:

```ts
connectors: ['Native Omnichannel Inbox', 'Instagram DMs', 'WhatsApp', 'SMS', 'Email'],
```

Run: `cd src/ui/next && pnpm exec vitest run src/app/agents/catalog.test.ts src/app/agents/page.test.tsx`

Expected: both catalog and rendered agents-page tests PASS.

- [ ] **Step 3: Remove active operational/documentation claims**

Apply these factual replacements:

```markdown
README service table: remove the Chat platform row entirely.
Invisible Ambassador architecture: replace the external platform with the native omnichannel domain and direct connectors.
```

Do not rewrite unrelated historical analysis.

- [ ] **Step 4: Mark historical research as superseded**

Insert immediately after each title in the three historical files:

```markdown
```

- [ ] **Step 5: Add the residue guard to CI and verify GREEN**

Add this step to the existing security/supply-chain job in `.github/workflows/ci.yml` before dependency audits:

```yaml
```

Run:

```bash
git diff --check
```

Expected: guard PASS with no active references; all historical files contain the required superseded marker; whitespace check exits 0.

- [ ] **Step 6: Commit catalog, docs, and CI contract**

```bash
git add .github/workflows/ci.yml README.md \
  src/ui/next/src/app/agents/catalog.ts src/ui/next/src/app/agents/catalog.test.ts \
  docs/technical/developer/developer-guide.md docs/business/cost-blueprint.md \
  docs/business/growth_strategy_audit.md 'docs/research/[customer_success]_invisible_ambassador_agent.md' \
  docs/technical/reports/cpp-migration-evaluation.md \
  docs/research/ohc_tool_integration_research_report.md \
  docs/reports/tool_integration_research_report_q3.md docs/research/triage_report_bazel.md \
```

### Task 6: Final Removal Verification and Evidence

**Files:**
- Modify: `docs/reports/production_agent_optimization_report.md`

- [ ] **Step 1: Run the full removal matrix**

Run:

```bash
cargo metadata --locked --format-version=1 --no-deps >/tmp/ohc-cargo-metadata-final.json
cargo check -p ohc-mono
bazel test //src/server/integrations:server_integrations_unit_test //src/ui/next:next_vitest //deploy:deploy_artifacts_test --test_output=errors
docker compose -f deploy/docker-compose.yml config >/tmp/ohc-compose-final.yaml
docker compose -f deploy/docker-compose.e2e.yml config >/tmp/ohc-compose-e2e-final.yaml
helm lint deploy/helm/ohc
helm template ohc deploy/helm/ohc >/tmp/ohc-helm-final.yaml
bazel query //... > /tmp/ohc-bazel-query.txt 2>/tmp/ohc-bazel-query.err
```

Expected: every positive command exits 0; every negative residue probe finds no match. If the full Bazel query itself fails, diagnose and fix/query again rather than treating an empty pipeline as success.

- [ ] **Step 2: Verify only approved historical/spec mentions remain globally**

Run:

```bash
```

Expected: matches are limited to the native design/spec/plan, the residue guard and its CI step, the production evidence section, and the three explicitly marked historical research documents.

- [ ] **Step 3: Record exact verification evidence**


```markdown
- Exact removed application/deployment surfaces.
- Exact commands, exit results, and test counts from Steps 1–2.
- The native inbox remains in place; feature expansion belongs to later native-chat projects.
- Any unavailable local tool or remote sandbox check is named as unverified, never reported as passed.
```

- [ ] **Step 4: Validate and commit evidence**

Run:

```bash
git diff --check
```

Expected: both commands PASS.

```bash
git add docs/reports/production_agent_optimization_report.md
```
