<<<<<<< HEAD
# Agent Feed Deep Dive

This document details the architecture and implementation strategy for the Agent Feed, a core component of the OHC platform that brings the "Invisible AI Automation" vision to life.

## Overview
The Agent Feed is the central nervous system for business owners using OHC. Unlike traditional dashboards that require the user to seek out information or initiate actions, the Agent Feed proactively pushes critical updates, suggested actions, and drafted communications directly to the user's mobile device for review and approval.

## Key Components

### 1. Event Ingestion Pipeline
- **Sources**: Webhooks (Stripe, Instagram Graph API), Internal State Changes (Inventory updates, New orders), Scheduled Jobs (Weekly summaries).
- **Mechanism**: Events are published to a central message bus (e.g., Redis Pub/Sub or Kafka) and processed by asynchronous workers.

### 2. Intent & Context Resolution (LLM Layer)
- **Classification**: When an event occurs (e.g., a customer DM is received), the LLM classifies the intent.
- **RAG Integration**: The system queries the user's specific business data (inventory, policies, FAQs) to build context.
- **Draft Generation**: The LLM generates a proposed response or action based on the context.

### 3. Notification & Approval UX
- **Mobile First**: All notifications are designed for a 375px viewport.
- **Action Cards**: Users receive "Action Cards" in their feed containing the drafted message/action and simple "Approve", "Edit", or "Discard" buttons.

## Example Workflow: The Ambassador
1. Customer DMs Maya on Instagram asking about vegan cake availability.
2. Instagram Graph API webhook triggers an event in the OHC backend.
3. Event is processed: Intent classified as "availability inquiry".
4. System queries Maya's inventory: Vegan cakes are in stock.
5. LLM drafts response: "Yes, we have vegan cakes available! Would you like to order?"
6. Action Card is pushed to Maya's OHC app feed.
7. Maya taps "Approve" -> Response sent to customer via Instagram Graph API.

## Implementation Priorities
- **Reliability**: Ensure the event pipeline is robust and handles retries gracefully.
- **Latency**: Optimize LLM calls to generate drafts quickly.
- **UX**: Focus on clear, intuitive action cards that require minimal cognitive load from the user.
=======
# Mobile-First Agent Workflow Deep Dive

## Introduction
Based on previous market mapping, this document dives deep into a specific feature to leapfrog the competition: **The Unified Agent Feed**. Current Small Business Platforms have clunky mobile experiences centered around complex dashboards. OHC's key advantage is invisible operations managed by AI, surfaced via a mobile-first UI.

## Persona Case Study: Carlos the Handyman
- **Context:** Carlos is at a job site. He receives an urgent repair request via his website, which he runs through OHC.
- **Competitor Flow:** Carlos gets an email notification. He has to open a web browser, login to a platform, navigate to "Inquiries", find the request, and draft a manual response or use a complex quote generator.
- **OHC Agent Feed Flow:** Carlos opens the OHC mobile app. The "Operations" agent has already drafted a response and a preliminary quote based on standard pricing. Carlos sees a card in his feed: "New Inquiry: Leaky Faucet. Drafted Quote: $150." Carlos taps one button: "Approve and Send".

## Core Unresolved Pain Points in the Market
1. **Desktop Dependency:** Most platforms demand desktop usage for anything beyond checking stats.
2. **Setup Friction vs. Operational Friction:** Some platforms like Wix ADI solve setup friction but fail to reduce ongoing operational friction.
3. **Reactive vs. Proactive AI:** Shopify Sidekick waits for the user to ask questions. Business owners often don't know what to ask.

## OHC's Agentic Solution: The "Approval" Interface Paradigm
The core of the OHC mobile experience is the shift from "configuration" to "approval".

### Design Requirements for the Agent Feed
*   **375px Constraint:** The feed must be designed strictly for mobile viewports without horizontal scrolling.
*   **Action-Oriented Cards:** Each item in the feed must represent a complete, proposed action by an agent.
*   **Touch Targets:** All primary actions (Approve, Edit, Dismiss) must be large, easily tappable buttons.
*   **Visual Hierarchy:** Urgent operational tasks (e.g., fulfilling an order) must be prioritized over advisory insights.

### Agent Types and Example Proposals
*   **Operations Agent:** "You have 3 new cake orders for Saturday. Would you like me to generate a consolidated ingredient shopping list?" -> `[Generate List]`
*   **Marketing Agent:** "It's been a week since your last post. I've drafted an Instagram post highlighting your new service." -> `[Preview & Post]`
*   **Customer Success Agent:** "A customer asked about return policies. I've drafted a reply based on your settings." -> `[Approve Reply]`

## Actionable Next Steps
1.  **Develop MVP Frontend:** Create the React components for the Unified Agent Feed, starting with the card layout and the "Approve/Edit/Dismiss" interaction patterns.
2.  **Mock Agent Data:** Since the backend agents are complex, the frontend should initially be driven by mocked proposals to finalize the UX flow.
3.  **Implement Glassmorphism:** Ensure the new UI adheres to the OHC Premium Token design system.
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
