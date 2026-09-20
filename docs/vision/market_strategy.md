<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# OmniSolo Product Vision & Market Strategy

## 1. Mission: an AI operations team for client work

**Strategy version: 2026-09-18.** [Root RESEARCH.md](../../RESEARCH.md) is the canonical business model, customer, scope and validation reference.

OmniSolo helps one person win, deliver and get paid for client work. The initial customer is a solo web/design/marketing service professional. The AI team carries repeatable digital work to verified results; the owner supplies business judgment, quality review and delegated authority. Broad small-business coverage is the long-term ambition, not a requirement to launch every industry at once.

### Operating principles
1. **Completed business outcomes:** Preserve context from inquiry through delivery and collection; measure net owner time saved, retained paid use and cost.
2. **Delegated execution:** Routine work runs inside explicit standing authority; new commitments outside it require approval. Provider evidence, recovery and hard budgets are mandatory.
3. **A clear owner experience:** Show decisions, completed work and exceptions. Reuse the existing design system and runtime instead of making the owner manage an agent graph.

---

## 2. Competitive reality and differentiation hypothesis

The dated primary-source comparison in [RESEARCH.md](../../RESEARCH.md) covers HoneyBook, Claude for Small Business, Jobber and the owner's current process. Forms, proposals, payments, integrations, AI and approval controls are competitive baselines, not unique selling points.

OHC must demonstrate a better complete service workflow: persistent client context, supported digital delivery, reliable cross-tool execution, provider-confirmed outcomes, exception recovery and less owner coordination. That is a hypothesis to test through paid pilots, not a proven moat or a claim that competitors cannot do it.

The former unsourced privacy/superiority matrix is withdrawn. Do not describe competitors as exfiltrating data or claim first-of-its-kind status without specific, verified evidence. Compare actual supported behavior, costs and owner effort instead.

---

## 3. The Hybrid RAG Workflow

The following architectural concepts remain implementation context, not independent proof of production support. Verify current code, deployment mode, permissions and tests before claiming offline operation, synchronization, privacy or scalability. Preserve useful standalone behavior; the initial managed commercial offer does not authorize deleting it.
The backbone of OmniSolo's intelligence is the seamless synchronization of context across the hybrid stack.

```mermaid
graph TD
    A[Standalone Mode] -->|Private Local State| B(SQLite DB)
    B -.->|Background Sync via OmniSolo-SIP| C{Sync Engine}
    C -->|Aggregated Insights| D(PostgreSQL DB)
    D -->|Global Context| E[Cloud Swarm Orchestration]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E premium;
```

---

## 4. Existing architectural themes (subordinate to the current workflow strategy)

### I. Identity-First Autonomy
Leveraging SPIFFE/SPIRE for zero-trust agent identity. Every agent is a first-class citizen with its own cryptographically verifiable identity, enabling secure delegation across the Teammate Mesh.

### II. Visual Excellence Mandate
OmniSolo is not just a tool; it's an experience. We reject the "utilitarian" CLI-only approach. OmniSolo provides high-fidelity, user-facing dashboards with "Glassmorphism" design tokens, making swarm orchestration visually delightful.

### III. The KAIROS Triad
Our orchestration stability rests on three pillars:
- **Shared Task List**: Durable, distributed state machine.
- **Teammate Mesh**: Low-latency communication via Centrifuge and Redis.
- **AutoDream**: Long-term memory consolidation using pgvector.

---

## 5. Market Positioning
**Your AI operations team for winning, delivering and getting paid for client work.** Start with the selected service professional, one repeatable offer and one complete customer journey. Test a $99/business/month managed subscription with 10 design partners and at least 5 paying customers; price and cohort numbers are hypotheses/targets, not current results. Expand only after the retention, net-time, reliability, safety and serving-cost gates in RESEARCH.md. The customer should not need to orchestrate a vast workforce of agents.

*For more technical details, see the [KAIROS Architecture](../technical/architecture/kairos/master-design-doc.md) and the [API Playbook](../api/playbook.md).*

</div>
