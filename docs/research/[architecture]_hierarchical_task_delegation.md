# [Architecture] Issue Brief: Hierarchical Task Delegation via K8s Operators

**Title:** Hierarchical Task Delegation via K8s Operators

**Problem Statement:**
When Maya (The Home Baker) says, "Launch my Valentine's Day campaign," a single agent attempting to design a landing page, write social copy, generate images, and schedule emails suffers from "context bloat," leading to hallucinations, slow performance, and high token costs. A single AI cannot effectively act as an entire marketing department simultaneously.

**Research Report:**
*   **Framework Analysis:** Monolithic agent contexts fail at orchestrating complex projects. Current ad-hoc routing algorithms in early frameworks are unstable compared to deterministic structures.
*   **User Pain Point:** "Operational Fatigue" (Rank #2). Users need the AI to handle complex, multi-step business goals without needing to micromanage every sub-task.
*   **OHC Advantage:** We can leverage our existing Kubernetes (K8s) infrastructure (using Custom Resource Definitions (CRDs) like `TeamMember`, `Subsidiary`) to natively model these hierarchies, allowing manager agents to dynamically allocate resources.

**Design Doc:**
*   **High-Level Architecture:**
    *   **Manager Agents:** High-level planning agents responsible for decomposing a complex user goal into discrete, actionable sub-tasks.
    *   **Dynamic Sub-Agent Spawning:** A `/scale` endpoint trigger or K8s Operator that allows a Manager Agent to spin up specialized, ephemeral sub-agents (e.g., a dedicated "Copywriter Agent" with only copywriting context).
    *   **VRAM/Resource Quotas:** Manager agents can define context bounds and resource quotas for their sub-agents to optimize cost and latency.
    *   **Result Synthesis:** Sub-agents report back to the Manager, who synthesizes the final outcome for the user.
*   **UI/UX (Mobile-First):**
    *   The user sees a simple, reassuring progress indicator: "Marketing Manager is preparing your campaign... Copywriter has drafted social posts... Designer is creating images."
    *   Final approval happens via a unified Action Feed card, requiring only 1 tap.
*   **AI Agent Integration:** Manager agents must be trained to recognize when a task is too large and decompose it using the hierarchical spawning mechanism.

**Implementation Prompt:**
Design and implement the infrastructure for Hierarchical Task Delegation. Create a mechanism (e.g., leveraging K8s Operators and CRDs) that allows a "Manager Agent" to dynamically spawn specialized "Sub-Agents" with isolated contexts and resource quotas. Implement a communication channel for the manager to assign tasks to sub-agents and aggregate their results. Provide an E2E test demonstrating a Manager Agent successfully breaking down a complex goal (e.g., a multi-channel marketing campaign) and coordinating at least two sub-agents to complete it before presenting the final result.

**Priority:** P1

**Estimated Scope:** Large
