issue_title: "Design: Autonomous Brand DNA & Operationalized Brand Book Engine"
issue_description: |
  # Autonomous Brand DNA & Operationalized Brand Book Engine

  ## Problem Statement
  Small business owners like Maya (baker), Carlos (handyman), and Priya (boutique owner) are domain experts but lack the skills of a Creative Director. Currently, they face "Design Paralysis" (Ranked #8 in SMB pain points). They struggle to maintain a consistent brand identity across their website, social media, and customer communications.

  Existing platforms treat "Branding" as a set of static theme settings (colors, fonts) that the user must manually configure. There is no central "Memory Primitive" that captures the business's unique "Vibe" and operationalizes it across all AI agents. OHC needs an invisible engine that extracts Brand DNA from a single paragraph of text or a few images and ensures every AI-generated artifact—from a website block to an automated DM reply—is perfectly on-brand.

  ## Research Report
  - **Market Analysis**: Tools like Canva and Adobe Express are powerful but remain standalone creative tools. "Brand DNA" tools like Durable or Wix ADI provide one-time generation but don't carry that identity into the day-to-day operations of the business (e.g., automated support replies).
  - **The OHC Advantage**: By treating Brand DNA as a **Core Tenant Primitive**, OHC ensures that all agents in the "Teammate Mesh" share a unified aesthetic and behavioral context. This is the difference between a generic chatbot and a "Virtual Teammate."
  - **Gap Identified**: The current `builder_brand_toolboxes` in OHC is limited to the Storefront Builder and stored as a JSONB blob. We need to elevate this into a platform-wide **Brand DNA Engine** with a modular schema that serves as the "Source of Truth" for visual and behavioral identity across all services (Marketing, CS, Finance).

  ## Design Doc

  ### Architecture Diagram (ERD)
  ```mermaid
  erDiagram
      TENANT ||--|| BRAND_DNA : "possesses"
      BRAND_DNA ||--o{ VISUAL_PRIMITIVES : "contains"
      BRAND_DNA ||--o{ BEHAVIORAL_PRIMITIVES : "contains"
      BRAND_DNA ||--o{ BRAND_BOOK_ASSET : "generates"

      BRAND_DNA {
          uuid id PK
          uuid tenant_id FK
          string business_vibe "e.g., Cozy, Modern, Professional"
          string mission_statement
          timestamp updated_at
      }

      VISUAL_PRIMITIVES {
          uuid id PK
          uuid tenant_id FK
          jsonb palette "Primary, Secondary, Accent, Background"
          jsonb typography "Headings, Body, Buttons"
          jsonb logo_kit "Icon, Wordmark, Favicon"
          string design_system_token "e.g., Glassmorphism-v2"
      }

      BEHAVIORAL_PRIMITIVES {
          uuid id PK
          uuid tenant_id FK
          string tone_of_voice "e.g., Warm, Witty, Expert"
          jsonb key_phrases "Approved brand language"
          string support_persona "Avatar/Identity for the Ambassador Agent"
      }

      BRAND_BOOK_ASSET {
          uuid id
          string asset_type "PDF, Social_Kit, Brand_Guide"
          string url
      }
  ```

  ### Sequence Diagram: The Zero-Click Discovery
  ```mermaid
  sequenceDiagram
      participant User as Maya (Mobile 375px)
      participant Advisor as The Advisor Agent
      participant DNA_Engine as Brand DNA Engine
      participant Memory as Tenant Vector Memory
      participant Agents as Agent Swarm (Promoter/Ambassador)

      User->>Advisor: "I bake organic sourdough in Brooklyn. Cozy vibes."
      Advisor->>Advisor: Extract Vibe: "Rustic, Organic, Warm"
      Advisor->>DNA_Engine: Trigger: Generate Brand DNA
      DNA_Engine->>DNA_Engine: Select Palette (Earth tones) & Typography (Serif)
      DNA_Engine->>DNA_Engine: Draft Tone of Voice & Mission
      DNA_Engine->>Memory: Persist Brand DNA Primitive
      DNA_Engine-->>User: Present "Brand Vibe" Cards (Glassmorphism UI)
      User->>User: 1-Tap "Approve Vibe"
      DNA_Engine->>Agents: Notify: "New Brand Identity Ready"
      Agents->>Agents: Re-align Website, Social Drafts, and CS Persona
  ```

  ### UI Wireframes & Mobile UX (375px First)
  1.  **The "Vibe Discovery" Screen**: A single, clean prompt: "In one sentence, what's your business vibe?"
  2.  **The Vibe Selection (Frosted Glass Cards)**: The AI presents 3 high-fidelity "Vibe Cards."
      - Each card shows a sample color palette, a font pairing, and a 2-word descriptor (e.g., "Rustic Charm," "Modern Minimalist").
      - **Visuals**: 20px blur, translucent materials, spring animations for selection.
  3.  **The Brand Dashboard**: A modular card showing the "Active DNA." Users see their logo, colors, and tone, but technical details like hex codes are hidden behind "Advanced Settings."

  ### AI Agent Integration
  - **The Advisor**: Responsible for the initial "DNA Extraction" from user inputs (text, photos).
  - **The Promoter (Marketing)**: Automatically applies the Visual Primitives to the Storefront Builder and Social Media drafts.
  - **The Ambassador (CS)**: Adopts the Behavioral Primitives (Tone of Voice) for all automated customer interactions.
  - **The Accountant (Finance)**: Uses the Brand DNA to theme invoices and receipts automatically.

  ### Key Design Decisions
  - **Identity over Implementation**: The engine focuses on defining *what* the brand is, leaving the *how* to the specific implementation agents (e.g., CSS generation for the web).
  - **Proactive Evolution**: The Brand DNA is not static. As the user interacts or provides new content, the Advisor suggests subtle "Brand Refinements" to the owner.
  - **Core Primitive**: Moving from a builder-specific JSON blob to a top-level `brand_dna` schema to enable platform-wide consistency.

  ## Implementation Prompt
  **Objective**: Build the "Autonomous Brand DNA Engine" as the foundational identity layer for the OHC platform.

  **User-Facing Outcome**: Maya types "Cozy organic bakery" during onboarding. The system instantly generates a full brand identity—including a color palette, font pairings, and a warm tone of voice. All subsequent AI-generated content (her website, her Instagram posts, her automated replies) automatically inherits this identity.

  **Critical User Journey (CUJ)**:
  1.  User provides a short business description on mobile (375px).
  2.  The Advisor Agent extracts the "Brand DNA" and persists it in the modular `brand_dna`, `visual_primitives`, and `behavioral_primitives` tables.
  3.  The user selects a "Vibe Card" from a glassmorphic carousel.
  4.  The system operationalizes this DNA, ensuring the next drafted social post and the current website preview instantly update to match the new identity.

  **Acceptance Criteria**:
  - **Core Entities**: Implement the `brand_dna`, `visual_primitives`, and `behavioral_primitives` models in PostgreSQL with strict multi-tenant isolation and RLS.
  - **DNA Extraction**: Integrate the LLM provider (Gemini/MiniMax) to derive structured brand data (palette, typography, tone) from unstructured user input.
  - **Mobile UI**: Build the 375px "Vibe Selection" interface using OHC premium design tokens (Glassmorphism, 20px blur).
  - **Agent Handoff**: Implement a notification/event system (via NATS/Event Mesh) so downstream agents (Promoter, Ambassador, Accountant) are alerted when Brand DNA changes.
  - **Grandmother Test**: The entire branding process must require < 3 taps and zero knowledge of hex codes or design theory.
  - **Tests**: 100% unit test coverage for new models and extraction logic. Playwright E2E test covering the "Vibe Selection" to "Operationalization" flow.

  ## Priority
  P0 (Foundational Primitive)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
