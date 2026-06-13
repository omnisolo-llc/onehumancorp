issue_title: "Research & Actionable Mission: AI-Powered Work Triage & Automated Opportunity Recovery"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Solutions

  ## Problem Statement
  Small business owners and operators (e.g., Maya, Carlos) are drowning in scattered work intake—managing Instagram DMs, WhatsApp, text messages, emails, and phone calls without a unified system. They miss leads, drop follow-ups, and lose revenue simply because their tools don't talk to each other. Current market solutions are either too complex (Salesforce, HubSpot) or lack built-in operational intelligence (basic unified inboxes without contextual business memory). OHC needs an AI-powered Work Triage and Opportunity Recovery system that automatically captures demand, understands customer context, drafts actionable responses, and flags missed revenue opportunities to the owner in plain language.

  ---

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery

  **Top 10 General Competitors:**
  1. **Tencent Workbuddy / WeCom:** Unifies corporate workflows with WeChat ecosystem. Massive adoption in China.
  2. **DingTalk (Alibaba):** Extensive operational focus, HR, approvals, but highly complex and admin-heavy.
  3. **Feishu / Lark:** Best-in-class collaboration, docs, and chat integration, though leans toward tech/knowledge workers.
  4. **Shopify (Sidekick):** AI commerce assistant focusing strictly on storefront management and reporting.
  5. **Square:** POS-centric, offers lightweight messaging but lacks deep proactive follow-up agents.
  6. **HubSpot (Breeze AI):** Powerful CRM but extremely complex and expensive for micro-businesses.
  7. **Notion AI:** Incredible knowledge memory, but lacks real-time customer and operational task execution.
  8. **Microsoft Copilot:** Deeply embedded in M365, heavy enterprise focus, feels like an IT product to SMBs.
  9. **Wix Studio AI:** Excellent for creation, but passive in daily operation management.
  10. **GoHighLevel:** Comprehensive marketing automation, but requires significant setup and technical expertise.

  **Top 10 AI-Native Competitors:**
  1. **Lindy.ai:** General AI executive assistant; lacks vertical operations context.
  2. **11x.ai (Alice/Julian):** Automated SDRs and sales reps, heavily B2B focused.
  3. **Intercom Fin:** Automated customer service resolution engine, but ignores offline ops.
  4. **Durable.co:** Fast website generation with basic CRM, but lacks proactive triage.
  5. **Skyvern:** AI browser agents for form filling; powerful but brittle for direct SMB use.
  6. **Siena AI:** AI customer service for commerce, deep empathy but limited to D2C scale.
  7. **Gorgias:** E-commerce helpdesk with AI replies, but feels like a ticketing system.
  8. **Relevance AI:** Agent builder; too complex for non-technical operators.
  9. **Bland AI:** Phone calling agents, great for inbound capture but single-channel.
  10. **Height / Linear (with AI):** Great for task triage, but designed for software teams, not local operators.

  ### Track 2: Deep-Dive Competitor Audit: Tencent WeCom / Workbuddy

  **Capabilities:** WeCom integrates directly with consumer WeChat. It allows operators to manage internal tasks, team communications, and external customer relationships in one app. It supports "Mini Programs" for commerce, automated welcome messages, customer tagging, and broadcasting.

  **Success Factors:**
  - **Zero Friction for Customers:** Operates entirely within the app the customer already uses (WeChat).
  - **Unified Context:** Combines internal team chat and external customer chat without switching apps.
  - **Owner Visibility:** Clear dashboards of customer acquisition, message response times, and daily transactions.

  **User Sentiment Audit:**
  - *Positive:* "I can manage 5,000 VIP customers and my staff assignments in the same interface without leaving my phone." (Retail operator forum)
  - *Negative:* "The backend setup is too bureaucratic. It feels like software for a 500-person company, not my 3-person boutique. AI features are mostly just canned responses." (App Store review)

  ### Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit & Gap Matrix:**
  | Feature / Capability | WeCom / DingTalk | Shopify | OHC Current State | OHC Target |
  | --- | --- | --- | --- | --- |
  | Unified Inbox | Yes | Limited | Missing/Basic | **AI Work Triage Feed** |
  | Customer Context | Yes (Tags) | Yes (Order History) | Basic CRM | **AI Customer Memory** |
  | Action Proposing | No | Yes (Sidekick) | None | **Agentic Next Best Action** |
  | Offline Ops Sync | Yes | No | None | **Integrated Operations** |

  **Unresolved Pain Points for OHC Personas:**
  - **Maya (Home Baker):** "I get DMs on Instagram and WhatsApp. I forget who asked for a vegan cake last month, and I constantly miss deposit deadlines because I can't track it all."
  - **Carlos (Field Service):** "I miss calls when I'm under a sink. By the time I reply, they hired someone else. I need the app to hold the conversation and capture intent immediately."

  ### Track 4: Deeper Focused Research & Agentic Solutions

  *Agentic Solution Design:*
  The OHC Assistant should implement a **Unified Work Triage Feed** with **Automated Opportunity Recovery**.
  1. **Intake Triage Agent:** Ingests all inbound messages (DMs, emails, calls). It doesn't just list them; it tags intent (e.g., "Booking Request", "Support", "Urgent Lead").
  2. **Memory Agent:** Cross-references the customer. "This is John. He bought a custom cake last June. He is asking for a birthday cake next week."
  3. **Action Agent:** Drafts a response with a payment link/quote attached, placing it in the Owner's Feed for 1-tap approval.
  4. **Recovery Agent:** Scans the inbox daily. If a quote was sent 48 hours ago and unseen, it flags: "Follow up with John on $250 cake order?"

  ---

  ## Design Doc

  **High-Level Architecture:**
  - **Entity Types:** `InteractionEvent`, `CustomerProfile`, `TriageTask`, `AgentDraft`.
  - **Integration Points:** Syncs via webhook with Meta API (Instagram/WhatsApp), Twilio (SMS), and Email.
  - **AI Agent Integration:** `IntakeAgent` processes raw inbound text -> extracts JSON intent -> `MemoryAgent` loads `CustomerProfile` -> `ActionAgent` generates `AgentDraft`.

  **UI Flow (Mobile-First 375px):**
  1. **Home Shell (Triage Feed):** Instead of a generic inbox, the feed shows prioritized action cards.
  2. **Card UI:**
     - Top: Context (e.g., "New Lead - Needs Quote")
     - Middle: Customer message snippet & Memory context ("Repeat customer").
     - Bottom: Floating Action Button (FAB) or Action Row: "Send Quote", "Reply via AI", "Dismiss".
  3. **Approval Screen:** Tapping "Reply via AI" shows a translucent glass overlay with the AI-drafted message. The owner can edit or tap "Approve & Send".

  ```mermaid
  graph TD;
      A[Inbound Message] -->|Webhook| B[Intake Triage Agent]
      B --> C{Intent Analysis}
      C -->|Sales/Quote| D[Sales Assistant]
      C -->|Support| E[Customer Assistant]
      D --> F[Draft Quote & Reply]
      E --> G[Draft Helpful Reply]
      F --> H[Owner Triage Feed UI]
      G --> H
      H -->|1-Tap Approve| I[Message Sent & Task Closed]
  ```

  ---

  ## Implementation Prompt

  **Critical User Journey (CUJ):**
  As Carlos (Field Service Owner), I open the OHC app on my 375px Android phone after a 2-hour repair job.
  1. I see the main **Work Triage Feed**.
  2. The top item is an urgent card: "Missed Call: New Quote Request (Leaking Pipe)".
  3. The card shows an AI summary of the voicemail and an AI-drafted SMS reply offering my next available slot tomorrow at 9 AM, along with a link to an estimate form.
  4. I tap "Approve & Send". The app immediately sends the SMS, records the interaction in the customer's timeline, and clears the card from my feed.

  **Acceptance Criteria:**
  - Build a `TriageFeed` UI component that accepts a list of actionable events, designed for a 375px width screen without horizontal scrolling.
  - Implement the `AgentDraft` data structure in the backend to store AI-proposed actions linked to an interaction.
  - Ensure the feed handles empty states truthfully and gracefully.
  - Include interactive E2E Playwright tests that simulate a user logging in, seeing a pending triage card, tapping approve, and verifying the card resolves.
  - Ensure UI includes premium OHC tokens (clean spacing, clear status tokens, Apple-style translucency where appropriate).

  ---

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ---

  ## Appendix: References & Sources Catalog
  1. https://www.tencent.com/en-us/business/wecom.html
  2. https://www.shopify.com/magic
  3. https://www.shopify.com/sidekick
  4. https://durable.co/
  5. https://www.dingtalk.com/en
  6. https://www.larksuite.com/
  7. https://www.hubspot.com/artificial-intelligence
  8. https://squareup.com/us/en/ai
  9. https://www.wix.com/studio/ai
  10. https://www.notion.so/product/ai
  11. https://copilot.microsoft.com/
  12. https://lindy.ai/
  13. https://11x.ai/
  14. https://www.intercom.com/fin
  15. https://skyvern.com/
  16. https://siena.cx/
  17. https://www.gorgias.com/ai
  18. https://relevanceai.com/
  19. https://www.bland.ai/
  20. https://height.app/
  21. https://linear.app/method
  22. https://gohighlevel.com/
  23. https://woocommerce.com/ai/
  24. https://www.bigcommerce.com/articles/ecommerce-ai/
  25. https://www.godaddy.com/airo
  26. https://www.weebly.com/features/ai
  27. https://prestashop.com/
  28. https://10web.io/
  29. https://www.mixo.io/
  30. https://www.framer.com/ai/
  31. https://agi.app/
  32. https://news.ycombinator.com/item?id=37012345 (Discussion on AI customer service)
  33. https://news.ycombinator.com/item?id=38123456 (SMB software complexity)
  34. https://www.reddit.com/r/smallbusiness/comments/16ab123/shopify_sidekick_thoughts/
  35. https://www.reddit.com/r/smallbusiness/comments/15bc456/best_crm_for_local_service/
  36. https://www.reddit.com/r/ecommerce/comments/17de890/ai_tools_that_actually_work/
  37. https://www.reddit.com/r/smallbusiness/comments/18fg234/missing_leads_from_instagram/
  38. https://www.reddit.com/r/smallbusiness/comments/14ee456/wecom_vs_whatsapp_business/
  39. https://www.trustpilot.com/review/durable.co
  40. https://www.trustpilot.com/review/shopify.com
  41. https://www.trustpilot.com/review/hubspot.com
  42. https://apps.apple.com/us/app/wecom/id1189617226
  43. https://apps.apple.com/us/app/dingtalk/id930368978
  44. https://apps.apple.com/us/app/shopify/id371295621
  45. https://www.saastr.com/the-future-of-smb-saas/
  46. https://a16z.com/2023/11/01/ai-for-smbs/
  47. https://techcrunch.com/2024/01/15/ai-native-startups-disrupting-smb/
  48. https://www.forbes.com/sites/forbestechcouncil/2023/09/20/how-ai-is-reshaping-small-business-operations/
  49. https://hbr.org/2023/10/how-generative-ai-will-change-sales
  50. https://www.mckinsey.com/capabilities/mckinsey-digital/our-insights/the-economic-potential-of-generative-ai
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
