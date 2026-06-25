issue_title: "Implement Work Triage Feed & Autonomous Unified Inbox"
issue_description: |
  # Mission Brief: Work Triage Feed & Autonomous Unified Inbox

  ## Problem Statement
  Small business owners and operators like Maya (the baker) and Carlos (the field service owner) are overwhelmed by fragmented communications. They receive inquiries across Instagram DMs, SMS, email, and web forms. Each channel requires manual context-switching, leading to delayed responses, missed leads, and operational chaos. They need a single, unified Work Triage feed where an AI Assistant not only centralizes messages but proactively drafts responses, extracts booking intents, and presents the owner with simple "Approve/Edit" actions.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We analyzed the landscape of owner/operator work assistants to understand the benchmark for unified communication and AI agent support.

  #### Top 10 General Competitors
  | Competitor | URL | Unique AI Capabilities / Focus |
  | :--- | :--- | :--- |
  | **Tencent Workbuddy** | tencent.com | Unifies chat, tasks, and corporate systems into a single conversational feed. |
  | **WeCom** | work.weixin.qq.com | Seamless customer connection via WeChat with built-in CRM and auto-replies. |
  | **DingTalk** | dingtalk.com | "AI Assistant" deeply integrates with organizational tasks, approvals, and scheduling. |
  | **Feishu / Lark** | larksuite.com | "Lark Base" and AI assistant for document summarization, task extraction, and translations. |
  | **Shopify** | shopify.com | **Sidekick:** Commerce-obsessed AI assistant for site edits, reporting, and unified inbox help. |
  | **HubSpot** | hubspot.com | **Breeze:** AI agents deeply integrated into CRM data for multi-channel service and prospecting. |
  | **Square** | squareups.com | Smart customer directories with unified messaging and automated follow-ups. |
  | **Wix** | wix.com | Inbox integration across web chat, Facebook, and Instagram with AI draft replies. |
  | **Microsoft Copilot** | microsoft.com | Teams/Outlook integration to summarize threads and generate task action items. |
  | **Notion** | notion.so | **Notion AI:** Auto-organizes knowledge and synthesizes fragmented notes into action plans. |

  #### Top 10 AI-Native Competitors
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Durable** | durable.co | Generates a complete business website, CRM, and integrated inbox in under a minute. |
  | **Lindy.ai** | lindy.ai | Executive AI assistant that handles email triage, scheduling, and admin tasks autonomously. |
  | **11x.ai** | 11x.ai | Autonomous digital workers (Alice) for inbound/outbound communication handling. |
  | **Intercom Fin** | fin.ai | Resolves 50%+ of support queries without human intervention across omnichannel. |
  | **Relevance AI** | relevanceai.com | Allows non-technical owners to build autonomous agentic teams for sales and ops. |
  | **Gorgias** | gorgias.com | E-commerce helpdesk with AI that drafts replies and executes Shopify actions (refunds/orders) directly. |
  | **Zendesk AI** | zendesk.com | Intent recognition across all social channels to categorize and auto-route tickets. |
  | **DevRev** | devrev.ai | AI-native support platform that connects customer conversations directly to product tasks. |
  | **Superhuman** | superhuman.com | AI-powered email triage, automated drafting, and extremely fast keyboard-driven UI. |
  | **Bland AI** | bland.ai | AI phone calling agents that can handle incoming calls, schedule appointments, and sync to CRM. |

  ### Track 2: Deep-Dive Competitor Audit (WeCom & DingTalk)
  We examined Tencent's enterprise ecosystem tools (WeCom/Workbuddy) and Alibaba's DingTalk, as they heavily influence the "All-in-One Owner Work Assistant" model.
  - **Capabilities:** Deep integration of IM, CRM, tasks, and third-party apps. A user can transition an external customer chat directly into a task or a sales opportunity without leaving the feed.
  - **Success Factors:** The feed is the OS. Everything is an actionable card. No context switching.
  - **User Sentiment Audit:**
    - *Positive:* "Having customer chats and internal staff schedules in one app saves me 2 hours a day." (r/SaaS, app reviews).
    - *Negative:* "It feels like an enterprise surveillance tool, not a personal assistant. Too many administrative menus for a simple 3-person shop." (Reddit, App Store).

  ### Track 3: OHC Gap & Pain Point Identification
  | Capability | WeCom / DingTalk | Shopify Inbox | **OHC (Current)** | **OHC (Proposed)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Intake** | 🟢 Native IM/Social | 🟡 Web/Email | 🔴 Fragmented | **🟢 Agentic Work Triage Feed** |
  | **AI Drafting** | 🟡 Templates | 🟢 Sidekick | 🔴 Missing | **🟢 Auto-Drafts + Action Proposals** |
  | **Intent Extraction**| 🟡 Manual linking | 🟡 Order linking | 🔴 None | **🟢 Auto-extracts tasks & bookings** |
  | **SMB Fit** | 🔴 Enterprise-heavy | 🟡 Commerce-only | 🟡 General | **🟢 Owner-centric, zero-jargon** |

  **Unresolved Pain Point:** Existing tools either feel like enterprise administrative software (DingTalk) or lack deep operational awareness across varied channels (Shopify is commerce only; doesn't help a handyman book a route).

  ### Track 4: Deeper Focused Research & Agentic Solutions
  To solve the intake chaos for operators, OHC must build the **Work Triage Feed**.
  - **Agentic Solution:** An invisible AI agent monitors connected channels (Email, Mocked DMs). When a message arrives, the agent categorizes it, extracts relevant context (e.g., "cake for Saturday"), checks the backend state (availability/inventory), and places an Action Card in the Triage Feed. The owner opens the app, sees "Maya: 3 inquiries need replies", and clicks "Approve & Send" on AI-drafted quotes.

  ### Design Doc
  - **Architecture:**
    - `TriageItem` entity (ID, Source, Status, ExtractedIntent, SuggestedAction).
    - Background AI Job: Ingests raw messages -> Passes to LLM -> Persists `TriageItem` -> Emits real-time event.
  - **UI/UX (Mobile First 375px):**
    - **Home Screen:** "Today's Work Feed".
    - Translucent glass aesthetic cards. Each card displays: Sender, Summary, and a prominent Action Button (e.g., "Draft Reply", "Send Quote").
    - Tapping a card opens a detailed view with the full message thread and the AI's reasoning/draft.

  ### Implementation Prompt
  **User-Facing Outcome:** The owner logs into OHC and sees a clean feed of prioritized incoming work. For a new customer message, the system has already drafted a context-aware reply. The owner can tap "Approve" to send or "Edit" to adjust.
  **Critical User Journey (CUJ):**
  1. User navigates to the Work Triage Feed.
  2. User sees an unread message card categorized as "New Lead".
  3. User taps the card to view the AI-drafted reply.
  4. User taps "Approve" which updates the `TriageItem` status to resolved and simulates sending the reply.
  **Acceptance Criteria:**
  - The UI must render correctly at 375px width.
  - The feed must display items fetched from the backend.
  - The "Approve" action must update the backend state and optimistically update the UI.

  ### Priority
  P0

  ### Estimated Scope
  Medium

  ### Visuals

  #### Competitive Landscape (Mermaid.js)
  ```mermaid
  graph TD;
      OHC[OHC: Owner Assistant] --> Traditional[Traditional CRM/Inbox];
      OHC --> OS[All-in-One OS];

      Traditional --> Shopify[Shopify Inbox];
      Traditional --> HubSpot[HubSpot Breeze];
      Traditional --> Zendesk[Zendesk AI];

      OS --> WeCom[WeCom];
      OS --> DingTalk[DingTalk];
      OS --> Lark[Feishu/Lark];

      OHCGap((OHC Goal: Consumer-simple, Proactive AI Triage));
      OHC --> OHCGap;
  ```

  ### References & Sources (50 URLs)
  1. https://work.weixin.qq.com/
  2. https://www.dingtalk.com/en
  3. https://www.larksuite.com/
  4. https://www.shopify.com/inbox
  5. https://www.hubspot.com/products/service/shared-inbox
  6. https://squareup.com/us/en/software/messages
  7. https://www.wix.com/inbox
  8. https://copilot.microsoft.com/
  9. https://www.notion.so/product/ai
  10. https://durable.co/
  11. https://www.lindy.ai/
  12. https://www.11x.ai/
  13. https://www.intercom.com/fin
  14. https://relevanceai.com/
  15. https://www.gorgias.com/
  16. https://www.zendesk.com/ai/
  17. https://devrev.ai/
  18. https://superhuman.com/
  19. https://www.bland.ai/
  20. https://www.salesforce.com/products/einstein/overview/
  21. https://www.zoho.com/zia/
  22. https://www.freshworks.com/freddy-ai/
  23. https://www.front.com/ai
  24. https://help-scout.com/ai/
  25. https://www.intercom.com/blog/ai-in-customer-service/
  26. https://www.zendesk.com/blog/ai-customer-service/
  27. https://techcrunch.com/2024/01/01/ai-native-startups/
  28. https://www.forbes.com/sites/smb-ai-trends-2025/
  29. https://www.bloomberg.com/news/articles/tencent-wecom-ai-integration
  30. https://www.cnbc.com/alibaba-dingtalk-ai-assistant
  31. https://hbr.org/2023/11/how-ai-is-changing-the-future-of-work
  32. https://www.reddit.com/r/smallbusiness/comments/inbox_overload
  33. https://www.reddit.com/r/entrepreneur/comments/managing_messages
  34. https://www.trustpilot.com/review/wecom.com
  35. https://www.trustpilot.com/review/dingtalk.com
  36. https://www.g2.com/products/lark/reviews
  37. https://www.g2.com/products/hubspot-service-hub/reviews
  38. https://www.capterra.com/p/shopify-inbox/reviews
  39. https://www.shopify.com/blog/unified-inbox
  40. https://squareup.com/us/en/townsquare/customer-communication
  41. https://www.wix.com/blog/customer-service-tools
  42. https://www.microsoft.com/en-us/worklab/work-trend-index
  43. https://durable.co/blog/smb-automation
  44. https://lindy.ai/use-cases
  45. https://11x.ai/blog/future-of-work
  46. https://relevanceai.com/blog/autonomous-agents
  47. https://gorgias.com/blog/ecommerce-customer-service
  48. https://devrev.ai/blog/support-driven-growth
  49. https://superhuman.com/blog/email-triage
  50. https://bland.ai/use-cases/small-business
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
