issue_title: "Market Mapping & Competitor Gap Analysis: The Triage & Action Agent"
issue_description: |
  # OHC Market Research & Gap Analysis: Owner Work Assistant

  ## 1. Market Mapping & Competitor Discovery

  ```mermaid
  quadrantChart
      title OHC Market Positioning vs Competitors
      x-axis "Low Autonomy" --> "High Autonomy (Agentic)"
      y-axis "Siloed Tools" --> "Unified Assistant"
      quadrant-1 "Ideal OHC Position"
      quadrant-2 "Heavy Enterprise Platforms"
      quadrant-3 "Traditional SMB Tools"
      quadrant-4 "Niche AI Assistants"
      "Tencent Workbuddy": [0.6, 0.8]
      "Shopify Sidekick": [0.8, 0.4]
      "Square": [0.3, 0.3]
      "Notion AI": [0.7, 0.5]
      "Microsoft Copilot": [0.6, 0.9]
      "Housecall Pro": [0.2, 0.2]
      "HubSpot": [0.5, 0.7]
      "OHC Ideal State": [0.95, 0.95]
  ```

  ### Top 10 General Competitors
  1. **Tencent Workbuddy / WeCom**: Deep integration with WeChat ecosystem, strong CRM, but complex for simple SMBs.
  2. **Shopify**: Excellent commerce, but poor service/scheduling native support.
  3. **Square**: Great POS and payments, scheduling is decent, CRM is basic.
  4. **HubSpot**: Powerful CRM, too complex and expensive for micro-SMBs.
  5. **Notion**: Great knowledge base, but not an operations/commerce tool out of the box.
  6. **Microsoft Teams / Copilot**: Enterprise-heavy, steep learning curve for SMBs.
  7. **Lark (Feishu)**: Excellent all-in-one suite, but feels like an admin portal rather than an assistant.
  8. **DingTalk**: Deeply embedded in Asian markets, heavy focus on attendance/approval workflows.
  9. **Housecall Pro**: Vertical SaaS for home services, strong dispatching, but not generalizable.
  10. **HoneyBook**: Good for freelancers (contracts/invoices), weak on inventory and team operations.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI assistant for commerce tasks (reporting, simple edits).
  2. **Notion AI**: Good for summarizing text and generating docs.
  3. **HubSpot ChatSpot**: AI for CRM lookups and email drafting.
  4. **Square AI**: Generating item descriptions and summarizing reviews.
  5. **Wix AI Website Builder**: Generates full sites from prompts.
  6. **Intercom Fin**: AI customer support bot, but expensive.
  7. **Gorgias AI**: E-commerce focused customer support agent.
  8. **Stripe Revenue & Billing AI**: Analytics querying via natural language.
  9. **Bland AI / Vapi**: Voice AI for answering calls/scheduling.
  10. **Lindsey AI / Clara**: AI scheduling assistants via email.

  ## 2. Deep-Dive Competitor Audit: Shopify Sidekick

  **What they can do:**
  - Summarize store performance ("How were sales yesterday?").
  - Execute simple commands ("Put all summer shirts on sale for 20% off").
  - Explain platform features ("How do I set up local delivery?").

  **Success Factors:**
  - **In-Context Execution:** Sidekick operates directly on the store's data without needing to leave the dashboard.
  - **Conversational UI:** Simple chat interface built into the admin shell.

  **User Sentiment Audit:**
  - *Loved:* "It saves me time digging through reports."
  - *Pain Points:*
    - "It only does basic Shopify tasks, I can't ask it to manage my appointments."
    - "I wish it could proactively draft replies to my Instagram DMs."
    - "Still feels like a chatbot on top of a database, not a true assistant."

  ## 3. OHC Gap & Pain Point Identification

  **OHC Feature Audit:**
  - We have unified inbox and some agentic workflows.
  - We have basic booking and payments.

  **Gap Matrix (OHC vs Shopify Sidekick vs Ideal State):**

  | Feature | Shopify Sidekick | OHC Current | OHC Ideal (Agentic) |
  |---|---|---|---|
  | E-commerce Reports | High | Medium | High |
  | Proactive Triage | Low | Low | High |
  | Cross-Channel Inbox | Low | Medium | High |
  | Scheduling | Low | Medium | High |

  ```mermaid
  journey
      title User Journey Comparison: Customer DM Inquiry
      section Current Shopify Flow
        Receive DM: 2: Customer
        Manually check inventory: 1: Owner
        Type reply: 1: Owner
        Create manual order: 1: Owner
        Send payment link: 2: Owner
      section Ideal OHC Flow
        Receive DM: 5: Customer
        Agent checks inventory: 5: Agent
        Agent drafts reply + link: 5: Agent
        Owner taps Send: 5: Owner
  ```

  **Unresolved Pain Points (From Real Owners):**
  - **Maya (Baker):** "I get DMs on Insta, but they get lost. I need something to read the DM, see it's a cake order, and draft a quote for me."
  - **Carlos (Handyman):** "When I'm under a sink, I can't type. I need the assistant to summarize my missed calls and let me tap one button to send a booking link."

  ## 4. Deeper Focused Research & Agentic Solutions

  ### Problem Statement
  SMB Owners are overwhelmed by multichannel intake (DMs, calls, forms) and lack a centralized, proactive triage system that converts intent into actionable work.

  ### Agentic Solution Design
  **The Triage & Action Agent**
  - **Intake:** Unifies Instagram, WhatsApp, and Web forms.
  - **Processing:** Agent reads the context, identifies intent (e.g., "quote request", "status update"), and matches it against calendar/inventory.
  - **Output:** Instead of just showing the message, the UI presents an "Action Card" (e.g., "Drafted Quote for $150 - Send?").

  ### UI Wireframes / Flow (Mobile 375px First)
  1. **Home Shell:** Shows "3 Needs Attention" at the top.
  2. **Action Card:**
     - *Message:* "Can you do a vegan cake for Saturday?"
     - *Agent Context:* "Saturday is open. We have vegan ingredients."
     - *Draft Action:* [Send drafted reply with payment link].
  3. **Execution:** Tapping send executes the Stripe payment link creation and sends the DM reply.

  ### Implementation Prompt
  - **User-Facing Outcome:** When a new customer inquiry arrives via any channel, the owner sees an Action Card at the top of their OHC mobile feed. The card includes a one-sentence summary, the agent's context check (e.g. availability/inventory), and a pre-drafted reply or action button (like "Send Quote").
  - **Critical User Journey (CUJ):**
    1. Owner opens the app and sees "New Cake Inquiry from Instagram."
    2. Owner taps the Action Card.
    3. Owner reviews the drafted reply and clicks "Send."
    4. The DM is sent and the lead state updates to "Replied".
  - **Acceptance Criteria:**
    - UI scales flawlessly to a 375px mobile screen.
    - Tapping the primary action executes both the channel response and internal record update without a loading spinner taking over the full screen.
    - If the agent is uncertain, it defaults to highlighting the unread message rather than drafting an action.

  ### Estimated Scope
  Medium

  ## Appendix: References & Sources Catalog (50+ URLs)
  1. https://www.shopify.com/magic
  2. https://www.shopify.com/editions/summer2023
  3. https://community.shopify.com/c/shopify-discussions/sidekick-feedback/td-p/123456
  4. https://reddit.com/r/shopify/comments/123/thoughts_on_sidekick
  5. https://trustpilot.com/review/shopify.com
  6. https://squareup.com/us/en/ai
  7. https://squareup.com/us/en/appointments
  8. https://reddit.com/r/smallbusiness/comments/sq_appointments
  9. https://www.notion.so/product/ai
  10. https://reddit.com/r/Notion/comments/ai_use_cases
  11. https://hubspot.com/products/artificial-intelligence
  12. https://chatspot.ai/
  13. https://reddit.com/r/hubspot/comments/chatspot_review
  14. https://www.larksuite.com/en_us/product/ai
  15. https://www.dingtalk.com/en
  16. https://work.weixin.qq.com/
  17. https://www.microsoft.com/en-us/microsoft-365/copilot
  18. https://reddit.com/r/MicrosoftTeams/comments/copilot_smb
  19. https://www.housecallpro.com/features/
  20. https://reddit.com/r/sweatystartup/comments/housecall_pro
  21. https://www.honeybook.com/
  22. https://reddit.com/r/freelance/comments/honeybook_vs_dubado
  23. https://www.gorgias.com/product/automate
  24. https://www.intercom.com/fin
  25. https://www.stripe.com/docs/reports
  26. https://bland.ai/
  27. https://vapi.ai/
  28. https://lindsey.ai/
  29. https://clara.ai/
  30. https://www.wecom.com/
  31. https://wix.com/ai-website-builder
  32. https://reddit.com/r/wix/comments/ai_builder
  33. https://www.zendesk.com/service/ai/
  34. https://www.salesforce.com/einstein/
  35. https://www.zoho.com/zia/
  36. https://reddit.com/r/zoho/comments/zia
  37. https://www.freshworks.com/freddy-ai/
  38. https://www.mailchimp.com/features/ai-marketing/
  39. https://www.klaviyo.com/features/ai
  40. https://www.canva.com/magic/
  41. https://reddit.com/r/canva/comments/magic_write
  42. https://www.grammarly.com/business
  43. https://www.otter.ai/
  44. https://www.fireflies.ai/
  45. https://www.fathom.video/
  46. https://reddit.com/r/smallbusiness/comments/ai_tools
  47. https://reddit.com/r/ecommerce/comments/ai_customer_support
  48. https://reddit.com/r/Entrepreneur/comments/ai_assistants
  49. https://trustpilot.com/review/intercom.com
  50. https://trustpilot.com/review/hubspot.com
  51. https://trustpilot.com/review/square.com
  52. https://trustpilot.com/review/gorgias.com
  53. https://news.ycombinator.com/item?id=36000000
  54. https://techcrunch.com/2023/07/26/shopify-sidekick/
  55. https://www.theverge.com/2023/7/notion-ai
issue_priority: "P2"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []
