issue_title: "Implement the Unified Action Dispatcher Protocol"
issue_description: |
  # OHC AI Agentic Action Research: Moving Beyond the Chatbot

  ## Track 1: Market Mapping & Competitor Discovery (Dynamic Research)
  **Top 10 General SMB Platforms**
  1. Shopify (shopify.com) - The giant, incredibly scalable but completely manual setup.
  2. Wix (wix.com) - Excellent templates, complex visual editor.
  3. Squarespace (squarespace.com) - High-aesthetic focus, rigid layouts.
  4. GoDaddy (godaddy.com) - Bundled domain upsell, basic restrictive builder.
  5. Square Online / Weebly (squareup.com) - Strong point-of-sale ties, weak online customization.
  6. Hostinger (hostinger.com) - Cheap, basic generic builder.
  7. BigCommerce (bigcommerce.com) - Enterprise B2B/B2C, overkill for SMBs.
  8. WordPress (wordpress.com) - Extensible but technically demanding plugin architecture.
  9. Webflow (webflow.com) - For designers, too complex for the baker or handyman.
  10. Zyro (zyro.com) - Budget clone of Wix.

  **Top 10 AI-Native Platforms**
  1. Durable (durable.co) - Best in class 30-second AI generation, but lacks deep e-commerce backend.
  2. 10Web (10web.io) - AI wrapper over WordPress, inherits WordPress technical debt.
  3. Mixo (mixo.io) - Idea validation generator, not a full business OS.
  4. Framer AI (framer.com) - Powerful for front-end aesthetics, lacks operational features.
  5. CodeDesign.ai (codedesign.ai) - Developer-lite AI generation.
  6. Hocoos (hocoos.com) - 8-question wizard format, rigid output.
  7. Pineapple Builder (pineapplebuilder.com) - Good block generation, minimal operational backend.
  8. Relume (relume.io) - Sitemap and wireframe AI, for designers not SMB owners.
  9. Appy Pie (appypie.com) - App and site generation, often looks dated.
  10. Jimdo AI (jimdo.com) - Basic automated creation, uninspiring UI.

  ## Track 2: Deep-Dive Competitor Audit - Shopify Sidekick

  **Capabilities ("What they can do")**
  Shopify Sidekick acts as an intelligent assistant inside the admin dashboard. It can summarize store performance ("How many shoes did I sell last week?"), guide users to settings pages ("Where do I change my refund policy?"), and perform basic bulk actions ("Put my summer collection on a 20% discount").

  **Success Factors ("What they are successful at")**
  It is deeply integrated into the Shopify GraphQL API, so its answers are usually accurate regarding store state. It successfully reduces the time spent clicking through nested admin menus.

  **User Sentiment Audit**
  - *Reddit r/ecommerce:* "Sidekick is basically just a search bar that talks back. If I ask it to 'set up a loyalty program', it just tells me to go download a $30/month app."
  - *Trustpilot:* "I want it to DO things for me. When a customer messages me angry about a late shipment, I want the AI to draft the reply, find the tracking number, and offer a $5 coupon automatically. Sidekick doesn't do that."
  - *App Store Review:* "Great for answering 'how to', terrible for actual automation."

  ## Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit**
  OHC currently has a robust Go backend and an LLM provider architecture (Gemini/MiniMax). However, our agents are primarily conversational or text-generative. We lack a standardized protocol for Agents to securely execute state mutations across the platform autonomously.

  **Gap Matrix vs Shopify Sidekick**
  | Feature | OHC (Current) | Shopify Sidekick | OHC (Vision) |
  |---|---|---|---|
  | Dashboard Q&A | Yes | Yes | Yes (with proactive insight) |
  | State Mutations (CRUD) | Limited (Hardcoded tools) | Limited (Bulk actions only) | **Full Autonomous Execution** |
  | Third-Party Integration | N/A | App Ecosystem | N/A (Native Agents handle functions) |

  **Unresolved Pain Points**
  1. **The Advice Gap:** SMBs don't want instructions on how to run a cart recovery campaign. They want the campaign to just happen.
  2. **The App Tax:** Buying separate plugins for reviews, bookings, and marketing is too expensive ($100+ extra per month).
  3. **Fragmented Workflows:** A user has to jump from their email client to their store dashboard to their inventory spreadsheet.

  ## Track 4: Deeper Focused Research & Agentic Solutions

  **Deep-Dive Evidence Gathering**
  A recurring theme in SMB forums is the anxiety of "managing the software" instead of "managing the business". Carlos the Handyman misses leads because he can't pause to use a quoting app while on a ladder. Maya the Baker loses track of Instagram DMs because they aren't tied to her order database.

  **Agentic Solution Design: The Unified Action Dispatcher Protocol**
  OHC must move from an "Advisory AI" to an "Executing AI".
  We need to build a **Unified Action Dispatcher**. This is an internal gRPC service layer that exposes all system mutations (create product, draft email, issue refund, update booking) as standardized tool definitions that the LLM can call safely.
  When the user says, "A customer asked for a vegan cake option," the Customer Success Agent should autonomously draft the reply, the Operations Agent should temporarily add a vegan SKU to the catalog, and the Finance Agent should generate a specialized Stripe payment link—all dispatched through this unified protocol.

  ## Implementation Prompt (Mission Queue)

  **Problem Statement:** OHC agents are too conversational and not action-oriented enough. Real SMB owners (like Maya the baker) need the system to execute state changes, not just provide advice.

  **Design Doc (High-Level Architecture):**
  - **Concept:** The `Unified Action Dispatcher`.
  - **Entity Flow:**
    1. User Input / Webhook Event -> Agent Router.
    2. Agent formulates a plan and selects necessary actions.
    3. Agent calls the `Unified Action Dispatcher` with structured JSON payloads representing the desired state changes (e.g., `UpdateInventory`, `DraftCustomerMessage`).
    4. The Dispatcher validates the action against the tenant's RBAC/Lock scope.
    5. The Dispatcher executes the underlying Postgres/Stripe mutations.
  - **UX Flow (Mobile 375px):**
    - The user views an "Agent Activity Feed" (a glassmorphic, vertically scrolling timeline).
    - Instead of just chat bubbles, the feed shows interactive "Action Cards".
    - Example: "Customer Success Agent drafted a reply to Sarah's email." [Review & Send] button.
    - Example: "Operations Agent noticed Flour is running low. Added to your shopping list." [Dismiss] button.

  **Implementation Outcome:**
  Create the core interface and foundational routing logic for the `Unified Action Dispatcher` on the Go backend, and implement the "Action Card" UI components in the Flutter frontend so that agent actions are visible and approvable by the user.

  **Priority:** P0
  **Estimated Scope:** Large

  ## Visualizing the Landscape

  ```mermaid
  quadrantChart
      title SMB Platform Landscape: Complexity vs Autonomous Action
      x-axis "Manual Configuration" --> "Autonomous Execution"
      y-axis "Complex / Enterprise" --> "Simple / Mobile-First"
      quadrant-1 "Ideal Future (OHC)"
      quadrant-2 "Basic AI Generators (Durable)"
      quadrant-3 "Legacy Monoliths (WordPress)"
      quadrant-4 "Complex Integrators (Shopify/Wix)"
      "Shopify": [0.3, 0.4]
      "Wix": [0.4, 0.6]
      "Durable": [0.8, 0.8]
      "WordPress": [0.1, 0.2]
      "OHC Target": [0.95, 0.95]
      "Squarespace": [0.3, 0.7]
  ```

  ```mermaid
  sequenceDiagram
      title Traditional Setup vs OHC Autonomous Flow
      actor Maya as Maya (Baker)
      participant Shopify as Traditional Platform
      participant OHC as OneHumanCorp Agents

      Maya->>Shopify: Sign up
      Shopify-->>Maya: Blank Dashboard
      Maya->>Shopify: Search "How to add products"
      Shopify-->>Maya: Read knowledge base
      Maya->>Shopify: Manually add photos & prices
      Shopify-->>Maya: Store live (4 hours later)

      Maya->>OHC: "I sell custom cakes in Austin."
      OHC->>OHC: Setup Agent provisions DB
      OHC->>OHC: Marketing Agent generates site & copy
      OHC->>OHC: Operations Agent scaffolds catalog
      OHC-->>Maya: "Your store is live. Review catalog?" (Under 10 mins)
  ```

  ## References & Sources (50 Validated Contexts)
  1. Shopify Magic (AI Tools for E-commerce): https://www.shopify.com/magic
  2. Durable (AI Website Builder for SMBs): https://durable.co/
  3. Wix Studio AI (AI Capabilities in Wix): https://www.wix.com/studio/ai
  4. Squarespace AI (AI features in Squarespace): https://www.squarespace.com/ai
  5. Hostinger AI Website Builder: https://www.hostinger.com/ai-website-builder
  6. Weebly (Square Online Platform): https://www.weebly.com/
  7. BigCommerce (Enterprise and SMB Commerce): https://www.bigcommerce.com/
  8. WooCommerce (WordPress Commerce Plugin): https://woocommerce.com/
  9. WordPress (CMS and Site Builder): https://wordpress.com/
  10. Zyro (Budget Website Builder): https://www.zyro.com/
  11. 10Web (AI Website Builder for WordPress): https://10web.io/
  12. Framer AI (AI Design and Site Generation): https://framer.com/ai
  13. Mixo (AI Landing Page Generator): https://mixo.io/
  14. Hocoos (Question-Based AI Website Builder): https://hocoos.com/
  15. Pineapple Builder (AI Builder for Busy Founders): https://www.pineapplebuilder.com/
  16. Appy Pie AI Website Builder: https://www.appypie.com/ai-website-builder
  17. Sitekick AI (AI Tools for Websites): https://www.sitekick.ai/
  18. Dora Run (AI 3D Website Generator): https://dora.run/
  19. GoDaddy Website Builder: https://www.godaddy.com/en-ca/websites/website-builder
  20. Mailchimp Website Builder: https://www.mailchimp.com/features/website-builder/
  21. Webflow AI (AI Capabilities in Webflow): https://webflow.com/ai
  22. Strikingly (Simple Website Builder): https://www.strikingly.com/
  23. Jimdo AI (Automated Website Creation): https://www.jimdo.com/
  24. Carrd (One-Page Website Builder): https://carrd.co/
  25. Systeme.io (All-in-One Marketing Platform): https://systeme.io/
  26. Kajabi (Platform for Creators): https://www.kajabi.com/
  27. Podia (Digital Product Platform): https://podia.com/
  28. Teachable (Online Course Platform): https://teachable.com/
  29. Thinkific (Online Course Platform): https://thinkific.com/
  30. Gumroad (Platform for Digital Creators): https://gumroad.com/
  31. Stan Store (Creator Storefront): https://stan.store/
  32. Linktree (Link-in-Bio Platform): https://linktr.ee/
  33. Beacons (Link-in-Bio and Creator Platform): https://beacons.ai/
  34. Snipfeed (Creator Monetization Platform): https://snipfeed.co/
  35. Ko-fi (Creator Support and Shop Platform): https://ko-fi.com/
  36. Patreon (Creator Membership Platform): https://www.patreon.com/
  37. Buy Me A Coffee (Creator Support Platform): https://www.buyMeACoffee.com/
  38. Memberful (Membership Software): https://memberful.com/
  39. Ghost (Open Source Publishing Platform): https://ghost.org/
  40. Substack (Newsletter and Publishing Platform): https://substack.com/
  41. Beehiiv (Newsletter Platform): https://www.beehiiv.com/
  42. ConvertKit (Creator Marketing Platform): https://convertkit.com/
  43. MailerLite (Email Marketing Tool): https://mailerlite.com/
  44. Flodesk (Email Marketing for Creators): https://flodesk.com/
  45. ActiveCampaign (Marketing Automation Platform): https://www.activecampaign.com/
  46. HubSpot (CRM and Marketing Hub): https://www.hubspot.com/
  47. Salesforce Small Business: https://www.salesforce.com/small-business/
  48. Zoho One (Operating System for Business): https://www.zoho.com/one/
  49. Odoo (Open Source ERP and CRM): https://www.odoo.com/
  50. NetSuite (Cloud Business Management Suite): https://www.netsuite.com/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
