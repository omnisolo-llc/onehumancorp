issue_title: "[Feature] Autonomous Short-Form Video Generator"
issue_description: |
  **Problem Statement:**
  Small business owners (like Maya the baker and Priya the boutique owner) suffer from "Content Creation Block" (Top 10 Pain Point #4). They lack the time and skills to turn static product photos into engaging, algorithm-friendly short-form videos (TikTok, Instagram Reels). They need a zero-effort way to transform their catalog into viral social content to drive acquisition.

  **Research Report:**
  - Based on the `ohc_small_business_platform_gap_analysis.md`, competitor platforms (Shopify, Wix) fail to provide autonomous video generation, offering only generic AI text or basic image cropping.
  - Video content is critical for discovery, yet the creation process is manual and overwhelming for non-technical users.
  - Introducing a Marketing AI Agent that automatically compiles product photos, adds trending audio, and generates captions creates a massive competitive moat and directly addresses the content creation bottleneck.

  **Design Doc:**

  - **UI Flow (375px Mobile First):**
    - The user receives a push notification: "✨ Your new Reel for the Vegan Cupcake is ready!"
    - Tapping opens a full-screen, vertical video preview (Translucent Glass UI layered over the video).
    - Floating Action Bar at the bottom with three options:
      1. "Approve & Post" (Primary, highly visible)
      2. "Regenerate" (Secondary)
      3. "Advanced Edit" (Tertiary, for technical adjustments)
  - **Mobile UX Flow:**
    - Designed for 1-tap approval. The user does not build the video; they merely approve the AI's work.
  - **AI Agent Integration Points:**
    - **Marketing Agent:** Continuously scans the catalog for new arrivals, high-margin items, or slow-moving stock.
    - **Video Generation Engine:** A sub-agent that takes product images, stitches them with dynamic transitions, adds text overlays based on product descriptions, and syncs to trending audio.
    - **Social Agent:** Handles the API handoff to TikTok/Instagram upon user approval.

  - **Architecture Diagram:**

  ```mermaid
  erDiagram
      CATALOG ||--o{ VIDEO_ASSET : "sources material for"
      MARKETING_AGENT ||--|{ CATALOG : "monitors"
      MARKETING_AGENT ||--|{ VIDEO_ASSET : "orchestrates creation"
      VIDEO_ASSET ||--o{ SOCIAL_POST : "is published as"
      USER ||--o{ SOCIAL_POST : "approves"
  ```

  ```mermaid
  sequenceDiagram
      participant Catalog
      participant MarketingAgent
      participant VideoEngine
      participant User
      participant SocialAgent

      MarketingAgent->>Catalog: Detect new product addition
      MarketingAgent->>VideoEngine: Request video (assets + trending audio context)
      VideoEngine-->>MarketingAgent: Return compiled vertical video
      MarketingAgent->>User: Push Notification (Review Video)
      User->>MarketingAgent: 1-Tap "Approve & Post"
      MarketingAgent->>SocialAgent: Handoff for scheduling/publishing
  ```

  **Implementation Prompt:**
  Implement the autonomous short-form video generation workflow. The backend must orchestrate a recurring job that identifies eligible products, triggers the video generation service, and stages the resulting media asset. Build the mobile-first frontend review screen featuring a full-screen vertical video player with a floating action bar for 1-tap approval. Ensure the user journey requires zero timeline editing by default. Do not prescribe the specific video rendering technology or the database schema for the assets—focus on the coordination logic and the flawless presentation of the review UI.

  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
