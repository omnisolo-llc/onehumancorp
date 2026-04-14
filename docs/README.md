<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# One Human Corp: Platform Documentation

## Identity
One Human Corp (OHC) is the world's most autonomous, aesthetically superior, and market-aware **Hybrid Agentic OS**. Utilizing a unique **Hybrid Architecture (OHC-HA)**, the platform seamlessly scales from a horizontally scalable Cloud-Native multi-tenant service to a resource-efficient Standalone Desktop deployment. This empowers a single individual to run an entire enterprise by orchestrating highly specialized AI agents with zero friction and maximum visual delight. Our primary goal is to provide a framework where a customer can tackle any business area. The core structure revolves around:
1. **Domain Knowledge**: The industry the corporation operates in. Our foundational domain is the "Software Company". The system allows continuous import of new skills, domains, and knowledge bases.
2. **Roles**: The specific positions required within the domain. For a Software Company, these include:
   - **CEO**: Always the human user, overseeing high-level goals.
   - **Director**: Middle-management AI (e.g., Engineering Director, Marketing Director) guiding sub-agents.
   - **Product Manager (PM)**: Gathers requirements and scopes projects.
   - **Software Engineer (SWE)**: Writes and tests code.
   - **Security Engineer**: Audits infrastructure and enforces data privacy compliance.
   - **QA Tester**: Ensures product quality via automated testing.
   - **Marketing Manager**: Executes GTM strategies.
   - **Sales Representative**: Handles leads and conversion.
   - **Customer Support**: Resolves user issues.
3. **Organization**: The management hierarchy. For example, the human CEO commands an Engineering Director, who in turn manages 3 SWEs, 1 QA Tester, and 1 Security Engineer.
4. **Collaboration (Virtual Meeting Rooms)**: When the CEO defines a goal, multiple agents (e.g., PM, SWE, and Director) convene in Virtual Meeting Rooms to define scopes, debate technical constraints, and finalize designs before execution.

## Architecture
Built on a modular, open-source stack (Model Context Protocol, SPIFFE/SPIRE, LangGraph), the system utilizes a **Hybrid Architecture (OHC-HA)**. It seamlessly scales from a multi-tenant **Cloud-Native Mode** using Kubernetes and PostgreSQL, to a fully localized **Standalone Desktop Mode** powered by SQLite and local in-memory messaging. The backend is written in Go (Bazel-based monorepo), and it integrates with a Flutter frontend (supporting Web, Mobile, and Desktop) to allow the human CEO to direct virtual meeting rooms, handle high-risk approvals, and monitor token usage and billing.

### Hybrid Architecture Modes
- **Cloud-Native Mode**: Multi-tenant, K8s-orchestrated (PostgreSQL, Redis). Optimized for horizontal/vertical scaling, pod high-concurrency, and strict tenant isolation.
- **Standalone Desktop Mode**: Local single-user (SQLite). Optimized for low resource consumption, host-machine efficiency, and local-to-cloud synchronization. Services designed to degrade gracefully when heavy dependencies (Redis/Chatwoot) are absent.
- **Thin Client Mode**: UI-only (Mobile/Desktop) connecting to Cloud via API/OAuth with configurable remote endpoints. Focus on API reliability, auth stability, and low-latency interaction.

```mermaid
graph TD;
    User[Human CEO] --> Frontend[React Next.js Frontend];
    Frontend --> Backend[Go Dashboard Server];
    Backend --> Hub[Orchestration Hub];
    Hub --> Rooms[Virtual Meeting Rooms];
    Hub --> K8s[Kubernetes Cluster];
    K8s --> Agents[AI Agents];
    Agents --> DB[(Database)];
    K8s --> MCP[Model Context Protocol];

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Hub,Backend,MCP,Agents,Frontend,Rooms,User,DB,K8s premium;
```

## Quick Links
- [Hybrid Architecture](features/hybrid-architecture.md)
- [API Playbook](api/playbook.md)
- [Help Portal](walkthroughs/help_portal.md)
- [Custom Agent Creation Walkthrough](walkthroughs/custom_agent_creation_walkthrough.md)
- [System Design](system-design.md)
- [Roadmap](roadmap.md)
- [UI Testing Guidelines](developer/ui_testing_guidelines.md)

### KAIROS Orchestration Layer
- [UltraPlan Deliberation Walkthrough](walkthroughs/ultraplan_deliberation.md)
- [Distributed State Machine](features/kairos/state_machine.md)
- [Sub-Agent Queue](features/kairos/sub_agent_queue.md)
- [AutoDream Pipeline](features/kairos/autodream_pipeline.md)
- [Hybrid MCP RAG Protocol](features/hybrid_mcp_rag_protocol.md)

## Quick Start
1. Ensure you have `bazelisk` and `npm` installed.
2. Build the backend:
   ```bash
   bazelisk build //...
   ```
3. Run all tests to verify setup:
   ```bash
   bazelisk test //...
   ```
4. Run the Go backend (Dashboard Server) locally on port `8080`.
5. Serve the Bazel-built Flutter web app:
   ```bash
   bazelisk run //srcs/app:start
   ```
6. Launch standalone desktop mode:
   ```bash
   bazelisk run //:desktop
   ```

## Developer Workflow
This project uses Bazel for deterministic builds and testing.
- **Build all modules:** `bazelisk build //...`
- **Run all tests:** `bazelisk test //...`
- **Format code:** Use standard `gofmt` for Go and Prettier for the frontend.
- **Documentation:** All feature additions must include a `cuj.md`, `design-doc.md`, and `user-guide.md` adhering to the standard templates.

## Configuration
The following environment variables and configurations are commonly used:
- `GEMINI_API_KEY`: API Key for Gemini models (if using Google models).
- `MCP_BUNDLE_DIR`: Directory for MCP bundles.
- `MONO_FRONTEND_DIST`: Path to the compiled frontend dist directory.
- Kubernetes Secrets are used to inject runtime credentials safely without committing secrets to the repo.

</div>
