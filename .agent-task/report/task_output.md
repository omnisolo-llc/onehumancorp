issue_title: "Implement Autonomous Agentic Unified Identity & Zero-Party Data Collection"
issue_description: |
  ## Problem Statement
  Small businesses struggle to collect zero-party data (customer preferences, sizing, birthdays, dietary restrictions) without employing intrusive popups or long checkout forms that reduce conversion rates. This data is critical for personalized marketing and increasing Customer Lifetime Value (CLV). Existing platforms require complex CRM setups or form-builder plugins that create a disjointed experience for the customer and the business owner.

  ## Research Report
  - **Competitor Systems Audit**: Shopify relies on third-party form apps (like Typeform or Klaviyo forms) which disrupt the user flow. Wix has basic contact forms but no AI-driven passive collection. None of these systems have a central "Identity Graph" that is proactively populated by conversational AI.
  - **Identify Gaps**: OHC currently lacks an autonomous mechanism to passively collect and structure zero-party data during natural customer interactions (e.g., DMs, checkout chat, support inquiries) and map it to a unified customer profile.
  - **Persona Fit**: Maya (the baker) often learns a customer is vegan during an Instagram DM, but forgets to record this. The next time they order, she has to ask again or risk sending a non-vegan recommendation.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Interaction: Instagram/WhatsApp/Chat] --> B(Omnichannel AI Inbox)
      B --> C[The Ambassador Agent]
      C -->|Intent: Support/Sales| D[Conversation]
      C -->|Passive Extraction| E[Zero-Party Data Extractor]
      E -->|Structure Entity| F[Unified Identity Graph DB]
      F --> G[The Promoter Agent]
      G -->|Personalized Campaign| H[Customer]
  ```

  ### Mobile UX Flow (375px)
  1. **Customer Profile View**: A highly readable, glassmorphism-styled profile card for each customer. It highlights "Known Preferences" (e.g., "Vegan", "Allergic to nuts", "Wears Size M").
  2. **Agent Feed**: A feed item appears: "The Ambassador learned that Sarah is Vegan during an Instagram DM. Profile updated."
  3. **Marketing Flow**: When the owner drafts a campaign for a new Vegan Cake, the system automatically suggests targeting the 45 customers with the "Vegan" preference tag.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent)**: Continuously monitors conversations for declarative statements ("I'm a size 8", "My dog's name is Buster", "I only eat gluten-free"). Uses a lightweight LLM extraction prompt to parse these facts.
  - **Zero-Party Data Extractor**: Validates extracted facts against a predefined tenant ontology and updates the `Unified Identity Graph`.

  ### Key Design Decisions
  - **Passive Collection**: The system never explicitly asks the customer to fill out a profile form unless configured to do so. It learns naturally from ongoing business interactions.
  - **Strict Multi-Tenancy**: Customer identities and preferences are strictly siloed per tenant using PostgreSQL Row-Level Security (`tenant_id`).

  ## Implementation Prompt
  **User-Facing Outcome:** Business owners will see rich, detailed customer profiles build themselves over time, enabling highly targeted, high-conversion marketing without any manual data entry.
  **CUJ & Acceptance Criteria:**
  1. Simulate a customer DM: "Hi, do you have any gluten-free options? I have celiac."
  2. The Ambassador agent processes the message and responds appropriately.
  3. The `Zero-Party Data Extractor` isolates the fact `Dietary Restriction: Gluten-Free` and attaches it to the customer's profile in the database.
  4. Provide a Playwright E2E test verifying that the updated preference is visible on the customer's detail screen in the owner's app.
  - **Note**: Ensure the extraction logic handles negations correctly (e.g., "I don't like chocolate").

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
