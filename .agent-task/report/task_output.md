# OHC Research Report: Unified Multimodal Autonomous Customer Support Engine

## Executive Summary
Small business owners, like Maya and Carlos, struggle with fragmented customer communications across Instagram DMs, WhatsApp, SMS, and Web Chat. The current tools fail to provide a unified, mobile-first interface combined with intelligent AI routing to manage these inquiries effectively. This report outlines the architecture and design for a Unified Multimodal Autonomous Customer Support Engine to address this critical gap.

## Problem Statement & Gaps Identified
- **Fragmented Channels:** Customer inquiries arrive across disparate platforms, forcing owners to constantly switch contexts.
- **Time Sink:** Non-technical owners spend hours daily answering repetitive questions (e.g., "Do you do vegan cakes?").
- **Lack of Intelligent Automation:** Existing solutions are either simple auto-responders or complex chatbots that lack the context of the business's specific operations.
- **Mobile Unfriendly:** Management tools for omnichannel support are typically desktop-first, alienating users who run their business primarily from a 375px mobile screen.

## Architectural Design

### 1. Omnichannel Gateway
A centralized ingestion service that normalizes incoming messages from various platforms into a standardized internal format.
- **Adapters:** Specific connectors for Instagram/Messenger API, WhatsApp Business API, Twilio (SMS), and a custom Web Chat widget.
- **Normalization:** Converts platform-specific payloads into a unified `Message` entity in the database, preserving metadata (channel, original sender ID).

### 2. Confidence-Based AI Routing Engine
An intelligent layer that evaluates incoming messages to determine the appropriate response strategy.
- **Context Injection:** Retrieves the customer's history, current business FAQs, and active inventory/booking status.
- **Evaluation:** Uses the LLM to draft a response and assign a confidence score.
- **Routing Logic:**
  - **High Confidence (>90%):** Auto-reply directly to the customer.
  - **Medium Confidence (70-90%):** Draft a response and place it in a queue for the owner to review and approve with one tap.
  - **Low Confidence (<70%):** Escalate immediately to the owner as a high-priority notification.

### 3. Mobile-First (375px) Review Interface
A dedicated, premium UI designed strictly for mobile use, allowing owners to rapidly triage communications.
- **Unified Inbox:** A single scrollable list of all interactions, regardless of the source channel.
- **Quick Actions:** Swipe gestures for approve, edit, or reject drafted AI responses.
- **Glassmorphism Design:** Follows OHC premium tokens with clear visual indicators for message urgency and AI confidence.

## Implementation Plan & Action Items

```yaml
issue_title: "[architecture] Implement Unified Multimodal Autonomous Customer Support Engine"
issue_priority: "P0"
issue_description: "Build the Omnichannel Gateway and Confidence-Based AI Routing Engine to centralize customer communications from Instagram, WhatsApp, SMS, and Web Chat. Provide a mobile-first (375px) UI for owners to review AI-drafted responses."
issue_todo_list:
  - [ ] Design and implement the Omnichannel Gateway to ingest and normalize messages.
  - [ ] Develop the Confidence-Based AI Routing Engine with context injection and scoring.
  - [ ] Create the mobile-first Unified Inbox UI for reviewing and approving AI drafts.
  - [ ] Write integration tests for the routing logic and Playwright E2E tests for the new UI.
issue_label: ["architecture", "high-impact", "mobile-first", "ai-integration"]
```
