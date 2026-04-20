# One Human Corp Documentation

This site is the canonical documentation root for the repository. It follows a markdown-first workflow: source content stays in `docs/`, site navigation is declared with g3doc-style metadata, and the rendered website is generated from markdown at build time.

## Identity

**One Human Corp (OHC)** is the world's most autonomous, aesthetically superior, and market-aware **Agentic Operating System**. OHC empowers a single human to orchestrate a vast swarm of AI agents with zero friction and maximum visual delight.

### OHC Hybrid Architecture (OHC-HA)

The system is built on a unified core that adapts to three distinct deployment archetypes:

*   **Cloud-Native Mode**: Multi-tenant, K8s-orchestrated (PostgreSQL, Redis). Optimized for horizontal/vertical scaling, pod high-concurrency, and strict tenant isolation.
*   **Standalone Desktop Mode**: Local single-user (SQLite). Optimized for low resource consumption, host-machine efficiency, and local-to-cloud synchronization. Services are designed to degrade gracefully when heavy dependencies (Redis/Chatwoot) are absent.
*   **Thin Client Mode**: UI-only (Mobile/Desktop) connecting to Cloud via API/OAuth with configurable remote endpoints. Focuses on API reliability, auth stability, and low-latency interaction.

### Core Values

1.  **Absolute Autonomy**: Agents do not ask for permission; they propose and execute based on the Vision and Market Reality.
2.  **Aesthetic Excellence**: Every interface and artifact must feel "Premium" (Glassmorphism, 20px blur, Outfit/Inter typography).
3.  **Continuous Evolution**: We study competitors and the global market to improve OHC bit by bit daily.
4.  **Swarm Intelligence**: All agents share state via the OHC Central Database (OHC-SIP).
5.  **Full-Spectrum Observability**: Every feature exposes high-fidelity metrics via OpenTelemetry and Prometheus, with corresponding Grafana visualizations and internal dashboards.

## What Changed

- All first-party source code now lives under `srcs/`.
- Local `.agent-task` tracking has been retired.
- GitHub issues are now the task source of truth.
- Legacy design docs that lived outside `docs/` were moved into the archive.

## Start Here

- Read the architecture hub in `docs/architecture/`.
- Use the developer hub in `docs/developer/` for setup and workflow guidance.
- Use the API and walkthrough sections for operator-facing flows.
- Use the operations section for issue tracking, docs governance, and migration notes.

## Site Conventions

- Markdown is the source format for documentation.
- Navigation metadata lives in `docs/_toc.yaml`, `docs/_book.yaml`, and `docs/_project.yaml`.
- The generated website is built with MkDocs from the markdown tree; no HTML output is committed to source.
- Historical material that is not part of the primary narrative belongs in `docs/archive/`.
