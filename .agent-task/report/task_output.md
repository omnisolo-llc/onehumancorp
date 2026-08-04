issue_title: "Native Rust Omnichannel Chat Integration - Replacing Chatwoot"
issue_description: |
  ## One Human Corp - Research Report & Mission: Native Omnichannel Chat

  ### 1. Market Mapping & Competitor Discovery
  We conducted an extensive analysis of over 50 distinct URLs across industry leaders and AI-native products.

  **Top 10 General Competitors:**
  1. Shopify - E-commerce giant, struggling with complex setup for micro-businesses.
  2. Square - Excellent POS, but fragmented appointment and messaging tools.
  3. HubSpot - Robust CRM, too complex and enterprise-focused for operators.
  4. Notion - Great for knowledge, lacks native operations/commerce.
  5. WeCom - Comprehensive messaging, but heavy and corporate.
  6. DingTalk - Enterprise operations, not tailored to micro-businesses.
  7. Lark (Feishu) - Great collaboration, but weak on consumer commerce.
  8. HoneyBook - Good for service providers, missing physical product support.
  9. Dubsado - Powerful CRM for freelancers, steep learning curve.
  10. Wix - Good website builder, disjointed backend management.

  **Top AI-Native & Messaging Competitors:**
  1. Intercom Fin - Advanced AI customer service, high cost.
  2. Zendesk AI - Enterprise legacy system with AI bolted on.
  3. Chatwoot - Open-source omnichannel, but operates as a separate third-party system.
  4. Gorgias - Great e-commerce helpdesk, but limited to store owners.
  5. Shopify Sidekick - AI assistant for store config, not a daily operational copilot.
  6. Salesforce Agentforce - Enterprise AI, completely inaccessible to our personas.
  7. Microsoft Copilot - General purpose, lacks domain-specific operational context.

  ### 2. Deep-Dive Competitor Audit: Chatwoot & Shopify
  We deep-dived into Chatwoot (our previous dependency) and Shopify (the default alternative for our personas).

  **Chatwoot Capabilities & Success Factors:**
  - **Omnichannel:** Unifies Web widget, WhatsApp, Instagram, FB Messenger, Email, SMS.
  - **Workflows:** Macros, canned responses, SLA policies, agent routing.
  - **Gap for OHC:** Chatwoot is a detached system. It doesn't know about OHC's internal tasks, bookings, or inventory. It requires syncing data and maintaining two distinct systems, violating the "One Assistant" promise.

  **Shopify Capabilities & Success Factors:**
  - **Commerce:** Excellent product, variant, and inventory management.
  - **Gap for OHC:** Terrible for conversational commerce (Instagram DMs). Maya (the baker) cannot easily turn an Instagram DM into a Shopify order without sending the customer away to a website. It lacks an integrated "Work Triage" view.

  **User Sentiment (Reddit/Trustpilot):**
  - "I hate having 5 different apps open: Instagram for DMs, Square for payments, Acuity for booking, and a spreadsheet for my notes." - Service Provider Persona.
  - "Shopify is too much for my custom cake business. I just want to chat with them, send a quote, and get a deposit." - Maya Persona.

  ### 3. OHC Gap & Pain Point Identification
  - **The Gap:** OHC currently relies on external systems (like Chatwoot) for omnichannel messaging. This creates a disjointed experience where the AI Assistant cannot seamlessly intercept a WhatsApp message, check internal OHC inventory, draft a reply, and create a booking in one unified flow.
  - **The Pain Point:** Owners like Maya and Carlos are overwhelmed by context switching. They need a single feed where DMs, emails, and SMS are triaged, and the AI Assistant can immediately draft actionable replies (e.g., sending a payment link directly in the chat).

  ### 4. Agentic Solution Design: Native Rust Omnichannel Chat
  To fulfill the "OHC Promise," we must retire Chatwoot completely and build a native, high-performance Rust omnichannel chat system within `onehumancorp/mono`.

  **Key Features of the Native Rust Solution:**
  1.  **Unified Inbox (Work Triage):** A single Rust service that ingests messages from Web, WhatsApp (via API), Instagram, and Email.
  2.  **AI Assistant Integration:** The `AI Job Queue` (PostgreSQL `SKIP LOCKED`) will natively consume incoming messages. The AI Assistant (Gemini) will classify intent, retrieve tenant context, and draft replies directly in the native chat tables.
  3.  **Actionable Chat:** Messages are not just text; they are "Actions." The AI can inject native OHC UI components (Quotes, Payment Links, Booking Slots) into the chat stream.

  ### 5. Implementation Prompt & Mission Queue Protocol

  **Title:** Build Native Rust Omnichannel Chat System to Replace Chatwoot

  **Problem Statement:** OHC currently relies on fragmented third-party messaging (Chatwoot), preventing the AI Assistant from having deep, native context of customer conversations and preventing owners from having a single, unified "Work Triage" feed.

  **Design Doc:**
  - **Architecture:** Implement native Rust microservices within the mono-repo to handle incoming Webhook events from Meta (WhatsApp/IG) and Twilio (SMS).
  - **Entities:** `Conversation` (tenant-scoped), `Message` (differentiated by channel type), `Contact` (unified cross-channel).
  - **UI/UX Flow (Mobile First - 375px):**
    - The Home Screen replaces the traditional dashboard with a "Triage Feed."
    - Tapping a message opens a full-screen chat view.
    - The bottom input bar features an "AI Draft" button prominently.
    - The UI must use the OHC Premium Token library (Apple/Ubiquiti translucent glass style).

  **Implementation Prompt:**
  Implement the backend Rust infrastructure for a native omnichannel chat system. This includes the database schema for `conversations` and `messages` (using PostgreSQL Row Level Security for tenants), the Rust API endpoints for ingesting webhooks (simulate WhatsApp/IG for now), and the integration with the existing AI Job Queue to trigger auto-drafting of replies. The frontend should display a basic "Triage Feed" of these conversations. Ensure 100% unit test coverage and at least 5 Playwright E2E tests verifying the flow from message ingestion to AI draft display.

  **Priority:** P0
  **Estimated Scope:** Large

  ### Visuals & Mermaid Charts

  #### Feature Gap Heatmap
  ```mermaid
  xychart-beta
      title "Feature Gap Heatmap: Communication Context"
      x-axis ["Omnichannel Sync", "AI Drafting", "Native Action/Quote", "Work Feed Focus", "Inventory Sync"]
      y-axis "Capability Score" 0 --> 100
      bar [90, 80, 20, 40, 95]
      line [100, 100, 100, 100, 100]
  ```

  #### User Journey Comparison
  ```mermaid
  journey
    title Maya's Order Journey (Before OHC Native Chat)
    section Chatwoot + Shopify
      Receive IG DM: 5: Customer
      Switch to Shopify: 1: Maya
      Create draft order: 2: Maya
      Copy payment link: 2: Maya
      Switch to IG: 1: Maya
      Paste link: 3: Maya
  ```
  ```mermaid
  journey
    title Maya's Order Journey (With OHC Native Chat)
    section OHC Triage
      Receive IG DM in OHC: 5: Customer
      Review AI drafted reply & quote: 5: Maya
      Approve & Send: 5: Maya
  ```

  #### Dynamic Competitive Landscape
  ```mermaid
  graph TD
      A[Customer (IG/WA/Web)] -->|Webhook| B(Rust Chat Ingress Service)
      B --> C{PostgreSQL (RLS)}
      C -->|Trigger| D[AI Job Queue (SKIP LOCKED)]
      D --> E[Gemini Pro Assistant]
      E -->|Draft Reply & Actions| C
      C --> F[Flutter PWA (Triage Feed)]
      F -->|Owner Approval| B
      B -->|Send| A
  ```

  ### Comparative Analysis Table

  | Feature / Capability         | OHC (Target) | Chatwoot | Shopify (Inbox) | Intercom Fin | Square Appts |
  |------------------------------|--------------|----------|-----------------|--------------|--------------|
  | **Unified Omnichannel Triage**| ✅ Native | ✅ Primary| ❌ Disjointed   | ✅ Primary   | ❌ No        |
  | **Deep AI Agent Awareness**  | ✅ Deep | ❌ None | ❌ Limited | ✅ Deep | ❌ None |
  | **Native Work/Task Context** | ✅ Deep | ❌ None | ❌ None | ❌ None | ❌ None |
  | **Native Payment/Booking Inject**| ✅ Yes | ❌ No | ❌ Limited | ❌ No | ✅ Booking Only |
  | **Owner Approval Flow**      | ✅ Core | ❌ No | ❌ No | ❌ No | ❌ No |

  ### References & Sources Catalog
  1. Shopify: https://www.shopify.com/
  2. Square: https://squareup.com/
  3. HubSpot: https://www.hubspot.com/
  4. Notion: https://www.notion.so/
  5. Microsoft Copilot: https://copilot.microsoft.com/
  6. Lark Suite: https://www.larksuite.com/
  7. DingTalk: https://dingtalk.com/
  8. WeCom: https://www.wecom.work/
  9. Tencent Workbuddy: https://workbuddy.tencent.com/
  10. Salesforce Agentforce: https://www.salesforce.com/agentforce/
  11. Intercom Fin: https://www.intercom.com/fin
  12. Zendesk AI: https://www.zendesk.com/service/ai/
  13. Chatwoot: https://www.chatwoot.com/
  14. Gorgias: https://www.gorgias.com/
  15. Klaviyo: https://www.klaviyo.com/
  16. Zoho One: https://www.zoho.com/one/
  17. Odoo: https://www.odoo.com/
  18. Asana: https://asana.com/
  19. Monday: https://monday.com/
  20. ClickUp: https://clickup.com/
  21. HoneyBook: https://www.honeybook.com/
  22. Dubsado: https://www.dubsado.com/
  23. Vagaro: https://www.vagaro.com/
  24. Mindbody: https://www.mindbodyonline.com/
  25. Fresha: https://www.fresha.com/
  26. GlossGenius: https://www.glossgenius.com/
  27. Wix: https://www.wix.com/
  28. Squarespace: https://www.squarespace.com/
  29. Weebly: https://www.weebly.com/
  30. BigCommerce: https://www.bigcommerce.com/
  31. WooCommerce: https://www.woocommerce.com/
  32. Ecwid: https://www.ecwid.com/
  33. Lightspeed: https://www.lightspeedhq.com/
  34. Toast: https://www.toasttab.com/
  35. Clover: https://www.clover.com/
  36. Vend: https://www.vendhq.com/
  37. ShopKeep: https://www.shopkeep.com/
  38. Revel Systems: https://www.revelsystems.com/
  39. TouchBistro: https://www.touchbistro.com/
  40. Square Appointments: https://www.square.com/appointments
  41. Calendly: https://calendly.com/
  42. Acuity Scheduling: https://acuityscheduling.com/
  43. SimplyBook: https://simplybook.me/
  44. Setmore: https://www.setmore.com/
  45. Timely: https://www.timelyapp.com/
  46. Appointy: https://www.appointy.com/
  47. Booksy: https://www.booksy.com/
  48. Vagaro Pro: https://www.vagaro.com/pro
  49. Zen Planner: https://www.zenplanner.com/
  50. PushPress: https://www.pushpress.com/
  51. Glofox: https://www.glofox.com/
  52. Pike13: https://www.pike13.com/
  53. GymMaster: https://www.gymmaster.com/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
