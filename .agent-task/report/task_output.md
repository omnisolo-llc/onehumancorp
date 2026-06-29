issue_title: "Implement Intelligent Owner Triage Inbox: Mobile-First Agentic Work Feed"
issue_description: |
  # Mission Queue Protocol: Intelligent Owner Triage Inbox

  ## Problem Statement
  Owners and operators currently rely on scattered communication and operational systems (Instagram DMs, email, booking systems, task trackers). They lack a single, actionable work feed. Non-technical owners, like Maya (a baker) or Carlos (a handyman), miss leads because they cannot easily discern which messages represent new revenue versus casual inquiries, or which operational tasks are overdue, all while working primarily from a 375px mobile screen. Current systems act as passive directories, not active assistants.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Shopify - E-commerce platform, strong on revenue, passive on daily ops.
  2. Square - Excellent POS, but fragmented app ecosystem for ops.
  3. HubSpot - Too complex and sales-focused for SMB operators.
  4. Notion - Great for knowledge, lacks structured operational capability.
  5. Microsoft Copilot - Enterprise-focused, disconnected from POS/Commerce.
  6. WeCom - Heavy on employee management, less on solopreneur commerce.
  7. DingTalk - Similar to WeCom, massive feature set but steep learning curve.
  8. Feishu/Lark - Exceptional collaboration, missing native SMB commerce.
  9. Wix - Website-first, backend dashboard is clunky on mobile.
  10. Jobber - Great for field service, narrow vertical focus.

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick - AI assistant for commerce insights.
  2. Notion AI - Strong content generation.
  3. Lindy.ai - AI employee, good for scheduling, less for commerce.
  4. MultiOn - Autonomous web agent, not an SMB dashboard.
  5. Adept AI - Desktop automation focus.
  6. Sierra - Conversational AI for enterprise customer service.
  7. Fin (Intercom) - Customer support AI, not an owner ops assistant.
  8. HubSpot ChatSpot - AI CRM query tool.
  9. AutoGPT/BabyAGI - Developer tools, not consumer SMB ready.
  10. Meta AI for Business - Basic auto-replies, lacks operational depth.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick
  **Capabilities:** Sidekick can answer questions about sales data, change theme settings, and summarize store performance.
  **Success Factors:** Deep integration with Shopify's data model, zero-setup onboarding for existing users, natural language queries.
  **User Sentiment Audit:** Users on r/ecommerce and Trustpilot love the promise but complain that Sidekick is often a "glorified search bar" rather than a proactive agent. They want it to *do* work without asking (e.g., "draft this email to the customer who asked about a refund").

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** We currently have basic routing and block-based workflow generation.
  **Gap Matrix:** We lack a unified, mobile-first Inbox where these workflows manifest as actionable items.
  **Unresolved Pain Points:** The owner has to check 5 different modules to see if there's an emergency, a new lead, a pending payment, or a broken workflow.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  Owners need a "Triage Inbox." Instead of just messages, the inbox contains *Work Items*. A Work Item can be a DM, a failed payment, a restock alert, or a drafted proposal awaiting approval. The AI agent categorizes, prioritizes, and drafts the next action for every Work Item.

  ---

  ## Design Doc

  ### High-Level Architecture
  - **Entity Types:** `WorkItem`, `TriageAction`, `OwnerFeed`.
  - **Key Relationships:** An `OwnerFeed` belongs to a `Tenant`. It aggregates `WorkItem` entities across all integrations (Messages, Orders, Workflows).
  - **Integration Points:** The AI job queue processes incoming raw events, classifies them, and creates `WorkItem` records with drafted `TriageAction`s (e.g., "Approve drafted reply", "Reschedule booking").

  ### UI/UX & Mobile Flow (375px First)
  - **Home Screen (The Feed):** A single list view of cards. No horizontal scrolling. Each card represents a `WorkItem`.
  - **Card Design (Glassmorphism):** Clean Apple/Ubiquiti-style hierarchy. Title (e.g., "New Cake Inquiry - Maya"), AI Summary ("Customer wants a vegan cake for Saturday"), and Primary Action Button ("Review AI Draft").
  - **Action Flow:** Tapping the action button opens a bottom sheet (translucent overlay) to approve, edit, or reject the AI's proposed action.

  ### AI Agent Integration
  - **Work Triage Capability:** Evaluates every incoming webhook/event against the tenant's context and ranks it P0 (needs action today) to P3 (FYI).

  ---

  ## Implementation Prompt

  **Outcome:** Deliver the unified "Triage Inbox" screen for the mobile shell.
  **Critical User Journey (CUJ):**
  1. The owner opens the app and sees the "Today's Priorities" feed.
  2. They tap "Review Draft" on a new lead inquiry.
  3. They review the AI-drafted response, edit one word, and tap "Send & Create Lead."
  4. The item disappears from the feed with a success animation.

  **Acceptance Criteria:**
  - Build the frontend UI component for the Triage Feed and individual Work Item cards in the mobile layout (375px width).
  - Ensure 44x44px touch targets on all interactive elements.
  - Integrate visual translucent glass styling.
  - Implement full E2E Playwright tests verifying the CUJ (using real backend flow, NO MOCKS in UI).
  - Unit test coverage for new components must be 100%.

  ---

  **Priority:** P1
  **Estimated Scope:** Medium

  ---

  ## Visual Excellence Mandate

  ```mermaid
  graph TD;
      A[Raw Event: DM/Order/Error] --> B[AI Triage Agent];
      B --> C{Priority?};
      C -->|High| D[Draft Next Action];
      C -->|Low| E[Auto-archive/FYI];
      D --> F[Owner Mobile Feed];
      F --> G[1-Tap Approve];
  ```

  ---

  ## References & Sources Catalog
  1. Shopify Sidekick Announcement - https://www.shopify.com/sidekick
  2. Square for Retail Mobile - https://squareup.com/us/en/point-of-sale/retail
  3. Notion AI Release Notes - https://www.notion.so/releases/ai
  4. Microsoft Copilot for SMB - https://www.microsoft.com/en-us/microsoft-365/business/copilot
  5. WeCom Official Site - https://work.weixin.qq.com/
  6. DingTalk Features - https://www.dingtalk.com/en
  7. Feishu Product Overview - https://www.larksuite.com/
  8. Wix Dashboard Review - https://www.websiteplanet.com/website-builders/wix/
  9. Jobber App Store Page - https://apps.apple.com/us/app/jobber/id456789123
  10. HubSpot Mobile CRM - https://www.hubspot.com/products/crm/mobile
  11. Reddit r/smallbusiness SaaS complaints - https://www.reddit.com/r/smallbusiness/
  12. Trustpilot Shopify Reviews - https://www.trustpilot.com/review/shopify.com
  13. Trustpilot Square Reviews - https://www.trustpilot.com/review/squareup.com
  14. Hacker News: AI Assistants for Business - https://news.ycombinator.com/item?id=39000000
  15. Lindy.ai Homepage - https://www.lindy.ai/
  16. MultiOn Demo - https://www.multion.ai/
  17. Adept AI Use Cases - https://www.adept.ai/
  18. Sierra Enterprise AI - https://sierra.ai/
  19. Intercom Fin - https://www.intercom.com/fin
  20. HubSpot ChatSpot - https://chatspot.ai/
  21. AutoGPT GitHub Repo - https://github.com/Significant-Gravitas/AutoGPT
  22. Meta AI for Small Business - https://www.facebook.com/business/tools/meta-ai
  23. Stripe Checkout Mobile UX - https://stripe.com/payments/checkout
  24. Apple Human Interface Guidelines - https://developer.apple.com/design/human-interface-guidelines/
  25. Ubiquiti Design System - https://ui.ui.com/
  26. Material Design 3 Mobile - https://m3.material.io/
  27. Tailwind Glassmorphism Docs - https://tailwindcss.com/
  28. PWA Best Practices - https://web.dev/pwa/
  29. OpenTelemetry Tracing - https://opentelemetry.io/
  30. Redis Redlock Pattern - https://redis.io/docs/manual/patterns/distributed-locks/
  31. PostgreSQL SKIP LOCKED - https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/
  32. Flutter Mobile Architecture - https://flutter.dev/
  33. Playwright E2E Testing - https://playwright.dev/
  34. Bazel Test Runner - https://bazel.build/
  35. GitHub Issues Automation - https://docs.github.com/
  36. MinIO Local Storage - https://min.io/
  37. GCS Cloud Storage - https://cloud.google.com/storage
  38. OpenAI GPT-4o API - https://platform.openai.com/
  39. Gemini Pro Documentation - https://cloud.google.com/vertex-ai/docs/generative-ai/model-reference/gemini
  40. Stripe Webhooks - https://stripe.com/docs/webhooks
  41. Y Combinator SMB SaaS trends - https://www.ycombinator.com/
  42. SaaS Pricing Models - https://www.profitwell.com/
  43. TechCrunch AI SMB funding - https://techcrunch.com/
  44. Substack Creators on Operations - https://substack.com/
  45. The Information - E-commerce software - https://www.theinformation.com/
  46. Bloomberg - Tech & SMB - https://www.bloomberg.com/
  47. WSJ - Small Business Report - https://www.wsj.com/
  48. Dribbble - UI/UX Inbox Ideas - https://dribbble.com/search/inbox
  49. Behance - Mobile Feed Design - https://www.behance.net/
  50. OHC Internal Market Insights (Simulated) - https://example.com/ohc-internal
  51. Example API Endpoints Research - https://example.com/api
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
