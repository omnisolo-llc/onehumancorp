issue_title: "[Architecture] Autonomous Multi-Platform Campaign & Promotion Engine"
issue_description: |
  ## Title
  Autonomous Multi-Platform Campaign & Promotion Engine

  ## Problem Statement
  Small business owners like Priya (Boutique) and Maya (Baker) spend hours managing fragmented marketing efforts. They manually create posts for Instagram, draft separate emails in tools like Mailchimp, and send distinct SMS blasts, completely disconnected from their actual inventory levels or real-time sales performance. They lack a unified way to seamlessly create, deploy, and autonomously optimize multi-channel promotional campaigns. The "Grandmother Test" fails when a user has to manage 3 distinct dashboards to run a weekend flash sale.

  ## Research Report
  ### Competitive Landscape
  *   **Shopify Campaigns**: Offers email and basic ad integration but still requires manual creation and coordination across separate apps. Too complex for simple flash sales.
  *   **Wix Ascend**: Integrated but often feels clunky and is not inherently driven by AI insights; requires manual targeting and asset generation.
  *   **Klaviyo / Mailchimp**: Powerful standalone marketing engines, but fully disconnected from the core business platform without extensive setup.

  ### The OHC Gap
  OneHumanCorp's current architecture has isolated AI agents and a data model for social inboxes and omnichannel reputation, but lacks a centralized, autonomous "Campaign Engine". This engine would allow the Marketing AI Department to proactively suggest, generate, and execute synchronized campaigns across Social (Meta/IG), Email, and SMS, directly tied to real-time inventory and ledger data.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CAMPAIGN : launches
      CAMPAIGN ||--o{ CAMPAIGN_ASSET : generates
      CAMPAIGN ||--o{ CHANNEL_EXECUTION : tracks
      CAMPAIGN ||--o| PROMOTION_CODE : utilizes

      TENANT {
          string id PK
          string brand_voice_config
      }
      CAMPAIGN {
          string id PK
          string goal "Flash Sale | Engagement | Restock"
          string status "Draft | Active | Paused | Completed"
          timestamp start_time
          timestamp end_time
      }
      CAMPAIGN_ASSET {
          string id PK
          string type "Image | Copy | Video_Snippet"
          string content_url
      }
      CHANNEL_EXECUTION {
          string id PK
          string channel "Email | SMS | Instagram"
          int metrics_sent
          int metrics_clicks
          int metrics_conversions
      }
      PROMOTION_CODE {
          string code PK
          float discount_value
          string discount_type "Percent | Fixed"
      }
  ```

  ### Mobile UX Flow (375px First)
  1.  **The Proactive Push:** Maya receives a notification: "Marketing Agent: You have 20 vegan cakes unsold for the weekend. Shall I run a 15% off flash sale to your top local customers?"
  2.  **1-Tap Approval:** Maya taps the notification. The OHC app shows a clean "Translucent Glass" preview card:
      *   Instagram Story mock-up.
      *   SMS text preview.
      *   Email snippet.
  3.  **Execution:** Maya taps "Approve & Launch". The AI handles the rest, automatically applying the discount code on the checkout link.
  4.  **Plain-Language Briefing:** After the weekend, Maya gets a simple SMS: "Your flash sale sold out the cakes and made $300! It cost $5 in SMS fees. Great work!"

  ### AI Agent Integration Points
  *   **Marketing AI Department:** Analyzes inventory thresholds (via Operations AI) to trigger campaign suggestions. Generates copy and visual assets adhering to the tenant's `brand_voice_config`.
  *   **Operations AI Department:** Provides the trigger (e.g., aging inventory, seasonal trends) and locks inventory allocated for promotions.
  *   **Finance AI Department:** Tracks the ROI of the campaign in real-time, attributing specific ledger deposits to the active `CAMPAIGN.id`.

  ### Key Design Decisions & Integrity
  *   **Unified Campaign Abstraction:** Instead of treating an email and an IG post as separate entities, they are `CHANNEL_EXECUTION` children of a unified `CAMPAIGN` parent, ensuring cross-platform synchronization.
  *   **Invisible Targeting:** The user does not build segments. The AI Marketing agent autonomously selects the target audience based on purchase history and engagement metrics.
  *   **Budget Guardrails:** Strict tenant-level constraints on SMS/Email sending costs to prevent unexpected bills, managed via Zero-Trust validation.
  *   **Edge-Performance Analytics:** Tracking pixels and redirect links are served via Edge Caching to guarantee <50ms latency for end customers interacting with promotional links.

  ## Implementation Prompt
  **To Implementer Agent:**
  Design and implement the core schema and internal API for the multi-channel `CampaignEngine`.

  The system must:
  1.  Define the database schema for Campaigns, Assets, and ChannelExecutions.
  2.  Provide a robust API to create a Draft Campaign and attach cross-channel assets to it.
  3.  Implement a state machine transitioning Campaigns from Draft -> Active -> Completed.
  4.  Ensure all endpoints and queries are strictly tenant-isolated.

  Do not implement the third-party integrations (SendGrid, Twilio, Meta) yet. Focus on the centralized Orchestrator component that holds the state and handles the event-driven triggers when a campaign state changes.

  **Acceptance Criteria:**
  *   A Campaign entity can be created, updated, and queried securely per tenant.
  *   State transitions correctly validate that necessary assets are present before moving to "Active".
  *   The system can associate multiple independent channel executions to a single unified campaign.

  ## Priority
  P1 (High)

  ## Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
