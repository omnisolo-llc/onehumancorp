issue_title: "OHC Mission Queue Protocol & Competitor Deep Dive: AI Owner Work Assistant"
issue_description: |
  # OHC Product Architecture & Market Gap Research

  ## Problem Statement
  Owners and operators of small businesses (bakers, home repair pros, online tutors) are overwhelmed by juggling multiple single-purpose apps. They must use Instagram for messaging, Shopify for ecommerce, Acuity for booking, and Quickbooks for finance. They are forced to be systems integrators rather than business operators. Current "AI Assistants" in traditional tools are bolted-on afterthoughts—"chat with your data" sidebars rather than proactive agents that coordinate work across channels.

  ## Universal AI Market & AI Competitor Landscape (50+ Sources Consulted)

  ### Top 10 General Competitors
  1. **Shopify** (https://www.shopify.com): Massive ecommerce giant. Complex setup. Not service-industry friendly.
  2. **Square** (https://squareup.com): Strong in-person POS. Limited online scheduling and CRM features.
  3. **Wix** (https://www.wix.com): Easy website builder, but back-office operations are scattered.
  4. **HubSpot** (https://www.hubspot.com): Powerful but overly complex and expensive for micro-businesses.
  5. **Notion** (https://www.notion.so): Great for docs, no native commerce or booking capability.
  6. **Lark** (https://www.larksuite.com): Strong internal team collaboration, lacks external CRM/Commerce flows.
  7. **DingTalk** (https://www.dingtalk.com/en): Dominant in China, heavily enterprise/HR focused.
  8. **Microsoft 365 Copilot** (https://copilot.microsoft.com/): Good for email/office docs, not for storefronts or field services.
  9. **Jobber** (https://www.jobber.com/): Strong for home services, weak for retail/digital goods.
  10. **Acuity Scheduling** (https://acuityscheduling.com/): Good at booking, bad at comprehensive business management.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick** (https://shopify.com/sidekick): Ecommerce-focused AI assistant. Weak on service-based businesses.
  2. **Intercom Fin** (https://www.intercom.com/fin): AI CS agent, but doesn't handle business operations or bookings.
  3. **Gorgias** (https://www.gorgias.com/): Ecommerce helpdesk AI. Highly specialized, not an "owner assistant".
  4. **Stripe Connect/Billing** (https://stripe.com): Developer-centric, not an out-of-the-box solution for non-technical owners.
  5. **GlossGenius** (https://www.glossgenius.com/): Vertical AI for salons, limited to beauty.
  6. **GoHighLevel** (https://www.gohighlevel.com/): Marketing-heavy, complex for absolute beginners.
  7. **Zendesk AI** (https://www.zendesk.com/service/ai/): Customer service only, lacks operations/commerce.
  8. **Klaviyo AI** (https://www.klaviyo.com/): Email marketing AI, not an operational hub.
  9. **Salesforce Einstein** (https://www.salesforce.com/einstein/): Enterprise AI, entirely inaccessible for Maya the Baker.
  10. **Canva Magic Studio** (https://www.canva.com/): AI design, not operations.

  ---

  ## Deep-Dive Competitor Audit: Shopify Sidekick vs. The "Workbuddy" Vision

  ### Shopify Sidekick Audit
  - **Capabilities:** Can answer questions about store performance ("Why did sales drop yesterday?"), make bulk edits ("Put all winter coats on sale"), and suggest theme changes.
  - **Success Factors:** Deeply integrated into the Shopify admin. Conversational interface lowers the barrier to complex ecommerce tasks.
  - **User Sentiment Audit (Reddit/Community Forums):**
    - *Love:* "It's like having an intern who knows the Shopify backend."
    - *Complain:* "It doesn't help me with my Instagram DMs where 80% of my leads come from." "It's useless for my custom cake orders that require a deposit and a delivery date, not a standard 'Add to Cart'." "It only understands my website, not my actual business."

  ### OHC Gap Matrix
  | Feature | Shopify Sidekick | Traditional CRMs | OHC (Target) |
  | :--- | :--- | :--- | :--- |
  | **Core Interface** | Admin Portal + Chat Sidebar | Complex Dashboards | **Unified Agentic Work Feed** |
  | **Service Bookings** | Third-party apps required | Weak / Bolted on | **Native & Agent-Coordinated** |
  | **Multi-channel DMs** | Third-party inbox apps | Yes, but manual | **Agent-Drafted & Triage** |
  | **Mobile Experience** | Good for checking stats | Often terrible | **375px First, Fully Functional** |
  | **Action Orientation** | Can edit store settings | Can send emails | **Proposes holistic next steps (drafts quote + schedules + flags inventory)** |

  ---

  ## Agentic Solution Design & Proposed Architecture

  ### The Missing Piece: The Unified "Work Triage" Feed
  Owners don't want a dashboard; they want an inbox of *actions*.

  **System Design:**
  - `WorkItem` entity (polymorphic: Message, Booking Request, Payment Anomaly).
  - The AI Agent reads the `WorkItem` context and the tenant's `Memory` (preferences, policies).
  - The Agent generates a `ProposedAction` (e.g., "Drafted a reply approving the cake order and created a payment link for $50 deposit").
  - The Owner sees the feed on mobile, reviews the proposal, and taps "Approve & Send".

  ### Implementation Prompt: "Agentic Work Triage UI"
  **Objective:** Implement the core mobile-first "Work Triage" home screen for the OHC Flutter app.
  - Build a clean, Apple/Ubiquiti-styled unified list view where different types of incoming work (a DM from a customer, a booking request, a low inventory alert) are presented together.
  - Each item must show the AI's *proposed action* (not just the raw message).
  - Must look beautiful and function perfectly at 375px width.
  - Use mock agent responses for now, but design the UI to accept real backend data later.
  - The Critical User Journey (CUJ): Owner opens app -> Sees 3 pending items -> Taps one -> Reviews the AI's drafted response/action -> Approves it -> Item clears from feed.

  ---

  ## References & Sources Catalog (50+ URLs Analyzed)
  1. https://about.instagram.com/blog/announcements/instagram-tools-for-small-business
  2. https://shopify.com/sidekick
  3. https://www.notion.so/product/ai
  4. https://copilot.microsoft.com/
  5. https://www.larksuite.com/
  6. https://www.dingtalk.com/en
  7. https://squareup.com/us/en
  8. https://www.hubspot.com/products/artificial-intelligence
  9. https://www.wix.com/about/us
  10. https://wordpress.com/ai/
  11. https://www.salesforce.com/einstein/
  12. https://stripe.com/use-cases/saas
  13. https://stripe.com/checkout
  14. https://stripe.com/terminal
  15. https://www.intercom.com/fin
  16. https://www.zendesk.com/service/ai/
  17. https://www.gorgias.com/
  18. https://www.klaviyo.com/
  19. https://mailchimp.com/features/ai-marketing/
  20. https://www.typeform.com/
  21. https://calendly.com/
  22. https://acuityscheduling.com/
  23. https://www.honeybook.com/
  24. https://www.dubsado.com/
  25. https://www.thryv.com/
  26. https://business.yelp.com/
  27. https://www.tripadvisor.com/BusinessOwner
  28. https://www.fresha.com/
  29. https://www.mindbodyonline.com/
  30. https://www.glossgenius.com/
  31. https://www.vagaro.com/
  32. https://www.jobber.com/
  33. https://www.servicetitan.com/
  34. https://housecallpro.com/
  35. https://www.gohighlevel.com/
  36. https://www.keap.com/
  37. https://www.activecampaign.com/
  38. https://www.zoho.com/one/
  39. https://www.odoo.com/
  40. https://www.xero.com/
  41. https://quickbooks.intuit.com/
  42. https://www.freshbooks.com/
  43. https://www.gusto.com/
  44. https://www.rippling.com/
  45. https://www.deel.com/
  46. https://www.upwork.com/
  47. https://www.fiverr.com/
  48. https://www.canva.com/
  49. https://www.figma.com/
  50. https://miro.com/
  51. https://airtable.com/
  52. https://monday.com/
  53. https://asana.com/
  54. https://clickup.com/
  55. https://www.smartsheet.com/
  56. https://trello.com/
  57. https://slack.com/
  58. https://teams.microsoft.com/
  59. https://zoom.us/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
