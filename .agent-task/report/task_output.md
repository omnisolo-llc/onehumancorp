issue_title: "[Architecture] Implement Zero-Click Autonomous Onboarding Agent for SMB Operators"
issue_description: |
  ## Title
  Implement Zero-Click Autonomous Onboarding Agent for SMB Operators

  ## Problem Statement
  Small business owners and operators (like Maya the Baker or Carlos the Handyman) currently abandon setup processes for new tools due to high technical complexity and setup paralysis. According to competitive analysis, platforms like Durable have shown that getting an SMB from zero to a live, transactional presence in under a minute dramatically increases activation and retention. OneHumanCorp (OHC) currently lacks an "assistant-first" zero-click onboarding flow. When an owner logs in on a 375px mobile screen, they should not be greeted with empty dashboards, complex navigation menus, or technical settings. Instead, they should be greeted by an AI agent that converses with them in natural language, automatically provisions their workspace, configures payments, and generates their initial catalog and storefront based on minimal input (e.g., a photo or a sentence).

  ## Research Report
  ### Market Context
  - **Shopify & Wix**: Have added "Sidekick" and "Wix Studio AI" to aid with store customization, but still require significant manual configuration of shipping zones, payments, and product variants. Setup can take hours or days.
  - **Durable**: Pioneers the "30-second website" model, generating a complete landing page, CRM, and invoicing system from a single prompt. However, it lacks deep post-onboarding operational integration.
  - **The OHC Opportunity**: We can merge Durable's magical 30-second onboarding with Shopify's deep operational robustness. By employing an Autonomous Onboarding Agent, an owner can text their business name and upload a picture of their product/service, and the agent provisions the OHC tenant, Stripe capabilities, and product catalog instantly.

  ### Findings
  1. SMB owners operate primarily on mobile (375px width).
  2. Setup drop-off is highest at the "Configure Payments" and "Add First Product" stages.
  3. Conversational UI (chat) is perceived as lower-friction than multi-step wizard forms.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Owner
      participant MobileUI as OHC Mobile UI (375px)
      participant API as API Layer (gRPC/REST)
      participant OnboardingAgent as KAIROS Onboarding Agent
      participant DB as Postgres (Multi-Tenant)
      participant Ext as Stripe / MinIO

      Owner->>MobileUI: Opens app, greeted by chat interface
      MobileUI->>API: Initiates Onboarding Session
      API->>OnboardingAgent: Trigger Agent Thread (Tenant ID: Null/Temp)
      OnboardingAgent-->>MobileUI: "Hi! What do you sell or do? You can just send a picture."
      Owner->>MobileUI: Uploads photo of a cake & texts "I make custom cakes"
      MobileUI->>API: Send message + image
      API->>OnboardingAgent: Analyze input (Vision LLM)
      OnboardingAgent->>DB: Provision New Tenant Workspace
      OnboardingAgent->>Ext: Initialize Draft Stripe Connect / Checkout
      OnboardingAgent->>DB: Seed initial catalog (Custom Cake, $50 deposit)
      OnboardingAgent-->>MobileUI: "Done! I've set up your cake shop. Want to see the preview or connect your bank?"
  ```

  ### Mobile UX Flow (375px First)
  1. **Splash/Welcome**: Full-screen translucent glass interface. A single input field at the bottom with a camera icon and microphone icon.
  2. **Conversational Setup**:
     - AI asks: "Welcome to OHC. What's your business?"
     - User types or uploads an image.
     - A skeleton loader (UniFi dashboard style card) appears while the agent works.
  3. **The Reveal**: The chat window slides down, revealing a beautiful, pre-populated "Today's Plan" dashboard card showing their new mock storefront link and a suggested next action ("Tap to connect bank account for deposits").
  4. **No Settings Visible**: No hamburger menus with "Settings" or "Billing" exist on this screen. Everything is routed through the agent.

  ### AI Agent Integration Points
  - **KAIROS Orchestrator**: Will route the initial interaction to a specialized `OnboardingDepartment` agent.
  - **Vision LLM (Gemini Pro Vision / GPT-4o)**: Used to parse uploaded images (e.g., a handwritten menu or a photo of a cake) to generate initial catalog items and pricing estimates.
  - **Tenant Provisioning Skill**: The agent must have strict access to a sandboxed tool that provisions a new `tenant_id` in Postgres and configures default row-level security (RLS) policies.

  ### Key Design Decisions
  - **Chat-First, Not Form-First**: To minimize cognitive load, the onboarding relies entirely on natural language and media upload rather than traditional SaaS wizards.
  - **Lazy Authentication**: The tenant workspace is built in a "draft" state before the user is forced to create a complex password or verify email, maximizing top-of-funnel conversion.
  - **Multi-Tenant Safety**: The agent executes provisioning within a strictly isolated context. Once the tenant is created, the user session is bound to that `tenant_id`.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the "Zero-Click Autonomous Onboarding Agent" flow for the OHC mobile experience.

  1. Build a 375px-first mobile UI using the OHC Premium Token library (translucent glass materials, clean card layouts). The initial screen must be a chat interface (no complex nav menus).
  2. Integrate the UI with the backend API to establish a session with the KAIROS Orchestrator.
  3. Implement the `OnboardingDepartment` agent behavior on the backend. This agent should accept natural language and image uploads, use an LLM to interpret the business type, and execute a tool to provision a new tenant workspace with basic seeded data (e.g., an initial product or service offering).
  4. Ensure all database writes for the new workspace enforce strict `tenant_id` RLS.
  5. The acceptance criteria: A user (like Maya or Carlos) can open the app, send a single text prompt or image, and within 30 seconds be presented with a provisioned workspace and a generated storefront preview without filling out any traditional forms.
  6. Ensure 100% unit test coverage for the new agent tools and Playwright E2E tests for the mobile chat UI flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
