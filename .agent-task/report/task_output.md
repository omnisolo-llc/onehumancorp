issue_title: "[feature] Implement Multi-Tenant Loyalty and Rewards Engine Core"
issue_description: |
  # Mission Queue Protocol Report: Multi-Tenant Loyalty and Rewards Engine Core

  ## Problem Statement
  Small business owners (Maya, Carlos, Priya, Leo, Fatima) struggle to retain customers and drive repeat business without complex, expensive third-party loyalty software. They need an automated, zero-technical-setup way to create and manage loyalty programs (punch cards, points, tiers) that integrates seamlessly into their existing OHC workflow and is managed by their AI assistants. Currently, OHC lacks a unified loyalty ledger and API to support these diverse program types.

  ## Research Report
  - **Market Context**: Platforms like Square and Toast offer integrated loyalty, which is a key retention driver for their SMB merchants. However, these often require manual configuration and don't leverage AI for proactive engagement.
  - **Competitor Analysis**: Standalone loyalty apps (e.g., Smile.io for Shopify) are powerful but add integration friction and recurring costs. An embedded, AI-driven approach within OHC offers a significant competitive advantage.
  - **Current State in OHC**: We have a basic `loyalty_ledger` table, but it lacks the structure to define specific programs (`loyalty_programs`), track program-specific progress (`customer_loyalty_accounts`), log detailed point transactions (`loyalty_transactions`), or define redeemable rewards (`rewards`).

  ## Design Doc
  **High-Level Architecture**:
  The solution expands the existing PostgreSQL schema with Row-Level Security (RLS) to ensure strict tenant isolation. The new tables will support flexible configuration via JSONB for various program types (points, punch card, tiers).

  *Database Entities*:
  - `loyalty_programs`: Defines rules, type, and config.
  - `customer_loyalty_accounts`: Tracks individual customer progress per program.
  - `loyalty_transactions`: Audit log of all points earned/redeemed.
  - `rewards`: Available rewards to redeem.

  **AI Agent Integration Points**:
  - **The Promoter (Marketing)**: Listens for `loyalty.points_awarded` events to trigger notifications or suggest program creation if retention is low.
  - **The Manager (Operations)**: Automates point allocation based on successful order/booking completion.
  - **The Salesperson (Sales)**: Uses points balance as an upsell tactic.

  **Mobile UX Flow (375px)**:
  - *Owner*: A simple toggle in settings: "Enable Loyalty Program". The AI asks 1-2 questions and configures the rest.
  - *Customer*: A visual progress bar (e.g., "3/5 punches") visible on their profile and during checkout.

  ## Implementation Prompt
  Implement the backend core for the Multi-Tenant Loyalty and Rewards Engine.
  1. Define the database schema (PostgreSQL) for `loyalty_programs`, `customer_loyalty_accounts`, `loyalty_transactions`, and `rewards` with proper RLS policies tied to `tenant_id`. Ensure fallback for SQLite in `src/server/db.rs` if necessary.
  2. Implement the core API endpoints (gRPC/REST) in `src/server/api/` (and corresponding services in `src/server/services/`) to support:
     - Creating/updating a loyalty program.
     - Earning points (with an audit trail).
     - Redeeming a reward.
     - Fetching a customer's loyalty status.
  3. Ensure the implementation integrates cleanly with the existing AI orchestration events (e.g., triggering a `loyalty.points_awarded` event).
  4. The implementation must include comprehensive unit and E2E (Playwright) tests.
  5. Do NOT prescribe specific UI implementations, but ensure the APIs support the described mobile-first UX.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
