<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# 🚀 Day One Onboarding: Simplifying the Hybrid OHC Experience

Welcome to **One Human Corp (OHC)**! This guide will walk you through your first day, getting your environment set up and launching your first Hybrid Agentic Swarm.

Whether you are targeting **Cloud-Native Mode**, **Standalone Desktop Mode**, or **Headless API Mode**, our simplified setup scripts take the friction out of configuration so you can start orchestrating agents immediately.

## 1. Prerequisites

Before starting, ensure your system has the following core dependencies:
- **Bazelisk:** The Bazel wrapper that standardizes build execution.
- **Go:** The primary language for the OHC Backend services.
- **Docker:** (Optional but recommended) For local isolated infrastructure (Postgres, Redis).

## 2. The Master Setup Script

To initialize your development environment, start with the master setup script:

```bash
./deploy/scripts/ohc-setup.sh
```

**What this does:**
- Validates system dependencies (Bazelisk, Docker).
- Creates a default `.env` configuration file.
- Verifies builds for both **Cloud Mode** and **Standalone Mode**.
- Bootstraps your local `.ohc/runtime/` environment directories.

## 3. The Interactive Environment Wizard

If you need to configure API keys (OpenAI, Anthropic) or customize ports and multi-tenancy settings, run the Interactive Wizard:

```bash
./deploy/scripts/ohc-env-wizard.sh
```

**Key Configuration Options:**
- **Mode Selection:** Toggle `OHC_MULTITENANT` to switch between Standalone and Cloud environments.
- **LLM Providers:** Securely inject your `OPENAI_API_KEY` and `ANTHROPIC_API_KEY`.
- **Database/Redis:** Configure external connection strings for Cloud Mode.

## 4. Launching Your First Swarm

Once your environment is configured, use the Quick Start script to launch the local backend:

```bash
./deploy/scripts/ohc-quick-start.sh
```

**What this does:**
- Enforces `OHC_SOURCE_MODE=standalone`.
- Starts the Go API server in the background.
- Runs a diagnostic check (`ohc-diagnostics.sh`) to ensure the agent hub and APIs are responsive.

## 5. The OHC Hybrid CLI Master Menu

For day-to-day operations, the OHC Hybrid CLI provides an interactive master menu. You can use it to switch contexts, run diagnostics, or provision new agents:

```bash
./deploy/scripts/ohc_hybrid_cli.sh
```

**Available Options:**
1. Run Developer Setup
2. Configure Environment (`.env`)
3. Run Diagnostics
4. Launch Quick Start (Standalone)
5. Provision AI Agent

## 6. Verifying the Gold Standard State

OHC mandates **Zero WIP**. At any time, you can verify your entire workspace by running the universal test command:

```bash
bazelisk test //...
```

This guarantees that all API endpoints, KAIROS Orchestration logic, and frontend components remain stable and ready for deployment.

---

**Next Steps:**
Now that your environment is running, check out the [Agent Lifecycle Walkthrough](./agent_lifecycle.md) to learn how KAIROS sub-agents are dispatched and tracked!

</div>
