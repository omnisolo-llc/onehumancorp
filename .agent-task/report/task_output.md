issue_title: "10-Minute Setup Wizard for Storefront Generation"
issue_description: |
  # Mission Queue Protocol: 10-Minute Setup Wizard for Storefront Generation

  ## Title
  10-Minute Setup Wizard for Storefront Generation

  ## Problem Statement
  According to our market research and competitive analysis, 73% of small business owners cite setup complexity and technical jargon (e.g., DNS, APIs) as their primary barrier to launching online. Our core business personas (Maya, Carlos, Priya, Leo, Fatima) need a "Zero-Config" experience. They want to answer simple plain-language questions and have a live, AI-generated, mobile-friendly storefront up and running in under 10 minutes, entirely from their phone.

  ## Research Report
  The SMB Platform Gap report clearly outlines that current legacy platforms (Shopify, Wix) take 30-60 minutes and still expose users to complex theme editors and configuration options.
  Our differentiator is "Autonomous AI Agents." The `autodream` agent framework should be responsible for synthesizing the website structure, initial copywriting, and product catalogs based on an incredibly simple intake flow. The system must use macOS-style Translucent Glass materials combined with clean Ubiquiti UniFi modular dashboard card layouts to maintain a premium feel. Every step MUST pass the "grandmother test."

  ## Design Doc
  - **Architecture**:
    - **Frontend**: Flutter app targeting Mobile (iOS/Android) and Web.
    - **UI Layout**: Conversational, jargon-free wizard. Apple/Ubiquiti-style hierarchy. Mobile-first design starting at 375px. Large touch targets (at least 44x44px).
    - **Backend**: Connect to OHC Go+Bazel backend to stream answers to the `autodream` pipeline, which processes the inputs.
  - **Mobile UX Flow**:
    1. **Welcome Screen**: "Let's set up your business in minutes." Large start button.
    2. **Business Type**: Simple selection (e.g., Physical Products, Services, Food).
    3. **Business Name & Vibe**: Plain text input.
    4. **Core Offering**: "What do you sell mostly?" (e.g., Custom Cakes, Plumbing Repair).
    5. **Generation Screen**: Loading state showing the `autodream` agent working ("Drafting layout...", "Writing copy...", "Preparing products...").
    6. **Preview/Approval**: Shows a 375px preview of the generated storefront. "Looks good!" or "Regenerate".
  - **AI Agent Integration**:
    - The UI feeds collected answers into the `autodream` event pipeline.
    - The `autodream` agent acts as the "Onboarder" to synthesize the site configuration and seed initial database records.
  - **Zero Trust & Data**: Uses standard multi-tenant row-level security (`tenant_id`). All backend interactions must be authenticated via SPIFFE SVIDs.

  ## Implementation Prompt
  Implement the Mobile-First 10-Minute Setup Wizard in the Flutter frontend.

  **User Facing Outcome**: When a new user (or a user triggering setup) opens the app, they enter a clean, conversational UI. They answer 3-4 plain-language questions about their business. Upon completion, a loading screen displays progress as the backend AI generates their storefront, followed by a preview screen of their new business site.

  **CUJ**: Launch App -> Tap "Start Setup" -> Answer Business Type -> Answer Business Name -> Answer Core Offering -> View "Generating" state -> View Storefront Preview.

  **Acceptance Criteria**:
  - The UI MUST be mobile-first and look perfect on a 375px screen without horizontal scrolling.
  - Apply the defined OHC premium design tokens: translucent glass materials, strong spacing, readable typography.
  - Form fields must use native mobile keyboards where appropriate and have large touch targets (>= 44x44px).
  - The wizard must collect at least: Business Type, Business Name, and Core Offering.
  - A mock generation state must be shown in the UI while simulating the backend call to the `autodream` pipeline (until backend integration is complete).
  - Add robust Playwright E2E tests simulating a real non-technical owner stepping through the wizard and reaching the preview screen. NO mock data allowed in the final product state; empty states must be truthful.
  - Ensure all new Flutter code achieves 100% test coverage.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
