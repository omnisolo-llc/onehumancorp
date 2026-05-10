# Research Report: OHC AI Agent Department Architecture

## Executive Summary
OneHumanCorp (OHC) is designed to empower non-technical small business owners (Maya, Carlos, Priya, etc.) by providing an invisible "digital staff" organized into functional departments. This research phase focused on defining the architecture and interaction patterns for these departments to ensure they act as proactive teammates rather than reactive tools.

## Key Findings
- **Market Gap**: Existing platforms like Shopify and Wix offer AI as isolated, reactive tools (e.g., text generators or chatbots). OHC differentiates by providing autonomous, event-driven agents that manage end-to-end business lifecycles.
- **Radical Simplicity**: To pass the "Grandmother Test," all AI interactions must be in plain language and integrated into a mobile-first (375px) dashboard with "1-Tap Approval" workflows.
- **GEO Advantage**: OHC prioritizes Generative Engine Optimization (GEO) over traditional SEO, ensuring businesses are recommended by LLM crawlers (ChatGPT, Gemini).

## Defined AI Departments
We have codified 7 autonomous departments, each with a specific mission and architecture:

1.  **Operations ("The Manager")**: Handles orders, bookings, and inventory autonomously.
2.  **Marketing ("The Promoter")**: Manages website generation, social media scheduling, and GEO.
3.  **Sales ("The Salesperson")**: Closes the loop on leads via automated quotes and follow-ups.
4.  **Customer Success ("The Ambassador")**: Manages the "never-ending inbox" with draft-for-review replies.
5.  **Finance ("The Accountant")**: Provides "Financial Fog" relief via profit tracking and plain-language reporting.
6.  **Legal ("The Protector")**: Safeguards the business with autonomous compliance and custom policies.
7.  **Business Advisory ("The Advisor")**: Orchestrates growth by analyzing cross-department data and suggesting strategic actions.

## Documentation Artifacts
Detailed architecture briefs have been created in `docs/research/`:
- `[operations]_the_manager.md`
- `[marketing]_the_promoter.md`
- `[sales]_the_salesperson.md`
- `[customer_success]_the_ambassador.md`
- `[finance]_the_accountant.md`
- `[legal]_the_protector.md`
- `[advisory]_the_advisor.md`

## Proposed Next Steps
1.  **Implementation of the "Draft-for-Review" Workflow**: Formalize the state machine in the KAIROS Orchestrator to support agentic task queuing and user approval.
2.  **Cross-Department Event Mesh**: Establish the core Pub/Sub event schemas (e.g., `tenant.order.placed`, `tenant.insight.detected`) to enable autonomous department coordination.
3.  **Mobile Dashboard V3**: Implement the "Agent Activity Feed" in the Slint/Rust UI to surface "1-Tap" actions from all departments.
4.  **Memory Layer Hardening**: Optimize the `pgvector` retrieval patterns to ensure agents have accurate, multi-tenant-safe access to business history and FAQs.

## Conclusion
By shifting the paradigm from "AI as a tool" to "AI as a teammate," OHC is positioned to leapfrog legacy e-commerce builders and become the operating system for the next generation of solopreneurs.
