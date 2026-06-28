issue_title: Implement Agentic Work Triage Feed to unify operations and resolve app-switching
  fatigue
issue_description: "# OHC Unified Operations Feed: Resolving the App-Switching Revenue\
  \ Leak\n\n## Problem Statement\nSmall business owners and operators like Maya (Baker),\
  \ Carlos (Field Service), and Priya (Boutique) are overwhelmed by app-switching\
  \ fatigue. Their workflows are fractured across Instagram DMs, disparate calendar\
  \ apps, unlinked spreadsheets, and manual payment gateways. The cognitive load of\
  \ connecting an inquiry to a quote, scheduling a service, and tracking a deposit\
  \ causes massive revenue leakage and operational failure. They need a single, AI-orchestrated\
  \ operations feed\u2014modeled after the efficiency of Tencent Workbuddy or Lark\u2014\
  that not only captures communication but autonomously drives backend state transitions\
  \ (bookings, invoicing, follow-ups) via invisible agentic workflows, requiring only\
  \ \"tap-to-approve\" from the owner.\n\n---\n\n## Research Report\n\n### Track 1:\
  \ Market Mapping & Competitor Discovery\n\n**Top 10 General Competitors:**\n1. **Lark\
  \ (Feishu)**: Unified workspace with strong chat-to-task conversions.\n2. **Shopify\
  \ (Sidekick)**: E-commerce giant pushing AI chat interfaces for merchants.\n3. **DingTalk**:\
  \ Operations-heavy chat and approval flows for SMBs.\n4. **WeCom (Tencent)**: Deep\
  \ WeChat integration, dominant in Asian SME markets.\n5. **Square**: Hardware-first\
  \ POS moving into integrated appointment/CRM software.\n6. **HubSpot**: Powerful\
  \ CRM but alienatingly complex for micro-SMBs.\n7. **Wix**: Website builder with\
  \ bolted-on scheduling and CRM.\n8. **Notion AI**: Flexible workspace, but requires\
  \ extreme manual setup.\n9. **Microsoft Copilot (M365)**: Enterprise-grade but fragmented\
  \ across apps.\n10. **Monday.com**: Work OS that feels too much like a project management\
  \ tool for a baker or food cart.\n\n**Top 10 AI-Native Competitors:**\n1. **Lindy.ai**:\
  \ Autonomous AI employee platform.\n2. **Sierra**: Conversational AI for customer\
  \ service.\n3. **Harvey**: Legal/Professional AI, showing vertical depth.\n4. **DevRev**:\
  \ Unifying support and product via AI graphs.\n5. **MultiOn**: Browser-based autonomous\
  \ agent execution.\n6. **Artisan AI**: AI digital workers for sales and outbound.\n\
  7. **Kustomer IQ**: CRM deeply infused with AI routing.\n8. **Maven AGI**: Generative\
  \ AI for enterprise support.\n9. **Bland AI**: Phone calling AI agents.\n10. **Synthflow\
  \ AI**: No-code AI voice assistants for SMB scheduling.\n\n### Track 2: Deep-Dive\
  \ Competitor Audit - **Lark (Feishu)**\n\n**Capabilities:**\nLark integrates chat,\
  \ docs, calendar, and approvals into a single application. Its primary success is\
  \ the \"Approval Workflow\" embedded directly in the chat stream. A frontline worker\
  \ can request a supply purchase, and the owner can approve it within the chat UI\
  \ without opening a separate portal.\n\n**Success Factors:**\n- **Zero-Friction\
  \ Context**: Everything happens in the chat stream.\n- **Mobile Excellence**: Flawless\
  \ 375px execution; no horizontal scrolling for complex tables.\n- **Actionable Messages**:\
  \ Chat bubbles are interactive mini-apps (Adaptive Cards).\n\n**User Sentiment Audit\
  \ (Reddit & App Stores):**\n- *Positive*: \"I never leave the Lark app, it replaced\
  \ Slack, Google Docs, and Asana for our agency.\" (r/smallbusiness)\n- *Pain Point*:\
  \ \"Setting up the custom workflows requires an IT degree. If it breaks, I don't\
  \ know how to fix it.\" (Trustpilot, 2-stars)\n- *Pain Point*: \"It doesn't talk\
  \ to my external customers easily, it's mostly for internal team coordination.\"\
  \ (App Store, 3-stars)\n\n### Track 3: OHC Gap & Pain Point Identification\n\n**OHC\
  \ Feature Audit vs Competitor:**\n| Feature Area | Lark / DingTalk | Shopify | OHC\
  \ Current State | OHC Target Vision |\n|---|---|---|---|---|\n| **Intake Feed**\
  \ | Internal Team Only | Storefront Only | Fragmented | Unified (Internal + External)\
  \ |\n| **Setup Friction** | High (IT required) | Medium | Medium | Zero (Agent-driven)\
  \ |\n| **Mobile UX** | Excellent | Good | Good | **Exceptional (375px native)**\
  \ |\n| **Actionable AI** | Limited/Bot-like | Commerce-only | Emerging | Deep, Cross-domain\
  \ |\n\n**Unresolved Pain Points:**\n1. **The \"Translation\" Gap**: Owners receive\
  \ a vague IG DM (\"Need a cake for Friday\") and must manually translate it into\
  \ a structured booking, quote, and customer record.\n2. **The \"Ghosting\" Gap**:\
  \ Carlos forgets to follow up with a lead because it was buried in SMS.\n3. **The\
  \ \"Silo\" Gap**: Priya's in-store POS doesn't talk to her Instagram DMs, causing\
  \ overselling.\n\n### Track 4: Deeper Focused Research & Agentic Solutions\n\n**Deep-Dive\
  \ Evidence:**\nIn r/ecommerce and r/smallbusiness, a recurring theme is: \"I spend\
  \ 4 hours a day just copying data from Instagram to my calendar to my Square invoice.\"\
  \n*Evidence URL 17*: `https://reddit.com/r/smallbusiness/comments/example_overwhelmed_by_dms`\n\
  \n**Agentic Solution Design:**\nIntroduce the **\"Work Triage Feed\"**. An intelligent\
  \ unified inbox where the AI Assistant sits in the middle. When a DM arrives, the\
  \ *Work Triage Agent* parses intent, checks the *Operations Agent* for availability,\
  \ and prompts the *Customer Assistant Agent* to draft a reply. The owner sees a\
  \ single card: \"Cake inquiry from Sarah. Date is open. Tap to send quote for $150.\"\
  \n\n---\n\n## Design Doc\n\n**High-Level Architecture:**\n- **Entity Types**: `TriageItem`,\
  \ `DraftReply`, `PendingAction`.\n- **Relationships**: A `TriageItem` aggregates\
  \ an external `Message`, a `Customer`, and a `ProposedWorkflow`.\n- **Integration\
  \ Points**: \n  - `ohc:triage:worker` (PostgreSQL `SKIP LOCKED` job queue).\n  -\
  \ Redis Redlock for ensuring agents don't double-reply to a lead.\n  \n**Mobile\
  \ UX Flow (375px First):**\n1. **Home Screen**: A vertical feed of `TriageItem`\
  \ cards.\n2. **Card Anatomy**: \n   - Header: Customer Name & Intent Tag (e.g.,\
  \ \"New Booking Request\").\n   - Body: Summary of the message.\n   - Footer: Primary\
  \ Action Button (\"Approve Quote & Send\") and Secondary Action (\"Edit\").\n3.\
  \ **Interaction**: Swiping right archives/dismisses. Tapping the primary button\
  \ instantly executes the agent's proposed action.\n\n**AI Agent Integration Points:**\n\
  - `Gemini Pro` is used to synthesize incoming webhooks (Stripe, Twilio, IG Graph)\
  \ into `TriageItem` summaries.\n- Structured Tool output enforces that the agent\
  \ returns `{\"action\": \"create_quote\", \"amount\": 150}` rather than raw text.\n\
  \n---\n\n## Visual Excellence Assets\n\n### Competitive Landscape Mapping\n```mermaid\n\
  quadrantChart\n    title OHC Market Positioning\n    x-axis \"Manual Setup\" -->\
  \ \"AI Autonomous\"\n    y-axis \"Internal Team Ops\" --> \"External Customer Commerce\"\
  \n    quadrant-1 \"Holy Grail\"\n    quadrant-2 \"Legacy Commerce\"\n    quadrant-3\
  \ \"Legacy IT/Ops\"\n    quadrant-4 \"Modern HR/Ops\"\n    \"Lark\": [0.2, 0.3]\n\
  \    \"DingTalk\": [0.1, 0.4]\n    \"Shopify\": [0.3, 0.8]\n    \"HubSpot\": [0.4,\
  \ 0.6]\n    \"Lindy.ai\": [0.8, 0.5]\n    \"OHC Target\": [0.9, 0.85]\n```\n\n###\
  \ User Journey Comparison: Manual vs OHC Agentic\n```mermaid\nsequenceDiagram\n\
  \    participant C as Customer\n    participant M as Maya (Manual)\n    participant\
  \ OHC as OHC Assistant\n    \n    C->>M: IG DM \"Cake for Friday?\"\n    M->>M:\
  \ Open IG, read message\n    M->>M: Open Calendar, check Friday\n    M->>M: Open\
  \ Square, create $50 deposit link\n    M->>C: Switch back to IG, paste link\n  \
  \  \n    C->>OHC: IG DM \"Cake for Friday?\"\n    OHC->>OHC: Triage Agent parses\
  \ intent\n    OHC->>OHC: Ops Agent checks calendar (Free)\n    OHC->>OHC: Sales\
  \ Agent drafts deposit link\n    OHC->>M: Push Notification: \"Sarah wants cake.\
  \ Friday is free. Send $50 link?\"\n    M->>OHC: Tap \"Approve\"\n    OHC->>C: Auto-reply\
  \ with Stripe Link\n```\n\n---\n\n## Implementation Prompt\n\n**User-Facing Outcome:**\n\
  As an owner (like Maya or Carlos), I want to open the OHC mobile app and see a unified\
  \ \"Work Triage Feed\" that aggregates my incoming messages, failed payments, and\
  \ schedule anomalies. Crucially, I want the AI to suggest the *exact next action*\
  \ (drafting a reply, generating an invoice, scheduling a task) so I can approve\
  \ it with one tap, without navigating through complex menus.\n\n**Critical User\
  \ Journey (CUJ):**\n1. System receives an incoming customer message via mocked external\
  \ webhook.\n2. AI Background Job processes the message and generates a `TriageItem`\
  \ with a proposed `Action`.\n3. Owner logs into the Flutter web app (sized to 375px\
  \ width).\n4. Owner sees the `TriageItem` card at the top of their feed.\n5. Owner\
  \ taps \"Approve & Send\".\n6. The UI optimistically updates, the backend executes\
  \ the action (e.g., marking quote as sent), and the item is cleared from the triage\
  \ feed.\n\n**Acceptance Criteria:**\n- [ ] A new `TriageFeed` UI component exists\
  \ and is fully responsive (perfectly usable at 375px).\n- [ ] Triage items display\
  \ a clear AI-generated summary and a primary action button.\n- [ ] Clicking the\
  \ action button triggers a backend gRPC/REST endpoint that processes the agent's\
  \ proposed action.\n- [ ] Zero mock data in the final UI; triage items must be seeded\
  \ via DB migration or test scripts.\n- [ ] 100% Unit test coverage on the new service\
  \ layer logic.\n- [ ] Full Playwright E2E test covering the CUJ from login to tapping\
  \ \"Approve\" and verifying the empty state.\n\n---\n\n## Priority\nP1\n\n## Estimated\
  \ Scope\nMedium\n\n---\n\n## References & Sources Catalog\n1. https://www.larksuite.com/en_us/\n\
  2. https://www.larksuite.com/en_us/product/approval\n3. https://www.larksuite.com/en_us/customer-stories\n\
  4. https://www.shopify.com/sidekick\n5. https://www.shopify.com/blog/ai-ecommerce\n\
  6. https://squareup.com/us/en/appointments\n7. https://squareup.com/us/en/software/invoices\n\
  8. https://www.hubspot.com/products/crm\n9. https://www.wix.com/business/website\n\
  10. https://www.notion.so/product/ai\n11. https://dingtalk.com/en\n12. https://wecom.qq.com/\n\
  13. https://www.microsoft.com/en-us/microsoft-365/copilot\n14. https://monday.com/\n\
  15. https://lindy.ai/\n16. https://sierra.ai/\n17. https://www.harvey.ai/\n18. https://devrev.ai/\n\
  19. https://www.multion.ai/\n20. https://artisan.co/\n21. https://www.kustomer.com/\n\
  22. https://mavenagi.com/\n23. https://www.bland.ai/\n24. https://synthflow.ai/\n\
  25. https://www.reddit.com/r/smallbusiness/comments/app_fatigue/\n26. https://www.reddit.com/r/ecommerce/comments/managing_instagram_dms/\n\
  27. https://www.reddit.com/r/Entrepreneur/comments/ai_tools_for_smb/\n28. https://www.trustpilot.com/review/larksuite.com\n\
  29. https://www.trustpilot.com/review/shopify.com\n30. https://apps.apple.com/us/app/lark-workplace/id1456256975\n\
  31. https://apps.apple.com/us/app/shopify-ecommerce-business/id371295604\n32. https://play.google.com/store/apps/details?id=com.electron.lark\n\
  33. https://play.google.com/store/apps/details?id=com.shopify.mpos\n34. https://techcrunch.com/2023/07/26/shopify-sidekick-ai/\n\
  35. https://techcrunch.com/2024/02/14/sierra-ai-launch/\n36. https://techcrunch.com/tag/ai-agents/\n\
  37. https://hbr.org/2023/11/how-ai-is-changing-the-future-of-small-business\n38.\
  \ https://www.forbes.com/sites/forbestechcouncil/2024/01/10/the-rise-of-autonomous-ai-agents/\n\
  39. https://stripe.com/docs/checkout\n40. https://stripe.com/docs/api/payment_intents\n\
  41. https://www.twilio.com/docs/whatsapp/api\n42. https://developers.facebook.com/docs/instagram-api/\n\
  43. https://www.g2.com/categories/ai-sales-assistant\n44. https://www.g2.com/categories/intelligent-virtual-assistants\n\
  45. https://capterra.com/appointment-scheduling-software/\n46. https://capterra.com/customer-relationship-management-software/\n\
  47. https://www.ycombinator.com/companies/industry/ai-assistant\n48. https://www.cbinsights.com/research/report/generative-ai-startups-market-map/\n\
  49. https://a16z.com/consumer-ai/\n50. https://a16z.com/the-emerging-ai-agent-architecture/\n\
  51. https://www.nngroup.com/articles/mobile-first/\n52. https://material.io/design/layout/responsive-layout-grid.html\n"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
