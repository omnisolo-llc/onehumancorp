issue_title: "Implement Tap-to-Pay Terminal & Unified In-Person POS Architecture"
issue_description: |
  # Research Report: Tap-to-Pay Terminal & Unified In-Person POS Architecture

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) and Fatima (Food Cart Operator) need a reliable way to accept in-person payments securely and sync those transactions with their online inventory and sales data in real-time. Currently, there is an architectural gap where in-person point-of-sale (POS) systems are disconnected from the primary digital platform or require complex third-party hardware and software integration. They need a unified system that works seamlessly from a mobile device (375px) using Tap-to-Pay on iPhone/Android, without requiring technical knowledge to set up.

  ## Research Findings
  Our user personas demand extreme simplicity and reliability.
  - **Priya (Boutique Operator)** needs tap-to-pay visibility, product variants synced, and inventory-aware offers that reflect immediately whether sold online or in-store.
  - **Fatima (Food Cart Operator)** needs a fast tap-to-pay checkout line for walk-ups that works even on slow mobile data connections.

  ### Competitive Analysis
  - **Square Terminal API**: Employs a cloud-based API where the server initiates checkout requests to a physical Terminal device connected to the internet. While powerful, it requires separate hardware.
  - **Shopify POS**: Uses a unified backend across e-commerce and POS, syncing inventory, orders, and customer data in real-time. It relies on a local mobile app (iOS/Android) talking to the Shopify API. It's robust but has a high learning curve and separate app requirements.
  - **Stripe Terminal (Tap to Pay)**: Offers local mobile SDKs (iOS/Android/React Native) allowing compatible smartphones to act as contactless readers without extra hardware.
  - **OHC's Differentiation**: We will integrate "Tap to Pay on iPhone/Android" directly into the OHC mobile-first PWA/Tauri app. The "Sales & Revenue Assistant" agent will handle the terminal session lifecycle invisibly, while the "Operations Assistant" ensures inventory is synced instantly via CRDTs for offline-tolerance.

  ## Architectural Design

  ### System Overview

  ```mermaid
  graph TD
      subgraph Frontend "Tauri App (Mobile-First 375px)"
          UI[POS UI / Checkout Screen]
          TapToPaySDK[Tap to Pay Native SDK]
          LocalDB[(SQLite Local Sync)]
      end

      subgraph Backend "Go + Bazel Backend"
          API[Terminal API Gateway]
          TerminalSessionMgr[Terminal Session Manager]
          SyncEngine[CRDT Sync Engine]
      end

      subgraph Integrations
          Stripe[Stripe Terminal API]
      end

      subgraph AI "AI Agent Departments"
          Sales[Sales & Revenue Agent]
          Ops[Operations Agent]
      end

      subgraph Storage
          DB[(PostgreSQL - Tenant DB)]
      end

      UI --> TapToPaySDK
      UI --> LocalDB
      LocalDB <--> SyncEngine

      TapToPaySDK --> API : Collects Payment Method
      API --> TerminalSessionMgr
      TerminalSessionMgr --> Stripe : creates PaymentIntent & captures

      API --> SyncEngine : Transaction Event
      SyncEngine --> DB

      TerminalSessionMgr -.-> Sales : Records Transaction & Revenue
      SyncEngine -.-> Ops : Adjusts Inventory
  ```

  ### Data Model & Invariants
  - **TerminalSession**: Tracks the active POS session, mapping `device_id` and `tenant_id` to a specific location or employee.
  - **Transaction**: Records the payment, linked to the `TerminalSession` and `PaymentIntent`.
  - **Inventory Delta**: CRDT-based inventory adjustment to ensure eventual consistency even if the transaction happens while briefly offline (though payment capture requires network).
  - **Multi-tenant Isolation**: All requests must strictly enforce `tenant_id` at the row level via PostgreSQL RLS.

  ### Mobile UX Flow (375px First)
  1. **Checkout Screen**: Owner adds items to the cart from the visual catalog. Big "Charge $X.XX" button fixed at the bottom.
  2. **Payment Method Selection**: Tapping "Charge" slides up a bottom sheet with "Tap to Pay on iPhone/Android" as the primary option.
  3. **Tap to Pay UI**: Native OS Tap to Pay screen appears. Customer taps their card or phone.
  4. **Success & Receipt**: Immediate success checkmark with Glassmorphism styling. Options to email/SMS receipt or "Next Order".

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant**: Monitors completed POS transactions, updates daily revenue dashboards, and can draft follow-up review requests if the customer is recognized.
  - **Operations Assistant**: Subscribes to POS transaction events to decrement inventory counts immediately and alerts the owner if stock is running low.

  ## Estimated Scope
  Large

  ## Top 5 Codebase Anomalies Identified During Review
  1. The documentation and README reference Rust (e.g., `src/server` being Rust), while the system architecture brief strictly specifies Go (`Go + Bazel, PostgreSQL, Redis, Kubernetes`).
  2. A legacy Next.js web client is retained alongside the canonical Tauri v2 desktop UI despite clear direction for mobile-first unification.
  3. Multiple deployment profiles (k8s Helm charts vs single Docker Compose) exist with scattered, inconsistent feature flags (`OHC_MULTITENANT`, `OHC_HEADLESS`).
  4. The presence of both SQLite SIPDB and PostgreSQL indicates potentially conflicting data layer priorities for local execution vs cloud-native mode.
  5. The `src/` directory mixes gRPC protobufs (`src/proto`), Rust backend logic, and frontend components without strict boundary isolation for agent domains.

  ## Implementation Prompt
  Implement the backend infrastructure and API endpoints for the Unified In-Person POS Architecture utilizing Stripe Terminal.
  - Develop the `TerminalSessionManager` to handle connecting to readers, starting sessions, creating `PaymentIntents`, and capturing payments securely via the Stripe Terminal API.
  - Ensure strict multi-tenant isolation utilizing the established SPIFFE/SPIRE identity framework.
  - Implement a CRDT-friendly inventory deduction mechanism triggered by successful POS transactions to handle potential network flakiness.
  - Ensure all endpoints are fully covered by unit tests (100% coverage requirement) and integrate into the KAIROS Orchestrator to notify the Sales and Operations AI agents upon successful payments.
  - Do not implement specific UI elements in this prompt, but ensure the APIs provided return clean, predictable JSON suitable for consumption by a 375px mobile-first frontend.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []