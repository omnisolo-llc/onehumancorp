issue_title: "Implement Agentic Universal Inbox for Unified Customer Communications"
issue_description: |
  # Research Report: Agentic Universal Inbox Architecture

  ## Executive Summary
  This report investigates the current landscape of customer communication for small business operators, specifically addressing the pain points of fragmented inboxes (Instagram DMs, WhatsApp, SMS, Web Chat, Email). The objective is to design an Agentic Universal Inbox architecture for OneHumanCorp (OHC) that unifies all communication channels and leverages our AI agents to triage, draft replies, and execute operational workflows seamlessly.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Intercom, Zendesk, and Front dominate the unified inbox space but are fundamentally designed for dedicated support teams, not solo owner/operators. Their interfaces are complex, requiring manual tagging, routing, and macro configuration. They lack deep, autonomous integration with the business's operational data (inventory, bookings, payments). While Shopify Inbox aggregates some channels, it remains a passive tool. The market gap is an inbox that acts as an active assistant, not just an aggregator.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Maya (Home Baker) receives custom cake inquiries across Instagram DMs, email, and WhatsApp. She needs to triage them, check her calendar, and send deposit links without switching apps.
  - **The Gap:** Currently, OHC lacks a unified system to aggregate these channels into a single, prioritized feed. More critically, it lacks the agentic layer to automatically draft context-aware responses, link inquiries to existing customer profiles, and suggest the next operational action (e.g., "Draft a quote for a 2-tier cake for this Saturday").

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Integration
  - **Unified Thread Schema (PostgreSQL):** A `CommunicationThread` table that normalizes messages from various providers (Meta Graph API, Twilio, SendGrid). It links to the `Customer` and `Tenant` entities.
  - **Webhook Ingestion Pipeline:** High-throughput, resilient webhooks to receive external messages, decoupled via a reliable message queue (e.g., PostgreSQL `SKIP LOCKED` or dedicated queue) to prevent dropped messages during spikes.

  ### AI Agent Coordination
  - **Triage Agent:** Analyzes incoming messages, determines urgency, identifies the intent (e.g., inquiry, complaint, order status), and prioritizes the thread in the owner's feed.
  - **Customer Assistant Agent:** Automatically drafts a context-aware reply based on the customer's history, the business's knowledge base (policies, pricing), and the current operational state (e.g., knowing the calendar is full next weekend).
  - **Operations Agent Linkage:** The Customer Assistant can trigger operational workflows directly from the thread (e.g., generating a checkout link or scheduling a booking draft).

  ### Mobile-First Implementation
  - **The "Work Feed" UI:** The inbox is not a traditional email client; it's a prioritized "Work Feed" designed for a 375px viewport.
  - **Actionable Cards:** Messages appear as cards with AI-drafted replies and suggested actions prominently displayed. One tap to approve and send, or edit if needed.
  - **Translucent Glass Design:** The interface should utilize the OHC Premium Token library, providing a clean, focused, Apple-style aesthetic.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** OHC Agentic Universal Inbox

  **Target Persona:** Maya the Home Baker

  **Outcome:** A single, prioritized feed where Maya sees all customer messages across platforms, complete with AI-drafted replies and suggested actions, allowing her to process inquiries in minutes from her phone.

  **Critical User Journey (CUJ):**
  1. A customer sends an Instagram DM asking about a custom vegan cake for Saturday.
  2. The webhook pipeline ingests the message and links it to an existing or new customer profile.
  3. The Triage Agent identifies the inquiry intent and prioritizes it.
  4. The Customer Assistant Agent drafts a reply acknowledging the request, confirming availability for Saturday (by querying the Operations Agent), and asking for design details.
  5. Maya opens the OHC app, sees the prioritized thread at the top of her Work Feed, reviews the drafted reply, and taps "Approve & Send".
  6. The system sends the response back to the customer via Instagram DM.

  **Next Actions for Engineering:**
  - **Step 1:** Define the `CommunicationThread` and `Message` schema in PostgreSQL, ensuring robust multi-tenant isolation.
  - **Step 2:** Implement the webhook ingestion pipeline for a primary channel (e.g., Web Chat or a simulated external provider) with resilient queueing.
  - **Step 3:** Develop the Triage and Customer Assistant agent workflows to process incoming messages and generate draft replies based on tenant context.
  - **Step 4:** Build the mobile-first "Work Feed" UI to display threads, drafts, and actionable next steps.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []