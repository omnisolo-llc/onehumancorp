# Native CI and versioned release packages

The supported release workflow is `.github/workflows/release.yml`. It retains the former Bazel platform coverage using Cargo, Next/Node and Tauri. The local scripts and CI share one release-identity and artifact-completeness contract in `scripts/release_contract.py`.

## Platforms and packages

| Component | Platforms | Published files |
|---|---|---|
| Backend, builtin agent and harness worker | Linux x86_64 / aarch64; macOS Intel / Apple Silicon; Windows x86_64 GNU / MSVC and aarch64 MSVC | A versioned `.tar.gz` for each Unix target, or `.zip` for each Windows target; three binaries, runtime migrations and a version/source/checksum manifest inside |
| Desktop | macOS Intel / Apple Silicon | Separate architecture-native DMG, ZIP, signed updater tarball and signature |
| Desktop | Linux x86_64 / arm64 | AppImage, its updater signature, DEB and RPM |
| Desktop | Windows x86_64 | MSI, its updater signature and portable ZIP with the matching Node runtime |
| Mobile | Android arm64, armv7, x86 and x86_64 | Signed universal APK and AAB |
| Web service | Linux amd64 | Versioned Docker image archive |

The former GNU Windows backend archive is retained alongside the native MSVC build. Windows SQLCipher uses statically vendored OpenSSL rather than relying on a runner-specific OpenSSL directory or a missing runtime DLL. Its source version is locked in Cargo.lock; the GNU lane installs MinGW tools, and MSVC lanes select the correct native compiler environment.

The old desktop macOS universal distribution is represented by **two architecture-specific installers**, each with its matching official Node runtime. This is architecture coverage, not a claim that these files are universal binaries. The previous release matrix contained Android only; unreachable iOS signing steps were removed. iOS store/device release validation is not provided by this workflow.

## Standard build tooling and compatibility

Desktop builds use `tauri-apps/tauri-action` **action-v1.0.0**, pinned to commit `1deb371b0cd8bd54025b384f1cd735e725c4060f`. Its custom command invokes the existing `scripts/run-tauri.mjs`, so the source-bound Next service and architecture-matched Node runtime are still prepared. Each row supplies an explicit native target, installer list and Cargo `--locked`. The Tauri action itself receives no release ID/tag and cannot publish a partial platform release.

`scripts/collect_desktop.py` consumes the action's `artifactPaths` and `appVersion`, rather than searching arbitrary old build folders. It preserves the published filenames, paired updater signatures and complete platform sets. Windows portable ZIP creation includes dot-directories such as `.next`. macOS ZIP creation uses `ditto`; for Developer ID releases, the collector verifies the already notarized application and separately submits/staples the DMG using a temporary-keychain profile before checksums are recorded. It does not modify the app or its signed updater tarball after Tauri has generated them.

Linux backend and desktop release rows use the Ubuntu 22.04 baseline on the matching CPU architecture; do not silently replace it with `ubuntu-latest`. This controls build-time compatibility assumptions, not a substitute for installation testing on every supported distribution. macOS release jobs use one Cargo compiler job to bound concurrent memory pressure on standard Apple Silicon runners; the other native rows use two. Desktop setup installs the root/web Node trees rather than the unrelated CLI dependency tree.

The backend keeps the small native Cargo archive script. A pinned `cargo-dist` 0.33.0 probe found that straightforward configuration created one archive per Cargo package, placed executables at the archive root, and collided when including the two same-named `migrations` directories. Including their parent preserved nested paths but still created three separate archives. Preserving OHC's single combined `bin/` runtime archive would require a generic-project wrapper or repacking step. That is not a reduction in maintained machinery, so no generated cargo-dist workflow or new product dependency was introduced. See the [implementation and parity record](../research/standard_release_pipeline_2026-09-19.md). This is a layout decision, not a claim that cargo-dist cannot be adapted.

Each backend/desktop/Android/web-image builder also uses a commit-pinned `actions/attest-build-provenance` action on its own finished files when publication is requested. Its OIDC/attestation write permissions are job-scoped. Attestation failures block the release; ordinary build-only dispatch does not publish attestations. Attestations supplement the package signatures and SHA-256 checks, rather than replacing either.

## Version authority

A tag such as `v0.4.48` produces product version `0.4.48` everywhere. The resolver validates canonical SemVer before expensive builds. It stamps the backend, builtin-agent, harness-worker and desktop Cargo packages, their local Cargo.lock records, root/web/CLI npm metadata and Tauri configuration together. External Cargo and npm dependency versions are not upgraded by stamping, and Cargo builds remain `--locked`.

Examples of assets for that tag:

- `omnisolo-v0.4.48-x86_64-unknown-linux-gnu.tar.gz`
- `OmniSolo-v0.4.48-Windows.msi`
- `OmniSolo-v0.4.48-Android-universal.apk`
- `omnisolo-server-web-v0.4.48-linux-amd64.tar`

Every SemVer prerelease suffix is recognized, not just alpha/beta/rc. Scheduled and branch-manual builds use `<checked-in major.minor.patch>-nightly.<run_number>.<run_attempt>` consistently for the tag, application versions, filenames and updater metadata. Prereleases never become GitHub's latest stable release. SemVer build metadata remains intact in packages; Docker tags replace the unsupported `+` with `_`.

MSI uses the numeric four-part version permitted by its packaging format. Stable versions use a zero fourth component; prereleases use the workflow run number. Android versionName remains the full product version, and versionCode uses the workflow run number. The APK's actual versionName/versionCode are checked with `aapt2` after signing. A rerun retains its run-based Android code; this is not an app-store re-upload policy. Numeric installer limits fail explicitly instead of wrapping or truncating a version.

## CI and publication gates

Release qualification calls the **same complete `ci.yml`** at the same source revision. It does not replace tests with a release smoke subset. The CI concurrency namespace is separate from the parent release workflow. CI's 30-minute reporter scopes its evidence to the named reusable CI invocation, excluding concurrently running release packaging jobs while still validating complete API pagination and exact run/attempt identity.

All backend, desktop, mobile and web-image jobs produce per-group manifests binding their artifacts to the tag, source commit, run, byte size and SHA-256 checksum. Assembly requires all fourteen artifact groups and rejects missing groups, mismatched versions/source, duplicate names, corrupted files, empty files and unreviewed extras. CI diagnostic artifacts are not mixed into the release download.

The complete payload contains 29 platform/package/signature files plus `latest.json`, `release-manifest.json` and `checksums.txt`. The checksum file includes both generated JSON files. All five desktop updater platforms must be present before publication, and URLs point to `omnisolo-llc/onehumancorp`, not the old repository name.

The publisher runs on a GitHub-hosted Ubuntu runner, uses the exact built commit as `target_commitish`, and refuses an existing release or a conflicting existing tag. It first uploads into an **unpublished draft**. `scripts/verify_release_upload.py` reads that draft and its paginated asset inventory, then checks all 32 names, byte sizes, GitHub-reported SHA-256 digests, uploaded states and local manifest/checksum consistency. Missing digests are an explicit failure, never assumed correct. Only the verified numeric release ID is passed to the final publication step. Wrong commit/tag/channel, missing files, corruption, duplicates or API failures prevent promotion. A failed upload/verification leaves the draft private and requires explicit operator recovery or a new version; automatic retries do not overwrite published assets.

## Triggers and operator prerequisites

Pushing a valid `v*` tag requests publication after all gates pass. The existing nightly schedule also requests a prerelease. Manual dispatch defaults to **`publish=false`**: it still builds and validates the complete payload and saves `verified-release-<tag>` as an Actions artifact, without creating a release or pushing the standard container images. Explicit `publish=true` enables publication; a non-tag publication must use the default branch. The specialized ARM64 cloud-image job retains its configured branch and requires the separate `publish_cloud_images=true` input.

Example build-only dispatch, after these workflow changes have been merged:

```sh
gh workflow run release.yml --ref main -f publish=false
```

Desktop updater signing requires the existing public/private Tauri signing keys and optional key password. Android requires its keystore, passwords, alias and configured HTTPS `OMNISOLO_MOBILE_WEB_URL`. macOS Developer ID signing/notarization remains conditional on the complete existing Apple credential set; a successful unsigned-app build is not notarization evidence. Registry publication retains its existing registry credentials. Missing required signatures or packages fail the release rather than silently dropping a platform.

## Verification boundary

Local workflow syntax, release metadata/assembly tests and version-stamping checks are not hosted cross-platform compilation or actual upload evidence. A full green CI run and the signed platform build matrix must pass on the final committed revision before a release can be certified. Existing application lint/test failures remain blocking; these workflow changes do not suppress them or turn earlier partial results into a full pass.

Relevant checks:

```sh
python3 scripts/release_contract_test.py
python3 scripts/release_pipeline_test.py
node --test scripts/release-workflow.test.mjs scripts/ci-performance.test.mjs
npm run test:scripts
npm run test:contracts
# With the reviewed actionlint binary installed:
actionlint .github/workflows/ci.yml .github/workflows/release.yml
```

See [native development and timing](native-build.md) and [the migration ledger](../research/native_migration_and_remediation.md) for the complete acceptance gates. Runtime package prerequisites and signing/device requirements are separate from the Linux CI time target.
