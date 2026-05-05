# Autonomous Department Agents

## Problem Statement
Small business owners wear too many hats. They don't just need a website builder; they need a team. Current "AI assistants" in platforms like Shopify (Sidekick) are reactive chat bots. Business owners need proactive agents that act like employees, organized into understandable departments (Marketing, Operations, etc.), to handle background tasks autonomously.

## Research Report
Based on a deep competitor audit and SMB pain point analysis:
- **Pain Point #6**: "Marketing Paralysis" - owners know they should do marketing but don't have the time or expertise.
- **Competitor Gap**: Competitors use AI for one-off tasks (e.g., generating a website layout initially) or offer reactive chatbots. None offer a persistent, autonomous agentic architecture organized by business function.
- **AI Differentiation**: OHC's core differentiator is the "AI Does the Work" principle, organizing AI into functional departments (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory) that operate persistently in the background.

## Design Doc
- **Core Architecture**: Agent framework with distinct profiles, tools, and system prompts for each department.
- **Key Relationships**: Agents interact with the core platform services (e.g., the Marketing agent interacts with the product catalog to generate social posts). Agents share context via a central vector database (memory).
- **UI Wireframes/Flow (Mobile-First 375px)**:
  - **"My Team" View**: A screen listing the different agents/departments and their current status or recent actions.
  - **Approval Queue**: A centralized feed where owners review and approve actions proposed by the agents (e.g., "The Promoter has drafted a new Instagram post for approval").
- **AI Agent Integration**: This is the core framework. It involves setting up the background job queue for agents, the tool execution environment, and the approval workflow UI.

## Implementation Prompt
Design and implement the base framework for Autonomous Department Agents. This includes creating the backend infrastructure for agents to run in the background (using the Redis distributed lock and job queue), defining the specific system prompts and toolsets for the "Marketing & Advertising" and "Operations" departments as initial prototypes, and building the frontend "Approval Queue" where the owner can review and approve agent-initiated actions.

## Priority
P0

## Estimated Scope
Large
