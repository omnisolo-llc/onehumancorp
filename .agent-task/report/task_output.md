issue_title: "[Research] Architect the Missing Feed Triage & Intelligent Grouping Engine"
issue_description: |
  # Research Report: Unified Agent Action Feed & Task Coordination System

  ## 1. Executive Summary
  Based on a codebase audit and competitive analysis of the SMB operator landscape (e.g., Shopify, Notion, Tencent Workbuddy), there is a critical architectural gap in OneHumanCorp's action feed system. While the storage layer (`AgentFeedRepository`) and basic ingestion endpoints exist (e.g., `/feed/action_required`), OHC completely lacks the **"Work Triage Engine"** needed to group, deduplicate, and prioritize the massive influx of system events. OHC must guide users from **unclear work → clear next action in minutes**. Without triage, the feed will quickly devolve into a noisy stream of disconnected alerts, confusing owners like Fatima (food cart) or Maya (baker) who need a concise daily brief on a 375px mobile screen.

  ## 2. Competitive Audit & The Feed Noise Problem
  - **Traditional SMB SaaS (Shopify, Wix, Square):** Highly siloed. You go to "Orders" for fulfillment, "Marketing" for promos, "Inbox" for messages. This requires manual polling across views.
  - **Modern Work Assistants (Tencent Workbuddy, Slack/Teams):** Often suffer from alert fatigue. The feed becomes a chronological dump.
  - **The OHC Differentiator:** The assistant must proactively identify the most important tasks across all domains, group them intelligently (e.g., collapsing 3 identical "Low Stock" alerts into 1), and present a single "Action Card" (e.g., "Draft reply to 3 new cake inquiries").

  ## 3. Product-use evidence (Startup Blocker)
  - **Persona:** Operator / Developer
  - **Attempted flow:** Start the local full-stack environment using `docker compose -f deploy/docker-compose.yml -f deploy/docker-compose.override.yml up -d --build` to inspect the UI manually.
  - **Observed issue:** The local Docker deployment fails during initialization due to a host overlayfs volume error (`Error response from daemon: failed to mount /tmp/containerd-mount... invalid argument`). Additionally, fetching bazel dependencies (`rules_perl`, `bats-core`) fails. Under the **Startup Exception Only** rule, this blocker restricts further UI E2E manual testing until fixed, but the backend structural gap regarding the Triage Engine remains valid for architectural scoping.

  ## 4. OHC Gap Identification
  A code search reveals that `AgentFeedRepository` implements raw `SELECT ... UNION ALL` queries across `agent_feed_items` and `agent_approvals`. The endpoints in `gateway.rs` and `agent_feed.rs` merely pass data through.

  **Missing Capabilities:**
  1.  **Deduplication & Grouping:** No backend mechanism to combine related alerts (e.g., 5 unread messages from the same user).
  2.  **Priority Scoring:** No engine to surface a P0 "Payment Failed" alert above a P3 "Weekly Report Draft" alert.
  3.  **Cross-Domain Coordination:** Salesperson and Customer Support agent tasks are inserted individually without a centralized `WorkTriageService` determining the priority.

  ## 5. Architectural Design Proposal (Track 2)

  ### 5.1 System Architecture Diagram
  ```mermaid
  graph TD
      subgraph Sources
          Sys[System Alerts]
          Msg[Customer Messages]
          Comm[Commerce/Orders]
          Ag[AI Agents / Jobs]
      end

      subgraph New Triage Engine
          Ingest[Ingestion Service]
          Group[Grouping & Deduplication]
          Prioritize[Priority Scoring Model]
      end

      subgraph Storage
          DB[(PostgreSQL)]
          Cache[(Redis - Feed Cache)]
      end

      subgraph Frontend (375px Mobile)
          FeedUI[Unified Action Feed]
      end

      Sys --> Ingest
      Msg --> Ingest
      Comm --> Ingest
      Ag --> Ingest

      Ingest --> Group
      Group --> Prioritize
      Prioritize --> DB
      DB <--> Cache
      Cache --> FeedUI
  ```

  ### 5.2 Data Model Principles
  - **Multi-Tenant Isolation:** Maintained via existing Row-Level Security and `tenant_id` on all entities.
  - **Grouping Invariants:** Introduce `group_id` or `correlation_id` in the `agent_feed_items` schema to tie related alerts together.
  - **Priority Scoring:** Introduce a `priority_score` (integer) to sort items chronologically *and* by urgency.

  ### 5.3 Mobile UX Flow (375px)
  1. **The Daily Briefing:** Clean, translucent glass UI showing the single most critical item at the top.
  2. **Grouped Cards:** Instead of 5 message alerts, show: "5 new messages needing replies [Review All]".
  3. **Interaction:** Tapping expands the group contextually. Large 44x44px touch targets.

  ## 6. Implementation Prompt (For Implementer Agents)
  **Objective:** Architect and implement the core `WorkTriageService` backend engine, update the database schemas, and create the mobile-first UI components to display intelligently grouped, prioritized action items.

  **Critical User Journey (CUJ):**
  1. (Backend) The system generates 3 low-stock alerts for the same product and 1 urgent "Deposit Failed" alert.
  2. (Backend) The new `WorkTriageService` intercepts these, dedupes the 3 stock alerts into a single grouped item, and scores the deposit failure as higher priority.
  3. (Frontend) The business owner opens the OHC PWA on a 375px mobile screen.
  4. The owner sees the urgent "Deposit Failed" card first, followed by a single "Low Stock (3 items)" card.
  5. The owner resolves the deposit issue, and the feed immediately updates.

  **Acceptance Criteria:**
  - Update `agent_feed_items` schema to support `priority_score` and `correlation_id`.
  - Implement a `WorkTriageService` in Rust that handles ingestion, grouping logic, and priority calculation before database insertion.
  - Update frontend components to handle grouped data and display priority styling.
  - Ensure 100% test coverage for the new triage logic and E2E Playwright tests for the feed interaction.
  - The UI MUST strictly adhere to 375px width constraints, 44x44px touch targets, and use translucent glass styling. No mock data.

  **Priority:** P0
  **Estimated Scope:** Medium/Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
