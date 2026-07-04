issue_title: "Feature Mission: Context-Aware Mobile Work Triage Inbox"
issue_description: |
  # OHC Competitor Analysis & Feature Mission: Context-Aware Customer Follow-Ups

  ## 1. Top 10 General Competitors & Top 10 AI-Native Competitors

  ### Top 10 General Competitors
  1. **Shopify** - Commerce giant moving into ops and AI (Sidekick).
  2. **Tencent Workbuddy / WeCom** - Unmatched in mobile-first integrated comms and ops for SME in Asia.
  3. **DingTalk** - Deeply integrated organizational operations and attendance.
  4. **Feishu / Lark** - Collaboration heavy, agentic AI workflows.
  5. **Square** - POS dominance, expanding into booking and staff management.
  6. **Wix** - Website builder expanding into full SME CRM/Ops.
  7. **HubSpot** - CRM leader pushing heavily into SME automation.
  8. **Notion** - Knowledge base evolving into a unified work OS.
  9. **Microsoft 365 Copilot** - Enterprise-first, but scaling down to SME email/doc workflows.
  10. **HoneyBook** - Vertical SaaS strong in freelance/agency booking and invoicing.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick** - Conversational AI for commerce ops.
  2. **Lindy.ai** - Autonomous AI employees for scheduling/email.
  3. **Sana AI** - AI workspace assistant combining knowledge and actions.
  4. **Julius AI** - Data analysis assistant, taking dashboarding to chat.
  5. **Reclaim.ai** - AI calendar assistant.
  6. **Chatbase** - Custom AI chatbots for customer service.
  7. **Bland AI** - Phone calling AI for SMEs.
  8. **Artisan** - AI BDRs and sales assistants.
  9. **Harvey** - Vertical AI (Legal), demonstrating the "copilot to autopilot" shift.
  10. **Airtable Cobuilder** - AI-driven app creation for ops.

  ## 2. Deep-Dive Competitor Audit: Shopify Sidekick

  **Selected Competitor:** Shopify Sidekick

  **Capabilities ("What they can do"):**
  - Context-aware answers about sales data.
  - Task execution via chat (e.g., "Put all red shirts on a 10% discount").
  - Content generation (e.g., rewriting product descriptions).

  **Success Factors:**
  - Deep, native integration with the merchant's catalog and sales data.
  - Conversational interface that removes the need to navigate complex admin menus.
  - "Show me, don't just tell me" UX, where Sidekick prepares the action and the user clicks approve.

  **User Sentiment Audit (Reddit/Trustpilot findings):**
  - *Positive:* Users love the time saved on tedious tasks like bulk editing. "It's like having a Shopify expert on call."
  - *Negative:* It is tied exclusively to the Shopify ecosystem. It struggles with multi-channel ops (e.g., syncing in-store ad-hoc sales, Instagram DMs). It's built for *commerce*, not general *operations* (e.g., Carlos the Handyman can't use Sidekick to schedule a repair).

  ## 3. Gap & Pain Point Identification (OHC vs Shopify Sidekick)

  ### Competitive Landscape Chart
  ```mermaid
  quadrantChart
      title Market Positioning
      x-axis Low AI Automation --> High AI Automation
      y-axis General Business Ops --> Vertical E-commerce
      quadrant-1 Specialized AI Commerce
      quadrant-2 Agentic General Operations
      quadrant-3 Traditional General Software
      quadrant-4 Traditional Storefronts
      "Shopify Sidekick": [0.8, 0.9]
      "Tencent Workbuddy": [0.4, 0.4]
      "Square": [0.2, 0.8]
      "Wix": [0.3, 0.7]
      "Notion AI": [0.7, 0.3]
      "Microsoft Copilot": [0.7, 0.2]
      "OHC (Current)": [0.4, 0.5]
      "OHC (Target)": [0.9, 0.5]
  ```

  ### Gap Matrix (Feature Heatmap)

  | Feature | Shopify Sidekick | Tencent Workbuddy | OHC (Current) | OHC (Target) |
  |---|---|---|---|---|
  | E-commerce Data Querying | High | Low | Low | High |
  | Task Automation (Discounts) | High | Low | Low | High |
  | Service & Booking Integration | Low | Low | Medium | High |
  | Multi-channel DM Triage | Low | High | Low | High |

  ### Unresolved Pain Point
  **Pain Point:** Owners (like Maya and Carlos) lose leads because they are too busy doing the work to reply to DMs or emails promptly. Shopify Sidekick helps manage the *store*, but it does not proactively manage *lead triage and follow-up across fragmented channels* on a mobile phone.

  ## 4. Agentic Solution Design: Autonomous "Work Triage" Inbox

  **Concept:** The AI acts as a smart filter. It ingests Instagram DMs, SMS, and emails into a single 375px-optimized feed. It drafts replies based on previous knowledge, catalog state, and calendar availability. The owner just taps "Approve" or "Edit."

  ### Persona Impact
  - **Maya (Baker):** Gets an IG DM asking for a vegan cake. AI checks past orders, knows she does vegan, drafts a reply with a quote link. Maya taps approve while baking.
  - **Carlos (Handyman):** Gets an SMS asking for a Tuesday repair. AI checks his calendar, sees Tuesday is full, drafts a reply offering Wednesday. Carlos approves at a red light.

  ### User Journey Comparison
  ```mermaid
  journey
      title User Journey: Handling a New Lead
      section Shopify (Status Quo)
        Check Instagram: 3: Maya
        Switch to Shopify to check price: 2: Maya
        Switch back to Instagram: 2: Maya
        Type reply manually: 1: Maya
      section OHC (Target)
        Push notification received: 5: Maya
        Open 375px OHC feed: 5: Maya
        Review pre-drafted AI reply: 5: Maya
        Tap 'Approve & Send': 5: Maya
  ```

  ---
  ## References & Sources
  - [Sidekick](https://www.shopify.com/sidekick)
  - [Sidekick](https://shopify.dev/docs/apps/build/sidekick)
  - [Sidekick](https://help.shopify.com/en/manual/shopify-admin/productivity-tools/sidekick)
  - [Shopify Sidekick](https://www.getmesa.com/blog/shopify-sidekick)
  - [Shopify Sidekick](https://roswell.nyc/insights/shopify-sidekick)
  - [Competitor Research Document](https://www.cwill.com/blogs/shopify-sidekick/)
  - [Workbuddy](https://www.tencentcloud.com/act/pro/workbuddy)
  - [Competitor Research Document](https://www.workbuddy.ai/)
  - [144100?Lang=En](https://www.tencentcloud.com/techpedia/144100?lang=en)
  - [Overview](https://www.workbuddy.ai/docs/workbuddy/Overview)
  - [Competitor Research Document](https://copilot.tencent.com/work/)
  - [Workbuddy Tencent Out Of The Box Ai Agent](https://www.revolutionai.io/blog/workbuddy-tencent-out-of-the-box-ai-agent)
  - [Appxlistingdetail?Listingid=A0N4V00000Fz2Wcuaz](https://appexchange.salesforce.com/appxListingDetail?listingId=a0N4V00000Fz2WCUAZ)
  - [Competitor Research Document](https://www.communicat-o.com/hubspot-wecom-integration/)
  - [Competitor Research Document](https://wecom.cn.com/)
  - [1624975185779.Wecomconnector?Tab=Overview](https://marketplace.microsoft.com/en-us/product/saas/1624975185779.wecomconnector?tab=overview)
  - [1624975185779.Wecomconnector?Tab=Overview](https://marketplace.microsoft.com/en-us/product/web-apps/1624975185779.wecomconnector?tab=overview)
  - [Introducing Charket Wecom.Html](https://www.charket.ai/articles/introducing-charket-wecom.html)
  - [Competitor Research Document](https://www.dingtalk.io/)
  - [Dingtalk How To Transform Business Communication 25121663](https://www.dingtalk-global.com/news/explain/dingtalk-how-to-transform-business-communication-25121663)
  - [Competitor Research Document](https://oa.dingtalk.com/)
  - [Dingtalk Digital Management Revolution 260529](https://www.dingtalk-asia.com/features/dingtalk-digital-management-revolution-260529)
  - [Index_New.Htm](https://oa.dingtalk.com/index_new.htm)
  - [How Dingtalk Transforms Enterprise Operations 26020772](https://www.dingtalk-global.com/news/explain/how-dingtalk-transforms-enterprise-operations-26020772)
  - [Feishu Cli Let Ai Actually Do Your Work In Feishu](https://open.feishu.cn/document/mcp_open_tools/feishu-cli-let-ai-actually-do-your-work-in-feishu)
  - [Feishu Cli Let Ai Actually Do Your Work In Feishu](https://open.larksuite.com/document/mcp_open_tools/feishu-cli-let-ai-actually-do-your-work-in-feishu)
  - [Feishu Cli](https://www.feishu.cn/feishu-cli)
  - [Competitor Research Document](https://feishu-cli.com/)
  - [Cli](https://github.com/larksuite/cli)
  - [Feishucli](https://github.com/lightencc/feishuCLI)
  - [Competitor Research Document](https://www.notion.com/)
  - [Pricing](https://www.notion.com/pricing)
  - [Startups Application Form](https://www.notion.so/startups-application-form)
  - [Competitor Research Document](https://makebusiness.eu/notion-ai-agents-small-team-automation-playbook/)
  - [Competitor Research Document](https://alicialyttle.com/notion-ai-for-work-entrepreneurs/)
  - [Competitor Research Document](https://www.aioperator.com/blog/notion-ai-for-business-a-practical-guide-to-building-your-ai-powered-workspace/)
  - [Competitor Research Document](https://www.microsoft.com/en-us/microsoft-365/blog/2025/12/02/microsoft-365-copilot-business-the-future-of-work-for-small-businesses/)
  - [Competitor Research Document](https://adoption.microsoft.com/en-us/copilot/smb/)
  - [Competitor Research Document](https://tminus365.com/microsoft-365-copilot-for-business-what-you-need-to-know/)
  - [Microsoft Copilot Pricing 2025 Full Breakdown Of Microsoft 365 Business And Enterprise Plans](https://www.datastudios.org/post/microsoft-copilot-pricing-2025-full-breakdown-of-microsoft-365-business-and-enterprise-plans)
  - [Microsoft 365 Copilot Business Everything You Need To Know](https://www.trustedtechteam.com/blogs/microsoft-365/microsoft-365-copilot-business-everything-you-need-to-know)
  - [Competitor Research Document](https://bondconsultingservices.com/blog/microsoft-365-copilot-business-pricing-smb-2026/)
  - [Competitor Research Document](https://squareup.com/us/en)
  - [Business Needs](https://squareup.com/us/en/solutions/business-needs)
  - [Competitor Research Document](https://fitsmallbusiness.com/what-is-square/)
  - [Competitor Research Document](https://www.merchantmaverick.com/what-is-square/)
  - [Competitor Research Document](https://www.techrepublic.com/article/what-is-square/)
  - [Competitor Research Document](https://www.cardpaymentoptions.com/credit-card-processing/the-impact-of-square-on-small-businesses/)
  - [Managing Your Site With The Wix App](https://support.wix.com/en/managing-your-site-with-the-wix-app)
  - [Wix App](https://www.wix.com/mobile/wix-app)
  - [Watch?V=S Gzp4 Fpta](https://www.youtube.com/watch?v=s-gzp4-fpTA)
  - [Id1545924344](https://apps.apple.com/my/app/wix-owner-websites-apps/id1545924344)
  - [Id1545924344](https://apps.apple.com/my/app/wix-owner-website-builder/id1545924344)
  - [Details?Id=Com.Wix.Admin&Hl=En Us](https://play.google.com/store/apps/details?id=com.wix.admin&hl=en-US)
  - [Ai Crm](https://www.hubspot.com/products/crm/ai-crm)
  - [Artificial Intelligence](https://www.hubspot.com/products/artificial-intelligence)
  - [Hubspot Ai](https://toolmango.com/tools/hubspot-ai)
  - [Competitor Research Document](https://www.simplemachinesmarketing.com/blog/hubspot-ai-whats-actually-useful-and-what-to-skip/)
  - [Hubspot Ai Tools](https://www.hublead.io/blog/hubspot-ai-tools)
  - [Competitor Research Document](https://en-us-hubspot.com/)


  ### Design Doc
  - **Architecture:** `WorkTriageAgent` (AI) listens to a unified messaging event queue (Webhook -> DB -> Queue). It reads `OwnerContext` (Calendar, Inventory, Past Orders) to draft replies.
  - **UI/UX (375px):** A single "Action Feed" screen. Each card is a message with a pre-drafted reply and two giant touch targets (44x44px minimum): "Approve & Send" and "Edit".
  - **Entity Types:** `UnifiedMessage`, `SuggestedAction`, `AgentDraft`.

  ### Implementation Prompt
  Implement the "Work Triage" mobile feed.
  - CUJ: The owner logs in on a 375px screen, sees 3 pending customer requests in the action feed, taps "Approve" on a drafted quote, and the item disappears from the queue.
  - Acceptance Criteria: The UI must be usable without horizontal scrolling at 375px. The backend must support creating a `UnifiedMessage` with a linked `AgentDraft` and handle the "Approve" mutation.

  ### Estimated Scope
  Medium

  ### Visual Mermaid Charts

  ```mermaid
  sequenceDiagram
      actor Maya
      participant OHC Triage Inbox
      participant AI Agent (Customer Success)
      participant Instagram

      Instagram->>OHC Triage Inbox: Customer DM "Vegan options?"
      OHC Triage Inbox->>AI Agent (Customer Success): Analyze DM + Catalog
      AI Agent (Customer Success)-->>OHC Triage Inbox: Draft reply "Yes, we do! Here is the link..."
      OHC Triage Inbox-->>Maya: Push Notification "New message drafted"
      Maya->>OHC Triage Inbox: Reviews draft on 375px screen
      Maya->>OHC Triage Inbox: Taps 'Approve'
      OHC Triage Inbox->>Instagram: Sends reply
  ```
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
