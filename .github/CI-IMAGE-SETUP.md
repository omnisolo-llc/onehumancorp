# CI Pipeline Optimization With GitHub Custom Images

This repository uses GitHub-hosted larger runner custom images to reduce CI setup time. The image is a pre-warmed VM image, not a Docker container image.

## What Changed

- `.github/workflows/build-ci-image.yml` generates a custom image with GitHub's `snapshot` workflow keyword.
- `.github/workflows/ci.yml` runs Linux CI jobs on the custom-image larger runner.
- Repeated CI setup steps were removed from CI jobs:
  - runner disk cleanup
  - Linux Tauri apt dependency installation
  - kind, kubectl, and Helm installation
  - Docker service image pull/cache steps

## GitHub Organization Setup

An organization owner or CI/CD admin must configure two GitHub-hosted larger runners before the workflows can run:

1. Image-generation runner
   - Platform: Linux x64
   - Image: GitHub-owned Ubuntu 24.04, or a clean Ubuntu base
   - Enable: "Enable this runner to generate custom images"
   - Recommended name: `ci-image-generation-ubuntu-24.04-x64`
   - Repository access: this repository only

2. CI custom-image runner
   - Platform: Linux x64
   - Image: Custom image `ohc-ci-ubuntu-24.04`
   - Image version: Latest
   - Recommended name: `ci-ubuntu-24.04-x64`
   - Repository access: this repository only

Set these repository or organization variables if the runner names differ:

| Variable | Purpose | Default |
| --- | --- | --- |
| `CI_IMAGE_GENERATION_RUNNER` | Runner used by `.github/workflows/build-ci-image.yml` to generate image versions | `ci-image-generation-ubuntu-24.04-x64` |
| `CI_CUSTOM_RUNNER` | Runner used by `.github/workflows/ci.yml` for Linux CI jobs | `ci-ubuntu-24.04-x64` |

## Image Contents

The image workflow installs and verifies:

- Ubuntu package updates
- build-essential, git, curl, jq, Python, OpenSSL headers
- Linux Tauri dependencies: GTK, Ayatana AppIndicator, librsvg, WebKitGTK, patchelf, pkg-config
- Bazelisk
- kind `v0.31.0`
- kubectl `v1.36.1`
- Helm `v3.20.2`
- Docker service images:
  - `pgvector/pgvector:pg16`
  - `valkey/valkey:8-alpine`

The workflow writes `/etc/ohc-ci-image.conf` with the image build timestamp and source Actions run URL.

## Build And Install The Image

1. Confirm custom images are enabled in the organization or enterprise Actions policy.
2. Confirm the image-generation runner exists and has repository access.
3. Run the "Generate CI Custom Image" workflow manually.
4. Wait for the workflow job to finish and for GitHub to finish provisioning the image version. GitHub notes this can take additional time after the workflow completes.
5. Create or update the CI custom-image runner to use custom image `ohc-ci-ubuntu-24.04` with image version `Latest`.
6. Run the "CI" workflow manually and confirm the Linux jobs execute on the custom runner without dependency installation steps.

## Maintenance

- The image workflow runs weekly to pick up OS and dependency updates.
- Each successful `snapshot` run creates a new image version. Review old versions periodically in organization settings under Actions -> Custom images.
- Keep the image-generation runner in a dedicated runner group. Do not grant broad repository access to image-generation runners.

## References

- GitHub Docs: [Using custom images](https://docs.github.com/en/actions/how-tos/manage-runners/larger-runners/use-custom-images)
- GitHub Docs: [Managing larger runners](https://docs.github.com/en/actions/how-tos/manage-runners/larger-runners/manage-larger-runners)
