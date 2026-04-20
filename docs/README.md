# One Human Corp Documentation

This directory is the source for the repository documentation site.

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

## Conventions

- Source documentation lives under `docs/`.
- Source code lives under `srcs/`.
- GitHub issues are the task source of truth.
- Historical or superseded material belongs in `docs/archive/`.

## Start Here

- `docs/index.md`
- `docs/architecture/index.md`
- `docs/developer/index.md`
- `docs/operations/index.md`

## Site Generation

The docs website is generated from markdown with MkDocs.

```bash
python3 -m pip install -r docs/requirements.txt
mkdocs serve
```
