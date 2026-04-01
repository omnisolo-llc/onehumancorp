<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# One Human Corp Hybrid Development Model

Welcome to the One Human Corp (OHC) Hybrid Development Model guide. The OHC platform is designed to effortlessly scale across single-user desktop environments and horizontal cloud deployments without modifying source code.

## The 3 Operating Modes

1. **Cloud-Native Mode**
   - **Scale:** Multi-tenant, Kubernetes-orchestrated (PostgreSQL, Redis).
   - **Traits:** Horizontally scaling, isolated workspaces, optimized for concurrency.
   - **Command:** `./deploy/ohc_hybrid_cli.sh cloud`

2. **Standalone Desktop Mode**
   - **Scale:** Single user, local machine (SQLite).
   - **Traits:** Minimal resource usage, host-machine efficiency. Backend services start on demand.
   - **Command:** `./deploy/ohc_hybrid_cli.sh standalone`

3. **Headless Cloud API**
   - **Scale:** Remote UI clients (Mobile/Desktop) connecting to the Cloud via API.
   - **Traits:** Purely API and metrics, no UI assets served from the backend.
   - **Command:** `./deploy/ohc_hybrid_cli.sh headless`

## Onboarding: First Day Zero Friction

We automated our dev paths into a unified CLI tool ensuring zero friction onboarding for new developers.

### Quick Start with OHC Hybrid CLI

The CLI tool orchestrates Bazel targets and Docker Compose configurations dynamically. Run the CLI tool from the repository root:

```bash
# Launch full Cloud-Native Multi-Tenant Stack
./deploy/ohc_hybrid_cli.sh cloud

# Launch Standalone Desktop Mode
./deploy/ohc_hybrid_cli.sh standalone

# Launch Headless API Server Stack
./deploy/ohc_hybrid_cli.sh headless
```

### Validating Your Workspace

OHC mandates **Zero WIP** and a **Gold Standard State**. You can verify your setup by running the full test suite with our customized build engine.

```bash
# To test all targets out-of-the-box:
bazelisk test //... --jobs=200
```

*Note: Mobile app targets have `allow_empty = True` configured in their Bazel globs so mobile SDKs are not strictly required for backend developers running tests out-of-box.*

</div>
