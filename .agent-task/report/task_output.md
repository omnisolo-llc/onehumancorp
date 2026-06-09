issue_title: "Agent-Driven Autonomous Proposal & Contract Generation"
issue_description: |
  # Research Report: Agent-Driven Autonomous Proposal & Contract Generation

  ## Title
  Agent-Driven Autonomous Proposal & Contract Generation

  ## Problem Statement
  Service-based independent professionals and agency principals (e.g., Nora the Agency Principal, Carlos the Handyman) spend a significant portion of their time drafting proposals, contracts, and quotes. This process is often manual, involving copying and pasting from old templates, leading to errors, inconsistent pricing, and delayed responses to clients. Existing tools like DocuSign or PandaDoc are detached from the core CRM/Inventory/Service catalog, requiring duplicate data entry and manual follow-ups.

  ## Research Report
  - **Market Context**: Platforms like HoneyBook and Dubsado offer proposal and contract management but require extensive manual template creation and setup. They lack true AI autonomy to draft documents based on a simple conversational intake.
  - **Competitor Gaps**:
    - *HoneyBook / Dubsado*: Powerful but steep learning curve; requires manual data entry for each proposal.
    - *PandaDoc / DocuSign*: Standalone signature tools; zero awareness of the user's service catalog or past client conversations.
  - **The OHC Opportunity**: By deeply integrating the Sales and Legal AI Agents into the work intake flow, OHC can instantly transform a casual client inquiry into a professional, ready-to-sign proposal. The system leverages the owner's service catalog and past projects as context (RAG), drastically reducing time-to-quote.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Work Intake Event] -->|Client DM/Form| B(Triage Agent)
      B --> C{Sales & Legal Agents}
      C -->|Query| D[Service Catalog & Pricing DB]
      C -->|Query| E[Past Proposals & Templates RAG]
      C --> F[Draft Proposal/Contract Generated]
      F --> G[Push Notification to Owner]
      G -->|Owner Approves| H[Proposal Sent to Client]
      H --> I[Stripe Deposit Link Attached]
  ```

  ### Mobile UX Flow (375px)
  1. **Notification Screen**: Owner receives a push notification: "New project inquiry from [Client]. Agent drafted a proposal based on your standard rate."
  2. **Review Screen (Card Layout)**: The app displays a clean, summary card of the proposed scope of work, estimated timeline, and total cost.
  3. **Action Bar**: Large, touch-friendly buttons at the bottom: "Approve & Send", "Edit Details", "Decline".
  4. **Edit Flow (If needed)**: Tapping "Edit" allows the owner to adjust key parameters (price, timeline) via sliders or simple inputs, rather than editing a complex text document.
  5. **Client View**: The client receives a responsive web link featuring a polished presentation of the proposal, an integrated "Accept & Sign" button, and an immediate Stripe deposit payment flow.

  ### AI Agent Integration
  - **Triage Agent (Work Intake)**: Parses incoming requests (emails, forms) to extract project requirements, client details, and desired timelines.
  - **Sales Agent ("The Closer")**: Cross-references the extracted requirements with the owner's service catalog/pricing to generate an accurate quote and persuasive proposal text.
  - **Legal Agent ("The Compliance Officer")**: Attaches the appropriate standard contract terms based on the service type and local jurisdiction rules (stored in tenant config).

  ## Implementation Prompt
  **Feature Name**: Agent-Driven Autonomous Proposal Generation
  **Target Persona**: Nora (Agency Principal) & Carlos (Handyman)
  **Outcome**: When a lead submits a project request, the system automatically drafts a complete proposal and contract. The owner receives a mobile push notification to review the summary card, tap "Approve", and immediately send the binding document and deposit link to the client.

  **Next Actions**:
  1.  **Data Model**: Create PostgreSQL tables for `Proposal` (linked to `Customer`, `Service`, and `Tenant`) and `ContractTemplate`.
  2.  **Agent Logic**: Implement the Sales Agent capability to draft proposals. This requires a RAG pipeline that can ingest the owner's service pricing and past successful proposals.
  3.  **Owner UX**: Develop the mobile-first (375px) Proposal Review Card UI, allowing 1-tap approval or simplified parameter editing.
  4.  **Client UX**: Develop the public-facing proposal acceptance page, integrating a signature capture component and Stripe Checkout for the initial deposit.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []