issue_title: "[Research] OHC AI Agentic Content & Asset Generation Architecture"
issue_description: |
  # Research Report: OHC AI Agentic Content & Asset Generation Architecture

  ## Problem Statement
  Currently, small business owners struggle with generating professional assets, branding, and content when setting up their business online. Traditional website builders (e.g., Shopify, Wix, Squarespace) require owners to bring their own high-quality imagery, write their own compelling copy, and manually design layouts. This creates a massive activation hurdle for personas like Maya (baker) or Carlos (handyman) who lack design expertise, copywriting skills, and professional photography. They are forced to use generic templates or spend hundreds of dollars on third-party services.

  **The Gap:** OHC currently lacks an integrated, multi-modal asset generation architecture that automatically creates brand-aligned content (text, imagery, product descriptions) during onboarding and ongoing operations without requiring the owner to leave the 375px mobile UI or write technical prompts.

  ## Research Report (Track 1)
  **Competitor Solutions:**
  - **Shopify/Wix:** Rely heavily on stock photo libraries (Unsplash integrations). Shopify's "Magic" generates basic text but fails at cohesive, multi-modal asset creation tailored to specific business types.
  - **Durable/10Web:** Fast AI generation, but often produce generic, templated assets that lack deep business context or a premium feel.
  - **Canva/Framer:** Excellent for design, but disconnected from the operational and commerce workflows. The user must manually export and upload assets.

  **Market Need:**
  Non-technical owners need an invisible "Creative Department" agent that understands their business context, auto-generates high-quality, localized, and brand-consistent assets (e.g., a localized hero image for Carlos's handyman business in Miami, or appetizing descriptions for Fatima's food cart menu items), and seamlessly integrates them into the storefront and marketing channels.

  ## Design Doc (Track 2 & 3)

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Owner UI - Mobile 375px] -->|Approves/Requests Asset| B(Creative Agent Coordinator)
      B --> C{Asset Generation Queue - Postgres SKIP LOCKED}
      C --> D[Text Generation Worker - Gemini Pro/MiniMax]
      C --> E[Image Generation Worker - Imagen/DALL-E]
      D --> F(Tenant Asset Storage - GCS/MinIO)
      E --> F
      F --> G[Edge CDN & WebP Compression]
      G --> H(Storefront / Marketing Channels)
      B --> I[(Tenant Memory - Context, Brand Voice, Preferences)]
      D -.-> I
      E -.-> I
  ```

  ### System Architecture
  - **Multi-Tenant Context:** Every generated asset must be strongly tied to the `tenant_id` and influenced by the tenant's global memory (e.g., brand colors, tone of voice, location).
  - **Asynchronous Generation:** Asset generation (especially images) is slow. The architecture must utilize the Postgres `SKIP LOCKED` job queue to process requests asynchronously.
  - **Mobile-First UX Flow:**
    1.  **Trigger:** User taps "Generate Product Image" or the Onboarding Agent auto-triggers asset creation based on a brief description.
    2.  **Pending State:** A premium translucent loading skeleton appears on the 375px screen, utilizing the OHC design system tokens.
    3.  **Review:** The generated asset is presented as a card. The owner can tap "Approve", "Regenerate", or "Tweak" (using simple conversational inputs like "make it more professional").
    4.  **Storage:** Approved assets are compressed (WebP) and stored in MinIO/GCS, linked to the tenant's asset registry.

  ### AI Agent Integration
  - **Creative Agent Coordinator:** Orchestrates the multi-modal generation. It translates the user's intent or the system's need into specific prompts for text and image workers.
  - **Integration Point:** Exposes an internal gRPC service (`AssetGenerationService`) that the Operations or Marketing agents can call when they need new content (e.g., generating an image for an Instagram post or a product variant).

  ## Implementation Prompt
  **Target Persona:** Maya (Home Baker) setting up a new cake offering on her phone.
  **CUJ:** Maya adds a new product called "Vegan Chocolate Dream Cake" but has no photo. She taps "Generate Magic Photo". The app shows a polished loading state. A beautiful, realistic image of a vegan chocolate cake appears, matching her bakery's aesthetic. She taps "Approve", and the image is instantly set as the product photo and optimized for web delivery.

  **Acceptance Criteria for Implementer:**
  1.  Design and implement the `AssetGeneration` data model (PostgreSQL) with strict `tenant_id` isolation.
  2.  Implement the asynchronous job queue logic for handling long-running generation tasks without blocking the main API thread.
  3.  Create the mobile-first (375px) Flutter UI flow: the trigger button, the premium pending state, and the approval/rejection card layout.
  4.  Integrate a mock/stub generation service for local development, ensuring the architecture can seamlessly swap in a real provider (like Gemini or OpenAI) later.
  5.  Ensure all generated assets are properly stored and referenced in the tenant's catalog.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
