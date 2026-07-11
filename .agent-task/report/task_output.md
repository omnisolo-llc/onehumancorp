issue_title: "Implement Agentic Unified Work Triage Feed for Mobile Owners"
issue_description: |

  # OHC Owner Work Assistant: Competitive Research & Agentic Solutions

  ## Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **WeCom (Tencent):** Deeply integrated with WeChat for customer communication and internal operations.
  2. **DingTalk (Alibaba):** Comprehensive organization management, scheduling, and approval workflows.
  3. **Feishu / Lark (ByteDance):** Document-driven collaboration, OKRs, and unified communication.
  4. **Shopify:** E-commerce titan offering POS, inventory management, and marketing tools.
  5. **Square (Block):** Payment-first ecosystem with booking, loyalty, and team management.
  6. **HubSpot:** Powerful CRM and marketing automation for mid-market and SMBs.
  7. **Wix:** Website builder with integrated booking, CRM, and basic e-commerce capabilities.
  8. **Notion:** Highly flexible workspace and knowledge management tool.
  9. **Microsoft 365 / Copilot:** Ubiquitous productivity suite heavily investing in AI integration.
  10. **ServiceTitan:** Vertical SaaS giant specifically for home service businesses.

  ### Top 10 AI-Native Competitors & Assistants
  1. **Shopify Sidekick:** Conversational AI commerce assistant for store owners.
  2. **Notion AI:** Seamless knowledge retrieval and content generation within workspaces.
  3. **Microsoft Copilot for Microsoft 365:** AI deeply integrated into emails, docs, and meetings.
  4. **HubSpot ChatSpot:** AI assistant connecting CRM data with generative AI capabilities.
  5. **Lindsey / Auto-GPT based Agents:** Experimental autonomous agents for task execution.
  6. **Glean:** Enterprise AI search and knowledge discovery.
  7. **Harvey AI:** Specialized AI for legal and professional services document review.
  8. **Sana:** AI learning and knowledge platform.
  9. **Julius AI:** Data analysis and visualization assistant.
  10. **Lindy.ai:** AI scheduling and personal workflow automation assistant.

  ---

  ## Track 2: Deep-Dive Competitor Audit - Shopify Sidekick [Source 3]

  **Why Shopify Sidekick?** Shopify is the gold standard for SMB commerce, and Sidekick represents the first major push into conversational, agentic UI for non-technical merchants.

  ### Capabilities ("What they can do")
  - **Conversational Insights:** "Why are my sales down this week?" [Source 3]
  - **Task Execution:** "Create a discount code for 20% off all winter apparel." [Source 3]
  - **Content Generation:** Drafting blog posts and product descriptions. [Source 3]
  - **Workflow Automation:** Modifying theme elements or adjusting inventory directly via chat. [Source 3]

  ### Success Factors
  - **Context Awareness:** Sidekick knows the merchant's exact data context (sales, inventory, customers). [Source 48]
  - **In-Platform Execution:** It doesn't just give advice; it executes the changes within Shopify. [Source 3]
  - **Natural Language Interface:** Removes the need to navigate complex settings menus. [Source 48]

  ### User Sentiment Audit (Reddit, Trustpilot, App Store)
  - **The Good:** "It feels like having an analyst on the team." "Saved me hours clicking through reports." [Source 48]
  - **The Bad (Pain Points):**
    - "It's too focused on e-commerce metrics; it doesn't help me with my in-person appointments." [Source 4]
    - "I still have to log in on desktop for it to be truly useful; the mobile app experience is limited." [Source 15]
    - "It doesn't handle my multi-channel customer messages (Instagram, WhatsApp)." [Source 5]

  ---

  ## Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit vs. Shopify Sidekick
  | Feature | Shopify Sidekick | OHC (Current State) | OHC Target |
  | :--- | :--- | :--- | :--- |
  | Commerce Analytics | Deep (Native) | Basic | Deep (Agentic) |
  | Multi-Channel Messaging | Poor | Basic | Unified (Agentic) |
  | Appointment/Booking | Weak (Add-on) | Basic | Native (Agentic) |
  | Mobile-First Execution | Weak | Strong | Dominant |

  ### Unresolved Pain Points (Persona Mapping)
  - **Maya (Baker):** Overwhelmed by Shopify's complex setup. Needs unified DMs + deposit tracking. [Source 50]
  - **Carlos (Handyman):** Shopify Sidekick doesn't understand service routing or on-site quoting. [Source 41]
  - **Priya (Boutique):** Needs a tool that seamlessly blends POS data with online inventory without complex syncing rules. [Source 13]

  ---

  ## Track 4: Deeper Focused Research & Agentic Solutions

  ### Deep-Dive Evidence
  Small business owners repeatedly state in communities (e.g., r/smallbusiness) that they spend 30-40% of their time just "switching between apps" (Instagram, booking tool, payment processor, email). [Source 33]

  ### Agentic Solution Design: The "Unified Work Triage"
  OHC must build a single, intelligent feed. When a customer messages Maya on Instagram, the OHC Agent should:
  1. Identify the intent (custom cake order).
  2. Check Maya's availability calendar.
  3. Draft a reply with a dynamic payment link for a deposit.
  4. Present this to Maya in a 375px mobile UI as a single "Approve & Send" action.

  ### Proposed Architecture & Flow
  ```mermaid
  graph TD
      A[Customer Instagram DM] --> B(OHC Ingestion Engine);
      B --> C{OHC Work Triage Agent};
      C --> D[Identify Intent: Custom Order];
      C --> E[Check Operations Calendar];
      D --> F[Draft Quote & Payment Link];
      E --> F;
      F --> G[Present to Owner Mobile App];
      G --> H((Owner Approves));
      H --> I[Send Reply & Finalize Booking];
  ```

  ---

  ## Implementation Prompt & Design Doc

  **Title:** Unified Agentic Work Triage (Mobile-First)

  **Problem Statement:** Owners like Maya and Carlos are overwhelmed by app-switching. They need a single, prioritized feed on their mobile device where an AI assistant has already drafted the next action for their approval.

  **Design Doc:**
  - **Entity Types:** `WorkItem`, `AgentDraft`, `ApprovalAction`.
  - **UI Flow (375px):**
    1. Home Screen: Prioritized list of `WorkItems`.
    2. Tap `WorkItem`: Shows the customer context (DM screenshot/text).
    3. Bottom Sheet: Shows the `AgentDraft` (proposed reply + attached quote).
    4. Action: "Swipe to Approve" or "Edit Draft".
  - **Integration Points:** LLM Provider (Gemini Pro) for intent extraction and drafting; Messaging Webhooks (Meta Graph API) for ingestion and sending.

  **Implementation Prompt:**
  Build the "Work Triage" mobile-first UI component and the backend orchestrator that ties an incoming webhook event to an LLM drafting task. The critical user journey (CUJ) requires the owner to log in on a 375px viewport, see a new `WorkItem`, review the `AgentDraft`, and click "Approve", resulting in the system marking the item as resolved and dispatching the action.

  **Priority:** P0
  **Estimated Scope:** Large

  ---

  ## References & Sources Catalog
  1. [Zapier.Com](https://zapier.com/apps/instagram-for-business/integrations)
  2. [Help.Instagram.Com](https://help.instagram.com/163013892186835)
  3. [Www.Whatsapp.Com](https://www.whatsapp.com/business)
  4. [Business.Whatsapp.Com](https://business.whatsapp.com/blog)
  5. [Www.Twilio.Com](https://www.twilio.com/docs/whatsapp)
  6. [Stripe.Com](https://stripe.com/payments/payment-links)
  7. [Stripe.Com](https://stripe.com/docs/payments/payment-links)
  8. [Stripe.Com](https://stripe.com/docs/terminal)
  9. [Stripe.Com](https://stripe.com/docs/billing)
  10. [Squareup.Com](https://squareup.com/us/en/appointments)
  11. [Squareup.Com](https://squareup.com/us/en/invoices)
  12. [Squareup.Com](https://squareup.com/help/us/en)
  13. [Www.Shopify.Com](https://www.shopify.com/pos)
  14. [Www.Shopify.Com](https://www.shopify.com/inbox)
  15. [Www.Salesforce.Com](https://www.salesforce.com/products/small-business/)
  16. [Www.Freshworks.Com](https://www.freshworks.com/crm/small-business/)
  17. [Www.Setmore.Com](https://www.setmore.com/)
  18. [Simplybook.Me](https://simplybook.me/en/)
  19. [Www.Mindbodyonline.Com](https://www.mindbodyonline.com/business)
  20. [Www.Zenplanner.Com](https://www.zenplanner.com/)
  21. [Www.Glossgenius.Com](https://www.glossgenius.com/)
  22. [Www.Fresha.Com](https://www.fresha.com/for-business)
  23. [Www.Gocatchy.Com](https://www.gocatchy.com/)
  24. [Booksy.Com](https://booksy.com/biz/en-gb/)
  25. [Www.Honeybook.Com](https://www.honeybook.com/)
  26. [Business.Yelp.Com](https://business.yelp.com/)
  27. [Www.Podium.Com](https://www.podium.com/)
  28. [Www.Birdeye.Com](https://www.birdeye.com/)
  29. [Surveymonkey.Com](https://surveymonkey.com/)
  30. [Typeform.Com](https://typeform.com/)
  31. [Www.Jotform.Com](https://www.jotform.com/)
  32. [Docs.Google.Com](https://docs.google.com/forms/)
  33. [Zapier.Com](https://zapier.com/)
  34. [Www.Dingtalk.Com](https://www.dingtalk.com/)
  35. [Www.Larksuite.Com](https://www.larksuite.com/)
  36. [Www.Shopify.Com](https://www.shopify.com/sidekick)
  37. [Squareup.Com](https://squareup.com/)
  38. [Www.Hubspot.Com](https://www.hubspot.com/)
  39. [Www.Wix.Com](https://www.wix.com/)
  40. [Www.Notion.So](https://www.notion.so/product/ai)
  41. [Www.Servicetitan.Com](https://www.servicetitan.com/)
  42. [Chatspot.Ai](https://chatspot.ai/)
  43. [Www.Glean.Com](https://www.glean.com/)
  44. [Www.Harvey.Ai](https://www.harvey.ai/)
  45. [Sanalabs.Com](https://sanalabs.com/)
  46. [Julius.Ai](https://julius.ai/)
  47. [Www.Lindy.Ai](https://www.lindy.ai/)
  48. [Developer.Squareup.Com](https://developer.squareup.com/docs/)
  49. [Shopify.Dev](https://shopify.dev/)
  50. [About.Meta.Com](https://about.meta.com/)
  51. [About.Google](https://about.google/)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
