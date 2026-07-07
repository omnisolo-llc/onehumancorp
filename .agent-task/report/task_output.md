issue_title: "Implement Agentic Conversational Custom Order & Quoting Engine"
issue_description: |
  # Research Report: Agentic Conversational Custom Order & Quoting Engine

  ## Executive Summary
  This report details an architectural and user-experience gap within OneHumanCorp's handling of bespoke requests and custom quotes. Currently, small business owners (like Maya the Baker or Carlos the Handyman) receive custom inquiries via social media DMs or text messages. Transitioning these unstructured inquiries into a structured quote with a deposit link requires manual data entry and context switching. By implementing an "Agentic Conversational Quoting Engine," OHC can empower the Customer Success Agent to parse unstructured DMs, interactively gather missing requirements, and generate a binding quote and deposit link without owner intervention until final approval.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  - **Traditional CPQ Tools:** Salesforce CPQ, HubSpot Quotes, and PandaDoc offer powerful quoting capabilities but are incredibly complex and entirely inappropriate for a 28-year-old baker operating from an iPhone.
  - **E-commerce Builders:** Shopify and Wix are designed for standardized products. They struggle with "custom order" flows unless heavy third-party form builders are attached. They lack native conversational intake.
  - **The Gap:** There is no tool that natively sits in a social media DM inbox, understands that a customer wants "a 3-tier vegan wedding cake for next Saturday," checks the calendar for availability, asks follow-up questions about flavor, and drafts a quote—all entirely autonomously.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Maya (Home Baker) & Carlos (Field Service Owner).
  - **The Problem:** Maya receives a DM on Instagram: "Hi, can I get a cake for my son's birthday this weekend?" She has to manually ask for the date, size, theme, and allergies. Then she has to manually calculate the price, create an invoice, and send a payment link. This process is slow, prone to lost leads, and tedious to perform on a 375px mobile screen.

  ## 3. Deep Dive Architecture Design (Track 2)

  ### Data Model & Invariants
  - **Quote/Order Entities:** Introduction of a `conversational_intake_sessions` table to track the state of the AI's conversation with the lead.
  - **Multi-Tenant Boundaries:** All sessions, generated quotes, and messages must be strictly isolated by `tenant_id` using PostgreSQL Row Level Security (RLS).

  ### AI Agent Coordination
  - **Customer Success Agent ("The Ambassador"):** Reads incoming DMs, extracts intent, and maps it against the business's custom order constraints (e.g., lead time, available dates, pricing rules).
  - **Operations Agent ("The Manager"):** Verifies inventory or calendar availability before the quote is finalized.
  - **Finance Agent ("The Accountant"):** Generates the Stripe Payment Link for the required deposit and attaches it to the final quote object.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer (IG DM)
      participant OHC as OHC Omni-Inbox
      participant Ambassador as CS Agent
      participant Ops as Ops Agent
      participant Finance as Finance Agent
      participant Owner as Maya (Owner UI)

      C->>OHC: "I need a birthday cake for Saturday."
      OHC->>Ambassador: Parse intent (Custom Order)
      Ambassador->>Ops: Check availability for Saturday
      Ops-->>Ambassador: Available
      Ambassador->>C: "We can do that! How many people, and what flavor?"
      C->>Ambassador: "12 people, chocolate."
      Ambassador->>Finance: Draft Quote ($50) & Deposit Link ($25)
      Finance-->>Ambassador: Quote Drafted
      Ambassador->>Owner: "Quote drafted for Saturday chocolate cake. Approve to send?"
      Owner->>Ambassador: Approve (1 tap)
      Ambassador->>C: "Here is your quote and deposit link!"
  ```

  ### Mobile-First Implementation
  - **UI/UX Flow (375px):**
    - The owner sees a single "Agent Drafts" card on their Home dashboard: "1 Quote ready for approval."
    - Tapping the card opens a clean, macOS Translucent Glass modal showing the conversation summary, the proposed price, and a large 44x44px "Approve & Send" button.
    - No manual text entry is required from the owner unless they choose to edit the quote.

  ## 4. Proposed Implementation Prompt

  **Feature Name:** Conversational Quoting Engine

  **Target Personas:** Maya (Baker) and Carlos (Handyman)

  **Implementation Prompt for Engineer:**
  Design and implement the Conversational Quoting Engine.
  1. Create the necessary backend tables (`conversational_intake_sessions`) with strict RLS multi-tenancy.
  2. Implement the AI tool logic for the Customer Success Agent to parse incoming inbox messages, identify missing required fields for a quote (date, size, type), and draft follow-up questions.
  3. Build the backend coordination so that when the agent has all information, it drafts a Quote and requests owner approval via the `agent_action_requests` table.
  4. Create the mobile-first (375px) UI component that surfaces this pending quote to the owner on their dashboard, allowing 1-tap approval.
  5. Ensure the entire Critical User Journey is covered by Playwright E2E tests, verifying that an incoming mock DM results in a drafted quote and owner approval flow.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
