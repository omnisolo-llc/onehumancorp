issue_title: "Implement Zero-Data-Entry AI CRM with Automated Customer Profile Insights"
issue_description: |
  # Research Report: Agentic Unified Customer Profile & CRM

  ## Executive Summary
  This report investigates the current landscape of small business CRM tools. The objective is to design a centralized customer profile and relationship management architecture for OneHumanCorp (OHC) that leverages our AI agents to provide a seamless experience for owners like Carlos (Handyman) and Priya (Boutique).

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  - **HubSpot:** Powerful but complex, often requires a dedicated administrator. Overkill for micro-SMEs.
  - **Salesforce Essentials:** Stripped down but still feels like enterprise software.
  - **Shopify Customers:** Basic purchase history, but lacks deep relationship context (e.g., specific preferences gathered from DMs).
  - **OHC Opportunity:** A "Zero-Data-Entry" CRM. Agents automatically build rich customer profiles by analyzing interactions across email, Instagram DMs, SMS, and purchase history.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Carlos (Handyman) relies on remembering customer details ("Mrs. Smith prefers afternoon visits"). When he gets busy, he forgets.
  - **The Gap:** OHC needs a unified `customer_profiles` entity that aggregates structured data (name, phone) and unstructured AI-extracted insights (preferences, sentiment, churn risk).

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):**
    - `customer_profiles`: Core data (id, tenant_id, name, contact_info).
    - `customer_interactions`: Log of all touchpoints.
    - `customer_insights`: AI-generated metadata (preferences, tags).

  ### AI Agent Coordination
  - **The Ambassador (Customer Success Agent):** Automatically updates the `customer_insights` table after every conversation (e.g., extracting "allergic to peanuts" from a DM).
  - **The Promoter (Marketing Agent):** Uses the insights to generate hyper-personalized outreach.

  ### Mobile-First Implementation
  - A simple, card-based customer view on mobile (375px) highlighting "What to know before you reply" (AI summarized insights).

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** Zero-Data-Entry AI CRM

  **Target Persona:** Carlos the Handyman

  **Outcome:** Carlos opens a customer profile and immediately sees an AI-generated summary of their preferences and past work, without ever having typed it in himself.

  **Critical User Journey (CUJ):**
  1. A customer texts Carlos a specific preference.
  2. The Ambassador agent handles the reply and extracts the preference.
  3. The preference is saved to the customer profile.
  4. Carlos opens the customer profile on his phone and sees the new preference highlighted.

  **Next Actions for Engineering:**
  - **Step 1:** Implement the `customer_profiles` and `customer_insights` Postgres schemas.
  - **Step 2:** Update the Omni-Inbox service to trigger the Ambassador agent to extract and save insights to the profile after conversations.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
