# Task Output Report: AI Agent Department Architecture

## Executive Summary
This report details the architectural foundation for OneHumanCorp's AI Agent Departments. Our research addresses the critical need for a cohesive structure governing how specialized AI agents operate, coordinate, and interact with non-technical small business owners (e.g., Maya, Carlos). The primary deliverable is a comprehensive issue brief designed to guide implementer agents in building trust-centric AI features.

## Research Findings
- **The Platform Gap:** Existing tools like Shopify and Wix require manual configuration and stitching together disparate apps. OHC's "Unfair Advantage" is proactive, department-based AI.
- **Trust and Control:** The biggest barrier to AI adoption among small business owners is fear of autonomous mistakes. Our research concludes that a strict separation of "Auto-Execute" and "Draft-for-Review" actions is paramount.
- **Context is King:** Siloed agents fail. A shared memory bus is required so "The Salesperson" knows what "The Manager" has in stock.

## Proposed Architecture (Highlights)
- **7 Core Departments:** Operations, Marketing, Sales, Customer Success, Finance, Legal, and Business Advisory.
- **Coordination Model:** Event-driven orchestration. Agents emit events (e.g., `New Order Paid`) rather than directly invoking each other, ensuring decoupled, scalable workflows.
- **Unified Memory:** Agents share short-term (session) and long-term (vectorized) memory to maintain consistent customer context across touchpoints.
- **Tier-Based Budgeting:** Multi-tenant usage is tightly controlled, with clear limits tied to SaaS tiers (Free, Starter, Pro) and enforced at the Orchestrator level.

## Actionable Next Steps
1. A detailed architectural design document has been published to `docs/research/[architecture]_ai_agent_department_architecture.md`.
2. The immediate implementation priority (P0) is the "Draft-for-Review" approval engine for the Customer Success department. This will establish the foundational trust mechanism for all future autonomous agent actions.
