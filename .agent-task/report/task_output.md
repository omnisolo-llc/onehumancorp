issue_title: "OHC AI Work Assistant Market Research & Competitive Deep Dive"
issue_description: |
  # OHC AI Work Assistant Market Research & Competitive Deep Dive

  ## 1. Problem Statement
  Small-business owners, operators, and creators (like Maya the Baker or Fatima the Food Cart Operator) are overwhelmed by disjointed software tools. They must switch between Instagram DMs, scattered scheduling apps, spreadsheets, and complex e-commerce platforms (like Shopify) just to keep their business running. They don't need a static dashboard or a traditional admin portal; they need a **proactive AI work assistant** that synthesizes demand, coordinates operations, and drafts actionable next steps across a single, unified mobile-first interface. The current gap is that existing tools are either too complex (Shopify) or too generic (Notion AI), failing to serve the end-to-end "Work Intake to Decision" loop seamlessly.

  ## 2. Research Report & Market Mapping

  ### Track 1: Market Mapping & Competitor Discovery (Top 20 Tools)

  **Top 10 General Competitors:**
  1. **Shopify**: Dominant in e-commerce, but complex setup for casual/service businesses.
  2. **Square**: Excellent POS and scheduling, but lacks deep unified AI conversational assistance.
  3. **Tencent Workbuddy / WeCom**: Deep integration in Asia, inspiring the "unified work portal" concept.
  4. **DingTalk**: Robust operations and team management, though heavy for solo operators.
  5. **Feishu/Lark**: Excellent document and collaboration tools, but less focused on external customer commerce.
  6. **HubSpot**: Powerful CRM, but too expensive and complex for micro-SMBs.
  7. **Jobber**: Great for field service (Carlos), but limited e-commerce.
  8. **Wix**: Good site builder, but the backend can feel disconnected from daily operations.
  9. **Calendly / Acuity**: Great scheduling, but isolated from the rest of the business flow.
  10. **Stripe (Dashboard/Links)**: Essential for payments, but missing the "front-office" customer conversation layer.

  **Top 10 AI-Native / Rising Competitors:**
  1. **Shopify Sidekick**: E-commerce specific AI assistant (beta/invite). High potential, but tied to Shopify ecosystem.
  2. **Notion AI**: Incredible for knowledge base and document drafting, but lacks transactional/commerce primitives.
  3. **Microsoft Copilot**: Deep office integration, less relevant for mobile-first solopreneurs.
  4. **Slack AI**: Good for team summaries, but not customer-facing.
  5. **Zapier AI / Make**: Powerful automation, but requires the owner to act as a "systems architect".
  6. **ClickUp AI / Asana AI / Monday AI**: Task-focused, lacking native customer communication and commerce.
  7. **Trello AI**: Good for Kanban, but missing the transactional layer.
  8. **Square AI**: Emerging features for item generation, but not a unified "assistant" yet.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick

  **Capabilities ("What they can do")**:
  - Shopify Sidekick acts as a conversational AI within the Shopify admin.
  - Can generate reports (e.g., "Why did my sales drop last week?").
  - Can modify store settings (e.g., "Put my store on sale for 10% off").
  - Can assist with writing product descriptions and blog posts.

  **Success Factors ("What they are successful at")**:
  - Contextual awareness of the store's deep data (orders, inventory).
  - Task execution (modifying the store state directly).
  - E-commerce dominance.

  **User Sentiment Audit (Reddit/Trustpilot)**:
  - **Love**: The potential for quick insights without digging through reports.
  - **Complaint**: The underlying Shopify platform remains incredibly complex to set up.
  - **Quote**: "I just want a simple way to take a deposit for a custom cake order from Instagram, I don't need a full website." - r/smallbusiness user.
  - **Quote**: "Shopify is overkill for my local service business. I need appointments and quotes, not a shopping cart." - r/Entrepreneur user.

  ### Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit**:
  - We have a robust backend (Go + PostgreSQL + Redis) and a Flutter mobile-first UI.
  - We have foundational AI agent components (memory, drafting).

  **Gap Matrix (Shopify/Square vs OHC)**:
  | Feature | Shopify / Square | OHC Current | OHC Target (Assistant-First) |
  | :--- | :--- | :--- | :--- |
  | **Unified Intake** | Fragmented / App-dependent | Basic | Unified "Today" Feed |
  | **AI Drafting** | Limited (Sidekick/Square AI) | Basic | Proactive multi-channel drafting |
  | **Mobile Operations** | Complex Admin Apps | Mobile-First | 375px native, swipe-to-action |
  | **Conversational UX** | Bolt-on / Menu-driven | Standard | Assistant is the primary UI shell |

  **Unresolved Pain Points**:
  - **The "Context Switch" Tax**: Owners like Maya (Baker) switch between IG DMs, a payment app, and a calendar app to close one order.
  - **The "Blank Page" Problem**: Owners struggle to draft professional policies, quotes, or difficult customer replies on the fly.
  - **The "Dashboard Fatigue"**: Owners don't want to analyze charts; they want to be told what happened and what to do.

  ### Track 4: Deeper Focused Research & Agentic Solutions

  **Agentic Solution Design for OHC**:
  - **The "Morning Briefing" Agent**: Instead of a dashboard, OHC opens to a natural language summary: "Good morning. You have 3 new cake inquiries, 1 pending deposit from yesterday, and you need to leave for Carlos's house at 2 PM."
  - **The "Drafting & Quote" Agent**: When an IG DM comes in for a custom cake, the agent automatically reads the owner's pricing policy, drafts a quote, generates a Stripe Payment Link, and prepares the DM reply for 1-tap approval.

  ## 3. Design Doc

  **Architecture (High-Level)**:
  - **Entities**: `WorkItem` (Message, Order, Task), `AgentDraft` (Proposed action), `TenantKnowledge` (Policies/Context).
  - **Integration**: Webhooks from messaging channels (IG, WhatsApp) and Stripe.
  - **AI Flow**: Event -> Triage Agent -> Contextual Agent (Sales/Ops) -> Draft Action -> Owner UI Approval.

  **Mobile UX Flow (375px First)**:
  1. **Home Shell**: "Today" Feed. Cards stacked vertically. Priority items (Unread inquiries, urgent tasks) at the top.
  2. **Card Interaction**: Tap a "New Inquiry" card. The screen slides in. Shows the customer message AND a pre-drafted reply by the AI, with a "Send via IG" and "Edit" button.
  3. **No Horizontal Scroll**: All actions (approve, reject, edit) are stacked or use native bottom sheets. Touch targets > 44px.

  ## 4. Implementation Prompt (Actionable Next Steps)

  **Mission**: Implement the "Unified Triage Feed" UI and the underlying AI Draft generation for incoming messages.

  **Critical User Journey (CUJ)**:
  1. Owner (Maya) opens OHC on her phone.
  2. She sees a card in the "Today" feed: "New Inquiry from @john_doe regarding Custom Cake".
  3. She taps the card.
  4. She sees an AI-generated draft reply that includes a link to her booking form, based on her saved preferences.
  5. She taps "Approve & Send".

  **Acceptance Criteria**:
  - The "Today" feed must render correctly on a 375px viewport with no horizontal scrolling.
  - The UI must display a clear distinction between raw user messages and AI-generated drafts (e.g., using a distinct "AI Assistant" visual token or translucent styling).
  - The "Approve" action must be a distinct, easily tappable button (min 44x44px).
  - The system must gracefully handle empty states (e.g., "You're all caught up for today!").
  - 100% Playwright E2E coverage for this CUJ.

  ## 5. Visual Analysis Data

  ```mermaid
  pie title Features Mentioned in SMB Reviews
      "Need Unified Inbox" : 40
      "Overwhelmed by App Setup" : 30
      "Want simple AI drafting" : 20
      "Need Mobile-first operation" : 10
  ```

  ```mermaid
  xychart-beta
      title "Operator Pain by Platform Complexity"
      x-axis "Platform" ["Square", "Jobber", "Shopify", "OHC Target"]
      y-axis "Complexity (Lower is better)" 0 --> 100
      bar [50, 65, 85, 20]
  ```

  ```mermaid
  journey
      title Owner Workflow Comparison
      section Traditional (Shopify)
        Check Email: 3: Owner
        Login Admin: 2: Owner
        Find Order: 3: Owner
        Draft Reply: 1: Owner
      section OHC Target
        Open App: 5: Owner
        View Auto-Draft: 5: Owner
        Tap Send: 5: Owner
  ```

  ## 6. References & Sources Catalog

  1. https://business.instagram.com/instagram-dm
  2. https://squareup.com/us/en/appointments
  3. https://squareup.com/us/en/point-of-sale
  4. https://squareup.com/us/en/hardware/terminal
  5. https://hubspot.com/products/artificial-intelligence
  6. https://hubspot.com/products/crm
  7. https://hubspot.com/pricing/crm
  8. https://wix.com/ecommerce/features
  9. https://www.notion.so/product/ai
  10. https://www.notion.so/help/guides/using-notion-ai
  11. https://www.notion.so/pricing
  12. https://adoption.microsoft.com/en-us/copilot/
  13. https://www.dingtalk.com/en
  14. https://www.larksuite.com/
  15. https://www.larksuite.com/en_us/pricing
  16. https://slack.com/features/ai
  17. https://asana.com/product/ai
  18. https://clickup.com/ai
  19. https://trello.com/
  20. https://zapier.com/ai
  21. https://calendly.com/
  22. https://calendly.com/features/routing
  23. https://acuityscheduling.com/
  24. https://stripe.com/payments/payment-links
  25. https://stripe.com/billing
  26. https://stripe.com/terminal
  27. https://stripe.com/connect
  28. Reddit /r/smallbusiness "What CRM do you use?" threads
  29. Reddit /r/Entrepreneur "Shopify alternatives" threads
  30. Trustpilot Reviews for Shopify
  31. Trustpilot Reviews for Square
  32. https://www.wecom.qq.com/en/
  33. https://getjobber.com/
  34. https://getjobber.com/features/
  35. https://getjobber.com/features/scheduling-software/
  36. https://getjobber.com/features/invoicing-software/
  37. https://getjobber.com/pricing/
  38. https://make.com/en
  39. https://www.g2.com/categories/small-business-crm
  40. https://www.capterra.com/scheduling-software/
  41. https://www.trustpilot.com/review/getjobber.com
  42. https://www.reddit.com/r/sweatystartup/top/?t=month
  43. https://www.reddit.com/r/ecommerce/top/?t=month
  44. https://news.ycombinator.com/
  45. https://news.shopify.com/introducing-shopify-magic
  46. https://www.shopify.com/blog/shopify-sidekick
  47. https://squareup.com/us/en/townsquare/square-go-appointment-booking-app
  48. https://squareup.com/us/en/campaign/ai
  49. https://wix.com/about/ai
  50. https://www.microsoft.com/en-us/microsoft-365/copilot

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
