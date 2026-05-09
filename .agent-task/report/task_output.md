# Comprehensive Research Report: AI Agent Department Architecture

## Executive Summary
This research report investigates the necessary architectural design for OneHumanCorp's (OHC) AI Agent Departments. The OHC platform aims to provide an invisible, autonomous operations layer for non-technical small business owners. Rather than relying on standard "AI tool" paradigms (reactive, prompting-based), OHC's architecture must treat AI as "teammates" (proactive, event-driven, autonomous).

## Findings
1.  **Current Paradigm Gap**: Small business owners (like Maya the Baker, Carlos the Handyman) do not have time to learn prompting or complex dashboard configurations. Existing solutions like Shopify or Wix integrate AI primarily as content generation tools (e.g., "Write a product description"). This creates work rather than reducing it.
2.  **The Teammate Model**: OHC's unique value proposition is the "Teammate Model." Agents act as autonomous workers grouped into understandable departments (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory).
3.  **Coordination Mechanism**: These departments require a robust orchestration layer (KAIROS) to trigger agents via events (e.g., new order, new booking), schedule tasks, and maintain long-term memory.
4.  **Trust & Verification**: High-risk actions (e.g., sending emails to customers, publishing social media posts, issuing refunds) cannot be fully autonomous without user trust. A "Draft-for-Review" mechanism with a 1-tap mobile approval flow is critical.

## Competitive Analysis
| Feature | Traditional Platforms (Shopify/Wix) | OHC AI Teammate Model |
| :--- | :--- | :--- |
| **Interaction** | Reactive (User prompts AI) | Proactive (Event triggers AI) |
| **Output** | Draft text/images for user to copy/paste | Fully executable actions or 1-tap approvals |
| **Context** | Single-session, stateless | Long-term memory (pgvector), historical context |
| **Complexity** | High (Requires technical understanding) | Low (Transparent "Department" metaphor) |

## Proposed Next Steps
1.  **Implement KAIROS Orchestration**: Define the core event mesh and task queue that routes events to specific AI departments.
2.  **Develop the Draft-for-Review System**: Implement the mobile-first UX for 1-tap approvals of agent-generated actions.
3.  **Establish Unified Memory Access**: Ensure all agents can query and append to a centralized `autodream_memories` store using semantic search, strictly isolated by `tenant_id`.

A detailed issue brief has been generated to guide the implementation of these architectural components.