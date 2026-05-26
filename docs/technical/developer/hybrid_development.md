<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# One Human Corp Hybrid Development Model

Welcome to the One Human Corp (OHC) Hybrid Development Model guide. The OHC platform is designed to effortlessly scale across single-user desktop environments and horizontal cloud deployments without modifying source code.

## The 3 Operating Modes

1. **Cloud-Native Mode**
   - **Scale:** Multi-tenant, Kubernetes-orchestrated (PostgreSQL, Redis).
   - **Traits:** Horizontally scaling, isolated workspaces, optimized for concurrency.
   - **Command:** `source deploy/scripts/ohc-mode.sh cloud`

2. **Standalone Desktop Mode**
   - **Scale:** Single user, local machine (SQLite).
   - **Traits:** Minimal resource usage, host-machine efficiency. Backend services start on demand.
   - **Command:** `source deploy/scripts/ohc-mode.sh standalone`

3. **Headless Cloud API**
   - **Scale:** Remote UI clients (Mobile/Desktop) connecting to the Cloud via API.
   - **Traits:** Purely API and metrics, no UI assets served from the backend.
   - **Command:** `source deploy/scripts/ohc-mode.sh headless`

## Onboarding: First Day Zero Friction

The maintained setup flow lives directly under `deploy/scripts/`, which avoids an extra wrapper layer while keeping setup and mode switching explicit.

### Quick Start with Deploy Scripts

Run the setup and launch helpers from the repository root:

```bash
# Initialize local config and verify the workspace
./deploy/scripts/ohc-setup.sh

# Launch standalone-oriented local backend flow
source deploy/scripts/ohc-mode.sh standalone
./deploy/scripts/ohc-quick-start.sh

# Prepare cloud/headless env variants
source deploy/scripts/ohc-mode.sh cloud
source deploy/scripts/ohc-mode.sh headless

# Cloud bootstrap helper
./deploy/scripts/ohc-cloud-start.sh
```

### Validating Your Workspace

Verify your setup with the Bazel-managed test suite.

```bash
# To test all targets out-of-the-box:
bazelisk test //...
```

Use narrower targets such as `//src/server/...` or `//src/e2e:playwright` when you only need to validate a backend or browser-facing change.

</div>
