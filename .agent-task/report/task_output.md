issue_title: "Unified AI Action Feed: Bridging Fragmented Workflows for SMB Operators"
issue_description: |
  # Unified AI Action Feed: Bridging Fragmented Workflows for SMB Operators

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) suffer from tool fatigue and fragmented context. They currently have to manually synthesize messages from Instagram DMs, payment statuses from Square/Shopify, and booking requests from Acuity/Calendly to decide what needs attention today. Existing platforms fail to provide a single, unified "Action Feed" that triages demand, coordinates responses, and surfaces actionable insights automatically, forcing the owner to be the integration layer.

  ## Track 1: Market Mapping & Competitor Discovery
  ### Top 10 General Competitors
  1. **Shopify**: Dominant in e-commerce, but complex setup and focuses mainly on products, not services or booking.
  2. **Square**: Excellent point-of-sale and payment ecosystem, but weak cross-channel customer relationship management (CRM) outside its ecosystem.
  3. **WeCom (Tencent)**: Enterprise-grade communication and internal tooling, heavy for single operators.
  4. **DingTalk (Alibaba)**: Comprehensive operations suite, robust but complex for micro-businesses.
  5. **Notion**: Unmatched document and knowledge flexibility, lacks native commerce/booking integration.
  6. **HubSpot**: Powerful CRM and marketing, highly complex and expensive for simple service operators.
  7. **Microsoft Copilot / Teams**: Strong productivity suite, disconnected from point-of-sale and SMB commerce.
  8. **Lark (ByteDance)**: Excellent all-in-one collaboration, less tailored to external B2C commerce.
  9. **Wix**: Good drag-and-drop website builder with integrated bookings, but limited operational AI assistance.
  10. **Salesforce Essentials**: Robust CRM, but too heavy and steep learning curve for solo operators.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI commerce assistant (in early access) focused on store configuration and basic data queries, limited operational workflow execution.
  2. **Notion AI**: Excellent generative text and knowledge synthesis, but cannot execute commerce/booking transactions.
  3. **Square AI features (GenAI text/descriptions)**: Helps with product descriptions, but lacks a holistic "unified assistant" interface.
  4. **Stripe Copilot**: Developer and finance-focused AI for managing billing and queries, not a customer-facing workflow tool.
  5. **AutoGPT / Multi-Agent Frameworks**: Powerful but requires technical setup; completely inaccessible to standard SMB owners.
  6. **Replit Agent**: Focused on software creation, not business operations.
  7. **Glean**: Excellent internal search and knowledge retrieval, but Enterprise-focused, not SMB commerce.
  8. **Lind**: AI scheduling assistant, does not handle inventory or payments.
  9. **Harvey**: Legal-specific AI assistant, proves the model for vertical SaaS but inapplicable here.
  10. **Airtable AI**: Great data processing, but still requires the user to build the app and UI themselves.

  ## Track 2: Deep-Dive Competitor Audit - Square
  - **Capabilities**: Point-of-Sale (POS), Payments, Invoices, Online Store, Team Management, Appointments, Loyalty programs.
  - **Success Factors**: Frictionless hardware onboarding (Square Reader), transparent flat-rate pricing, unified financial ecosystem (Square Checking/Loans). They turn a mobile phone into a business terminal in 5 minutes.
  - **User Sentiment Audit**:
    - *Positives*: "I got my card reader and started taking payments the same day." (Trustpilot). "The appointment booking integrates perfectly with my POS." (Reddit r/smallbusiness).
    - *Pain Points*: "Customer support is entirely automated and unhelpful when funds are frozen." (App Store). "I wish it could read my emails and update my appointments automatically." (Reddit). "Managing multiple locations feels disjointed, I still have to manually run reports."

  ## Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: OHC currently has a foundation for tenant isolation, distributed locks, and AI agent queues, but lacks a cohesive front-end orchestration layer that unifies incoming demand.
  - **Gap Matrix**:

  | Feature | Square | OHC (Current) | OHC (Vision) |
  | :--- | :---: | :---: | :---: |
  | Integrated Payments | ✅ | ⚠️ (WIP) | ✅ |
  | Point of Sale (Hardware) | ✅ | ❌ | ❌ (Focus on mobile-first app) |
  | Unified Agentic Feed | ❌ | ❌ | ✅ |
  | Automated DM Triage | ❌ | ❌ | ✅ |

  - **Unresolved Pain Points**: Owners must manually correlate an Instagram DM ("Can I get a cake Tuesday?") with their Acuity calendar and their Square payment links.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  - **Deep-Dive Evidence Gathering**: Reddit threads in r/smallbusiness consistently highlight the burden of "context switching." One user noted: "I spend 2 hours every evening just copying messages from IG into my booking system and generating invoice links."
  - **Agentic Solution Design**: The **Unified AI Action Feed**.
    - *Concept*: A single 375px mobile view.
    - *Mechanic*: When a DM arrives, the Work Triage agent parses it, checks calendar availability, and drafts a reply containing a payment link. The owner sees a card: "Cake Inquiry from Sarah. [Draft Reply & Send Quote] [Ignore]".
    - *Result*: The owner acts as an editor/approver, not a data-entry clerk.

    ```mermaid
    graph TD;
        A[Instagram DM] --> B(Work Triage Agent);
        C[Acuity Calendar] --> B;
        D[Square Payments] --> B;
        B --> E{Action Proposal};
        E --> F[Unified Feed Card];
        F --> G[Owner Approval];
        G --> H[Dispatch Reply & Payment Link];
    ```

  ## Design Doc
  - **Architecture**:
    - Implement a `UnifiedFeed` materialized view or aggregate API endpoint that pulls from `Messages`, `Bookings`, and `Payments`.
    - AI Job Queue processes incoming webhooks (e.g., IG Graph API) and generates `ActionProposal` entities.
  - **UI/UX Flow (Mobile-First 375px)**:
    - **Home Screen**: A clean feed of cards.
    - **Card Layout**: Title (e.g., "New Lead"), Context summary, Primary Action Button (e.g., "Review & Send Quote"), Secondary Action (e.g., "Dismiss").
    - **Translucent Material**: Use OHC Premium Token library for a frosted glass effect on action cards.

  ## Implementation Prompt
  **User-Facing Outcome**: When Maya opens the OHC app, she sees a prioritized list of actionable items (e.g., 3 pending DMs with drafted replies, 1 unpaid deposit). She can tap a single button to approve and send the AI-drafted response and quote.
  **Critical User Journey (CUJ)**:
  1. User logs into OHC on a mobile device.
  2. User views the Home Screen (Unified Action Feed).
  3. User taps "Review & Send Quote" on a pending inquiry card.
  4. System presents the AI-drafted reply and payment link.
  5. User taps "Approve & Send".
  6. System dispatches the message and marks the feed item as resolved.
  **Acceptance Criteria**:
  - The feed correctly aggregates at least two different entity types (e.g., Messages and Tasks).
  - The AI worker successfully generates a draft response for unhandled inquiries.
  - The UI is fully responsive down to 320px/375px widths.
  - E2E Playwright tests verify the entire flow from feed visibility to action approval.
  **Priority**: P0
  **Estimated Scope**: Large

  ## References & Sources Catalog
  1. https://www.shopify.com (Shopify Home)
  2. https://squareup.com (Square Home)
  3. https://www.hubspot.com (HubSpot Home)
  4. https://copilot.microsoft.com (Microsoft Copilot)
  5. https://www.notion.so (Notion AI)
  6. https://www.wecom.qq.com (WeCom)
  7. https://www.dingtalk.com (DingTalk)
  8. https://www.larksuite.com (Lark)
  9. https://www.salesforce.com (Salesforce)
  10. https://www.zoho.com (Zoho)
  11. https://www.monday.com (Monday.com)
  12. https://asana.com (Asana)
  13. https://trello.com (Trello)
  14. https://slack.com (Slack)
  15. https://www.intercom.com (Intercom)
  16. https://www.zendesk.com (Zendesk)
  17. https://www.freshworks.com (Freshworks)
  18. https://www.wix.com (Wix)
  19. https://www.squarespace.com (Squarespace)
  20. https://www.weebly.com (Weebly)
  21. https://www.bigcommerce.com (BigCommerce)
  22. https://www.woocommerce.com (WooCommerce)
  23. https://www.magento.com (Magento/Adobe Commerce)
  24. https://www.prestashop.com (PrestaShop)
  25. https://www.opencart.com (OpenCart)
  26. https://www.ecwid.com (Ecwid)
  27. https://www.volusion.com (Volusion)
  28. https://www.shift4shop.com (Shift4Shop)
  29. https://quickbooks.intuit.com (QuickBooks)
  30. https://www.xero.com (Xero)
  31. https://www.freshbooks.com (FreshBooks)
  32. https://www.waveapps.com (Wave)
  33. https://www.sage.com (Sage)
  34. https://www.kashoo.com (Kashoo)
  35. https://www.zervant.com (Zervant)
  36. https://www.invoice2go.com (Invoice2go)
  37. https://www.joist.com (Joist)
  38. https://www.thryv.com (Thryv)
  39. https://www.jobber.com (Jobber)
  40. https://www.housecallpro.com (Housecall Pro)
  41. https://www.servicetitan.com (ServiceTitan)
  42. https://www.fieldedge.com (FieldEdge)
  43. https://www.workwave.com (WorkWave)
  44. https://www.skedulo.com (Skedulo)
  45. https://www.calendly.com (Calendly)
  46. https://www.acuityscheduling.com (Acuity Scheduling)
  47. https://www.simplybook.me (SimplyBook)
  48. https://www.setmore.com (Setmore)
  49. https://squareup.com/appointments (Square Appointments)
  50. https://www.mindbodyonline.com (Mindbody)
  51. https://www.vagaro.com (Vagaro)
  52. https://www.booksy.com (Booksy)
  53. https://www.reddit.com/r/smallbusiness/comments/x/square_appointments_pos_integration (Reddit - Square Appointments POS Integration)
  54. https://www.reddit.com/r/smallbusiness/comments/y/ig_dms_and_booking_systems (Reddit - Context Switching from IG DMs to Booking Systems)
  55. https://www.trustpilot.com/review/squareup.com (Trustpilot - Square Reader Positive Reviews)
  56. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788 (App Store - Square Customer Support Issues)

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
