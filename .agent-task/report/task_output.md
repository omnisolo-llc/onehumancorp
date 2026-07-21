issue_title: "Implement Autonomous Triage Agent & Unified Inbox Mobile UI"
issue_description: |
  # OHC Market Research & Feature Brief: Unified Inbox & Autonomous Triage Agent

  ## Problem Statement
  Small business owners, independent operators, and creators are overwhelmed by fragmented communication channels. They receive inquiries via Instagram DMs, WhatsApp, SMS, email, and web forms.
  Currently, tracking these conversations requires switching between multiple apps, leading to missed opportunities, delayed responses, and lost revenue. They don't just want a unified inbox—they want an assistant that automatically triages messages, extracts intents (e.g., booking request, customer complaint, general inquiry), drafts contextual responses based on their business data, and flags urgent items for their immediate attention.

  ## Track 1: Market Mapping & Competitor Discovery
  The market for owner/operator work assistants is rapidly evolving, bridging the gap between traditional CRMs and AI-native automation.

  **Top 10 General Competitors (Traditional & Suite Tools):**
  1. **Tencent Workbuddy (WeCom)**: Dominant in China, deeply integrates chat, CRM, and task management.
  2. **Shopify**: Powerful e-commerce backend, but relies heavily on third-party apps for robust customer communication.
  3. **HubSpot**: Excellent CRM, but often too complex and expensive for micro-businesses and solopreneurs.
  4. **Square**: Strong point-of-sale and basic booking, but limited conversational AI capabilities.
  5. **Wix**: Offers a unified inbox, but it functions more as a notification center than an autonomous assistant.
  6. **DingTalk**: Alibaba's enterprise communication and collaboration platform.
  7. **Feishu/Lark**: ByteDance's productivity suite, strong internal team tools but less focused on external SMB customer triage.
  8. **Notion AI**: Great for knowledge management, but lacks native omni-channel messaging integration.
  9. **Microsoft Copilot (M365)**: Powerful enterprise AI, but not tailored for the mobile-first local service provider.
  10. **Zendesk**: Industry standard for support tickets, but too formal for casual Instagram/WhatsApp interactions.

  **Top 10 AI-Native Competitors (Rising Stars):**
  1. **Shopify Sidekick**: AI assistant within the Shopify admin dashboard for merchants.
  2. **Intercom (Fin AI)**: Advanced AI bot for customer service, but priced for mid-market/enterprise.
  3. **ManyChat**: Popular for Instagram/Messenger automation, but uses rigid decision trees rather than fluid AI.
  4. **Gorgias**: E-commerce focused helpdesk with AI capabilities.
  5. **Kustomer**: CRM designed for high-volume support, now heavily utilizing AI.
  6. **Reply.io**: AI-driven sales engagement and outreach.
  7. **Chatfuel**: AI chatbot builder for WhatsApp and social media.
  8. **Sendbird**: Chat API with AI capabilities.
  9. **Levity**: No-code AI workflow automation for text and documents.
  10. **Lindy.ai**: AI autonomous assistant for scheduling and triage.

  ## Track 2: Deep-Dive Competitor Audit (WeCom / Tencent Workbuddy)
  We selected **WeCom** for a deep-dive analysis because it successfully bridges internal team collaboration and external customer communication seamlessly via WeChat integration.

  ### Capabilities ("What they can do")
  WeCom provides a unified interface where operators can talk to their team, assign tasks, and directly message customers on the consumer WeChat app. It includes customer tagging, automated welcome messages, broadcast messaging, and integrated mini-programs for bookings and payments.

  ### Success Factors ("What they are successful at")
  - **Seamless Omnichannel**: The line between internal operations and external customer service is non-existent.
  - **Mobile-First Excellence**: The entire business can be run from a mobile device without sacrificing functionality.
  - **Customer Context**: Every chat thread includes the customer's purchase history and CRM tags alongside the conversation.

  ### User Sentiment Audit
  - *The good*: Users love the ability to maintain professional boundaries while communicating where the customer already is (WeChat). "I don't have to force my clients to download a new app."
  - *The bad*: The AI capabilities are mostly basic automation rules rather than generative, context-aware assistance. Users still spend significant time manually drafting replies to common variations of questions.

  ## Track 3: OHC Gap & Pain Point Identification
  Cross-referencing WeCom and Shopify Sidekick against OneHumanCorp's vision reveals a critical gap.

  ### OHC Feature Audit
  OHC currently has strong foundational models for Work Triage and Assistant-First shell designs, but lacks the specific, autonomous omni-channel ingestion and response drafting engine optimized for mobile-first interaction.

  ### Gap Matrix

  | Feature | WeCom | Shopify Sidekick | OHC (Current) | OHC (Target) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Inbox** | High | Low | Low | **High** |
  | **Generative AI Drafting** | Low | Medium | Medium | **High (Autonomous)** |
  | **Mobile-First Experience** | High | Medium | High | **High (375px Optimized)** |
  | **Customer Context Overlay** | High | High | Low | **High** |
  | **Proactive Task Extraction**| Low | Low | Low | **High** |

  ### Unresolved Pain Points
  1. **The Context Switch Penalty**: Owners are jumping between Instagram, WhatsApp, and email.
  2. **Blank Page Anxiety**: Owners struggle to quickly draft polite, accurate responses while on the go.
  3. **Lost Action Items**: A customer agrees to a quote via SMS, but the owner forgets to log it in the booking system.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  To solve these pain points, OHC must introduce an **Autonomous Triage Agent** that sits between all incoming communication channels and the owner.

  ### Deep-Dive Evidence Gathering
  In r/smallbusiness, a recurring complaint is: *"I get 50 DMs a day asking for my pricing or availability. I tried automated replies, but they sound robotic and annoy customers. I need something that reads the message, checks my calendar, and suggests a reply I can just tap to send."*

  ### Agentic Solution Design
  The OHC Autonomous Triage Agent will:
  1. Ingest messages from connected channels.
  2. Use an LLM to classify intent (e.g., "Pricing Inquiry", "Booking Request").
  3. Retrieve context (Calendar availability, Pricing Docs, Past Orders).
  4. Draft a context-aware reply.
  5. Present the draft as a highly visible, 1-tap "Approve & Send" card in the 375px mobile UI.

  ## Design Doc

  ### High-Level Architecture
  - **Ingestion Layer**: Webhooks from Meta (Instagram/WhatsApp), Twilio (SMS), SendGrid (Email).
  - **Processing Engine**: A temporal worker that picks up new messages, queries the `Knowledge & Compliance Assistant` for context, and calls the Gemini LLM for intent classification and draft generation.
  - **Data Model**: `Message`, `Thread`, `AgentDraft`, `CustomerContext`.
  - **UI Integration**: The Assistant-First Shell features a "Needs Attention" feed.

  ### UI/UX Flow (Mobile-First 375px)
  1. **Home Feed**: The top card is a semi-translucent, priority alert: "3 New Inquiries Drafted".
  2. **Triage View**: Tapping the card opens a vertical list of threads. Each thread shows the customer's message, their CRM tags (e.g., "Repeat Customer"), and the AI-generated draft in an editable text field.
  3. **Interaction**:
     - Tap "Approve & Send" (Green button, 44x44px minimum touch target).
     - Tap the text to edit before sending.
     - Swipe left to "Dismiss Draft" and reply manually.

  ### Mermaid Architecture Chart

  ```mermaid
  graph TD
      A[Customer IG/WA/SMS] -->|Webhook| B(Ingestion Layer)
      B --> C{Autonomous Triage Agent}
      C -->|Fetch Context| D[(OHC Postgres: CRM & Calendar)]
      C -->|Generate Draft| E[Gemini LLM]
      E --> F[Draft Review Queue]
      F --> G[Mobile UI Needs Attention Feed]
      G -->|Owner Taps Approve| H(Dispatch Layer)
      H --> A
  ```

  ## Implementation Prompt

  **Title:** Autonomous Triage Agent & Unified Inbox Mobile UI
  **Target Persona:** Carlos (Field Service Owner) who manages everything from his Android phone while on job sites.
  **Outcome:** Carlos receives text messages and WhatsApps from clients. Instead of typing out replies while driving, OHC ingests the messages, checks his schedule, and prepares drafted replies. Carlos opens the app, sees the drafts, and taps "Approve" to send them.

  **Critical User Journey (CUJ):**
  1. Carlos logs into the OHC mobile app.
  2. A simulated customer sends a WhatsApp message: "Are you available next Tuesday for a repair?"
  3. The system processes the message, queries Carlos's availability, and drafts: "Hi! Yes, I have a slot open next Tuesday at 2 PM. Does that work for you?"
  4. Carlos's home screen updates with a priority notification card.
  5. Carlos taps the card, reviews the draft, and clicks "Approve & Send".
  6. The system dispatches the message back to the simulated webhook endpoint and updates the thread status to "Responded".

  **Acceptance Criteria:**
  - The Inbox UI must be flawlessly responsive on a 375px viewport with no horizontal scrolling.
  - Draft approval buttons must meet the 44x44px touch target requirement.
  - Playwright E2E tests must simulate an incoming webhook, verify the draft appears in the UI, and verify the outbound dispatch upon approval.
  - No direct SQL/API mocking in the test; it must test the full end-to-end flow using the test environment.

  ## Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Large

  ---

  ## References & Sources Catalog
  1. https://www.tencent.com/en-us/business/wecom.html
  2. https://work.weixin.qq.com/
  3. https://www.shopify.com/sidekick
  4. https://www.shopify.com/inbox
  5. https://www.hubspot.com/products/crm
  6. https://squareup.com/us/en/point-of-sale
  7. https://www.wix.com/ecommerce/website
  8. https://www.dingtalk.com/en
  9. https://www.larksuite.com/
  10. https://www.notion.so/product/ai
  11. https://www.microsoft.com/en-us/microsoft-365/copilot
  12. https://www.zendesk.com/
  13. https://www.intercom.com/fin
  14. https://manychat.com/
  15. https://www.gorgias.com/
  16. https://www.kustomer.com/
  17. https://reply.io/
  18. https://chatfuel.com/
  19. https://sendbird.com/
  20. https://levity.ai/
  21. https://www.lindy.ai/
  22. https://developers.facebook.com/docs/instagram-api/
  23. https://developers.facebook.com/docs/whatsapp/
  24. https://www.twilio.com/docs/sms
  25. https://sendgrid.com/solutions/email-api/
  26. https://stripe.com/docs/api
  27. https://calendly.com/integration
  28. https://zapier.com/apps
  29. https://www.make.com/en/integrations
  30. https://www.reddit.com/r/smallbusiness/
  31. https://www.reddit.com/r/smallbusiness/comments/16a1b2c/unified_inbox_recommendations/
  32. https://www.reddit.com/r/Entrepreneur/
  33. https://www.reddit.com/r/ecommerce/
  34. https://www.trustpilot.com/review/www.shopify.com
  35. https://www.trustpilot.com/review/wecom.qq.com
  36. https://www.g2.com/products/wecom/reviews
  37. https://www.g2.com/products/shopify/reviews
  38. https://www.capterra.com/p/136006/Shopify/
  39. https://www.capterra.com/p/192284/WeCom/
  40. https://news.shopify.com/
  41. https://techcrunch.com/tag/smb/
  42. https://www.bloomberg.com/technology
  43. https://www.forbes.com/small-business/
  44. https://hbr.org/topic/small-business
  45. https://www.wsj.com/news/business/small-business
  46. https://www.inc.com/technology
  47. https://www.entrepreneur.com/topic/technology
  48. https://www.sba.gov/business-guide
  49. https://www.score.org/
  50. https://www.ycombinator.com/library
  51. https://a16z.com/category/enterprise/
  52. https://www.sequoiacap.com/our-companies/
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
