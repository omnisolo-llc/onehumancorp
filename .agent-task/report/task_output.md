---
title: "Unified Multimodal Autonomous Customer Support Engine Research Report"
author: "Principal Product Researcher & Oracle (L7)"
status: "Completed"
tags: ["architecture", "research", "customer_support", "omnichannel"]
---

# Unified Multimodal Autonomous Customer Support Engine Research Report

## 1. Executive Summary

Small business owners on OneHumanCorp (like Maya the Baker and Carlos the Handyman) receive customer inquiries across fragmented communication channels (Instagram DMs, WhatsApp, SMS, Web Chat). Managing these disparate channels manually introduces significant friction, delays response times, and disrupts business operations.

This research report outlines the architecture and design for a **Unified Multimodal Autonomous Customer Support Engine**. By unifying omnichannel ingestion and leveraging a confidence-based AI routing system, OHC will automatically process, classify, and intelligently route incoming messages. High-confidence AI drafts can be auto-sent or presented to the owner for one-tap approval, seamlessly integrating within the mobile-first OHC UI.

## 2. Key Insights and Gap Analysis

### Identified Friction Points
*   **Channel Fragmentation:** Owners switch between 4-5 apps just to check for messages, leading to dropped leads.
*   **Context Loss:** Inquiries often lack necessary context (e.g., product availability or service pricing), forcing the owner to manually piece information together from the OHC dashboard.
*   **Response Delays:** During working hours (or while sleeping), owners cannot reply instantly.
*   **Technical Overload:** Existing solutions require complex webhooks or API configurations that are inappropriate for non-technical users.

### The Persona Perspective
*   **Maya (The Baker):** Gets "Do you do vegan cakes?" via Instagram DM at 2 AM. Needs the AI to auto-draft a reply based on her OHC catalog and present it for her approval when she wakes up, or auto-reply immediately if she configures it to do so.
*   **Carlos (The Handyman):** Receives SMS asking for pricing on plumbing fixes while on a job. Needs a simple, unified inbox where the AI suggests a standard quote based on his OHC service list.

## 3. Proposed Architecture Design

The solution requires three core layers: Omnichannel Ingestion, AI Routing & Drafting, and the Mobile-First Review UI.

### 3.1. Omnichannel Gateway (Ingestion Layer)
A unified webhook and polling service that normalizes incoming messages into a standard `OhcMessage` struct.
*   **Supported Channels (V1):** SMS (Twilio), Instagram DMs (Meta Graph API), Web Chat (OHC Widget).
*   **Schema (PostgreSQL):** Messages are stored in a `tenant_messages` table, uniquely associated with `tenant_id` to ensure strict tenant isolation.
*   **Event Bus:** Incoming normalized messages are pushed to the AI Job Queue (PostgreSQL `SKIP LOCKED`) for processing.

### 3.2. Confidence-Based AI Routing (Processing Layer)
The "Customer Success (The Ambassador)" agent processes the `OhcMessage`.
*   **Context Gathering:** The agent queries the tenant's context (e.g., active catalog, inventory levels, business hours, FAQ docs) via RAG (Retrieval-Augmented Generation) using pgvector embeddings.
*   **Draft Generation:** The LLM generates a response draft.
*   **Confidence Scoring:** The LLM (or a secondary evaluator) assigns a confidence score (0.0 to 1.0) based on how well the tenant context answers the query.
*   **Routing Logic:**
    *   **Low Confidence (< 0.70):** Flagged for "Manual Review". The draft is saved, and a push notification is sent to the owner.
    *   **High Confidence (>= 0.70):** (Optional based on owner settings) The response is automatically sent back through the source channel.

### 3.3. Mobile-First Inbox (UI Layer)
The OHC Inbox must be the single pane of glass for all communications.
*   **Unified View:** All messages, regardless of source, appear in one stream. Each message displays a source icon (📱 SMS, 📸 Instagram).
*   **AI Draft Presentation:** Messages with AI drafts display the draft directly inline, enclosed in a premium, translucent glassmorphism container (`backdrop-filter: blur(20px) saturate(200%)`).
*   **One-Tap Actions:** The owner can tap **"Send"** (to approve and send the draft), **"Edit"** (to modify the text), or **"Generate Checkout Link"** (if the AI detects intent to purchase).
*   **Constraints:** Entirely functional on a 375px width screen. Tap targets >= 44x44px. No technical jargon.

## 4. Proposed User Flow (Critical User Journey - CUJ)

**Persona:** Maya (Baker)
1.  **Ingestion:** Customer sends an SMS: "Are you open today?"
2.  **Processing:** OHC gateway ingests the SMS. The AI Agent fetches Maya's business hours and inventory.
3.  **Drafting:** AI generates draft: *"Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?"*
4.  **Notification:** Maya receives an OHC app notification: "New message needs review."
5.  **Review (UI):** Maya opens the OHC Inbox. She sees the incoming SMS and the AI draft directly below it.
6.  **Action:** Maya taps "Send". The message is dispatched via Twilio to the customer.

## 5. Implementation Roadmap (Next Steps)

1.  **Database Migration:** Create the `tenant_messages` table and setup RLS.
2.  **API Endpoints:** Implement internal `POST /api/messages/ingest` and `GET /api/messages` routes.
3.  **Agent Integration:** Connect the "Customer Success" agent to the ingestion queue to handle draft generation and confidence scoring.
4.  **UI Implementation:** Refactor `/inbox` page to support real data fetching and the One-Tap AI Draft review component.
5.  **E2E Testing:** Implement Playwright tests verifying the ingestion, drafting, and manual approval flows.
