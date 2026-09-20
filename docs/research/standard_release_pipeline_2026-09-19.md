# Standard release tooling and full-matrix verification

Date: 2026-09-19. Project: OneHumanCorp. Source baseline: `c3716d0875df6403322af4fb47d9f56f9042af3c` plus the existing uncommitted native migration and this release-pipeline change. A HEAD hash alone does not identify the dirty source used by local checks.

## Approved objective and scope

Adopt established open-source release practices without another build-system rewrite: official Tauri desktop packaging, existing native platform runners, a measured cargo-dist archive-compatibility evaluation, draft-first publication, artifact provenance and explicit Linux compatibility. Preserve the seven backend targets, five desktop targets, Android universal APK/AAB, web-image archive, version propagation and all strict CI gates.

The workflow is implementation, not proof that every platform has built. No tag, commit, push, GitHub Actions dispatch, public or draft release, notarization request or real provider/customer operation was performed during local fixture verification. External tool binaries were downloaded from their official repositories and checked against SHA-256 before execution.

## Findings and changes

### 1. Replace custom desktop discovery with the official action

The previous workflow built installers with direct commands and independently searched output folders using `find`, `head -1`, and PowerShell first-match selection. Those searches were not tied to the action's reported target or artifact set, and could select stale or ambiguous outputs.

The workflow now uses `tauri-apps/tauri-action` action-v1.0.0 at immutable commit `1deb371b0cd8bd54025b384f1cd735e725c4060f`. Its documented custom runner invokes `node ../../../scripts/run-tauri.mjs` from `src/ui/tauri`; explicit target, bundles and Cargo `--locked` are passed through. This retains the existing source-bound Next service, Node runtime pin and native-resource preparation rather than replacing them with static HTML.

The action has no release ID, tag name or release name inputs and does not upload a platform release itself. `scripts/collect_desktop.py` uses `artifactPaths` and `appVersion`, checks target containment, requires exact paired installer/signature files, rejects duplicate/missing/empty output, and preserves the OHC release names. Only the completed full-matrix publisher can make a release public.

Windows portable packaging now uses a ZIP operation that retains hidden directories, including `.next`, alongside `OmniSolo.exe`, Node, runtime manifests and licenses. macOS ZIP packaging continues to use `ditto` so application bundle metadata is preserved. The collector is policy and compatibility code; it does not rebuild Tauri's platform bundlers.

### 2. Correct macOS notarization ordering

Upstream source inspection distinguishes two outputs: Tauri notarizes and staples the `.app`, then creates its signed updater tarball; its DMG bundler signs the `.dmg` but does not itself perform the same notarization submission. The previous generic post-build staple/validate sequence was insufficient to establish DMG notarization and risked changing the application after update artifacts were created.

When the complete Developer ID credential set is configured, the workflow stores a notarytool profile in its temporary signing keychain. The collector verifies the already-notarized app without changing it, submits the separately signed DMG through that profile, requires an `Accepted` response, staples and verifies only that DMG, then copies packages and records checksums. The updater tarball and signature are not rewritten. Failure or rejection occurs before release payload recording. No Apple credentials are embedded in the collected files or in the helper's command arguments; the credential-bearing setup occurs only in the protected workflow step.

Conditional macOS signing behavior is retained: an unsigned/ad-hoc build is not reported as notarized. Actual Apple validation requires a macOS runner and configured credentials; injected subprocess responses in unit tests are explicitly not notarization evidence.

### 3. Verify remote drafts before publication

Previously `softprops/action-gh-release` used `draft: false`. The local package set was checked first, but failure during upload could expose an incomplete public release.

The upload now creates an unpublished draft. `scripts/verify_release_upload.py` reads the returned numeric draft ID and paginated assets using GitHub CLI. It checks the draft flag, exact version tag, source SHA, stable/prerelease channel, local complete manifest, all 32 expected filenames, unique asset IDs, uploaded states, byte sizes and GitHub-reported SHA-256 digests. It also checks the generated checksum list and updater version. Missing digest data is a failure, not guessed success.

Only successful verification writes `verified_release_id` to the step output. The later PATCH uses that validated numeric ID, has a normal success dependency rather than `always()`, and turns `draft` false while applying the selected prerelease/latest policy. Build-only runs do not execute either network-write step. Interrupted upload or verification leaves the draft unpublished for explicit operator recovery; existing-release refusal and non-overwrite policies remain.

This is a verified snapshot at the publication boundary, not a claim that GitHub provides an atomic upload transaction or that a privileged administrator cannot modify assets concurrently.

### 4. Preserve the matrix and add provenance

No OS, CPU or archive variant was dropped. Both macOS architectures remain separate installers with matching Node binaries, not a claim of universal binaries. Windows GNU is retained alongside Windows x64/ARM64 MSVC. Android still produces one APK and AAB supporting the declared four ABIs; iOS was not in the prior release matrix.

Linux x86_64 backend release now uses Ubuntu 22.04, matching the declared baseline used by the Linux ARM and desktop rows instead of drifting with `ubuntu-latest`. This controls compiler/runtime-library assumptions; installation tests on supported distributions remain necessary. The backend and desktop macOS rows use one Cargo compiler job; other rows use two. This reduces concurrent compiler pressure on the smaller standard Apple Silicon machines without removing a platform. Desktop dependency setup only installs root/web Node trees; CLI testing remains in the unchanged full CI quality gate.

Each of the four builder families is configured to attest its own finished `dist/*` files using `actions/attest-build-provenance` v3 at immutable commit `977bb373ede98d70efdf65b84cb5f73e068dcc2a`. OIDC and attestation-write permissions are job-scoped. Attestations run only for publication-intended builds, supplement signatures/checksums, and cannot be substituted for tests or actual asset upload verification.

## cargo-dist compatibility investigation

The probe used the official Linux x86_64 prebuilt `cargo-dist` 0.33.0. Archive SHA-256:

```text
4b3f0a5f0ebbdb798f6db649d01b32ba1518376b6f7a0502b7d92b75cc2c8293
```

The executable in that distribution is named `dist`, not `cargo-dist`; an initial extraction-name assertion caught this before execution. The downloaded checksum matched the official release checksum. No arbitrary install script was executed.

### Representative packaging fixture

A temporary three-package Cargo workspace used the actual OHC package/executable names and version 1.2.3, but each executable only printed `PACKAGING PROBE ONLY`. It contained fixture files at `src/server/migrations/fixture.sql` and `src/server/db/migrations/fixture.sql`. This models topology, not actual product behavior. `dist build --artifacts=local --target=x86_64-unknown-linux-gnu --output-format=json` compiled these small programs; no fake-artifact/`lies` mode was used.

| Configuration / attempt | Observed result | Meaning |
|---|---|---|
| Include both migration directories directly | Failed with a same-name destination collision at archive-root `migrations/` | The simple include configuration does not preserve the two nested paths. |
| Include the parent `src` directory | Linux probe succeeded; produced three archives, one per Cargo package | Nested paths were preserved, but each archive had one executable at its root and duplicated migrations. The required combined `omnisolo/bin/` layout was not preserved. |
| Build the GNU Windows probe target from Linux | Stopped because `cargo-xwin` was not installed | No Windows cargo-dist package was produced or certified. This is a tool prerequisite, not a failure of an OHC Windows binary. |

Local probe logs/layouts are under `target/release-verification/dist-parity/`; temporary fixtures were removed. The current backend package requires one archive per target containing all three executables under `bin/`, both nested migration trees and a version/source/file manifest. Adopting cargo-dist here would require a generic-project wrapper, special build command or repacking layer. Retaining the existing small archiver is simpler and preserves compatibility. This is the explicit outcome of the approved parity-first evaluation, not an assertion that cargo-dist is incapable of more customized layouts.

No cargo-dist configuration or generated workflow was added to production. A separate actual OHC Windows-target Cargo check, when available, is recorded independently from this fixture investigation.

## Verification record and boundaries

The new regressions were first executed against the missing/old behavior and failed. After implementation, they check all five desktop platform policies, macOS notarization rejection, hidden portable files, mismatched versions and artifact paths, remote draft corruption/missing files/channel errors, and publish-step order. Existing release tests continue to check all seven backend archive layouts, all fourteen artifact groups and version/lockfile propagation.

Final verification passed: **13 new pipeline regressions**, **12 existing release-contract regressions**, the complete **66-entry Node script suite** with no failures/skips, and **13/13 preserved deployment/security contract groups**. The Python suites are also invoked by Node wrappers, so these counts overlap and must not be added into a misleading unique-test total. Actionlint accepted both CI/release workflows; changed JavaScript ESLint, targeted Rust formatting and `git diff --check` passed. These are scoped pipeline/compatibility results, not full application or signed-release certification.

The actual signed macOS/Windows/Android build matrix, Linux installers and hosted complete CI on the final committed revision are still required for release certification. This change keeps these requirements blocking. It neither fixes unrelated remaining application lint/test debt nor claims a 30-minute hosted run from local script execution.

### Actual Windows-target source check

Unlike the small cargo-dist topology probe, the following command checks the actual OHC backend, agent and worker sources:

```sh
CARGO_TARGET_DIR=target/release-verification/windows-check \
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 MALLOC_ARENA_MAX=2 \
cargo check --locked --target x86_64-pc-windows-gnu \
  -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker --bins
```

Its first run completed successfully in Cargo-reported **11m 05s**, including native dependency build scripts, with one platform-specific unused-parameter warning. `request_shim_shutdown` used its child parameter only under Unix. The helper is now compiled separately for Unix and non-Unix, retaining the same Unix signal and existing bounded non-Unix wait/kill behavior without suppressing a lint. The post-fix `--offline --locked` check passed in **157.37 seconds**, with **zero compiler warnings** and all **1,093 covered Rust/protocol/Cargo files unchanged** throughout. Its source fingerprint is `c33123fffed1f01a432b675e96250aacfce730f4c1c6104b6312935c10a75a51`; the command, result and evidence limit are stored locally in `target/release-verification/windows-cargo-final.json`. This rerun reused dependencies and is not a cold-build timing. This is a Cargo compatibility/type/build-script check, **not executable linking, a native Windows launch, or an installer test**. It must not be described as the entire Windows release matrix passing.

## Primary references reviewed

- [Pinned official Tauri action documentation](https://github.com/tauri-apps/tauri-action/blob/1deb371b0cd8bd54025b384f1cd735e725c4060f/README.md) is complemented by the exact [action inputs/outputs](https://raw.githubusercontent.com/tauri-apps/tauri-action/1deb371b0cd8bd54025b384f1cd735e725c4060f/action.yml) and [build implementation](https://raw.githubusercontent.com/tauri-apps/tauri-action/1deb371b0cd8bd54025b384f1cd735e725c4060f/src/build.ts).
- [Tauri application notarization implementation](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-bundler/src/bundle/macos/app.rs) and [DMG bundler](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-bundler/src/bundle/macos/dmg/mod.rs).
- [GitHub release-asset API](https://docs.github.com/en/rest/releases/assets) documents uploaded asset states, sizes and SHA-256 digests.
- [cargo-dist 0.33.0 release](https://github.com/axodotdev/cargo-dist/releases/tag/v0.33.0) and [configuration reference](https://axodotdev.github.io/cargo-dist/book/reference/config.html).
- [Tauri AppImage guidance](https://v2.tauri.app/distribute/appimage/) explains Linux baseline and ARM packaging constraints.

See [the operational release guide](../development/native-releases.md) for triggers, credentials and manual build-only use.
