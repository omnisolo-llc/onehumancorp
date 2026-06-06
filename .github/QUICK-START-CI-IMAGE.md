# Quick Start: GitHub Custom CI Image

## Runner Names

Default runner names used by the workflows:

- Image generation: `ci-image-generation-ubuntu-24.04-x64`
- CI execution: `ci-ubuntu-24.04-x64`

If GitHub Actions uses different names, set repository or organization variables:

- `CI_IMAGE_GENERATION_RUNNER`
- `CI_CUSTOM_RUNNER`

## Build The Image

1. In GitHub, open Actions -> "Generate CI Custom Image".
2. Click "Run workflow".
3. Wait for the job to complete.
4. Wait for GitHub to finish provisioning the generated image version.

## Install The Image On The CI Runner

1. Go to organization Settings -> Actions -> Runners.
2. Create or edit the CI larger runner.
3. Select the Custom image tab.
4. Choose `ohc-ci-ubuntu-24.04`.
5. Choose image version `Latest`.
6. Grant this repository access through the runner group.

## Verify CI

1. Open Actions -> "CI".
2. Click "Run workflow".
3. Confirm Linux jobs run on the custom runner.
4. Confirm the old setup steps are absent from job logs:
   - `Free runner disk space`
   - `Install Linux Tauri desktop dependencies`
   - `Install Kubernetes tools`
   - `Pull E2E Docker images`
   - Docker compose image cache restore/load/pull steps

## Full Guide

See [CI-IMAGE-SETUP.md](./CI-IMAGE-SETUP.md).
