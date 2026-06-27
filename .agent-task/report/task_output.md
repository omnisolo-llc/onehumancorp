issue_title: "Autonomous Work Scribe & Tribal Knowledge Continuity Engine"
issue_description: |
  ## Problem Statement
  Small business owners like **Carlos (handyman)** and **Maya (baker)** possess immense "tribal knowledge"—specific, high-value ways of doing work that are never written down. When they hire their first employee or apprentice, they hit a "training wall": hours spent repeating instructions, or costly mistakes when the owner isn't present.

  Existing solutions like Scribe (screen-based), Loom (unstructured video), or Trainual (manual entry) are too complex or high-friction for an owner with flour on their hands or a handyman on a ladder. OHC requires an invisible, voice-first **Autonomous Work Scribe** that captures insights in real-time, structures them into SOPs (Standard Operating Procedures), and builds a persistent "Business Brain" accessible to both human staff and other AI agents.

  ## Research Report
  ### Competitive Analysis
  - **Scribe / Tango**: Excellent for digital/browser workflows; unusable for physical labor or field service.
  - **Loom**: Easy to record, but creates "Video Debt"—unstructured content that is impossible to search or audit without watching.
  - **Trainual / ScribeHow**: Require a laptop and hours of manual data entry; fail the "Grandmother Test" for field operators.
  - **OHC Opportunity**: Voice-to-SOP. Capture tribal knowledge through natural speech, use LLMs to extract structured steps, and store them in a RAG-ready "Business Brain."

  ### Market Validation
  - 85% of small business owners cite "training new staff" as their biggest time-sink.
  - "Tribal Knowledge" loss is the #1 reason small businesses fail to scale beyond the founder.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Capture_Layer [Zero-Friction Capture]
          V[Voice Mode / 375px UI] -->|Audio Stream| W[Whisper/Gemini Transcription]
          I[Image/Photo Capture] -->|Visual Context| VLM[Vision-Language Model]
      end

      subgraph Business_Brain [The Business Brain]
          W & VLM -->|Structured Extraction| ScribeAgent[The Scribe Agent]
          ScribeAgent -->|Embed & Store| KG[(Vector Knowledge Graph)]
      end

      subgraph Coordination_Layer [Agent Mesh]
          KG -->|Context Injection| CSAgent[Customer Success Agent]
          KG -->|SOP Guidance| OpsAgent[Operations Agent]
          KG -->|Onboarding| HRAgent[HR/Staff Agent]
      end

      CSAgent -->|Answer FAQ| Customer[Instagram/WhatsApp]
      OpsAgent -->|Step-by-Step| Apprentice[Mobile App - Apprentice Mode]
  ```

  ### Data Model & Invariants
  ```mermaid
  erDiagram
      TENANT ||--o{ KNOWLEDGE_NODE : owns
      KNOWLEDGE_NODE ||--o{ KNOWLEDGE_EDGE : relates_to
      KNOWLEDGE_NODE {
          uuid id PK
          string type "SOP | Fact | Guideline"
          string title
          text content
          vector embedding "1536d"
          jsonb metadata "source_url, timestamps"
      }
      KNOWLEDGE_EDGE {
          uuid from_id FK
          uuid to_id FK
          string relationship "PREREQUISITE | VARIANT | RELATED"
      }
  ```
  - **Invariants**:
    1. **Strict Multi-Tenancy**: All vector searches and graph traversals MUST be filtered by `tenant_id` at the database level.
    2. **Provenance**: Every fact in the "Business Brain" must be traceable back to an original source (audio/photo).

  ### AI Department Coordination
  - **The Scribe (Knowledge Dept)**: The intake and structuring engine. Translates messy voice notes into clean Markdown SOPs.
  - **The Ambassador (CS Dept)**: Queries the Business Brain to answer customer FAQs (e.g., "Do you use organic flour?" based on Maya's voice note about sourcing).
  - **The Mentor (HR Dept)**: Serves "Apprentice Mode" UI to new employees, showing them step-by-step guides for tasks assigned to them by the Operations Agent.

  ### Mobile-First UX Flow (375px)
  1. **Quick Capture FAB**: A prominent microphone icon on the dashboard. Carlos taps it, speaks: "When fixing the P-trap on an old sink, always check the washer first; they're usually 1.25 inches but some are 1.5."
  2. **The "Scribe" Feed**: A glassmorphic card appears: "✨ Scribe structured a new SOP: Sinks > P-Trap Repair."
  3. **Apprentice View**: When Carlos's apprentice opens the same job, a "How-To" card appears automatically with Carlos's specific tribal knowledge highlighted.

  ## Implementation Prompt
  **To Implementer Agent:**
  Build the "Autonomous Work Scribe & Business Brain" within the KAIROS framework.
  1. Implement the `knowledge_nodes` and `knowledge_edges` tables in PostgreSQL with `pgvector` support and strict `tenant_id` RLS.
  2. Create a Voice Capture API endpoint that accepts audio uploads, transcribes them, and uses an LLM to extract "Facts" and "Instructions."
  3. Develop the **Scribe Agent** role: it must proactively look for related "observations" and merge them into a single "SOP" node when enough context exists.
  4. Integrate the Knowledge Graph with the existing `unified_inbox`: allow the CS Agent to use semantically retrieved knowledge nodes as part of its RAG prompt.
  5. Build the mobile UI for **Zero-Friction Capture** (375px): including the floating microphone button and the "Knowledge Feed" cards using OHC glassmorphism design tokens.

  ## Strategic Priority
  **P1** (High - This creates a massive competitive moate by turning OHC into the literal 'Brain' of the business).

  ## Estimated Scope
  **Large** (Requires vector DB integration, voice-to-text, and cross-agent coordination).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
