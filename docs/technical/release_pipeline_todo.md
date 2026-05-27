# Release Pipeline Hermeticity TODO

Last updated: 2026-05-19

This note captures the current release/CI work so it is easy to resume later.
The target state is:

- `bazel build //... --config=linux` works in CI.
- `bazel build //... --config=windows` works on `windows-latest`.
- Release GitHub Actions produce all supported artifacts from Bazel targets.
- Release artifacts include:
  - server web image
  - Windows installer/archive, eventually MSI and/or EXE
  - macOS DMG for Apple Silicon and Intel
  - Linux DEB and RPM
  - Android APK and preferably AAB
  - iOS IPA, when Apple signing/tooling is available
- Outputs are cacheable Bazel outputs.
- Toolchains are fetched, pinned, and driven by Bazel wherever practical.

## Current State

Done:

- Android Tauri mobile Bazel rules exist and build APK/AAB targets.
- Android builds use Bazel-managed versions/constants for JDK, Android SDK, build tools, API level, NDK, Gradle, and Tauri CLI.
- Gradle distribution is provided by a Bazel repository/toolchain instead of relying on a host Gradle install.
- Cargo crates for Tauri mobile are vendored through Bazel from `Cargo.lock`.
- Tauri mobile actions write a local Cargo config and build with `CARGO_NET_OFFLINE=true`.
- `buildifier` and `buildozer` are available as Bazel-provided binaries.
- RPM packaging is guarded so it only participates when Linux and `rpmbuild` are available.
- Local validation after the latest work passed:
  - `bazelisk build //src/ui/tauri:tauri_android_apk --config=linux`
  - `bazelisk build //src/ui/tauri:tauri_android_aab --config=linux`
  - `bazelisk build //release:all_release_artifacts --config=linux --nobuild`
  - `bazelisk build //... --config=linux --nobuild`
  - `bazelisk build //... --config=windows --nobuild`
  - `buildifier --mode=check` on touched Bazel/Starlark files
  - `git diff --check`

Known recent fixes:

- The CI failure `duplicate keyword argument: release` was not present in local
  `release/BUILD.bazel` after the latest changes. Current file has only one
  `release = "1"` in the `pkg_rpm` target.
- The earlier RPM error from `rules_pkg`:
  `None of the release or release_file attributes were specified`
  is addressed by explicitly setting `release = "1"`.
- The local/no-rpmbuild analysis issue is addressed by gating RPM with
  `linux_with_rpmbuild`.

## Highest Priority Next Steps

### 1. Make Gradle dependency resolution hermetic

The Gradle binary itself now comes from Bazel, but Gradle may still resolve
Maven/Google dependencies inside the action.

Goal:

- No network access during Gradle execution.
- All Maven artifacts are represented as Bazel-managed inputs.
- The Gradle action is reproducible and remote-cache friendly.

Suggested direction:

1. Inspect Android/Tauri Gradle logs with a cold Gradle cache.
2. Identify every Maven repository and module Gradle resolves.
3. Choose one hermetic strategy:
   - Preferred: use Bazel-managed Maven artifacts, for example via
     `rules_jvm_external`, and generate/preseed a Gradle offline repository
     from those artifacts.
   - Alternative: create a Bazel repository rule that reads a checked-in Gradle
     lockfile and downloads every Maven artifact with pinned SHA-256.
4. Make the Tauri mobile Bazel rule write a Gradle init script or local Maven
   repository configuration that points only at Bazel-provided inputs.
5. Run Gradle with offline mode enabled.
6. Verify with a clean environment:
   - empty `GRADLE_USER_HOME`
   - no network, if practical
   - successful `bazelisk build //src/ui/tauri:tauri_android_apk --config=linux`
   - successful `bazelisk build //src/ui/tauri:tauri_android_aab --config=linux`

Acceptance checks:

- Logs do not show Maven/Google remote downloads.
- Gradle succeeds from only Bazel-declared inputs.
- Re-running the Bazel target hits cache when inputs are unchanged.

### 2. Make Android SDK package fetching more strictly pinned

The Android SDK setup is Bazel-driven, but the SDK repository rule still obtains
packages from Android's SDK manager flow.

Goal:

- SDK commandline tools, platform tools, build tools, platform API, and NDK are
  fully versioned from one place.
- Downloads are pinned by checksum wherever possible.

Suggested direction:

1. Keep the current central version constants.
2. Audit the Android SDK repository rule.
3. Decide whether to:
   - keep `sdkmanager` for now, with pinned package names and versions, or
   - replace package fetching with explicit URLs and SHA-256 values.
4. If keeping `sdkmanager`, document this as a remaining non-hermetic edge.
5. Add a smoke target that proves the SDK repository can be created on a clean
   Linux runner.

Acceptance checks:

- Android SDK versions are changed in one Bazel/Starlark location.
- `bazelisk sync` or first build creates the SDK deterministically.
- The Android build does not depend on a host Android SDK.

### 3. Move desktop release packaging behind Bazel targets

The release workflow should not call raw `cargo tauri build` commands directly.

Goal:

- Windows, Linux, and macOS desktop packages are built by Bazel targets.
- GitHub Actions only invokes Bazel release targets and uploads their outputs.

Suggested direction:

1. Inventory current desktop release workflow commands.
2. Add Bazel targets for Tauri desktop artifacts:
   - Windows archive initially; MSI/EXE later.
   - Linux archive plus DEB/RPM integration.
   - macOS app bundle/DMG for both architectures.
3. Reuse the same Bazel-managed Cargo vendor repository where possible.
4. Ensure desktop rules use Bazel-provided Cargo/Rust toolchains instead of host
   `cargo`.
5. Wire `release:all_release_artifacts` to include the platform-appropriate
   desktop artifacts.

Acceptance checks:

- `bazelisk build //release:all_release_artifacts --config=linux`
  builds Linux artifacts.
- `bazelisk build //release:all_release_artifacts --config=windows`
  builds Windows artifacts on `windows-latest`.
- macOS release jobs call Bazel targets instead of raw shell packaging.

### 4. Replace system RPM dependency or make it explicit

`pkg_rpm` currently depends on `rpmbuild` being present. The target is now
compatibility-gated, but that is not fully hermetic.

Goal:

- RPM output should either be fully hermetic or clearly documented as requiring
  a system toolchain in CI.

Suggested direction:

1. Investigate whether `rules_pkg` can use a Bazel-provided `rpmbuild`.
2. If yes, add a pinned RPM toolchain/repository rule.
3. If no, keep the current compatibility gate and make CI install `rpm`.
4. Add comments in `release/BUILD.bazel` explaining why RPM differs from DEB.

Acceptance checks:

- Linux CI with `rpm` installed includes `:ohc_rpm`.
- Local Linux without `rpmbuild` can still analyze/build `//release:all_release_artifacts --nobuild`.
- The release workflow either provides or documents the RPM toolchain.

### 5. Finish Windows release on `windows-latest`

Goal:

- CI verifies `bazel build //... --config=windows`.
- Release workflow builds Windows application artifacts on `windows-latest`.
- Eventual output should be MSI and/or EXE, not only a ZIP.

Suggested direction:

1. Confirm `//... --config=windows` still passes in GitHub Actions.
2. Add a release job on `windows-latest`.
3. Start by uploading the Bazel-built Windows archive.
4. Add a Bazel target for Windows installer generation.
5. Evaluate installer technology:
   - WiX for MSI.
   - NSIS or Tauri bundler for EXE.
6. Make installer tools Bazel-provided and version-pinned.

Acceptance checks:

- Windows CI build passes.
- Release run uploads a Windows artifact produced under `bazel-bin`.
- Installer generation does not depend on manually installed local tools.

### 6. Add server web image to the release workflow

The release pipeline should produce the server web image as a first-class
release artifact.

Goal:

- Build and publish the server web image from Bazel.
- Use the same release workflow that uploads application packages.

Suggested direction:

1. Identify the existing server image target, or add one if missing.
2. Prefer Bazel container rules rather than ad hoc Docker commands.
3. Make image tags deterministic:
   - Git SHA
   - release tag
   - optionally `latest` only for official releases
4. Push to the intended registry from GitHub Actions.
5. Include image digest in release notes or release assets.

Acceptance checks:

- `bazelisk build` can produce the image or image tar.
- Release workflow pushes the image.
- The release summary includes the image name and digest.

## Apple Platform Work

Treat Apple separately from the Linux/Windows/Android hermeticity pass.

### macOS universal/dual architecture

Goal:

- Support both Apple Silicon and Intel macOS builds.
- Produce macOS DMG artifacts.
- Provide a clear signing/notarization path.

Suggested direction:

1. Decide whether to build:
   - separate `x86_64-apple-darwin` and `aarch64-apple-darwin` artifacts, or
   - a universal binary/app bundle via `lipo`.
2. Add Bazel configs for both macOS architectures.
3. Build on GitHub-hosted macOS runners:
   - `macos-13` or equivalent Intel runner for x86_64.
   - `macos-latest`/Apple Silicon runner if available for arm64.
4. Add signing inputs through GitHub Actions secrets:
   - Developer ID certificate
   - certificate password
   - keychain password
   - Apple ID/App Store Connect credentials for notarization
5. Keep unsigned local builds available for development.

Acceptance checks:

- macOS release job creates DMG.
- Unsigned build works without secrets.
- Signed release build works when secrets are present.
- Release assets clearly identify architecture or universal status.

### iOS IPA

Goal:

- Add a Bazel target for Tauri iOS packaging.
- Produce IPA in release workflow when Apple signing is configured.

Suggested direction:

1. Do not model iOS as fully hermetic initially.
2. Use Xcode from macOS GitHub Actions runners.
3. Use Bazel to orchestrate inputs/outputs and keep the action cacheable where
   possible.
4. Add signing/provisioning support through secrets:
   - signing certificate
   - provisioning profile
   - bundle identifier
   - team ID
5. Keep iOS release optional/skipped when secrets are absent.
6. Later, investigate Bazel-native iOS rules if they can wrap the Tauri iOS
   project cleanly.

Acceptance checks:

- `bazelisk build //src/ui/tauri:tauri_ios_ipa --config=macos` exists.
- Release workflow produces IPA on macOS when signing secrets are present.
- Workflow skips gracefully or produces unsigned simulator/dev output when
  signing is unavailable.

## CI Matrix TODO

Build verification jobs:

- Linux:
  - `bazelisk build //... --config=linux`
  - `bazelisk test //... --config=linux`
- Windows:
  - `bazelisk build //... --config=windows`
  - release build target for Windows artifacts
- macOS:
  - `bazelisk build //... --config=macos`
  - macOS app/DMG targets
  - iOS IPA target when secrets are present
- Android:
  - `bazelisk build //src/ui/tauri:tauri_android_apk --config=linux`
  - `bazelisk build //src/ui/tauri:tauri_android_aab --config=linux`

Release jobs:

- Linux release:
  - DEB
  - RPM
  - Linux archive/AppImage if desired
  - server web image
- Windows release:
  - ZIP first
  - MSI/EXE when installer target is ready
- macOS release:
  - DMG for Intel
  - DMG for Apple Silicon
  - optional universal DMG
- Mobile release:
  - Android APK
  - Android AAB
  - iOS IPA when signing is configured



## Useful Commands

```bash
bazelisk build //... --config=linux --nobuild
bazelisk build //... --config=windows --nobuild
bazelisk build //release:all_release_artifacts --config=linux --nobuild
bazelisk build //src/ui/tauri:tauri_android_apk --config=linux
bazelisk build //src/ui/tauri:tauri_android_aab --config=linux
bazelisk run //:buildifier -- --mode=check MODULE.bazel release/BUILD.bazel bazel/rules/*.bzl
bazelisk run //:buildozer -- 'print kind' //release:all_release_artifacts
git diff --check
```

For a stricter Android hermeticity check, run with empty user caches and inspect
for network-like Gradle/Maven output:

```bash
rm -rf /tmp/ohc-gradle-home /tmp/ohc-cargo-home
GRADLE_USER_HOME=/tmp/ohc-gradle-home \
CARGO_HOME=/tmp/ohc-cargo-home \
  bazelisk build //src/ui/tauri:tauri_android_apk --config=linux --sandbox_debug
```

## Files To Revisit

- `MODULE.bazel`
- `MODULE.bazel.lock`
- `bazel/rules/versions.bzl` or equivalent version constants file
- `bazel/rules/android_sdk_repository.bzl`
- `bazel/rules/gradle_distribution_repository.bzl`
- `bazel/rules/cargo_vendor_repository.bzl`
- `bazel/rules/tauri_mobile.bzl`
- `release/BUILD.bazel`
- `.github/workflows/release.yml`
- `.github/workflows/ci.yml`
- `src/e2e/global-setup.ts`

## Definition Of Done

This work is complete when:

- CI builds at least Linux, Windows, and Android from Bazel.
- Release workflow produces server image, Linux packages, Windows package,
  macOS DMG, Android APK/AAB, and iOS IPA where signing is configured.
- Release jobs upload only Bazel outputs or publish Bazel-built images.
- Tool versions are centralized.
- Host machine assumptions are either removed or documented with explicit
  compatibility gates.
- Android/Tauri builds can run with empty user Cargo/Gradle caches.
- The release workflow can be re-run and produce the same artifacts for the same
  source inputs, modulo signing timestamps and registry metadata.
