# Manual dependency inventory and reporting

`python3 scripts/dependency_report.py` inventories the maintained install graphs
and saves unmodified scanner output with source hashes. This is a manual reporting
command, not a new release gate. The required CI job still runs the existing root
and Next `npm audit --omit=dev` commands, without a severity threshold change.
No workflow, product dependency, exception or advisory ignore is added here.

## Scope and policy

[The inventory](../../scripts/dependency-audit/inventory.json) names six npm
locks, the root Cargo graph, eleven tracked focused Cargo locks, four declared
Python requirement sets, and image boundaries that still require an actual image
inventory. New or missing tracked supported locks fail inventory validation. The
exact historical `src/server/Cargo.lock` is excluded because it has no adjacent
manifest or identified consumer; activating it requires inventory review. Ignored
focused locks generated from the root are not additional authoritative install
graphs. Their maintained harness runners retain their existing identity checks.
Those checks do not prove identical feature activation: root-derived ignored
focused harness feature graphs are not individually scanned by this bounded
inventory.

| Boundary | Reporting behavior |
| --- | --- |
| npm | Production and full graphs for all six locks, using `--package-lock-only`; explicitly include optional/peer packages and include development packages only for full graphs. Conflicting graph environment settings are removed. Preserve every reported severity. An affected-node count is not a count of unique advisories. |
| Root Cargo | Seven backend/desktop target triples, all workspace roots, default features, development dependencies included. No `--all-features`, `--exclude-dev` or unpublished-member exclusion. |
| Tauri Android | Four Android triples, Tauri as the sole workspace root, default features. Dependency analysis is not an Android build. |
| Focused Cargo | Each of the eleven independent locks on Linux x86_64, default features. `--locked --offline` never rewrites a lock or resolves a new graph. |
| Cargo advisories/sources | Include vulnerability, unsoundness, all unmaintained and yanked diagnostics separately. Crates.io is the observed allowed registry; unknown registries/git are denied by the scanner. No CVSS/severity suppression. |
| Python | Audit all declared pinned rows with `--disable-pip --no-deps --strict`; require hashes for the three hashed application locks. Parse declarations with the pinned requirements library and require exact returned name/version coverage, with no duplicates or skipped rows. No installation, product resolver or auto-fix. Declared rows do not prove installed closure or wheel/OS integrity. |
| Images | Explicit `not_audited` records remain in every report. Global CLI transitives, un-hashed Python installs, copied images/binaries and OS packages require actual image inventory/SBOM. The test-tools npm lock does not govern a Docker global install. |

The current feature boundary does not activate rust_decimal's optional rkyv
features. A package existing in Cargo.lock or optional metadata alone is not proof
of an active feature. Additional release-used features or targets must enter a
reviewed inventory; this command does not claim all-feature clearance.

Cargo license and duplicate-version gates are deliberately not enabled. The
baseline license inventory includes local packages without declarations, MPL-2.0,
CDLA-Permissive-2.0 and Apache-2.0 WITH LLVM-exception. The repository license is
Business Source License 1.1 with its additional grant; dependency `BSL-1.0` denotes
Boost, not that repository license. Exact declarations/notices and distribution
obligations require review before a license allow policy can become required.

## Tools and setup

Use an isolated **Linux x86_64 / Python 3.12** reporting environment. Keep tools
separate from product dependencies. The committed [29-package tool hash lock](../../scripts/dependency-audit/requirements.lock)
was captured from verified official wheel installations and passed a full resolver
dry run. It pins pip-audit **2.10.1** and pip **26.2.1**, including their transitives.
Its wheel selection is not a cross-platform product support claim.

```bash
python3.12 -m venv /tmp/ohc-dependency-audit-venv
/tmp/ohc-dependency-audit-venv/bin/python -m pip install \
  --require-hashes --only-binary=:all: \
  -r scripts/dependency-audit/requirements.lock
/tmp/ohc-dependency-audit-venv/bin/python -m pip check
```

Use the official [cargo-deny 0.20.2 Linux x86_64 musl release](https://github.com/EmbarkStudios/cargo-deny/releases/tag/0.20.2).
Verify the archive SHA-256 before extraction:
`9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`.
The executable must hash to
`b329e25933d01c36dd7c47d84ea5716694f9b7caf53a5003d45674703a8ed54a`.
The reporter checks that executable hash and version on every Cargo run; no Cargo
build is needed to install the tool. Official tool metadata records MIT OR
Apache-2.0, MSRV 1.88 (below repository Rust 1.95). pip-audit is Apache-2.0; its
tool environment contains permissive packages and certifi's MPL-2.0. Preserve
their notices when redistributing tools; no assembled-tool security or legal
clearance is inferred from version pinning.

Use repository-pinned Node from `.node-version`. The reporter checks that version
and records npm's version. For Python it checks the entire installed version set
against the tool lock and runs `pip check`; installation above verifies wheel
hashes. Runtime metadata matching is not an independent re-verification of wheel
bytes. The reporter uses a writable cache inside its new output directory.

For Cargo, first populate the required locked source/index cache and refresh a
dedicated RustSec DB cache using cargo-deny's `fetch db` with a temporary config:

```toml
[advisories]
db-path = "/tmp/ohc-rustsec-db"
db-urls = ["https://github.com/RustSec/advisory-db"]
maximum-db-staleness = "P7D"
```

```bash
/path/to/verified/cargo-deny --config /tmp/ohc-rustsec-fetch.toml fetch db
```

The reporter requires one clean RustSec Git repository at that cache, verifies its
origin and last commit timestamp, records its commit, and rejects age over seven
days, future timestamps or a commit change during scanning. Freshness means the
database's commit age, not merely the time a stale cache was copied. No automatic
database mutation happens during the offline graph scans. An unavailable or stale
cache, missing crate/index data or locked resolution failure is an operational
error, never a clean audit. Coordinate metadata cache access with active builds.
Offline yanked-version checks also depend on the pre-populated registry/index cache;
its fetch freshness is not separately receipted here. The seven-day rule applies
to RustSec, not to current registry yank status.

## Run and interpret

Choose a new output directory on each run; the command refuses to overwrite one.

```bash
python3 scripts/dependency_report.py --inventory-only --output /tmp/ohc-inventory

python3 scripts/dependency_report.py \
  --audit-python /tmp/ohc-dependency-audit-venv/bin/python \
  --cargo-deny /path/to/verified/cargo-deny \
  --db-cache /tmp/ohc-rustsec-db \
  --output /tmp/ohc-dependency-report
```

`--ecosystem npm`, `--ecosystem python` or `--ecosystem cargo` can be repeated to
produce a clearly marked partial report. `--root /path/to/checkout` selects the
source checkout; tool/policy source hashes remain recorded separately. There is
no saved-report reuse mode: every selected graph is scanned again.

`report.json` records source commit/status, hashes of graph declarations and install
consumers, an aggregate source-map hash, tool versions, exact scanner commands,
durations, raw stdout/stderr paths and hashes, selected ecosystems, and uncovered
image boundaries. Generated Cargo configuration bytes are hashed and checked for
drift around their scan. Scanner outputs remain intact. Source hashes and the tracked
lock inventory are rechecked after the run. Findings and operational errors are
distinct states, including partial reports containing both.

- Exit **0** means the selected scans completed, or inventory-only validation
  completed. The JSON state can still be `findings`. It never means release
  approval or image coverage.
- Exit **2** means report generation failed: tool/input mismatch, stale DB,
  source drift, scanner error/timeout, incomplete/invalid JSON, skipped Python
  packages, or disagreement between scanner exit code and result summary.

Known findings are retained, not ignored to produce exit zero. This deliberate
reporting policy must not be wired into a required release status as though it
were a vulnerability gate. Ratify precise failure policy separately before any
such change. No exception is currently installed. A future exception must bind
exact advisory, package/version, lock/hash, target/features and exposure evidence
to a named owner, upstream tracking, verification command and short expiry; this
reporter provides no blanket-ignore option.

## Baseline and remediation limits

The 2026-10-05 baseline has all six npm production graphs clean. Next's full graph
has five affected nodes from the one braces advisory GHSA-vfj7-8cjw-p6xm, with no
patched braces version; a Tailwind 4 migration is not a same-package patch.
Cargo's supported union has nine RustSec IDs plus yanked spin 0.9.8, including
glib unsoundness, unpatched RSA timing exposure and seven maintenance advisories.
The Android graph has five Unicode maintenance advisories. These are scanner
findings, not proofs of application exploitability.

Four Python sets contain **87 distinct alias-deduplicated advisories globally** in
the saved baseline. The reporter's `advisory_records` is the scanner's raw row
count, not that global unique count. Kimi 1.49.0 pins vulnerable aiohttp/pillow/lxml;
FastAPI 0.115.12 constrains Starlette below 0.47. Blind transitive rewrites violate
those application constraints. Remediate through compatible, tested top-level
application changes, with no forced major upgrades or invented fixed versions.

The saved baseline and primary package/advisory evidence live in the task's
`/workspace/review-output/t2-dependency-policy-baseline.md` and `t2/` evidence
directory. Those external artifacts are historical evidence, not prerequisites
for this committed command or automatic evidence about later source.

Run reporter protocol tests with `python3 scripts/test_dependency_report.py`.
They exercise actual controlled subprocesses for findings, crashes, missing
commands, timeout, partial output, skipped packages and changing inputs. Full
repository `make lint`/`make test` and required hosted acceptance remain separate.
