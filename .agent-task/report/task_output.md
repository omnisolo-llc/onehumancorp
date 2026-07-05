issue_title: "Implement Zero-Click Onboarding Agent"
issue_description: |
  # Zero-Click Onboarding Agent

  ## Problem Statement
  Small business owners face "Setup Paralysis" when trying to launch a digital presence. For example, Maya (the home baker) wants to sell cakes, not configure DNS, payment gateways, and website templates. Currently, setting up an OHC storefront takes manual effort (~1 hour), requiring the owner to act as a part-time web developer and IT admin. Our goal is to reduce this to under 10 minutes by shifting from manual tools to an invisible autonomous agent.

  ## Research Report
  Based on competitive analysis:
  - **Shopify & Wix**: Require complex manual setup, app installations, and design decisions upfront.
  - **Durable AI**: Offers a 30-second site generation but lacks deep operational features and customization.
  - **Market Gap**: OHC needs a "Zero-Click Onboarding" experience where the user simply talks to the assistant, and the system autonomously provisions the domain, configures Stripe for custom deposits, and creates the first product from a simple photo. 34% of SMBs abandon platform setups due to technical complexity.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Maya as Owner (Maya)
      participant OHC as OHC Mobile App (375px)
      participant Triage as Work Triage Agent
      participant Onboard as Onboarding Agent
      participant Core as OHC Core Backend (Go)
      participant Stripe as Stripe API

      Maya->>OHC: "I want to sell custom vegan cakes" (Text/Voice)
      OHC->>Triage: Send intent
      Triage->>Onboard: Route to Onboarding Flow
      Onboard->>Core: Provision Tenant & Subdomain
      Core-->>Onboard: Tenant ID & URL
      Onboard->>Stripe: Setup Connected Account / Payment Links
      Stripe-->>Onboard: API Keys & Endpoints
      Onboard->>Maya: "Your store is ready! Upload a photo of a cake to add your first product."
      Maya->>OHC: Uploads Cake Photo
      OHC->>Onboard: Process Photo
      Onboard->>Core: Generate Product Listing (AI Vision)
      Core-->>Onboard: Product ID
      Onboard->>Maya: "Product added! Here is your live link."
  ```

  ### Mobile UX Flow (375px First)
  1. **Splash Screen**: Clean, translucent glass UI welcoming the owner.
  2. **Chat Interface**: A simple input prompt: "What are you building today?"
  3. **Progress Indicators**: "Provisioning store...", "Setting up payments...", "Generating design..." displayed as subtle toast notifications while the user continues to chat.
  4. **Product Upload**: A native bottom sheet to capture/upload a photo.
  5. **Completion**: A prominent shareable link and a "View Store" button.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Intercepts the initial prompt to determine it's a new setup.
  - **Onboarding Agent**: Orchestrates backend calls (tenant creation, payment setup) and maintains conversation context.
  - **Vision Agent**: Analyzes uploaded product photos to generate titles, descriptions, and pricing suggestions.

  ## Implementation Prompt
  Implement the Zero-Click Onboarding Agent flow in the OHC Flutter mobile app and Go backend.

  **CUJ (Critical User Journey):**
  1. A new user opens the OHC app and enters "I want to start selling [product]".
  2. The system autonomously creates their tenant, sets up a basic storefront, and configures a payment deposit link.
  3. The user uploads a photo, and the system automatically creates a ready-to-sell product listing.

  **Acceptance Criteria:**
  - The entire flow must be completable via chat/voice without the user manually filling out multi-step forms.
  - The UI must perfectly fit a 375px mobile screen using the OHC Premium Token library (translucent materials).
  - Backend must enforce multi-tenant isolation for the newly created resources using row-level security.
  - E2E Playwright tests must be included verifying the chat-to-storefront flow.

  ## Details
  **Priority**: P0
  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []