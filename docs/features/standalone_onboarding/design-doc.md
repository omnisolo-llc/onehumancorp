# Standalone and Cloud Onboarding Wizard Design Doc

## Overview
This design document details the Day One setup and verification process for the Hybrid Agent OS across two key modes: Standalone Desktop Mode and Cloud-native K8s Mode. The onboarding process eliminates setup friction by skipping unnecessary cloud dependency checks in Desktop mode and explicitly verifying critical Kubernetes variables in Cloud mode.

## Architecture
- **Standalone Mode (`RunDesktopOnboarding`)**: Checks for or provisions the `~/.ohc` local directory to ensure the environment is ready for local SQLite database creation and file-based state storage.
- **Cloud Mode (`RunCloudOnboarding`)**: Verifies that the K8s pod is running properly by checking standard Kubernetes orchestration environment variables (`KUBERNETES_SERVICE_HOST`).

## Integration
Both checks run at the startup of the OS orchestration layer inside `service.go` (`newHub`). They emit warnings on failure, ensuring resilience and visibility for the user or administrator without immediately crashing the system unnecessarily.
