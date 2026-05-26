issue_title: "[architecture] Autonomous Cross-Channel Social Ad Buying Engine"
issue_description: |
  # Title: Autonomous Cross-Channel Social Ad Buying Engine

  ## Problem Statement
  Small business owners like Priya (Boutique owner) and Leo (Music tutor) struggle with the "Marketing Dread" and "Invisible Discovery" pain points. While they know they need to advertise on Meta (Instagram/Facebook) or TikTok to drive sales, the cognitive load is immense. Setting up Business Manager accounts, installing tracking pixels, configuring custom audiences, and writing ad copy are overly complex for non-technical users. They often abandon advertising efforts or waste small budgets on ineffective "boosted" posts. They need an invisible, intelligent system that acts as their personal Chief Marketing Officer, translating simple business goals ("I want to sell more of my new summer collection") into high-converting, cross-channel ad campaigns fully autonomously.

  ## Research Report
  *   **Current Architecture Limits:** OHC merchants currently have to rely on third-party integrations or external agencies to run ads. There is no native, AI-driven media buying integration.
  *   **Competitor Analysis:**
      *   *Shopify:* Integrates with Meta and Google for tracking and catalog syncing, but the merchant still has to log into Meta Ads Manager to design and optimize campaigns.
      *   *Wix:* Offers basic Facebook ad integration within their dashboard, but lacks autonomous budget allocation across multiple platforms based on real-time ROAS (Return on Ad Spend).
  *   **Discovery:** OHC needs an "Autonomous Social Ad Buying Engine" that bridges the gap between the merchant's catalog/inventory and external ad networks via API. It must use the "Promoter" (Marketing AI) to generate ad creatives (copy and cropped images) and the "Analyst" (Data AI) to dynamically shift budgets between Meta, TikTok, and Google based on performance, requiring zero manual configuration of ad sets or pixels from the user.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT-GOAL ||--o{ MARKETING-AGENT : "Sets Goal & Budget"
      MARKETING-AGENT ||--o{ CATALOG-API : "Fetches Products/Images"
      MARKETING-AGENT ||--o{ CREATIVE-GENERATOR : "Creates Ad Variants"
      CREATIVE-GENERATOR }|--|| AD-BUYING-ROUTER : "Submits Assets"
      AD-BUYING-ROUTER ||--o{ META-API : "Provisions Campaign"
      AD-BUYING-ROUTER ||--o{ TIKTOK-API : "Provisions Campaign"
      AD-BUYING-ROUTER ||--o{ GOOGLE-API : "Provisions Campaign"
      META-API }|--|| ROAS-ANALYSIS-ENGINE : "Reports Conversions"
      TIKTOK-API }|--|| ROAS-ANALYSIS-ENGINE : "Reports Conversions"
      ROAS-ANALYSIS-ENGINE ||--o{ MARKETING-AGENT : "Optimizes Budget Allocation"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Global Viewport:** 375px width (Mobile First), Translucent Glass materials.
  *   **Action:** Priya wants to run ads for a new dress.
  *   **Screen 1: The Marketing Hub** A clear card showing current traffic. A primary button: `[ ✨ Start a Campaign ]`.
  *   **Screen 2: Goal Definition (Conversational)** The AI asks: "What do you want to promote?" Priya selects the new dress from her catalog. The AI asks: "What is your monthly budget?" Priya enters "$100".
  *   **Screen 3: AI Approval** A summary card appears:
      *   **Preview:** AI-generated ad image (dress cropped perfectly) and compelling copy ("Get ready for summer...").
      *   **Strategy:** "We'll test this on Instagram and TikTok, focusing on women 18-35 in your area. We will automatically shift money to whichever platform gets more sales."
      *   **Action:** `[ Approve & Launch ]`.
  *   **Screen 4: Plain English Reporting** A week later, a notification: "Your campaign generated 5 sales ($250 revenue) from $30 spent on Instagram. We've paused the TikTok ad as it wasn't performing."

  ### Key Design Decisions
  *   **Zero-Config Pixel Management:** The engine must automatically inject and manage tracking pixels/CAPI (Conversions API) payloads on the OHC storefront without the user touching code.
  *   **Dynamic Budget Reallocation:** The system acts as a high-frequency trading bot for ad spend, pulling money from losing ad sets and pushing to winning ones daily.
  *   **Abstracted Ad Accounts:** OHC provisions "child" ad accounts via Meta/TikTok business APIs under a master OHC business manager, hiding the complex setup process completely from the merchant.

  ### AI Agent Integration Points
  *   **The Promoter (Marketing AI):** Ingests catalog images and uses Vision + LLMs to generate high-CTR ad copy and formats (e.g., Stories vs. Feed).
  *   **The Analyst (Data AI):** Continuously ingests conversion events via event mesh, calculating real-time ROAS and adjusting bids/budgets via the Ad-Buying Router.

  ## Implementation Prompt
  Implement the Autonomous Cross-Channel Social Ad Buying Engine for OneHumanCorp. The system must allow non-technical merchants to launch optimized ad campaigns on Meta and TikTok using plain-language goals and simple budget inputs. Focus on building the `AD-BUYING-ROUTER` to interact with external Ads APIs to provision child accounts, create campaigns, upload creatives, and the `ROAS-ANALYSIS-ENGINE` to ingest conversion callbacks via webhooks. Ensure the frontend (375px native app) completely abstracts away targeting, bidding, and pixel configuration. The AI agents must handle creative generation and continuous budget optimization. Ensure strict tenant isolation for ad spend tracking and billing.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
