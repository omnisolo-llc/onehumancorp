issue_title: "Implement OHC Invisible AI Agent Action Feed Architecture"
issue_description: |
  # Research Report: OHC Invisible AI Agent Action Feed Architecture

  ## Executive Summary
  This report details the design for the OHC Agent Feed, addressing the core vision of "Invisible AI Automation." The objective is to build a centralized nervous system for non-technical owners that proactively pushes critical updates, drafted communications, and suggested operational actions directly to a 375px mobile viewport for one-tap approval.

  ## 1. Market Mapping & Competitor Discovery
  Traditional platforms (e.g., Shopify, Notion AI, HubSpot) rely heavily on pull-based dashboards and separate, disconnected bots requiring extensive rule configuration. Competitors like ManyChat or Wix Automations are too complex for our target personas (e.g., Maya, Carlos). The gap is a unified, push-based "Action Feed" where LLMs handle intent resolution invisibly and present a single card with "Approve", "Edit", or "Discard" actions.

  ## 2. OHC Gap & Pain Point Identification
  - **Persona Focus:** Maya (Home Baker) & Carlos (Field Service Owner).
  - **The Gap:** Maya receives DMs overnight but has to manually triage them across Instagram, WhatsApp, and email, leading to lost sales. Carlos has disjointed service requests. Currently, OHC lacks a unified async pipeline that converts incoming webhooks directly into LLM-drafted action cards in the mobile feed.

  ## 3. Deep Dive Architecture Design

  ### Data Model & Event Pipeline
  - **Ingestion:** Centralized event bus using Redis Pub/Sub and PostgreSQL `SKIP LOCKED` job queues for asynchronous background processing.
  - **Event Entities:** `AgentActionCard` stored in PostgreSQL with fields for `tenant_id`, `intent_type`, `draft_payload`, `status` (pending, approved, discarded), and `source_event_id`.

  ### AI Agent Coordination
  - **Work Triage / Classifier Agent:** Listens to the event bus, classifies intent using Gemini Pro (e.g., "availability inquiry", "booking request"), and routes to the correct department.
  - **Customer Success Agent ("The Ambassador"):** Uses RAG against Maya's inventory/policies to draft replies for DMs.
  - **Operations Agent ("The Manager"):** Generates action cards for inventory alerts or schedule conflicts.

  ### Mobile-First Implementation (375px Target)
  - **UX Flow:** A single vertical timeline feed. Each `AgentActionCard` is a distinct UI component.
  - **Interactions:** Large (≥ 44x44px) touch targets for quick "Approve", "Edit", and "Discard" actions.
  - **Design System:** Uses OHC Premium Token translucent glass styling.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** OHC Invisible Agent Action Feed

  **Target Persona:** Maya the Home Baker

  **Outcome:** Maya opens the OHC app in the morning and sees a prioritized feed of 3 drafted Instagram DM replies and 1 inventory restock suggestion. She taps "Approve" on each without writing any text or opening a separate tool.

  **Critical User Journey (CUJ):**
  1. A webhook payload from Instagram is ingested via the event pipeline.
  2. The Work Triage agent classifies the message intent.
  3. The Ambassador agent drafts a response using context from the tenant's inventory and policies.
  4. An `AgentActionCard` is persisted and pushed via WebSockets to Maya's mobile app.
  5. Maya reviews the card on her phone (375px), taps "Approve", and the system executes the response via the Instagram Graph API.

  **Next Actions for Engineering:**
  - **Step 1:** Implement the PostgreSQL schema and `SKIP LOCKED` job queue workers for `AgentActionCard` generation.
  - **Step 2:** Integrate the Work Triage agent classification pipeline with the unified event ingestion layer.
  - **Step 3:** Build the mobile-first (Flutter/PWA) Action Feed UI component to display pending cards and handle Approve/Edit/Discard state mutations optimistically.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []