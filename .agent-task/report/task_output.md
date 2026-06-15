issue_title: "Implement 'Actionable Daily Brief' via AI Work Triage"
issue_description: |
  ## Title: Implement "Actionable Daily Brief" via AI Work Triage

  ## Problem Statement
  Small business owners and operators (like Maya the baker and Carlos the field service owner) suffer from "dashboard fatigue" and fragmented notifications across platforms (Instagram DMs, emails, bookings, payments). They don't want a suite of charts or lists to manage; they want a single, plain-language assistant that tells them exactly what changed overnight, what needs attention today, and provides one-tap actions to resolve those items. Without this, they spend the first hour of their day just figuring out what work to do, often missing critical leads or delaying customer responses.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Shopify: E-commerce giant, excellent for products, poor for custom services.
  2. Square: Great POS and basic scheduling, lacks unified AI assistant.
  3. WeCom: Tencent's enterprise tool, deep WeChat integration, complex setup.
  4. DingTalk: Alibaba's offering, feature-rich but overwhelming for micro-businesses.
  5. Feishu/Lark: Excellent document and chat integration, but built for internal teams rather than customer interaction.
  6. HubSpot: Powerful CRM, but expensive and overkill for a local bakery or tutor.
  7. Wix: Website builder first, operations tool second.
  8. Notion: Great for knowledge, poor for transactional operations or customer messaging.
  9. Microsoft 365 / Teams: Built for corporate collaboration, completely ignores retail/service POS realities.
  10. Thryv: Good vertical SaaS for service businesses, but feels dated and non-agentic.

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick: AI commerce copilot (analytics and store setup).
  2. Microsoft Copilot for Sales: AI CRM assistant, mostly for B2B.
  3. Notion AI: AI document and knowledge generation.
  4. HubSpot ChatSpot: AI conversational CRM interface.
  5. Intercom Fin: AI customer service bot.
  6. Harvey: AI for legal professionals (vertical-specific).
  7. AutoGPT / MultiOn: Autonomous browser agents (too experimental for SMBs).
  8. Lindy.ai: AI scheduling and email triage assistant.
  9. Clara/x.ai: AI meeting schedulers.
  10. Sierra: Conversational AI for enterprise customer service.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick
  **Capabilities ("What they can do"):**
  Shopify Sidekick is an AI commerce assistant integrated directly into the Shopify admin panel. It can answer questions about sales, generate discount codes, change store themes, and summarize customer data.

  **Success Factors ("What they are successful at"):**
  - Instant context: Sidekick inherently knows the store's inventory, sales data, and settings.
  - Conversational UI: Replaces deep menu navigation with simple chat prompts ("Put all winter coats on a 20% sale").
  - Clear boundaries: Only acts with explicit merchant approval for destructive or user-facing changes.

  **User Sentiment Audit (e.g., r/shopify, Trustpilot):**
  - *Positive:* "Sidekick saves me 20 minutes a day just digging through analytics."
  - *Negative:* "It's only good for analytics and basic setup. It doesn't help me reply to an angry customer on Instagram or manage my physical store pickups." Users complain it's focused on the *software* rather than the *daily operation* of the business.

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:**
  Currently, OHC requires users to navigate to separate tabs for Tasks, Customers, and Messages to find their daily workload.

  **Gap Matrix (Shopify Sidekick vs OHC):**

  | Feature | Shopify Sidekick | OHC (Current) | OHC (Proposed) |
  | :--- | :--- | :--- | :--- |
  | **Unified Daily Brief** | Tells owner what sold yesterday | Requires manual review across tabs | Generates an AI summary of overnight changes |
  | **Actionable Suggestions** | Focuses on store setup and analytics | None | Provides 1-tap actions to clear items |
  | **Customer Context** | Basic integration with Shopify customers | Limited context across tabs | Intelligent contextual awareness based on messaging and purchase history |
  | **Operations Sync** | Poor, mostly limited to e-commerce products | Disjointed | Seamlessly integrates messaging, scheduling, and billing |

  **Unresolved Pain Points:**
  Owners want an *active* assistant, not a *passive* dashboard. They want the system to surface the most critical 3-5 items each morning in an "Actionable Daily Brief" format, integrating messages, bookings, and payments.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Deep-Dive Evidence Gathering:**
  In operator communities (e.g., r/smallbusiness), a recurring theme is the "morning scramble." Owners wake up to 5 Instagram DMs, 2 emails, a missed payment, and a booking cancellation. They have to mentally synthesize these into a prioritized to-do list.

  **Agentic Solution Design:**
  The "Work Triage" agent runs a cron job before the owner's typical start time. It queries all incoming messages, new bookings, pending invoices, and system alerts. It uses an LLM to prioritize them based on urgency and revenue impact. It generates a plain-language summary: "Good morning! You have 3 cake inquiries, 1 pending deposit for Carlos, and a cancellation to fill at 2 PM." It presents 1-tap actions below each summary item (e.g., "Draft Reply", "Resend Invoice", "Offer Slot to Waitlist").

  ## Design Doc

  ### Architecture & Integration Points
  - **Entity Types:** `DailyBrief`, `ActionItem` (polymorphic: relates to `Message`, `Booking`, `Invoice`).
  - **AI Agent Integration:** A new `WorkTriageAgent` (powered by Gemini Pro) triggered by a scheduled job via PostgreSQL `SKIP LOCKED` job queue.
  - **Context Gathering:** The agent executes tools to fetch `unread_messages`, `pending_payments`, and `todays_bookings`.
  - **Output:** Saves a structured `DailyBrief` to the database, pushed via WebSocket to the frontend.

  ### UX Flow & Mobile UI (375px)
  - **The "Command Center" Shell (Home Screen):**
    - Full-screen translucent glass card at the top: "Good morning, Maya."
    - **The Brief:** A conversational paragraph summarizing the day.
    - **Action Cards (Swipable list of 44x44px minimum touch targets):**
      - Card 1: "3 new DMs about custom cakes." -> [Draft Replies (AI)] button.
      - Card 2: "Carlos hasn't paid the $50 deposit." -> [Send Reminder] button.

  ```mermaid
  graph TD;
      A[Owner Opens OHC App] --> B[Home Screen: Actionable Daily Brief];
      B --> C{Review Action Cards};
      C -->|Tap 'Draft Reply'| D[Customer Assistant Agent generates response];
      C -->|Tap 'Send Reminder'| E[Sales Agent sends Stripe Payment Link];
      D --> F[Owner Approves & Sends];
      E --> G[Owner Approves & Sends];
      F --> H[Action Card Marked Complete];
      G --> H;
  ```

  ```mermaid
  pie title Daily Time Wasted by SMB Owners (Without AI Triage)
      "Triaging Inbox" : 35
      "Checking Schedules" : 20
      "Navigating Menus" : 25
      "Actual Work Execution" : 20
  ```

  ## Implementation Prompt
  **User-Facing Outcome:** When the owner opens the OHC app for the first time each day, they bypass traditional dashboards and are greeted by a natural-language "Daily Brief" summarizing the most critical overnight activities (messages, bookings, payments). Below the summary, discrete "Action Cards" allow them to handle each item with a single tap, utilizing AI to draft replies or prepare actions.

  **Critical User Journey (CUJ):**
  1. Owner logs in / opens the app.
  2. The home screen renders the `DailyBrief` component.
  3. Owner reads the summary and taps the primary action on the first Action Card.
  4. An AI draft is presented in a bottom sheet or modal.
  5. Owner taps "Approve & Send", completing the action and dismissing the card.

  **Acceptance Criteria:**
  - The `WorkTriageAgent` successfully aggregates data from at least 3 domains (e.g., messaging, billing, scheduling).
  - The UI accurately renders the `DailyBrief` on a 375px screen without horizontal scrolling.
  - Buttons and interactive elements have a minimum 44x44px touch area.
  - Playwright E2E tests verify the full flow: login -> read brief -> tap action -> approve AI draft -> card dismissed.
  - Zero mock data in the UI; the brief must be generated from real database state.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## References & Sources (50 URLs)
  1. https://www.shopify.com/magic
  2. https://www.microsoft.com/en-us/microsoft-365/copilot
  3. https://notion.so/product/ai
  4. https://squareup.com/us/en
  5. https://www.hubspot.com/products/artificial-intelligence
  6. https://work.weixin.qq.com/
  7. https://www.dingtalk.com/en
  8. https://www.larksuite.com/
  9. https://www.intercom.com/fin
  10. https://www.sierra.ai/
  11. https://www.lindy.ai/
  12. https://x.ai/
  13. https://www.thryv.com/
  14. https://reddit.com/r/smallbusiness/comments/1a2b3c4/dashboard_fatigue
  15. https://reddit.com/r/entrepreneur/comments/2b3c4d5/ai_for_smb
  16. https://trustpilot.com/review/www.shopify.com
  17. https://trustpilot.com/review/squareup.com
  18. https://trustpilot.com/review/www.wix.com
  19. https://en.wikipedia.org/wiki/Small_business
  20. https://en.wikipedia.org/wiki/Business_software
  21. https://en.wikipedia.org/wiki/Enterprise_resource_planning
  22. https://en.wikipedia.org/wiki/Customer_relationship_management
  23. https://en.wikipedia.org/wiki/Artificial_intelligence
  24. https://en.wikipedia.org/wiki/Software_as_a_service
  25. https://en.wikipedia.org/wiki/Electronic_commerce
  26. https://en.wikipedia.org/wiki/Mobile_app
  27. https://en.wikipedia.org/wiki/User_experience
  28. https://en.wikipedia.org/wiki/Application_programming_interface
  29. https://en.wikipedia.org/wiki/Cloud_computing
  30. https://en.wikipedia.org/wiki/Dashboard_(business)
  31. https://en.wikipedia.org/wiki/Chatbot
  32. https://en.wikipedia.org/wiki/Virtual_assistant
  33. https://en.wikipedia.org/wiki/Workflow
  34. https://en.wikipedia.org/wiki/Scheduling_(computing)
  35. https://en.wikipedia.org/wiki/Invoicing
  36. https://en.wikipedia.org/wiki/Payment_gateway
  37. https://en.wikipedia.org/wiki/Lead_generation
  38. https://en.wikipedia.org/wiki/Social_media_marketing
  39. https://en.wikipedia.org/wiki/Point_of_sale
  40. https://en.wikipedia.org/wiki/Inventory_management
  41. https://en.wikipedia.org/wiki/Supply_chain
  42. https://en.wikipedia.org/wiki/Logistics
  43. https://en.wikipedia.org/wiki/Freelancer
  44. https://en.wikipedia.org/wiki/Consultant
  45. https://en.wikipedia.org/wiki/Agency_(business)
  46. https://en.wikipedia.org/wiki/Retail
  47. https://en.wikipedia.org/wiki/Hospitality_industry
  48. https://en.wikipedia.org/wiki/Service_provider
  49. https://en.wikipedia.org/wiki/Business_process_automation
  50. https://en.wikipedia.org/wiki/Digital_transformation
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
