issue_title: "Autonomous Zero-Click Onboarding Agent: Conversational Setup Flow"
issue_description: |
  # Autonomous Zero-Click Onboarding Agent

  ## Problem Statement
  Existing platforms provide tools to run a business but require significant initial setup and configuration. Small business owners (like Maya the Baker or Carlos the Handyman) suffer from "Setup Paralysis". As indicated by our competitive research against Shopify, Wix, and Durable, users often abandon platform onboarding when confronted with configuring DNS, setting up payment processors, adding inventory, and organizing web layouts. They do not want to learn how to be a web administrator; they want their business to be online, ready to sell, immediately.

  ## Research Report
  - **Traditional Platforms (Shopify, Wix, Squarespace):** Rely heavily on self-serve dashboards. High flexibility but immense cognitive load.
  - **AI-Native Platforms (Durable, 10Web, Framer AI):** Fast initial generation (e.g., Durable's 30-second site). However, they usually result in static, hard-to-customize shells and don't natively integrate deep operations (pos, quoting, real inventory).
  - **OHC Opportunity:** Combine the <1 min setup speed of generative platforms with OHC's operational KAIROS backend. Rather than dropping the owner into a dashboard after generating the site, OHC should use an onboarding agent to ask questions and *autonomously execute the backend actions* (creating products, provisioning a URL, activating Stripe test mode or prompting for connection).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner
      participant OnboardingAgent
      participant KAIROS
      participant CoreServices

      Owner->>OnboardingAgent: "I sell custom vegan cakes"
      OnboardingAgent->>KAIROS: Infer Business Type: F&B / Bakery
      KAIROS->>CoreServices: Provision Tenant & Initial Profile
      OnboardingAgent->>Owner: "Got it! Give me a photo of your best cake."
      Owner->>OnboardingAgent: Uploads Photo
      OnboardingAgent->>KAIROS: Extract Title, Desc, Price (Vision LLM)
      KAIROS->>CoreServices: Create Product Entry & Enable Booking
      OnboardingAgent->>Owner: "Your product 'Vegan Chocolate Cake' is live at maya.ohc.com."
  ```

  ### Mobile UX Flow (375px First)
  1. **Landing:** Owner opens OHC mobile web app.
  2. **Chat Interface:** Instead of a complex form, a chat bubble appears: "Hi! What kind of business do you run?"
  3. **Natural Input:** Owner types or uses voice dictation.
  4. **Progressive Unveiling:** The agent updates a visual progress bar (Business Name -> First Product -> Payment Setup).
  5. **Photo Capture:** Uses native mobile camera API for instant product creation via LLM Vision.
  6. **Launch:** Shows a success screen with a QR code and shareable link.

  ### AI Agent Integration
  - **Model:** Gemini Pro / Vision.
  - **Tools:** `CreateTenant`, `ProvisionDomain`, `CreateProductFromImage`, `SetBusinessHours`.
  - **State Management:** Agent maintains conversation state until mandatory fields are collected, then executes operations via KAIROS.

  ## Implementation Prompt
  **Feature Name:** Autonomous Zero-Click Onboarding System
  **Target Persona:** Maya (Home Baker) & Carlos (Field Service Owner).

  **Outcome:** A conversational interface where the AI agent autonomously provisions the workspace, creates a storefront, and lists the first product based purely on chat and image uploads. The owner never sees a complex settings dashboard during initial setup.

  **Critical User Journey (CUJ) to Implement:**
  1. Owner accesses the new conversational onboarding route on a 375px mobile viewport.
  2. Owner inputs their business concept (e.g., "I'm a handyman").
  3. The agent provisions the tenant schema and applies a default service template.
  4. Owner is prompted to upload a photo of past work; the agent generates a service listing.
  5. The agent asks for payment details (or skips to manual invoicing) and presents the live share link.

  **Acceptance Criteria:**
  - Build a chat-based React/Flutter UI for onboarding (must fit 375px without horizontal scrolling).
  - Implement the backend agent capable of calling at least 3 provisioning tools automatically based on conversation context.
  - Add a Playwright E2E test covering the complete conversational flow from chat start to product creation.
  - No complex forms during the primary onboarding path.

  ## Priority & Scope
  - **Priority:** P0 (Critical for Acquisition & Activation)
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
