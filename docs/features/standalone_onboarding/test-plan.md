# Standalone Onboarding Test Plan

## Objective
Verify that the onboarding wizards for both Desktop and Cloud correctly evaluate their environments without mutating the host system dangerously.

## Test Cases

### 1. TestRunDesktopOnboarding
- **Given**: A temporary mocked `$HOME` directory.
- **When**: `RunDesktopOnboarding()` is called.
- **Then**: It returns `true` and the `.ohc` directory is created inside the mocked `$HOME`.

### 2. TestRunCloudOnboarding (Success)
- **Given**: `KUBERNETES_SERVICE_HOST` is set to a dummy IP.
- **When**: `RunCloudOnboarding()` is called.
- **Then**: It returns `true`.

### 3. TestRunCloudOnboarding (Failure)
- **Given**: `KUBERNETES_SERVICE_HOST` is unset.
- **When**: `RunCloudOnboarding()` is called.
- **Then**: It returns `false`.
