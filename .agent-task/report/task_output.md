# OHC Architecture Research Report: Closing Strategic Feature Gaps

## Executive Summary
This research report identifies and designs three critical architectural additions to the OneHumanCorp (OHC) platform to achieve parity with competitors (Shopify, Wix) while doubling down on the "AI-as-Infrastructure" and "Mobile-First" mandates.

The identified gaps — **Storefront Editing**, **Unified Fulfillment**, and **Legal Compliance** — are the primary blockers for the target personas (Maya, Carlos, Priya, Fatima).

---

## 1. [Frontend] Mobile-First Storefront & Catalog Editor
**File:** `docs/technical/research/[frontend]_mobile_first_storefront_editor.md`

### Research Findings
Current e-commerce editors are desktop-centric and rely on complex grid systems. OHC's advantage is an **AI-Assisted Conversation** that designs the storefront invisibly.

### Key Architecture
- **Marketing AI Agent**: Uses Vision LLMs to extract product details from photos and auto-suggest layouts.
- **Glassmorphic Preview**: Real-time rendering of premium UI tokens (20px blur, 200% saturation) for immediate user feedback.

---

## 2. [Backend] Unified Order Fulfillment & Operations Orchestration
**File:** `docs/technical/research/[backend]_unified_fulfillment_orchestration.md`

### Research Findings
Persona needs are modal-specific (Food vs. Service vs. Physical). A unified state machine is required to orchestrate these different lifecycles reliably.

### Key Architecture
- **Unified State Machine**: A robust gRPC-driven engine that handles `PLACED -> PROCESSING -> FULFILLED` across all modalities.
- **Operations Department ("The Manager")**: An AI agent that monitors for "stuck" orders and proactively triages delays.

---

## 3. [AI] Protector: Legal & Compliance Suite
**File:** `docs/technical/research/[ai]_protector_legal_compliance_suite.md`

### Research Findings
Legal jargon is a major friction point for first-time founders. Automating policy generation based on business context (industry/location) removes this barrier.

### Key Architecture
- **Legal & Compliance Department ("The Protector")**: An AI agent that interviews the owner and generates tailored Terms, Privacy, and Refund policies.
- **Risk Scanner**: Proactive monitoring of product descriptions for high-risk or non-compliant claims.

---

## Proposed Next Steps
1. **P0: Implement Storefront Editor Prototype**: Focus on the Maya (Baker) CUJ to prove the 10-minute "Idea -> Live" promise.
2. **P1: Operations Service Scaffolding**: Implement the base state machine and `order_history` audit logs.
3. **P2: Protector Interview Flow**: Integrate the Legal & Compliance interview into the `SetupWizardScreen`.

---

## Verification Summary
- [x] Research briefs created for all three identified gaps.
- [x] Mermaid diagrams included for architectural clarity.
- [x] Persona-specific journeys mapped to design decisions.
- [x] Bazel build stability verified.
