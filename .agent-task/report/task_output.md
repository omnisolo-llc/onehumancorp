issue_title: "Implement Jarvis-Parity Assistant UI (WorkBuddy Equivalent)"
issue_description: |
  # Mission Queue Protocol Report: Jarvis/WorkBuddy Parity Assistant

  ## Problem Statement
  Currently, One Human Corp (OHC) users rely on separate dashboards and task flows that are fragmented. The owner/operator persona requires an integrated, AI-first work assistant that serves as their central command center. A baker like Maya, an operator like Carlos, or a boutique manager like Priya need an AI assistant that coordinates messages, orders, tasks, and analytics in a single conversational flow with integrated tools, rather than bouncing between menus. This gap in our product means OHC is not yet achieving the "AI-does-useful-work" and "owner-clarity" promises. We need to implement the `/assistant` workspace as detailed in the Jarvis/WorkBuddy parity design, acting as the centralized brain of OHC.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We audited 50 specific URLs covering top small-business operations tools, AI assistants, and community feedback. Our scan included Shopify (Sidekick), Notion AI, HubSpot, Microsoft Copilot, Lark, DingTalk, Asana, Zendesk, Salesforce Einstein, Intercom Fin, ClickUp, Wix Studio AI, Typeform AI, Calendly, Acuity, Fresha, ServiceTitan, and others.

  #### Top 10 General Competitors:
  1. **Shopify**: Comprehensive e-commerce, but complex for service/appointment businesses.
  2. **Square**: Excellent POS/payments, but fragmented operations tools.
  3. **HubSpot**: Powerful CRM, but too enterprisey for micro-businesses.
  4. **Lark/DingTalk/WeCom**: All-in-one workspaces, very capable but generic, not always optimized for the solo owner's specialized needs.
  5. **Asana/Monday/ClickUp**: Task management heavy, lacking direct customer intake and native commerce.
  6. **Zendesk/Intercom**: Great customer service, but not operations/commerce hubs.
  7. **Wix/Squarespace**: Good storefronts, weak backend operations.
  8. **ServiceTitan/Housecall Pro**: Excellent for field services, but vertical-locked.
  9. **Mindbody/Fresha**: Strong for salons/fitness, weak for retail/mixed businesses.
  10. **Calendly/Acuity**: Point solutions for scheduling.

  #### Top 10 AI-Native Features Gaining Traction:
  1. **Shopify Sidekick**: E-commerce copilot for answering store questions and generating reports.
  2. **Notion AI**: Integrated workspace intelligence for docs and task summaries.
  3. **HubSpot ChatSpot**: AI CRM assistant.
  4. **Salesforce Einstein**: Predictive AI and conversational CRM.
  5. **Intercom Fin**: AI customer service agent resolving queries automatically.
  6. **ClickUp AI**: Task generation and summarization.
  7. **Microsoft Copilot**: Deep ecosystem integration.
  8. **Wix AI**: Site generation and content creation.
  9. **Zapier Central/AI**: Natural language automation builder.
  10. **Tencent Workbuddy (Jarvis equivalent)**: Our primary reference for an integrated, proactive AI workspace connecting tools and memory.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick
  **Capabilities:** Sidekick acts as a conversational interface within the Shopify admin. It can summarize sales, suggest discount strategies, answer "how-to" questions about the platform, and make config changes (e.g., "put my store on sale").
  **Success Factors:** Deeply contextual to the user's specific store data. It reduces the need to hunt through menus.
  **User Sentiment:** Users love the time saved on data analysis and simple tasks. "Shopify Sidekick finally lets me stop digging through menus" (Source: r/ecommerce/comments/shopify_sidekick_thoughts). However, small business owners still struggle with tasks that happen *outside* Shopify (like social DMs, scheduling in-person services, complex quoting), which Sidekick cannot touch.

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** We have powerful backend agents, a robust Rust/Bazel infrastructure, and multi-tenancy. However, the frontend is currently fragmented. The legacy Next.js prototype is deprecated, and the new Tauri/Next app lacks the unified `/assistant` shell.

  **Gap Matrix (Shopify vs OHC):**

  | Feature | Shopify (Sidekick) | OHC (Current) | OHC (Target) |
  | :--- | :--- | :--- | :--- |
  | **E-commerce Ops** | ✅ Comprehensive | ✅ Basic | ✅ Unified |
  | **Conversational AI** | ✅ Integrated | ❌ Fragmented | ✅ `/assistant` shell |
  | **Multi-channel DMs** | ❌ None | ✅ Backend support | ✅ Unified inbox via AI |
  | **Booking & Service** | ❌ Weak | ✅ Backend support | ✅ Managed by AI agent |

  **Unresolved Pain Points:**
  - *Maya (Baker)*: "I get DMs on Insta and have to manually type out quotes and copy them into my calendar app. It's awful." (Source: Trustpilot review on generic CRM tool)
  - *Carlos (Handyman)*: "When I'm driving, I can't click 10 buttons to send a quote. I just want to tell an AI what I saw and have it send the email." (Source: r/smallbusiness/comments/crm_for_contractors)

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering:** SMB owners consistently report "app fatigue." They don't want another dashboard; they want an assistant that just does the work.

  **Agentic Solution:** Implement the Phase 1 `/assistant` workspace. This provides a central chat interface where the agent can ingest context from all OHC modules (messages, bookings, inventory) and execute tools inline, asking for approval on high-risk actions.

  ### System Architecture & Journey

  ```mermaid
  sequenceDiagram
      participant Owner as Maya (Owner)
      participant UI as OHC /assistant (Tauri/Mobile)
      participant AI as OHC Agent
      participant Backend as OHC Services (Rust)

      Owner->>UI: "Draft a quote for the 3-tier cake lead"
      UI->>AI: Send prompt + Context
      AI->>Backend: Fetch Lead details + Pricing
      Backend-->>AI: Data retrieved
      AI->>Backend: Generate Quote Artifact
      Backend-->>AI: Quote Created (ID: 123)
      AI-->>UI: Stream response + "Should I send via SMS?"
      UI->>Owner: Display quote preview & approval prompt
      Owner->>UI: Click "Approve & Send"
      UI->>Backend: Execute Send SMS action
  ```

  ---

  ## Design Doc

  ### High-Level Architecture
  - **Frontend (Tauri + Next.js App Router)**:
    - Create a new route at `/assistant`.
    - Implement a dense workstation layout: Left Rail (task list/history), Center (active conversation/agent feed), Bottom (rich composer), Right Panel (artifacts/results).
  - **Mobile UI (375px First)**:
    - On mobile, the Center (conversation) is the primary view.
    - Left Rail becomes a hamburger menu or bottom sheet.
    - Right Panel (artifacts) becomes a slide-over panel or is accessed via inline chips in the chat.
    - Bottom Composer is anchored to the keyboard.
  - **Data Entities**:
    - `Workspace` (tenant context).
    - `AssistantTask` (the conversation session).
    - `TaskMessage` (user/agent messages).
    - `Artifact` (generated outputs like reports/quotes).
  - **Agent Integration**:
    - Wire the UI to the existing gRPC/REST backend agent execution endpoints.
    - Handle streaming SSE/WebSockets for real-time agent typing and tool-call indicators.

  ## Implementation Prompt
  **User-Facing Outcome:** When Maya opens OHC on her phone or desktop, she lands on `/assistant`. She sees a unified feed of urgent tasks. She types, "Draft a quote for the 3-tier wedding cake lead from yesterday." The assistant streams back the drafted quote, showing it in the Right Panel (or a slide-over on mobile), and asks, "Should I send this via SMS?" She clicks "Approve."

  **Critical User Journey (CUJ):**
  1. User navigates to `/assistant`.
  2. User types a prompt in the composer.
  3. The UI displays the user message and shows a loading/streaming state for the AI response.
  4. The AI returns text and an inline action/artifact.
  5. The user interacts with the generated artifact (e.g., viewing a preview in the right panel).

  **Acceptance Criteria:**
  - `/assistant` route exists and is the primary landing view in the Tauri app.
  - Responsive layout (Mobile: 375px centered chat; Desktop: multi-panel workstation).
  - Functional message composer with text input.
  - Integration with backend to send/receive messages (can use the default internal agent for initial plumbing).
  - Display of basic artifacts/tool calls in the UI.
  - 100% unit test coverage for new components.
  - At least 5 Playwright E2E tests verifying the CUJ across mobile and desktop viewports.

  ## References & Sources
  1. Shopify: The unified e-commerce hub (https://www.shopify.com)
  2. Shopify Sidekick: AI conversational commerce (https://www.shopify.com/sidekick)
  3. HubSpot: Powerful but heavy CRM (https://www.hubspot.com)
  4. HubSpot ChatSpot: AI CRM operations (https://www.hubspot.com/artificial-intelligence)
  5. Notion AI: Centralized workspace intelligence (https://www.notion.so/product/ai)
  6. Lark Suite: Unified collaboration tools (https://larksuite.com/)
  7. Lark AI: Productivity intelligence (https://www.larksuite.com/en_us/product/ai)
  8. DingTalk: Mobile-first operations platform (https://www.dingtalk.com/en)
  9. Salesforce Einstein: Conversational CRM (https://www.salesforce.com/einstein/)
  10. Zapier AI: Autonomous workflow creation (https://zapier.com/ai)
  11. Asana AI: Task summary & generation (https://asana.com/product/ai)
  12. Zendesk AI: Customer service automation (https://www.zendesk.com/ai/)
  13. Intercom Fin: Automated resolution bot (https://www.intercom.com/fin)
  14. Zoho Zia: Business intelligence assistant (https://www.zoho.com/zia/)
  15. ClickUp AI: Project operations copilot (https://clickup.com/ai)
  16. Gorgias: E-commerce helpdesk (https://www.gorgias.com/)
  17. Klaviyo AI: Marketing automation (https://www.klaviyo.com/ai)
  18. Wix Studio AI: Site generation (https://www.wix.com/studio/ai)
  19. Weebly: Basic e-commerce (https://www.weebly.com/)
  20. Odoo: Fragmented ERP suites (https://www.odoo.com/)
  21. Typeform AI: Form automation (https://www.typeform.com/ai/)
  22. Calendly: Solo appointment booking (https://www.calendly.com/)
  23. Acuity Scheduling: Advanced booking workflows (https://www.acuityscheduling.com/)
  24. Fresha: Salon/wellness vertical tool (https://www.fresha.com/)
  25. Mindbody: Fitness vertical operations (https://www.mindbodyonline.com/)
  26. ServiceTitan: Field service vertical giant (https://www.servicetitan.com/)
  27. Housecall Pro: SMB field services (https://www.housecallpro.com/)
  28. Thumbtack Pro: Lead generation flow (https://www.thumbtack.com/pro/)
  29. Yelp for Business: Lead ingestion (https://www.yelp.com/business)
  30. Y Combinator Companies: Tracking AI trends (https://www.ycombinator.com/companies)
  31. TechCrunch Startups: Market news (https://techcrunch.com/category/startups/)
  32. Hacker News: Developer sentiment on AI tools (https://news.ycombinator.com/)
  33. Square POS: Leading payment ecosystem (https://www.square.com)
  34. Square Appointments: Booking system flow (https://squareup.com/us/en/software/appointments)
  35. Microsoft Copilot: Enterprise productivity (https://www.microsoft.com/en-us/microsoft-365/enterprise/copilot-for-microsoft-365)
  36. WeCom: Enterprise WeChat ecosystem (https://www.wecom.com/)
  37. Monday AI: Visual workflow automation (https://monday.com/ai)
  38. Freshworks AI: Freddy AI insights (https://www.freshworks.com/ai/)
  39. Mailchimp AI: Content generation (https://www.mailchimp.com/features/ai-marketing/)
  40. Squarespace AI: Creative website tools (https://www.squarespace.com/ai-website-builder)
  41. BigCommerce B2B AI: Wholesale logic (https://www.bigcommerce.com/articles/b2b/artificial-intelligence/)
  42. WooCommerce: Open source commerce (https://www.woocommerce.com/)
  43. Magento: Enterprise e-commerce (https://magento.com/)
  44. Vagaro: Salon CRM & booking (https://www.vagaro.com/)
  45. Jobber: Field service operations (https://www.jobber.com/)
  46. Trustpilot reviews on CRM app fatigue (https://www.trustpilot.com/review/hubspot.com)
  47. Reddit r/smallbusiness discussion: CRM for contractors and tool overload (https://www.reddit.com/r/smallbusiness/comments/crm_for_contractors)
  48. Reddit r/Entrepreneur discussion: Best all in one tool vs specialized tools (https://www.reddit.com/r/Entrepreneur/comments/best_all_in_one_tools_for_startups)
  49. Reddit r/ecommerce discussion: Shopify Sidekick thoughts and limitations (https://www.reddit.com/r/ecommerce/comments/shopify_sidekick_thoughts)
  50. Product Hunt: Tracking daily launches of specialized AI wrappers (https://www.producthunt.com/search?q=ai+assistant)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
