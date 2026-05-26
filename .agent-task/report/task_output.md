issue_title: "[Architecture] Autonomous Visual and Video Marketing Engine"
issue_description: |
  # [Architecture] Autonomous Visual and Video Marketing Engine

  ## Title
  Autonomous Visual and Video Marketing Engine

  ## Problem Statement
  Business owners like **Maya (baker, 28)** and **Leo (music tutor, 22)** rely heavily on highly visual social media platforms like Instagram and TikTok to attract new customers. However, creating high-quality, engaging short-form video content and visual assets is time-consuming, requires specialized editing skills (using complex tools like Premiere, CapCut, or Canva), and distracts them from their core business operations. Existing platforms like Shopify and Wix provide static templates but lack the ability to autonomously generate, edit, and publish dynamic video content tailored to the business owner's unique brand and inventory.

  ## Research Report
  Current SMB platforms and marketing tools approach content creation as a manual, human-driven process:
  - **Shopify / Wix / Squarespace:** Offer basic image optimization and static layout templates, occasionally with simple GenAI text-to-image integration. They do not generate video content or automate the multi-platform publishing workflow.
  - **Canva / CapCut:** Powerful tools but require the user to act as a part-time graphic designer and video editor, selecting templates, syncing audio, and managing exports.
  - **OneHumanCorp (OHC) Opportunity:** Shift from providing *tools* to providing an *agentic solution*. OHC's Autonomous Visual and Video Marketing Engine will act as an invisible, silent creative agency. It will synthesize product catalogs, user-provided raw assets (e.g., photos of cakes or video clips of lessons), and brand guidelines to autonomously generate, render, and schedule short-form videos (Reels, TikToks, Shorts) and high-converting visual posts.

  ## Design Doc

  ### Architecture Overview

  ```mermaid
  graph TD;
      RawAssets[Raw Assets: Photos/Clips] -->|Ingest| AssetManager[Asset Management Engine]
      Catalog[Product/Service Catalog] -->|Context| AssetManager
      AssetManager -->|Contextualized Assets| VideoGen[Video Generation Agent]
      BrandStyle[Brand Guidelines & Tokens] --> VideoGen
      VideoGen -->|Rendered Drafts| OHCInbox[OHC Unified Inbox for 1-Tap Approval]
      OHCInbox -->|Approved| Publisher[Multi-Platform Publishing Engine]
      Publisher -->|API Integration| TikTok[TikTok API]
      Publisher -->|API Integration| Meta[Instagram/FB Graph API]
      Publisher -->|API Integration| YouTube[YouTube Shorts API]
  ```

  ### Data Model & Entity Relationships

  ```mermaid
  erDiagram
      TENANT ||--o{ ASSET : owns
      ASSET {
          string id PK
          string tenant_id FK
          string type "image|video_clip"
          string url
          json metadata "tags, duration"
      }
      TENANT ||--o{ MARKETING_CAMPAIGN : manages
      MARKETING_CAMPAIGN {
          string id PK
          string tenant_id FK
          string status "draft|scheduled|published"
          string target_platform
          json rendered_media_urls
          json copy_text
      }
      MARKETING_CAMPAIGN }o--|{ ASSET : utilizes
  ```

  ### Key Design Decisions
  1. **Agentic Generation over Manual Editing:** The system relies on backend agents (coordinating with generative video/image APIs) to assemble clips, sync trending audio, and apply brand colors autonomously, removing the need for a timeline-based editor UI.
  2. **1-Tap Approval via Unified Inbox:** Instead of a complex marketing dashboard, drafted videos are surfaced in the user's unified inbox (or via mobile push notification) for a simple "Approve & Post" or "Regenerate" action, passing the "grandmother test."
  3. **Multi-Tenant Isolation & Zero Trust:** All raw assets and generated media are strictly scoped to the `tenant_id` using the OHC-HA Hybrid Architecture (SPIFFE/SPIRE). Media processing jobs run in isolated queues to guarantee data privacy.
  4. **Mobile-First UX:** The entire review and approval workflow is optimized for a 375px viewport. Users can preview the video natively and approve it with a single tap.

  ## Implementation Prompt
  **Objective:** Implement the Autonomous Visual and Video Marketing Engine.
  **Context:** This system must ingest raw images/video clips from a tenant's catalog, generate short-form promotional videos using AI agents, and surface them in the OHC mobile app for 1-tap approval before publishing to social media platforms.
  **Requirements:**
  1. Create the backend service to orchestrate asset ingestion, video generation, and publishing.
  2. Implement strict multi-tenant isolation for all assets and generated content.
  3. Develop the mobile-first approval UI (conforming to macOS-style Translucent Glass and UniFi card layouts).
  4. Integrate with the OHC Unified Inbox for notifications and approval workflows.
  5. Do NOT prescribe specific generative APIs (e.g., Runway, Sora) at this stage; build the integration layer flexibly.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
