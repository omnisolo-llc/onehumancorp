issue_title: "[Architecture] Multimodal Agentic CRM & Episodic Memory Consolidation"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (the baker) and Carlos (the handyman) rely heavily on visual and auditory information to run their business. Maya receives inspiration photos for cakes via Instagram DMs; Carlos gets videos of leaky sinks via WhatsApp. Currently, OneHumanCorp (OHC) is "blind" to these assets. Our AI agents can only "see" the text messages, missing the critical context contained in images and audio. This leads to generic AI drafts and forces owners to manually explain details the AI should have already understood. We need a system that converts multimodal inputs into searchable episodic memory, allowing agents to "remember" visual preferences and auditory instructions.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify/Wix:** Use AI for text/image generation but do not autonomously "read" customer-provided images to update CRM state or drive operational workflows.
  - **HubSpot:** Offers powerful CRM but lacks native, autonomous multimodal ingestion that feeds directly into a "digital staff" coordination loop.
  - **OHC Unfair Advantage:** By integrating Gemini Pro Vision and MiniMax transcription directly into the `AutoDream` pipeline, OHC creates a "Unified Sensory Memory" where agents collaborate using visual context as effectively as text.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Ingestion
          Meta[Instagram/WhatsApp Webhook] --> AssetGW[Multimodal Asset Gateway];
          Twilio[SMS/Audio Webhook] --> AssetGW;
      end

      subgraph Processing
          AssetGW --> Storage[(GCS / MinIO)];
          AssetGW --> Vision[Gemini Pro Vision / MiniMax-M3];
          AssetGW --> Audio[Whisper / MiniMax Audio];
      end

      subgraph Memory Consolidation
          Vision --> Desc[Dense Text Descriptions];
          Audio --> Trans[Transcriptions];
          Desc & Trans --> AutoDream[AutoDream Pipeline];
          AutoDream --> VecStore[(pgvector: Consolidated Memory)];
      end

      subgraph Agentic CRM
          VecStore --> CSAgent[Customer Success: Personalized Drafts];
          VecStore --> SalesAgent[Sales: Vision-Based Quotes];
          VecStore --> Advisor[Advisory: Trend Detection];
      end

      subgraph Mobile UX
          Mobile[OHC App 375px] --> Inbox[Unified Inbox w/ Thumbnails];
          Mobile --> CRM[Rich Customer Profile w/ AI Insights];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Multimodal Inbox:** Maya sees a new message from a customer with an image attachment. A small "AI Analyzed" badge appears.
  2. **Agent Insight Card:** Tapping the message reveals a card: *"The Ambassador noted: Customer wants a cake similar to this photo but in vegan chocolate. Drafted a $150 quote."*
  3. **Rich CRM Card:** Maya swipes to the customer profile. Under "Preferences," she sees visual tags: *"Prefers minimalist designs," "Blue floral patterns."*

  ### AI Agent Integration Points
  - **Operations Agent:** Uses visual descriptions to verify inventory (e.g., "Do I have the specific blue dye shown in the photo?").
  - **Sales Agent:** Automatically generates a line-item quote by analyzing the complexity of the customer's provided photo.

  ### Key Design Decisions & Security
  - **Dense Textualization:** Assets are converted to dense text *before* embedding. This ensures that even if the vector search is simple, the LLM has high-fidelity descriptions in its context window.
  - **Zero-Trust Asset Isolation:** Images are stored in tenant-scoped GCS buckets. Access URLs are short-lived and signed via SPIFFE-authenticated proxies.
  - **WebP Compression:** All incoming images are instantly transcoded to WebP to minimize mobile data usage for personas like Fatima.

  ## Implementation Prompt
  Implement the Multimodal Agentic CRM ingestion and memory consolidation layer.
  - **User-Facing Outcome:** AI agents must autonomously describe images and transcribe audio received via webhooks and store them as searchable episodic memory linked to the customer.
  - **CUJ:** A customer sends a photo of a cake to Maya. The system analyzes the photo, extracts "minimalist" and "blue theme," and stores these in the customer's CRM profile. The next time the agent drafts a reply, it mentions these specific visual details.
  - **Acceptance Criteria:**
    - Extend `AutoDream` pipeline to handle `IMAGE` and `AUDIO` source types.
    - Implement a Vision/Audio adapter in `src/agents/builtin/llm/` (leveraging Gemini Pro).
    - Update `inbox_messages` and `customers` schemas to support multimodal metadata.
    - Ensure 100% tenant isolation for all binary assets.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
