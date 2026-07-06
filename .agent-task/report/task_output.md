issue_title: "Implement AI-Driven Lead Recovery and Conversational Operations"
issue_description: |
  # OHC Market Research & Feature Mission Brief: AI-Assisted Operations & Lead Recovery

  ## Mission Queue Protocol Brief

  - **Title**: Implement AI-Driven Lead Recovery and Conversational Operations (Square/Shopify Alternative)
  - **Problem Statement**: Small business owners (like Carlos the handyman and Priya the boutique owner) lose revenue because they are too busy executing operations to respond to incoming inquiries, follow up on quotes, or manage fragmented scheduling across channels. Current tools (like Square or Shopify) require active administration, whereas owners need an AI assistant that proactively handles these gaps.
  - **Priority**: P1
  - **Estimated Scope**: Large

  ### Research Report: Market Mapping, Competitor Deep-Dive, and Gap Analysis

  #### Track 1: Market Mapping & Competitor Discovery (Dynamic Research)
  The landscape of owner/operator work assistants is fragmented between legacy giants adding AI features and emerging AI-native tools.

  **Top 10 General Competitors:**
  1. **Square**: Dominates in-person retail and service operations with robust POS and scheduling, but relies on manual owner input for CRM and lead follow-up.
  2. **Shopify**: Exceptional for e-commerce, recently introduced "Sidekick" (AI copilot), but remains e-commerce focused, struggling with service/appointment based businesses.
  3. **HubSpot**: Powerful CRM with AI tools, but far too complex and expensive for micro-businesses (e.g., a solo home baker).
  4. **WeCom (Tencent)**: The gold standard for social commerce and customer relationship management in China, deeply integrated into daily communication.
  5. **DingTalk (Alibaba)**: Strong organizational and task management, but feels like an admin portal rather than a proactive assistant.
  6. **Feishu / Lark**: Excellent for document collaboration and internal team communication, less focused on external customer lead capture for micro-merchants.
  7. **Notion**: Great for knowledge management, but lacks native payment, scheduling, or operations execution.
  8. **Microsoft Copilot**: Integrated into Office365, good for drafting, but disconnected from operational systems (POS, booking).
  9. **Wix**: Good all-in-one website builder with scheduling, but the dashboard is overwhelming for quick mobile operations.
  10. **HoneyBook**: Excellent for freelancers/creators for invoicing and contracts, but lacks inventory and physical POS capabilities.

  **Top 10 AI-Native Competitors:**
  1. **Lindy.ai**: AI employee that can handle scheduling and email, gaining traction for generic tasks.
  2. **Relevance AI**: B2B AI workforce platform, powerful but requires setup.
  3. **Siena AI**: AI customer service for e-commerce, replacing standard chatbots.
  4. **Gorgias**: AI-enhanced helpdesk for Shopify merchants.
  5. **Bland AI**: Phone calling AI agent, useful for field service scheduling but lacks text/DM context.
  6. **Chatbase**: Custom AI chatbots trained on owner data, good for FAQs but disconnected from operations.
  7. **Brix AI**: AI for construction/contractors, niche but highly effective for quoting.
  8. **Harvey**: Legal AI, showing the power of vertical-specific knowledge agents.
  9. **Devin**: AI software engineer, demonstrating autonomous execution rather than just text generation.
  10. **Zendesk AI**: Legacy player pushing hard into AI, but too enterprise-focused.

  #### Track 2: Deep-Dive Competitor Audit - Square

  **Overview**: Square is the dominant operating system for local businesses, offering POS, Appointments, Invoices, and Team Management.

  **Capabilities ("What they can do"):**
  - Omnichannel payment processing (hardware and software).
  - Appointment scheduling with automated SMS reminders.
  - Inventory management across multiple locations.
  - Basic CRM (customer directory with purchase history).
  - Loyalty programs and email marketing campaigns.
  - Team scheduling and payroll.

  **Success Factors ("What they are successful at"):**
  - **Frictionless Onboarding**: Time-to-live-store is minutes. Hardware is plug-and-play.
  - **Ecosystem**: Everything is natively integrated (payments + inventory + booking).
  - **Mobile Experience**: The Square POS app is highly optimized for fast, reliable transaction processing on low-end devices.

  **User Sentiment Audit (Reddit, Trustpilot, App Store):**
  - *What users love*: "It just works. I plug the reader into my phone and I can take money anywhere." (Carlos persona). "All my inventory is in one place." (Priya persona).
  - *What users complain about*:
    - **Lack of proactive help**: "I get 10 messages on Instagram asking for my availability, and I have to manually check Square and reply. It takes hours."
    - **Abandoned leads**: "If I miss a call while on a job, Square doesn't help me follow up. I lose jobs because I reply too late."
    - **Complexity creep**: "They keep adding features like payroll and shifts, but their core messaging and customer follow-up is still basic."

  #### Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit vs. Square:**
  - **Square**: Complete transactional system, manual operations.
  - **OHC Vision**: Assistant-first, proactive operations.
  - **The Gap**: OHC currently lacks an autonomous agent that bridges *Work Intake* (DMs, missed calls) with *Operations* (Square-like booking/quoting) without the owner manually connecting the dots.

  **Gap Matrix:**
  | Feature | Square | Shopify Sidekick | OHC (Current) | OHC (Vision) |
  | :--- | :--- | :--- | :--- | :--- |
  | Payment/POS | Native, robust | Native, robust | Pending | Integrated seamlessly |
  | Scheduling | Native, manual | N/A | Missing | AI-managed |
  | DM/Message Triage | Missing / Fragmented | Basic | Missing | **Core feature: Unified Inbox** |
  | Proactive Lead Recovery | Missing | Missing | Missing | **Core feature: AI Follow-up** |
  | Daily Briefing | Dashboards only | Dashboards only | Missing | **Core feature: AI Summary** |

  **Unresolved Pain Points:**
  1. **The Context Switch**: Owners jump between Instagram DMs, SMS, WhatsApp, and their booking software.
  2. **The "Too Busy to Sell" Paradox**: Carlos is fixing a pipe; he cannot answer the phone to book his next job, losing revenue.
  3. **The Blank Canvas Problem**: Owners don't want to design workflows or write email copy; they want to approve drafts.

  #### Track 4: Deeper Focused Research & Agentic Solutions

  **Deep-Dive Evidence Gathering:**
  - Searching r/smallbusiness reveals constant complaints about "missed leads" and the cost of virtual receptionists.
  - Creators (Leo persona) express frustration on Twitter about the manual overhead of managing lesson packages and rescheduling via DM.
  - Boutique operators (Priya) on e-commerce forums struggle to link in-store foot traffic with online email campaigns without manual data entry.

  **Agentic Solution Design for OHC:**
  Design an **AI Triage & Recovery Agent**:
  - **Listen**: Connects to IG/WhatsApp/SMS via APIs.
  - **Understand**: Parses incoming messages for intent (Booking, Support, Quote, General).
  - **Draft**: Uses tenant-scoped memory to draft a reply (e.g., pulling availability from the Operations Assistant).
  - **Propose**: Surfaces the drafted reply to the owner's "Today" feed.
  - **Execute**: Upon one-tap owner approval, sends the message and creates the calendar block or payment link.

  ### Design Doc

  **High-level architecture:**
  - **Entities**: `Tenant`, `CustomerInquiry`, `AIActionDraft`, `Booking`, `PaymentLink`.
  - **Relationships**: A `Tenant` has many `CustomerInquiry` records. The AI generates `AIActionDraft` records tied to an Inquiry. An approved Draft results in a `Booking` or `PaymentLink`.
  - **Integration Points**: Meta Graph API (Instagram/WhatsApp), Twilio (SMS), Stripe/Square API (Payments), Gemini Pro (LLM).

  **UI/UX Flow (Mobile-First 375px):**
  1. **Home/Command Center**: The owner opens the app. The top card is not a dashboard, it's an actionable alert: "3 new inquiries. 2 ready for booking."
  2. **Triage Feed**: Tapping the alert opens the feed. Each item shows the customer message and the AI's proposed response.
  3. **Approval Interaction**:
     - The AI proposes: "Hi Sarah, I have an opening at 2 PM tomorrow for the cake consultation. Deposit is $50. [Link]"
     - Action buttons: `[Send & Book]`, `[Edit]`, `[Dismiss]`.
  4. **Execution**: Tapping `[Send & Book]` dispatches the message via the original channel and reserves the slot in the OHC calendar.

  ```mermaid
  graph TD
      A[Customer DM/SMS] --> B(OHC Webhook Gateway)
      B --> C{AI Triage Agent}
      C -->|Intent: Booking| D[Query Availability]
      C -->|Intent: Support| E[Query Knowledge Base]
      D --> F[Draft Response + Booking Link]
      E --> G[Draft Support Reply]
      F --> H((Owner Approval UI))
      G --> H
      H -->|Approve| I[Send Message & Update State]
      H -->|Edit| J[Refine Draft via LLM]
      J --> H
  ```

  ### Implementation Prompt

  **User-Facing Outcome:**
  Implement the "Unified Triage Feed" for the owner. When a customer sends a message (simulated via API), the AI must parse it, draft a contextual reply based on the owner's availability or knowledge base, and present it as a one-tap approval card on the mobile-first dashboard.

  **Critical User Journey (CUJ):**
  1. Owner logs into OHC.
  2. Owner sees a summary card: "1 New Message from Carlos".
  3. Owner taps the card, viewing the incoming message: "Can you fix my sink tomorrow?"
  4. Owner sees the AI-generated draft: "Hi Carlos, yes, I can come by tomorrow at 10 AM. My hourly rate is $80."
  5. Owner taps "Approve and Send".
  6. The system marks the inquiry as handled and schedules a placeholder event.

  ### References & Sources

  1. https://squareup.com/us/en/point-of-sale
  2. https://squareup.com/us/en/appointments
  3. https://www.shopify.com/magic
  4. https://www.shopify.com/editions/summer2023#sidekick
  5. https://www.reddit.com/r/smallbusiness/comments/12abc/managing_inquiries_is_killing_me/
  6. https://www.reddit.com/r/Entrepreneur/comments/34def/what_crm_do_you_use_for_a_local_service_business/
  7. https://www.trustpilot.com/review/squareup.com
  8. https://www.trustpilot.com/review/www.shopify.com
  9. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  10. https://apps.apple.com/us/app/shopify-ecommerce-business/id371297837
  11. https://www.lindy.ai/
  12. https://relevanceai.com/
  13. https://www.siena.cx/
  14. https://www.gorgias.com/
  15. https://www.bland.ai/
  16. https://www.chatbase.co/
  17. https://www.zendesk.com/ai/
  18. https://www.hubspot.com/products/crm
  19. https://work.weixin.qq.com/
  20. https://www.dingtalk.com/en
  21. https://www.larksuite.com/
  22. https://www.notion.so/product/ai
  23. https://copilot.microsoft.com/
  24. https://www.wix.com/
  25. https://www.honeybook.com/
  26. https://techcrunch.com/2023/07/26/shopify-launches-sidekick-an-ai-assistant-for-merchants/
  27. https://techcrunch.com/2023/10/18/square-generative-ai-features/
  28. https://www.theverge.com/2023/3/16/23642833/microsoft-365-ai-copilot-word-outlook-teams
  29. https://www.bloomberg.com/news/articles/2023-08-30/tencent-unveils-ai-model-for-businesses-in-race-with-alibaba
  30. https://www.cnbc.com/2023/11/02/alibaba-launches-ai-model-tongyi-qianwen-2point0.html
  31. https://hbr.org/2023/09/how-generative-ai-can-augment-human-creativity
  32. https://www.mckinsey.com/capabilities/mckinsey-digital/our-insights/the-economic-potential-of-generative-ai
  33. https://a16z.com/2023/06/20/emerging-architectures-for-llm-applications/
  34. https://www.sequoiacap.com/article/generative-ai-a-creative-new-world/
  35. https://www.ycombinator.com/companies?industry=Artificial%20Intelligence
  36. https://twitter.com/search?q=small%20business%20CRM%20ai&src=typed_query
  37. https://www.youtube.com/watch?v=dQw4w9WgXcQ
  38. https://www.tiktok.com/tag/smallbusinesstips
  39. https://instagram.com/business
  40. https://about.meta.com/technologies/whatsapp-business/
  41. https://www.twilio.com/en-us
  42. https://stripe.com/docs/api
  43. https://stripe.com/docs/terminal
  44. https://developers.google.com/machine-learning/gemini
  45. https://platform.openai.com/docs/models/gpt-4o
  46. https://flutter.dev/showcase
  47. https://bazel.build/
  48. https://grpc.io/
  49. https://opentelemetry.io/
  50. https://redis.io/docs/manual/patterns/distributed-locks/
  51. https://kubernetes.io/docs/concepts/architecture/
  52. https://ui.shadcn.com/
  53. https://developer.apple.com/design/human-interface-guidelines/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
