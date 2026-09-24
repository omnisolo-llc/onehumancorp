issue_title: "OHC-04: Customer inquiry to qualified quote"
issue_description: |
  # Mission Queue Protocol: OHC-04

  ## Problem Statement
  Nora, a solo web/design/marketing professional, receives inquiries across multiple channels. Qualifying these leads, establishing scope, and sending a priced proposal takes significant manual effort. Existing CRM tools like HoneyBook and Dubsado require extensive template setup and manual drafting, while generic AI tools lack context of her specific business rules and pricing models, leading to incomplete or disconnected proposals.

  ## Research Report
  ### Market Benchmark
  1. **HoneyBook**: Offers "Smart Files" for combined proposals, contracts, and invoices. However, automation requires rigid pre-built templates and manual trigger configuration.
  2. **Dubsado**: Powerful workflow automation, but steep learning curve for non-technical owners. Form building and proposal generation are manual processes.
  3. **Claude for Small Business**: Provides conversational assistance and can draft proposals if provided with context, but lacks a persistent, structured data model for the customer and project.

  ### Findings & Sentiment
  - *Friction*: Setting up proposal templates in traditional tools is overwhelming ("steep learning curve").
  - *Pain Point*: Translating an unstructured email inquiry into a structured, priced quote takes too long.
  - *Gap*: No tool automatically ingests an inquiry, grounds it in the owner's persistent pricing rules, and generates a ready-to-send proposal linked to a durable customer record.

  ### Persona Mapping
  For **Nora**, an email asking for "a website redesign like X" should automatically trigger the AI to look up her base redesign rate, draft a scoped proposal, and present it for her approval before sending.

  ## Design Doc
  ### Architecture & Flow
  ```mermaid
  flowchart TD
      A[Inbound Inquiry via Email/Widget] --> B[Lead Agent]
      B --> C{Extract Requirements}
      C --> D[Lookup Pricing Rules & Availability]
      D --> E[Draft Proposal & Quote]
      E --> F[Owner Approval Queue]
      F -->|Approve| G[Send to Client]
      F -->|Revise| E
  ```

  ### UI/UX (Mobile-First 375px)
  - **Incoming Lead Card**: Displays inquiry summary and estimated value.
  - **Proposal Review Screen**: Shows drafted scope, line items, and total price. Primary actions: "Approve & Send" or "Edit Details" (hidden behind 'Advanced Settings').

  ## Implementation Prompt
  Implement the OHC-04 workflow:
  1. Ingest a customer inquiry and create a persistent Lead entity.
  2. Use the Lead Agent to evaluate the inquiry against the owner's established offer and pricing rules.
  3. Generate a draft Proposal (quote) linked to the Lead.
  4. Present the draft in the Owner Outcome Feed for review and approval.
  *Acceptance Criteria*: A real inbound message creates a lead, a grounded response, and a priced quote. Follow-up includes delivery evidence.

  ## Metadata
  - **Strategy Admission**: OHC-04
  - **Stage**: Launch/Run
  - **Observed Gap**: Manual transcription from inquiry to quote.
  - **Dependencies**: OHC-03 (Offer context).
  - **Authority Class**: Draft only (requires explicit approval to send).
  - **Cost Plan**: Monitored per token/inference.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
