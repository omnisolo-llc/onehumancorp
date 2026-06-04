issue_title: "Implement Autonomous Brand Voice & Knowledge Tuning Engine"
issue_description: |
  # [architecture]_autonomous_brand_voice_and_knowledge_tuning_engine

  ## Title
  Autonomous Brand Voice & Knowledge Tuning Engine

  ## Problem Statement
  Small business owners like Maya (The Home Baker) and Carlos (The Freelance Handyman) have very different brands. Maya's interactions with customers are bubbly, use emojis, and focus on celebration. Carlos is direct, professional, and focuses on efficiency. Currently, out-of-the-box AI agents tend to sound generic, robotic, or uniformly "polite," which dilutes the unique identity of each small business. Furthermore, agents lack specific, nuanced business knowledge (e.g., Carlos's specific policy on weekend emergency rates or Maya's preference for a specific type of vegan fondant) unless it's explicitly programmed into the core system. The problem is: how can non-technical owners easily tune the tone, style, and specific factual boundaries of their AI agents without writing complex system prompts?

  ## Research Report
  *   **Competitor Landscape:**
      *   **Shopify Sidekick:** Offers basic tone adjustments (e.g., "make it more professional") but lacks durable, cross-channel brand voice consistency.
      *   **Wix/GoDaddy:** Standard chat implementations are highly rigid. They allow basic Q&A pairs but fail at nuanced conversational style.
      *   **Stand-alone AI tools (e.g., ChatGPT/Claude):** Require users to maintain long, complex "custom instructions" which is beyond the technical comfort zone of our personas.
  *   **The OHC Differentiator:** OHC's Autonomous Brand Voice Tuning Engine will implicitly learn the brand voice by analyzing the owner's past communications, website copy, and manual overrides. It will offer a "Zero-Config" tuning experience where the system asks simple, human-readable questions (e.g., "Would you ever say 'Hey there!' to a new customer?") and translates that into a durable `pgvector` system prompt overlay for all AI departments.
  *   **Goal Targets:**
      *   Maintain a consistent brand voice across all departments (Sales, Customer Success, Marketing).
      *   Seamlessly ingest existing copy (from connected Instagram, email drafts) to "clone" the owner's voice automatically.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Business Owner] -->|Provides Sample Texts / Edits Drafts| B[Voice Tuning Interviewer]
      C[Connected Platforms: IG, Email] -->|Ingests Past Comms| D[AutoDream Context Parser]
      B --> D
      D --> E[Voice & Knowledge Embedding Layer]
      E --> F[(pgvector: brand_identity_embeddings)]

      subgraph KAIROS Orchestrator
          G[Department Agent e.g., CS Ambassador] -->|Query Context| F
          F -->|Returns Tuned System Prompt Overlay| G
          G -->|Generates Response| H[Customer]
      end
  ```

  ### Mobile UX Flow
  1.  **Discovery:** During onboarding (or via the "Marketing" tab), the user sees a card: "Let's teach your AI how you sound."
  2.  **Implicit Learning:** The system automatically analyzes the website's "About Us" page and any connected social media.
  3.  **The "A/B Test" Tuning:** On a 375px mobile screen, the app presents a scenario. "A customer asks about shipping." It shows two AI-generated drafts.
      *   Option A: "Hi! 🍰 We ship within 2 days! Let me know if you need it sooner! ✨"
      *   Option B: "Our standard processing time is 48 hours. Expedited shipping is available upon request."
  4.  **Selection:** The user taps the one that sounds more like them.
  5.  **Durable Tuning:** The system updates the `brand_voice_profile` in the database, adjusting the core system prompt parameters for all future agent interactions.

  ### AI Agent Integration Points
  *   **Prompt Architecture:** The core system prompt for every department will dynamically inject a `{{brand_voice_guidelines}}` string fetched from the tuning engine.
  *   **Feedback Loop:** When an owner manually edits an AI-drafted message before sending, the AutoDream memory pipeline captures the diff. If the owner consistently removes emojis, the system automatically updates the voice profile to reduce emoji usage.

  ## Implementation Prompt
  Design and implement the Autonomous Brand Voice & Knowledge Tuning Engine. Create the data models to store a tenant's `BrandVoiceProfile` (including tone descriptors, vocabulary preferences, and specific knowledge facts). Build the backend service that dynamically injects this profile into the system prompts of all KAIROS agents. Develop a mobile-first onboarding flow that uses an A/B testing mechanism (presenting two varied responses to a mock customer query) to implicitly deduce the user's preferred tone without requiring them to write a complex prompt. Ensure that manual edits to AI drafts are fed back into the tuning engine to continuously refine the voice over time.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
