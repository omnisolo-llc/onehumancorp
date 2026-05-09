# Aggregate Research Report: AI Agent Department Architecture

## Executive Summary
This report defines the AI Agent Department Architecture for OneHumanCorp (OHC). OHC is designed to empower non-technical users to build and run real small businesses in under 10 minutes without touching code or reading a manual. A core component of this vision is shifting the burden of "invisible work" (e.g., handling customer inquiries, reconciling payments, and tracking inventory) onto autonomous agents that operate in the background.

The architecture organizes AI capabilities into familiar business "Departments" (e.g., "The Manager", "The Ambassador"), giving business owners an intuitive mental model to manage their AI team rather than configuring technical integrations. This report provides a detailed breakdown of these departments, how they operate and coordinate, and the overall system design.

## Key Findings
1.  **Mental Model Pivot**: Instead of technical terminology ("AI Assistant", "Webhooks", "Tokens"), users need familiar business concepts ("The Salesperson", "Approval Inbox", "Actions").
2.  **Autonomous Operation**: Unlike standard AI chatbots that wait for user prompts, OHC agents must proactively respond to events (e.g., a new Instagram DM, low inventory levels) and coordinate with each other.
3.  **Approval Workflows**: Crucial for user trust. High-risk actions (refunds, contracts) must default to "Draft-for-review" in an Approval Inbox, while low-risk tasks (inventory updates) should be "Auto-execute".
4.  **Mobile Parity**: The management and overview of these agents must be 100% usable on mobile (375px), as many personas (like Maya or Carlos) run their businesses entirely from their phones.

## Next Steps
-   **Implementation Hand-off**: An Implementer agent needs to execute the `Implementation Prompt` detailed in `docs/research/[Architecture]_AI_Agent_Departments.md`. This involves building the UI for the "Approval Inbox" and "Agent Activity Log".
-   **Backend Integration Design**: Future architectural tasks should detail the specific queue systems, state management, and LLM orchestration that will power these departments on the backend.
-   **Refinement of Memory and Context**: Further design work is needed on how agents securely and efficiently share context within a tenant (e.g., how the Ambassador passes lead information to the Salesperson).
