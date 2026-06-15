issue_title: "Zero-Click Onboarding & Storefront Generation Agent"
issue_description: |
  # Research Report: Zero-Click Onboarding Agent

  ## Problem Statement
  Based on the competitive research (`docs/business/market_research/ohc_owner_work_assistant_competitive_research.md`) and SMB pain point analysis, **Setup Paralysis** is a massive hurdle. Around 34% of small business owners abandon store setup due to technical complexity (e.g., DNS configuration, Stripe setup, UI design). For our target persona—Maya, the Home Baker—building a website is not her core business; selling cakes is. She needs to go from login to a published, ready-to-sell product link in under 10 minutes without touching complex settings.

  ## Research Report
  - **Market Landscape**: Traditional platforms like Shopify take hours to set up properly, often requiring third-party apps for basic functionality. Competitors like Durable.co have introduced 30-second generative websites, but they lack deep operational tools (like robust commerce and multi-channel inventory sync).
  - **The Gap**: OHC's current onboarding flow still requires manual service configuration and widget-based setup, taking roughly an hour. We must reduce this to <10 minutes using natural language generation.
  - **Opportunity**: By implementing a "Zero-Click Onboarding Agent," OHC can bridge the gap between generative ease (Durable) and operational depth (Shopify), allowing a user to text an AI assistant to auto-provision their domain, connect Stripe, design a storefront, and publish their first product from a photo.

  ## Design Doc
  ### High-Level Architecture
  ```mermaid
  graph TD
      A[Mobile Chat UI (375px)] --> B[Onboarding Orchestrator (KAIROS)]
      B --> C[Identity & Provisioning Agent]
      B --> D[Design & Storefront Agent]
      B --> E[Product Generation Agent (Vision Model)]
      C --> F[(Postgres Tenant DB)]
      D --> G[Edge Cached Storefront Generator]
      E --> F
  ```

  ### Mobile UX Flow (375px First)
  1. **Initial State**: User opens the OHC app. Instead of a complex dashboard, they see a conversational interface.
  2. **Prompt**: "What are you selling today?"
  3. **Interaction**: User replies (e.g., "I'm Maya, I sell custom vegan cakes in Portland").
  4. **Agent Action**: The onboarding agent immediately provisions a tenant, generates a temporary domain, and proposes a base storefront design based on the prompt.
  5. **Product Ingestion**: Agent asks for a photo of a cake. User uploads a photo.
  6. **Completion**: Vision model extracts product details, drafts copy, and presents a 1-tap "Publish Store" approval card.

  ### AI Agent Integration Points
  - **KAIROS Orchestration**: Manages the conversational state machine for the onboarding flow.
  - **Generative Design**: Agent selects and customizes Tailwind/CSS tokens to build a responsive storefront UI.
  - **Multimodal Extraction**: Uses Gemini Vision (or equivalent) to process user-uploaded photos and auto-generate product descriptions and variants.

  ## Implementation Prompt
  - **Objective**: Build the "Zero-Click Onboarding Agent" conversational flow and backing API.
  - **Outcome**: A user must be able to start with a blank tenant, interact via a chat-based UI on a mobile device, and arrive at a published storefront with at least one product created from a photo upload.
  - **CUJ**:
    1. User logs in.
    2. User inputs business description via chat.
    3. User uploads one product photo.
    4. System presents an "Approve & Publish" card.
    5. User taps "Approve" and the storefront goes live.
  - **Acceptance Criteria**: Ensure the UI relies strictly on conversational input and 1-tap approval cards. No multi-step configuration forms should be required for initial launch. Backend must handle tenant initialization, Stripe connect setup (or stub for MVP), and product creation seamlessly.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
