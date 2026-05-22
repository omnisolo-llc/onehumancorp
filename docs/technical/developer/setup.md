<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# OHC Hybrid Developer Setup

This guide walks you through setting up a development environment capable of working across OHC's various hybrid execution modes.

## Master CLI
For developers, the unified entry point for environment operations is the OHC Hybrid CLI. This interactive menu allows you to navigate onboarding scripts interactively.

```bash
bash ohc_hybrid_cli.sh
```

## The Problem
OHC is designed as a single monorepo that targets vastly different runtime environments:
- **Cloud-Native Mode**: Orchestrates horizontally scalable containers with PostgreSQL and Redis.
- **Standalone Desktop Mode**: Packages the backend natively for local execution against SQLite.
- **Headless API**: An API-only target optimized for remote client connection.

Manually toggling `OHC_MULTITENANT`, `OHC_HEADLESS`, and `DATABASE_URL` during development can cause significant friction.

## Automated Setup
To completely configure your environment, simply execute the `ohc-setup` script.

```bash
./deploy/scripts/ohc-setup.sh
```

**What this does:**
1. Checks for required dependencies (`bazelisk`, `docker`).
2. Generates a `.env` template if one does not exist.
3. Performs a dry-run build for both `Standalone` and `Cloud` targets to ensure your environment is fully operational.
4. Generates an intelligence log in `.ohc/runtime/memory/` (or the configured `OHC_MEMORY_DIR`) confirming successful provisioning.

## Mode Switching CLI
Once the initial setup is complete, you will frequently need to compile or run the server under different modes. We provide the `ohc-mode.sh` script to configure your current terminal session cleanly.

Always use `source` when calling this script, as it exports environment variables to your current session.

```bash
# Target the Standalone Desktop SQLite build
source deploy/scripts/ohc-mode.sh standalone

# Target the Multi-Tenant Postgres Cloud build
source deploy/scripts/ohc-mode.sh cloud

# Target the Headless API server
source deploy/scripts/ohc-mode.sh headless
```

### Manual Configuration
Under the hood, `ohc-mode.sh` manages the following environment variables:

| Mode | `OHC_MULTITENANT` | `OHC_HEADLESS` | `OHC_SOURCE_MODE` |
|------|-------------------|----------------|-------------------|
| **cloud** | `true` | `false` | `cloud` |
| **standalone** | `false` | `false` | `standalone` |
| **headless** | `false` | `true` | `cloud` |

## Visual Excellence
As outlined in the OHC Architecture guidelines, all interfaces (including the CLI setup logs) must maintain absolute clarity.

> Note: For any E2E frontend development, if you require dynamic network conditions (such as simulating a degraded connection in Headless Mode), refer to the Playwright intercept patterns outlined in the central developer guide.

</div>
