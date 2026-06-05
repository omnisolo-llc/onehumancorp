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
