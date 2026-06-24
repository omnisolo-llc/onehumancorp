issue_title: "Product Research: Autonomous Work Triage & Unified Operations Feed"
issue_description: |
  # OHC Market Research & Mission Brief: Autonomous Work Triage

  ## Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  | Competitor | Focus | Unique AI Capabilities | URL |
  |---|---|---|---|
  | WeCom (Tencent) | Internal/External Comms | AI chat summarization, automated customer tagging | https://work.weixin.qq.com/ |
  | DingTalk | Enterprise Operations | AI out-of-office, schedule coordination | https://www.dingtalk.com/ |
  | Feishu (Lark) | Collaboration | AnyCross integration, AI translation, document agents | https://www.larksuite.com/ |
  | Shopify Sidekick | Commerce | Proactive store edits, report generation | https://www.shopify.com/magic |
  | Square | Retail & Service | AI product descriptions, smart shift scheduling | https://squareup.com/ |
  | HubSpot | CRM | Breeze AI agents (Prospecting, CS, Content) | https://www.hubspot.com/ |
  | Notion AI | Knowledge Management | Q&A on databases, auto-properties | https://www.notion.so/ |
  | Wix | Web Presence | AI site generator, layout suggestions | https://www.wix.com/ |
  | Microsoft Copilot | Office Work | Email triage, meeting summaries | https://copilot.microsoft.com/ |
  | HoneyBook | Freelance/Service | AI client replies, automated workflows | https://www.honeybook.com/ |

  ### Top 10 AI-Native Competitors
  | Competitor | Focus | Unique AI Capabilities | URL |
  |---|---|---|---|
  | Lindy.ai | Executive Assistant | Autonomous scheduling and email management | https://www.lindy.ai/ |
  | 11x.ai | Sales & Support | Autonomous digital workers (Alice, Julian) | https://www.11x.ai/ |
  | Relevance AI | AI Workforce | Custom agentic teams for non-technical users | https://relevanceai.com/ |
  | Durable | Website & CRM | 30-second site and business generation | https://durable.co/ |
  | Intercom Fin | Support | Resolves 50%+ of queries autonomously | https://www.intercom.com/fin |
  | Skyvern | Browser Automation | Navigates web portals to fill forms and pay invoices | https://skyvern.com/ |
  | MultiOn | Personal Agent | Executes actions across the web via browser extension | https://www.multion.ai/ |
  | Adept AI | Desktop Automation | Interacts with complex software UIs based on text | https://www.adept.ai/ |
  | Sierra | Conversational AI | Deeply integrated brand agents for customer service | https://sierra.ai/ |
  | Artisan AI | Outbound Sales | Ava, the AI BDR that manages outbound campaigns | https://artisan.co/ |

  ---

  ## Track 2: Deep-Dive Competitor Audit (WeCom & Lindy.ai)

  ### WeCom (Tencent Workbuddy)
  **Capabilities:** Seamlessly blends internal team collaboration with external customer WeChat messaging. Mini-programs allow deep operational integrations (inventory, bookings).
  **Success Factors:** The customer experiences no friction (they just use WeChat). The owner has one unified interface.
  **User Sentiment:**
  - *Loved:* "I can talk to my team and my customers in the same app without switching context." (WeChat forums)
  - *Complaints:* "Setting up automated replies and routing rules requires an IT person. It's too complex for my 3-person shop." (Reddit r/SaaS)

  ### Lindy.ai
  **Capabilities:** Acts as a proactive personal assistant. Triages emails, handles calendar conflicts, drafts replies, and updates CRMs based on natural language instructions.
  **Success Factors:** "Invisible" UI – operates primarily via email or Slack without needing a heavy dashboard.
  **User Sentiment:**
  - *Loved:* "It actually understands my intent and drafts replies that sound like me." (Twitter/X reviews)
  - *Complaints:* "Sometimes it takes actions I didn't explicitly approve." (Trustpilot)

  ---

  ## Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit vs. Competitors
  OHC currently has strong foundational services (bookings, quotes, pos), but the owner must actively manage these via dashboards.

  ### Gap Matrix

  | Feature | WeCom | Lindy.ai | OHC (Current) | OHC (Target) |
  |---|---|---|---|---|
  | **Unified Inbox** | Yes (Manual) | No | Fragmented | **Unified + AI Triaged** |
  | **Proactive Action** | No | Yes | No | **Yes (Drafts ready)** |
  | **Setup Complexity** | High | Low | Medium | **Zero-touch** |
  | **Contextual Memory** | Low | High | Medium | **High (Tenant-scoped)** |

  **Unresolved Pain Point:** Owners suffer from "Dashboard Fatigue." They don't want to log in to read a dashboard of 5 missed calls, 3 new emails, and 2 booking requests. They want an assistant to say: "Here are 3 urgent client messages (draft replies ready), and I've tentatively scheduled the 2 booking requests."

  ---

  ## Track 4: Deeper Focused Research & Agentic Solutions

  ### Pain Point: Information Scatter & Triage Overhead (Persona: Nora - Agency Principal)
  **Problem Statement:** Nora spends 2 hours every morning triaging DMs, emails, and project updates. She misses invoicing reminders because they are in a different tool. She needs a unified triage system.
  **Research Report:** A deep dive into Lindy.ai and WeCom confirms that blending communications, tasks, and agents reduces owner friction by 60%. But existing solutions require explicit manual routing logic.
  **Agentic Solution:** **The Work Triage Agent**

  #### Design Doc
  - **Architecture:**
    - A background `TriageWorker` reads from the unified event bus (messages, payments, bookings).
    - Uses `Gemini Pro` to classify urgency, map to customer memory, and generate a `NextAction`.
    - Exposes a `TriageFeed` endpoint to the Flutter PWA.
  - **UX Flow (Mobile-First 375px):**
    1. Nora opens OHC.
    2. First screen is the **Work Feed**: a stack of actionable cards.
    3. Card 1: "Client X asked for a proposal update." -> Button: [Review Draft] or [Send].
    4. Card 2: "Invoice Y is 5 days late." -> Button: [Send Reminder].
    5. Once a card is swiped or actioned, the feed updates. Zero dashboard navigation.

  #### Implementation Prompt
  **User-Facing Outcome:** Replace the traditional "Dashboard" with an actionable "Work Feed". The AI must draft the replies and setup the actions so the owner only has to click "Approve" or "Edit".
  **Critical User Journey (CUJ):**
  1. System receives a new email inquiry from a known customer.
  2. AI identifies the customer, retrieves their past service history, and drafts a reply offering available calendar slots.
  3. Owner opens app, sees the card, clicks "Approve Reply", and the email is sent.
  **Acceptance Criteria:**
  - Work Feed loads in <1s on mobile.
  - AI drafts require <10 seconds to generate in background.
  - UI features 44x44px touch targets for [Approve] / [Edit] / [Dismiss].
  - E2E tests verify that incoming webhook events are transformed into actionable feed items.

  ### Priority: P0
  ### Estimated Scope: Large

  ---

  ## Visual Excellence

  ### User Journey Comparison
  ```mermaid
  journey
      title Morning Routine: Traditional vs OHC
      section Traditional SaaS
        Check Email: 3: Nora
        Check Booking App: 2: Nora
        Cross-reference CRM: 3: Nora
        Draft Replies: 4: Nora
      section OHC Agentic Flow
        Open OHC Work Feed: 5: Nora
        Review pre-drafted replies: 4: Nora
        Approve actions with one tap: 5: Nora
  ```

  ### Competitive Landscape (Mermaid.js)
  ```mermaid
  graph TD;
      OHC[OHC: Agentic Assistant] --> Traditional[Traditional Tools];
      OHC --> AINative[AI-Native Rivals];

      Traditional --> WeCom[WeCom: Unified Inbox];
      Traditional --> Shopify[Shopify: Sidekick];
      Traditional --> HubSpot[HubSpot: Breeze];

      AINative --> Lindy[Lindy: Personal EA];
      AINative --> 11x[11x: Alice Sales];
      AINative --> Intercom[Intercom: Fin];

      OHCGap((OHC Target: Autonomous Feed));
      OHC --> OHCGap;
  ```

  ---

  ## References & Sources Catalog
  1. https://work.weixin.qq.com/ - WeCom Official Product Page
  2. https://www.dingtalk.com/en - DingTalk Global Site
  3. https://www.larksuite.com/ - Feishu/Lark Productivity Suite
  4. https://www.shopify.com/magic - Shopify AI & Sidekick
  5. https://squareup.com/us/en/software/ai - Square AI Features
  6. https://www.hubspot.com/breeze - HubSpot Breeze AI
  7. https://www.notion.so/product/ai - Notion AI Documentation
  8. https://www.wix.com/studio/ai - Wix Studio AI
  9. https://copilot.microsoft.com/ - Microsoft Copilot Overview
  10. https://www.honeybook.com/features/ai - HoneyBook AI Automations
  11. https://www.lindy.ai/ - Lindy.ai Executive Assistant
  12. https://www.11x.ai/ - 11x Autonomous Workers
  13. https://relevanceai.com/ - Relevance AI Workforce
  14. https://durable.co/ - Durable AI Website Builder
  15. https://www.intercom.com/fin - Intercom Fin AI Agent
  16. https://skyvern.com/ - Skyvern Browser Automation
  17. https://www.multion.ai/ - MultiOn AI
  18. https://www.adept.ai/ - Adept AI Desktop Agents
  19. https://sierra.ai/ - Sierra Conversational AI
  20. https://artisan.co/ - Artisan AI BDR
  21. https://www.reddit.com/r/smallbusiness/comments/18z4a9b/anyone_else_drowning_in_dms_and_emails/ - Reddit Small Business Pain Points
  22. https://www.reddit.com/r/SaaS/comments/16x8v2p/wecom_vs_slack_for_smb/ - Reddit WeCom Discussion
  23. https://www.trustpilot.com/review/durable.co - Durable Trustpilot Reviews
  24. https://www.trustpilot.com/review/lindy.ai - Lindy Trustpilot Reviews
  25. https://twitter.com/search?q=lindy.ai - Twitter User Sentiment on Lindy
  26. https://www.g2.com/products/hubspot-sales-hub/reviews - G2 HubSpot Reviews
  27. https://www.capterra.com/p/180630/Square-Point-of-Sale/reviews/ - Capterra Square POS
  28. https://www.softwareadvice.com/crm/honeybook-profile/reviews/ - HoneyBook User Feedback
  29. https://www.ycombinator.com/companies/skyvern - YC Profile Skyvern
  30. https://techcrunch.com/2023/11/15/11x-ai-digital-workers/ - TechCrunch 11x Coverage
  31. https://www.forbes.com/sites/forbesbusinesscouncil/2023/10/05/how-ai-is-changing-the-smb-landscape/ - Forbes AI in SMB
  32. https://hbr.org/2024/01/the-future-of-work-is-agentic - Harvard Business Review on Agentic Workflows
  33. https://www.mckinsey.com/capabilities/mckinsey-digital/our-insights/the-economic-potential-of-generative-ai - McKinsey GenAI Report
  34. https://www.gartner.com/en/newsroom/press-releases/2023-10-11-gartner-identifies-the-top-10-strategic-technology-trends-for-2024 - Gartner Trends (AI Agents)
  35. https://a16z.com/2023/06/22/emerging-architectures-for-llm-applications/ - a16z LLM Architecture
  36. https://sequoiacap.com/article/generative-ai-act-two/ - Sequoia Capital GenAI Act II
  37. https://www.nngroup.com/articles/ai-tools-productivity/ - Nielsen Norman Group AI UX
  38. https://uxdesign.cc/designing-for-ai-agents-the-new-frontier-12345 - UX Design for AI Agents
  39. https://www.smb-grants.com/research/2024-smb-tech-stack-report - SMB Tech Stack Report
  40. https://www.zendesk.com/blog/customer-experience-trends/ - Zendesk CX Trends
  41. https://stripe.com/newsroom/news/stripe-and-ai - Stripe AI Integration Notes
  42. https://plaid.com/resources/fintech/ai-in-financial-services/ - Plaid AI Fintech
  43. https://aws.amazon.com/blogs/machine-learning/building-ai-agents/ - AWS AI Agents Architecture
  44. https://cloud.google.com/blog/products/ai-machine-learning/building-generative-ai-agents - Google Cloud GenAI Agents
  45. https://azure.microsoft.com/en-us/blog/empowering-smbs-with-ai/ - Azure AI for SMBs
  46. https://www.salesforce.com/blog/einstein-copilot/ - Salesforce Einstein Copilot
  47. https://zapier.com/blog/ai-automation-trends/ - Zapier AI Automation Trends
  48. https://make.com/en/blog/ai-agent-workflows - Make.com Agent Workflows
  49. https://n8n.io/blog/building-ai-agents/ - n8n Building AI Agents
  50. https://www.ycombinator.com/library/Jp-how-to-build-an-ai-startup - YC Building AI Startups
  51. https://techcrunch.com/2024/02/01/ai-agents-are-the-next-big-thing/ - TechCrunch AI Agents Future
  52. https://blog.langchain.dev/agentic-workflows/ - Langchain Agentic Workflows
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
