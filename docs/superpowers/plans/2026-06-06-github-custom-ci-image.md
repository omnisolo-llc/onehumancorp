# GitHub Custom CI Image Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Generate a GitHub-hosted larger-runner custom VM image and run CI on the custom-image runner without repeated dependency setup steps.

**Architecture:** Use GitHub's `snapshot` workflow keyword to create versions of a custom image from a dedicated image-generation larger runner. CI jobs run on a separate larger runner configured in GitHub Actions to use the latest custom image.

**Tech Stack:** GitHub Actions larger runners, GitHub Actions custom images, Bazel/Bazelisk, Docker, kind, kubectl, Helm, Tauri Linux dependencies.

---

### Task 1: Replace Docker Container WIP With Snapshot Workflow

**Files:**
- Modify: `.github/workflows/build-ci-image.yml`
- Delete: `.github/docker/ci-ubuntu/Dockerfile`
- Delete: `.github/docker/ci-ubuntu/README.md`
- Delete: `.github/docker/ci-ubuntu/SECURITY.md`
- Delete: `.github/docker/ci-ubuntu/build-ci-image.sh`

- [x] Replace the GHCR build workflow with a single `snapshot` job.
- [x] Install all Linux CI dependencies in that job.
- [x] Pull recurring Docker service images into the runner image.
- [x] Use `${{ vars.CI_IMAGE_GENERATION_RUNNER || 'ci-image-generation-ubuntu-24.04-x64' }}` for `runs-on`.

### Task 2: Move CI Jobs Onto The Custom Runner

**Files:**
- Modify: `.github/workflows/ci.yml`

- [x] Remove the `use_custom_image` workflow input and GHCR container image environment variables.
- [x] Set Linux CI jobs to `${{ vars.CI_CUSTOM_RUNNER || 'ci-ubuntu-24.04-x64' }}`.
- [x] Delete repeated disk cleanup, apt install, Kubernetes download, Docker pull/cache steps.
- [x] Keep Bazel cache setup and actual build/test commands.

### Task 3: Update Documentation

**Files:**
- Modify: `.github/CI-IMAGE-SETUP.md`
- Modify: `.github/QUICK-START-CI-IMAGE.md`
- Delete: `.github/GITHUB-DOCS-COMPARISON.md`

- [x] Document required GitHub org setup: enable custom images, create image-generation runner, create custom-image runner.
- [x] Document repo/org variables: `CI_IMAGE_GENERATION_RUNNER`, `CI_CUSTOM_RUNNER`.
- [x] Document manual validation: run image workflow, install image on runner, run CI.

### Task 4: Verify, Commit, Push, Monitor

- [x] Run `git diff --check`.
- [x] Check workflow YAML parses locally with Python's YAML parser because Ruby is not installed locally.
- [ ] Commit and push the branch.
- [ ] Trigger the custom image workflow.
- [ ] Monitor image workflow logs.
- [ ] Trigger CI and monitor until jobs pass or a concrete runner/permission blocker is confirmed.
