# OHC Developer Setup Guide

This guide describes how to set up the OneHumanCorp hybrid development environment.

## The Hybrid CLI Master Menu

The primary entry point for all local setup and operations is the **Hybrid CLI Master Menu**. You do not need to run individual setup scripts; the master menu provides a unified, interactive terminal interface for all developer workflows.

To launch the CLI, run:

```bash
./deploy/scripts/ohc_hybrid_cli.sh
```

### Available Workflows

The Master CLI interactive menu exposes the following core operations:

1. **Run Developer Setup**
   - Bootstraps `.env` configuration, installs pre-commit hooks, and ensures build prerequisites like Bazelisk and Docker are available.

2. **Configure Environment (.env)**
   - Launches the interactive wizard to set required and optional environment variables (such as `OHC_MULTITENANT`, `DATABASE_URL`, and API keys).

3. **Run Diagnostics**
   - Verifies the state of your environment, checking dependencies, database connections, and Docker daemon health.

4. **Launch Quick Start (Standalone)**
   - Configures the environment for Standalone mode (local SQLite-backed storage) and attempts to compile and launch the Slint desktop app and local backend.

5. **Provision AI Agent**
   - Guides you through provisioning an AI agent instance (e.g., Nova or Jules) against the local backend using the `/api/agents/hire` endpoint.

6. **Standalone DB Health Check**
   - If running in standalone mode, this checks the `local_standalone.db` file and prints its tables.

7. **Swarm Status Viewer**
   - Launches an interactive status viewer querying the connected task tracking system to view current open issues and recently closed issues within your organization.

8. **Seed Database with Mock Data**
   - Calls the `/api/dev/seed` endpoint on the local server to prepopulate test tenants, products, and mock analytics.

9. **Launch Cloud Native Quick Start**
   - Configures the environment for Cloud mode (multi-tenant) and prepares a cloud-native configuration including K8s context setup for remote operation.

0. **Exit**
   - Gracefully stops the interactive loop.

By centralizing these commands into a single script, new developers do not need to remember paths to individual `.sh` files.
