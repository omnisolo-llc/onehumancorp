issue_title: "Implement Autonomous Abandoned Cart Recovery Agent"
issue_description: |
  # Research Report: Autonomous Abandoned Cart Recovery Agent

  ## Problem Statement
  Small business owners lose significant revenue to abandoned carts. Traditional platforms like Shopify require installing third-party apps (e.g., Klaviyo), designing email templates, configuring complex trigger logic, and launching flows manually. This process is highly technical, time-consuming, and overwhelming for non-technical users, leading to a "Franken-stack" of disparate tools.

  ## Research Report
  - **Traditional Flow (Competitors)**: User must install a plugin (e.g., Klaviyo), design an email template, configure the trigger logic (wait 1 hour, check if purchased), and launch the flow.
  - **OHC Agent Flow**: The \`Customer Success Agent\` observes an abandoned cart event in the unified event stream, automatically drafts a personalized email based on the user's brand voice settings and previous interaction history with that specific customer, and sends it. Zero configuration required from the business owner.
  - **Competitor Gaps**: Shopify requires apps for this, Wix requires manual setup, and Squarespace has limited proactive features.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Adds to Cart] --> B[Cart Event Stream]
      B --> C[Abandoned Cart Detector]
      C -->|Trigger after 1h| D[Customer Success Agent]
      D --> E[Draft Personalized Email]
      E --> F[Owner Approval / Auto-Send]
      F --> G[Email Dispatcher]
  ```

  ### Mobile UX Flow (375px First)
  - **Owner Dashboard (Mobile):** A new card appears in the feed: "3 Abandoned Carts Detected. Agent drafted recovery emails."
  - **Interaction:** Owner taps the card to review the drafted emails.
  - **Action:** A simple "Approve All" or "Review Individually" button. If auto-send is enabled, this step is skipped.

  ### AI Agent Integration
  - **Customer Success Agent:** Uses RAG to pull customer history and brand voice to draft highly personalized recovery emails.

  ## Implementation Prompt
  **Feature Name**: Autonomous Abandoned Cart Recovery Agent
  **Target Persona**: Maya the Baker
  **Outcome**: When a customer abandons their cart, the OHC system automatically detects it, drafts a personalized recovery email, and sends it (either automatically or after Maya's 1-tap approval from her mobile dashboard).

  **Next Actions**:
  1. Implement an event listener for cart abandonment.
  2. Integrate the Customer Success Agent to draft the recovery email.
  3. Create the UI for the owner to review and approve the drafts on mobile.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
