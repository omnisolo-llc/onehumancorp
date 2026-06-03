# OHC Research Report: Unified Multimodal Autonomous Customer Support Engine

## Executive Summary
This report outlines the architectural design and implementation strategy for the **Unified Multimodal Autonomous Customer Support Engine**, aimed at resolving the critical pain point of multi-channel customer communication fragmentation for small business owners (e.g., Maya and Carlos).

## Problem Statement & Gap Identification
Small business owners currently handle customer inquiries across multiple disconnected channels:
- Instagram DMs
- WhatsApp
- SMS
- Web Chat

This fragmentation leads to:
1. Missed leads and delayed responses.
2. High cognitive load and time wasted on repetitive questions.
3. Inconsistent customer service experiences.

Competitors offer basic chatbots (e.g., Shopify Sidekick) but lack a unified, proactive, and autonomous agent capable of operating across all these channels seamlessly.

## Proposed Architecture Design

### 1. Omnichannel Gateway
A unified ingestion and routing layer that normalizes messages from various platforms into a standardized internal format.
- **Webhook Receivers:** Dedicated endpoints for Instagram/Meta, WhatsApp Business API, Twilio (SMS), and native Web Chat.
- **Message Normalization:** Converts platform-specific payloads into an `OHCMessage` schema containing `sender_id`, `channel`, `content`, `timestamp`, and `attachments`.

### 2. Confidence-Based AI Routing
The core decision engine powered by the **Customer Success Agent ("The Ambassador")**.
- **Context Retrieval:** Uses pgvector to retrieve past interactions, customer order history, and business FAQs.
- **Inference & Scoring:** LLM generates a response and assigns a confidence score.
  - **High Confidence (>90%):** Auto-reply directly to the customer.
  - **Medium Confidence (70-90%):** Draft the response and hold for owner review.
  - **Low Confidence (<70%):** Escalate immediately to the owner without a draft, categorizing the intent (e.g., "Custom Order Request").

### 3. Mobile-First Review UI (375px)
A dedicated inbox view within the OHC mobile app where the business owner can quickly review drafted responses.
- **Unified Inbox:** All channels aggregated into one feed.
- **Draft Review Flow:** "Swipe right to approve & send, swipe left to edit."
- **Contextual View:** Shows the drafted reply alongside the customer's original message and relevant order details.

## Implementation Plan

### Phase 1: Omnichannel Gateway & Normalization
- Set up Webhook endpoints for Twilio (SMS) and Web Chat (MVP).
- Define PostgreSQL schema for unified messages and conversations.

### Phase 2: AI Routing & Confidence Engine
- Integrate with Gemini Pro for response generation.
- Implement pgvector similarity search for FAQs and history.
- Build the confidence scoring logic and routing queue.

### Phase 3: Mobile-First Review UI
- Develop the unified inbox Flutter screens (375px optimized).
- Implement the "approve/edit/discard" draft review flow.
- Add real-time updates via WebSocket.

## Proposed Action (Issue Brief)

```yaml
issue_title: "[feat] Implement Unified Multimodal Autonomous Customer Support Engine"
issue_priority: "P0"
issue_description: "Build an omnichannel gateway and confidence-based AI routing engine to unify customer inquiries from SMS, WhatsApp, IG, and Web Chat, including a 375px mobile-first draft review UI."
issue_todo_list:
  - [ ] Define Unified Message schema in PostgreSQL
  - [ ] Implement Omnichannel Webhook Receivers
  - [ ] Build Confidence-Based AI Routing logic (The Ambassador agent)
  - [ ] Develop 375px Mobile-First Unified Inbox & Draft Review UI in Flutter
issue_label: ["feat", "architecture", "ai-agent", "mobile-first"]
```
