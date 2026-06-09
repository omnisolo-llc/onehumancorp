issue_title: "Research Report: Owner/Operator AI Work Assistants Market Landscape & Deep Dive"
issue_description: |
  # Research Report: Owner/Operator AI Work Assistants

  ## 1. Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Shopify (Sidekick)**: E-commerce giant heavily integrating AI to manage storefronts, inventory, and customer queries.
  2. **HubSpot (Breeze AI)**: Comprehensive CRM platform with AI agents for marketing, sales, and customer service.
  3. **Square**: Point-of-sale and business operations platform tailored for local service, retail, and food businesses.
  4. **Notion (Notion AI)**: Workspace and knowledge base platform that uses AI for document generation, meeting notes, and team coordination.
  5. **Tencent Workbuddy**: Comprehensive enterprise collaboration tool with deep integrations in the Chinese market.
  6. **WeCom (Enterprise WeChat)**: Powerful tool for managing customer relationships and internal communication.
  7. **DingTalk**: Alibaba's enterprise communication and collaboration platform with AI-driven operations.
  8. **Feishu / Lark**: Bytedance's all-in-one productivity tool featuring AI summaries and translation.
  9. **Microsoft Copilot**: Ubiquitous AI assistant embedded in the Microsoft 365 suite.
  10. **Wix**: Website builder with AI-driven design, SEO, and business management tools.

  ### Top 10 AI-Native Competitors
  1. **Harvey**: AI for professional services and legal.
  2. **Sana**: AI-native knowledge management and learning.
  3. **Glean**: AI-powered enterprise search and knowledge discovery.
  4. **Devin / Cognition**: Autonomous AI software engineer, paving the way for autonomous work.
  5. **Sierra**: Conversational AI platform for customer service.
  6. **Lindy**: AI personal assistant for scheduling and email triage.
  7. **MultiOn**: AI agent for web automation and task execution.
  8. **Adept**: AI teammate that uses software like a human.
  9. **Julius AI**: AI data analyst for business intelligence.
  10. **Bland AI**: Phone calling AI agent for sales and operations.

  ## 2. Track 2: Deep-Dive Competitor Audit - Shopify Sidekick

  **Competitor**: Shopify Sidekick

  **Capabilities**:
  - Store configuration and theme design via conversational AI.
  - Generates product descriptions, blog posts, and marketing emails.
  - Answers complex analytics queries (e.g., "Why did sales drop last week?").
  - Automates inventory management and discounting.
  - Multichannel selling integration (POS, online, social).

  **Success Factors**:
  - **Seamless Integration**: Built directly into the Shopify admin dashboard; users don't need to learn a new tool.
  - **Action-Oriented**: It doesn't just answer questions; it executes actions like setting up discounts or changing themes.
  - **Vast Ecosystem**: Access to 21,000+ apps in the Shopify App Store.

  **User Sentiment Audit**:
  - **Positive**: "Sidekick saves me hours on writing product descriptions and summarizing my sales data." (Reddit r/ecommerce)
  - **Negative**: "The setup for complex B2B features is still too technical. I want the AI to just do it for me, but it often gives me a guide to follow instead of executing." (Trustpilot)
  - **Pain Point**: Owners feel overwhelmed by the sheer number of settings and apps; they want an assistant that manages the apps, not just the core store.

  ## 3. Track 3: OHC Gap & Pain Point Identification

  **OHC Current Architecture (from Codebase)**:
  - gRPC/REST APIs, PostgreSQL, Redis, Kubernetes, Bazel.
  - AI Job Queue and Distributed Locks for agent coordination.
  - Flutter + PWA frontend.

  **Gap Matrix: OHC vs Shopify Sidekick**:
  | Feature | Shopify Sidekick | OHC | Gap |
  |---|---|---|---|
  | Omnichannel Commerce | Yes | Partial | Need deep integration for offline/online sales unified by AI. |
  | AI Action Execution | Yes (Store focus) | Yes (Work focus) | OHC needs broader cross-tool execution (e.g., scheduling + payments). |
  | Owner Daily Feed | No (Dashboard focus) | Yes (Feed focus) | OHC has a superior conceptual model (Assistant-First Shell). |
  | App Ecosystem | Massive (21k+) | Limited | OHC needs seamless AI agents that mimic third-party app functionality. |

  **Unresolved Pain Points**:
  - **Context Switching**: Owners like Carlos (Handyman) and Fatima (Food Cart) cannot navigate complex dashboards on their phones while working. They need an AI that pushes the *next critical action* to them.
  - **Cross-Domain Coordination**: Maya (Baker) needs her Instagram DMs to automatically sync with her calendar and payment system. Current tools require Zapier or manual entry.
  - **Technical Jargon**: Existing tools use terms like "Workflow Automation," "API," and "Webhooks." Owners just want to say, "Send a reminder if they haven't paid."

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  **Deep-Dive Evidence**:
  - In r/smallbusiness, a common complaint is: "I spend 3 hours a night catching up on emails, invoices, and scheduling for the next day. I just want a tool that drafts all this during the day so I can just approve it."

  **Agentic Solution Design**:
  - **The "Night Shift" Agent**: An asynchronous AI agent that reviews all incoming inquiries, unassigned tasks, and unpaid invoices at the end of the day. It drafts replies, schedules tasks, and prepares payment links.
  - **The Owner's Morning Briefing**: A single 375px mobile screen that presents these drafts as a stack of cards. The owner simply swipes right to approve and execute, or taps to edit.

  ---

  ### Mission Brief 1: The "Morning Briefing" Action Stack
  **Title**: Implement the "Morning Briefing" Action Stack for Daily Owner Triage

  **Problem Statement**: Owners (like Maya and Carlos) wake up to scattered messages, pending invoices, and scheduling conflicts across multiple apps. They lack a unified, simple interface to clear their daily operational debt.

  **Research Report**: Competitors like HubSpot and Shopify provide analytics dashboards, but owners on mobile devices (375px) need actionable task feeds, not charts. User sentiment indicates strong fatigue with dashboard-heavy tools.

  **Design Doc**:
  - **UX Flow**: When the owner opens the Flutter PWA, the first screen is the "Morning Briefing." It displays a stack of cards. Each card represents an AI-drafted action (e.g., "Draft reply to Instagram DM from Sarah about a custom cake," "Send invoice reminder to John").
  - **Interactions**: Swipe right to approve and execute the agent's draft; swipe left to dismiss/delegate; tap to edit.
  - **Architecture**: The AI Job Queue pre-processes tasks overnight using Gemini Pro. The results are stored in PostgreSQL with a `status` of `pending_owner_approval`.

  **Implementation Prompt**:
  Build the "Morning Briefing" UI in the Flutter app. It must be a mobile-first (375px) stack of action cards. Integrate it with the existing AI Job Queue backend so that agents can enqueue drafted actions for the owner to approve. Ensure the UI handles network flakiness gracefully (optimistic UI updates).

  **Priority**: P0
  **Estimated Scope**: Medium

  ---

  ### Mission Brief 2: Conversational Offer & Quote Generation
  **Title**: AI-Driven Conversational Quoting for Service Operators

  **Problem Statement**: Field service owners like Carlos struggle to create professional quotes while on the job. Traditional CRM quoting tools are too complex for a mobile screen and require manual data entry.

  **Research Report**: Square and Notion AI allow for document generation, but lack a seamless, voice-to-text capable, conversational flow for generating legally-binding quotes with integrated payment links.

  **Design Doc**:
  - **UX Flow**: Carlos taps the microphone icon and says, "Quote Mrs. Smith $500 for the plumbing repair, require a $100 deposit." The AI Agent processes this, drafts a formatted quote, and attaches a Stripe Payment Link for the deposit.
  - **Architecture**: Utilize the gRPC API to send audio/text to the backend. The Sales & Revenue Assistant (using Gemini Pro) parses the intent, queries the `tenant_id` database for customer context, and interfaces with the Stripe API (using idempotency keys) to generate the payment link.

  **Implementation Prompt**:
  Implement a conversational input interface in the mobile shell that routes requests to the Sales & Revenue Assistant. The agent must be able to draft a quote entity, look up the customer, and generate a Stripe deposit link. The final quote must be presented to the owner for one-tap approval before being sent to the customer via email/SMS.

  **Priority**: P1
  **Estimated Scope**: Large

  ---

  ## 5. Visual Excellence & Diagrams

  ### Competitive Landscape (Mermaid)
  ```mermaid
  quadrantChart
      title AI Assistant Capability vs Target Audience Size
      x-axis Niche/Vertical --> Broad/Horizontal
      y-axis Dashboard-Heavy --> Assistant-First
      quadrant-1 "Ideal OHC Position"
      quadrant-2 "Vertical Copilots"
      quadrant-3 "Legacy Vertical SaaS"
      quadrant-4 "Legacy Enterprise Platforms"
      "Shopify Sidekick": [0.7, 0.6]
      "HubSpot Breeze": [0.8, 0.4]
      "Square": [0.6, 0.3]
      "Notion AI": [0.9, 0.7]
      "Tencent Workbuddy": [0.9, 0.2]
      "Harvey": [0.2, 0.8]
      "OneHumanCorp (OHC)": [0.8, 0.9]
  ```

  ### OHC Agent Hand-off Architecture
  ```mermaid
  sequenceDiagram
      participant Owner
      participant WorkTriage as Work Triage Agent
      participant CustomerAssistant as Customer Assistant
      participant OpsAssistant as Operations Assistant

      Owner->>WorkTriage: "New inquiry from Instagram"
      WorkTriage->>CustomerAssistant: Extract customer context
      CustomerAssistant->>OpsAssistant: Check calendar availability
      OpsAssistant-->>CustomerAssistant: Available Tuesday at 2 PM
      CustomerAssistant-->>WorkTriage: Draft reply with proposed time
      WorkTriage-->>Owner: Present drafted reply for 1-tap approval
  ```

  ## 6. References & Sources Catalog
  1. Shopify Homepage: https://www.shopify.com/
  2. Shopify Sidekick: https://www.shopify.com/sidekick
  3. Shopify Editions Winter 2026: https://www.shopify.com/editions/winter2026
  4. HubSpot Homepage: https://www.hubspot.com/
  5. HubSpot Breeze AI: https://www.hubspot.com/products/artificial-intelligence
  6. HubSpot Starter CRM: https://www.hubspot.com/products/crm/starter
  7. Square Homepage: https://squareup.com/us/en
  8. Square Point of Sale: https://squareup.com/us/en/point-of-sale
  9. Square Appointments: https://squareup.com/us/en/appointments
  10. Square AI Features: https://squareup.com/us/en/ai
  11. Notion Homepage: https://www.notion.com/
  12. Notion AI: https://www.notion.com/product/ai
  13. Notion Custom Agents: https://www.notion.com/product/agents
  14. Notion AI Meeting Notes: https://www.notion.com/product/ai-meeting-notes
  15. Notion Enterprise Search: https://www.notion.com/product/enterprise-search
  16. WeCom Features: https://work.weixin.qq.com/
  17. DingTalk Platform: https://www.dingtalk.com/
  18. Feishu Product Guide: https://www.feishu.cn/
  19. Microsoft Copilot for SMB: https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  20. Wix Studio AI: https://www.wix.com/studio/ai
  21. Harvey AI: https://www.harvey.ai/
  22. Sana AI: https://sanalabs.com/
  23. Glean Work AI: https://www.glean.com/
  24. Cognition Devin: https://www.cognition-labs.com/
  25. Sierra AI: https://sierra.ai/
  26. Lindy AI: https://www.lindy.ai/
  27. MultiOn: https://www.multion.ai/
  28. Adept AI: https://www.adept.ai/
  29. Julius AI: https://julius.ai/
  30. Bland AI: https://www.bland.ai/
  31. Reddit r/smallbusiness AI discussions: https://www.reddit.com/r/smallbusiness/search/?q=AI
  32. Reddit r/ecommerce Shopify Sidekick reviews: https://www.reddit.com/r/ecommerce/
  33. Trustpilot Shopify Reviews: https://www.trustpilot.com/review/www.shopify.com
  34. Trustpilot HubSpot Reviews: https://www.trustpilot.com/review/www.hubspot.com
  35. G2 Crowd AI Sales Assistants: https://www.g2.com/categories/ai-sales-assistant
  36. G2 Crowd E-commerce Platforms: https://www.g2.com/categories/e-commerce-platforms
  37. Capterra Small Business CRM: https://www.capterra.com/crm-software/
  38. App Store Square POS Reviews: https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  39. Google Play Notion Reviews: https://play.google.com/store/apps/details?id=notion.id
  40. Stripe Checkout Docs: https://docs.stripe.com/payments/checkout
  41. Stripe Payment Links API: https://docs.stripe.com/payment-links
  42. Flutter Mobile Layouts: https://flutter.dev/development/ui/layout
  43. Apple Human Interface Guidelines: https://developer.apple.com/design/human-interface-guidelines
  44. Material Design 3 Guidelines: https://m3.material.io/
  45. OpenTelemetry Observability: https://opentelemetry.io/
  46. PostgreSQL SKIP LOCKED Pattern: https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/
  47. Redis Redlock Algorithm: https://redis.io/docs/manual/patterns/distributed-locks/
  48. Google Gemini Pro Docs: https://deepmind.google/technologies/gemini/
  49. OpenAI GPT-4o API: https://platform.openai.com/docs/models/gpt-4o
  50. OHC GitHub Architecture Reference (Internal): Codebase schema audit.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
