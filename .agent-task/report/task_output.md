issue_title: "AI-Driven Instant Quoting & Proposal Generation (The Estimator)"
issue_description: |
  ## Title
  AI-Driven Instant Quoting & Proposal Generation (The Estimator)

  ## Problem Statement
  Service-based small business owners like Carlos (Handyman) and Nora (Agency Principal) spend hours manually drafting quotes, estimates, and proposals. They receive disjointed details via text or email, have to calculate material costs, estimate labor, and format a professional PDF. This manual process causes delayed responses, leading to lost leads, and requires desktop-based word processors or complex CRM tools which are not mobile-friendly.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square / Invoice2go / Joist:** Offer mobile quoting but rely entirely on manual data entry. They require the user to build every line item from scratch on a small screen.
  - **HoneyBook / Dubsado:** Geared toward creatives and agencies, but setup is heavy and proposal building is a desktop-first experience.
  - **OHC Opportunity:** Introduce "The Estimator" (Sales/Finance Agent). When a lead requests a service via the unified inbox (e.g., "Need my kitchen sink fixed, here's a picture"), the agent uses Gemini Vision to analyze the image, queries the central ledger for standard hourly rates and parts catalog, and drafts an instant, itemized estimate. It is presented as a mobile card in the Agent Feed for the owner to 1-tap approve or adjust.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Lead Intake: SMS/WhatsApp/Email] -->|Webhook| B(Omnichannel Gateway)
      B --> C[Event Mesh]
      C --> D[The Estimator Agent]
      D -->|Vision API| E[Image/Context Analysis]
      D -->|Query| F[Service Catalog & Pricing DB]
      D -->|Generate Draft| G[Action Required Queue]
      G --> H[Mobile App Agent Feed 375px]
      H -->|1-Tap Approve| I[Stripe Payment Link / Quote Generation]
      I --> J[Omnichannel Dispatcher sends to Lead]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed:** Top card displays "New Quote Drafted: Kitchen Sink Repair for John".
  - **Interaction:** Tapping the card opens a split view. The top half shows the customer's request and provided image. The bottom half shows the itemized quote (e.g., Labor: $150, Parts: $50).
  - **Action:** A prominent primary button "Approve & Send" (44x44px min touch target) and a secondary button "Edit Line Items".
  - **Visual Design:** Follows OHC Premium Tokens with translucent glass styling, ensuring all text is legible and developer terms are hidden.

  ### AI Agent Integration Points
  - **The Estimator (Sales/Finance Agent):** Triggered by new service inquiries. Leverages multimodal LLM (e.g., Gemini Vision) to extract scope of work from text and images.
  - **RAG & Tenant Isolation:** Queries the tenant's specific `ServiceCatalog` to fetch accurate, localized pricing. All queries use strictly tenant-scoped memory boundaries.

  ### Key Design Decisions
  - **Zero-Typing Approval:** The AI must predict costs and format well enough that the majority of quotes require zero typing on mobile.
  - **Integrated Deposit:** Approval automatically generates a Stripe Payment Link for the deposit, turning a quote into a transaction instantly.

  ## Implementation Prompt
  **User-Facing Outcome:** As a handyman, when a customer texts a photo of a broken pipe, I open OHC to find a fully itemized repair quote already drafted. I tap "Send" and the customer gets a link to approve the quote and pay the deposit.

  **Critical User Journey (CUJ):**
  1. Customer sends an SMS/WhatsApp message with text and an image of the job.
  2. The unified inbox receives the message and triggers The Estimator agent.
  3. The agent analyzes the image, matches it to "Plumbing Repair" in the tenant's Service Catalog, and drafts an itemized quote.
  4. Carlos (the owner) opens the OHC mobile app and sees a "Quote Drafted" card in his feed.
  5. Carlos reviews the $200 estimate and taps "Approve & Send".
  6. The system generates a Stripe Payment Link for the deposit and sends it back to the customer via the original channel.

  **Acceptance Criteria:**
  - Must seamlessly integrate with the existing Agent Feed UI on a 375px mobile viewport.
  - Backend must enforce row-level multi-tenant isolation when querying the service catalog.
  - The E2E tests must verify the full loop (Lead -> Agent Draft -> Approval -> Payment Link).

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
