issue_title: "Implement AI-Native Booking & Intake Flow for Service Businesses"
issue_description: |
  # Mission Brief: AI-Native Booking & Intake Flow for Service Businesses

  ## Problem Statement
  Owners and operators of service businesses (e.g., Carlos the handyman, Leo the music tutor) face significant friction when capturing demand and turning it into scheduled work. Traditional booking systems are overly complex, forcing the owner to configure availability calendars, service menus, and pricing tiers upfront. For non-technical operators, this is overwhelming. Customers often prefer natural language communication (e.g., text, DMs) over navigating a rigid booking portal. When busy, operators miss leads because they cannot instantly reply to coordinate availability and quote prices. The gap is a lack of an AI assistant that seamlessly bridges conversational intake with structured operational scheduling.

  ## Research Report

  ### Market Mapping & Competitor Discovery
  Our broad crawl covered top general competitors (Shopify, Square, HubSpot, Lark, WeChat Work, DingTalk, Notion, Microsoft Copilot, Salesforce, Zendesk) and AI-native challengers (Zapier AI, Intercom Fin, Adept, MultiOn, Lindy, Tome, Asana AI, Monday AI, Wix Studio, Weebly, Mailchimp, Gorgias, Klaviyo, Freshworks, Zoho, Airtable, ClickUp, Coda, Typeform, Calendly, Acuity, Vagaro, Mindbody, Booksy, Honeybook, Dubsado, HelloBonzai, Jobber, Housecall Pro, ServiceTitan, Thryv, Podia, Kajabi, Teachable, Patreon, Gumroad, BuyMeACoffee, Linktree).

  ### Deep-Dive Competitor Audit: HubSpot & Calendly Integration vs. Lindy.ai
  - **Capabilities:** Traditional flows (like Calendly linked to a CRM) require the user to configure availability rules and send static links. AI-native tools (like Lindy.ai) allow natural language scheduling, automatically parsing intent and checking calendar availability.
  - **Success Factors:** The primary success factor for traditional tools is reliability and calendar sync. However, AI-native tools delight users by eliminating the "link-sending" step, handling back-and-forth negotiation contextually.
  - **User Sentiment Audit:** Research on r/smallbusiness and Trustpilot highlights that while operators appreciate booking links, they often complain about low conversion rates when customers abandon the flow. Quote from an operator: "I send them the link, but they still just text me asking when I'm free. I end up manually checking anyway."

  ### OHC Gap & Pain Point Identification
  - **OHC Feature Audit:** Current capabilities may cover basic entity creation but lack a conversational intake agent that seamlessly transitions a DM inquiry into a confirmed booking slot and deposit request.
  - **Gap Matrix:** OHC lacks the "Triage to Booking" flow. Competitors offer either structured forms (Calendly) or pure chat (Intercom), but none offer an owner-centric assistant that negotiates the time, sets up the operational task, and requests a deposit in one unified step.
  - **Unresolved Pain Point:** Operators lose context between the chat app, the calendar, and the invoicing tool.

  ### Agentic Solution Design
  The solution is the **Work Triage & Booking Agent**.
  - **Intake:** The agent monitors incoming channels (DMs, SMS).
  - **Negotiation:** When a booking intent is detected, it converses with the customer to find a time, checking the owner's availability context.
  - **Confirmation & Deposit:** It drafts a booking summary and payment link, presenting it to the owner for one-tap approval before sending.

  ## Design Doc
  - **Entity Types:** `IntakeConversation`, `ProposedBooking`, `WorkTask`.
  - **Architecture:**
    - The AI Job Queue handles incoming messages.
    - The `Customer Assistant` agent drafts the reply.
    - The `Operations Assistant` agent validates calendar availability using Distributed Locks.
  - **UI/UX Flow (Mobile First - 375px):**
    - **Owner Feed:** A unified list showing "Pending Bookings".
    - **Card View:** A translucent, Apple/Ubiquiti-style card showing the customer's message, the AI's proposed time, and the estimated value.
    - **Action:** A prominent (44x44px minimum) touch target to "Approve & Send Link" or "Edit Details".
  - **Visuals:** Use OHC Premium Token library with restrained translucent materials and strong typography.

  ```mermaid
  graph TD;
      A[Customer DM] --> B(Work Triage Agent);
      B --> C{Intent Analysis};
      C -->|Booking| D(Operations Agent checks calendar);
      D --> E[Draft Proposed Booking];
      E --> F(Owner Review UI);
      F -->|Approve| G[Send Confirmation & Deposit Link];
  ```

  ## Implementation Prompt
  **User-Facing Outcome:** The owner opens the app on their phone and sees a pending task: "Carlos wants a repair on Tuesday. Accept and send deposit link?" One tap confirms it.
  **Critical User Journey (CUJ):**
  1. An external inquiry triggers the intake flow.
  2. The AI assistant parses the intent, checks availability, and creates a `ProposedBooking`.
  3. The owner navigates to the Feed, sees the translucent pending booking card.
  4. The owner taps "Approve". The system updates the booking state and dispatches the confirmation.
  **Acceptance Criteria:**
  - The UI must render correctly on a 375px width screen without horizontal scrolling.
  - The "Approve" button must be at least 44x44px.
  - The flow must be verified via browser/Playwright E2E tests, starting from the Feed to the approval action.

  ## References & Sources Catalog
  1. [https://about.meta.com/technologies/whatsapp-business/](https://about.meta.com/technologies/whatsapp-business/)
  2. [https://www.shopify.com/tour](https://www.shopify.com/tour)
  3. [https://squareup.com/us/en](https://squareup.com/us/en)
  4. [https://www.hubspot.com/products/crm](https://www.hubspot.com/products/crm)
  5. [https://www.larksuite.com/](https://www.larksuite.com/)
  6. [https://work.weixin.qq.com/](https://work.weixin.qq.com/)
  7. [https://www.dingtalk.com/en](https://www.dingtalk.com/en)
  8. [https://www.notion.so/product/ai](https://www.notion.so/product/ai)
  9. [https://www.microsoft.com/en-us/microsoft-365/copilot](https://www.microsoft.com/en-us/microsoft-365/copilot)
  10. [https://squareup.com/us/en/point-of-sale](https://squareup.com/us/en/point-of-sale)
  11. [https://www.salesforce.com/einstein/](https://www.salesforce.com/einstein/)
  12. [https://www.zendesk.com/service/ai/](https://www.zendesk.com/service/ai/)
  13. [https://zapier.com/ai](https://zapier.com/ai)
  14. [https://www.intercom.com/fin](https://www.intercom.com/fin)
  15. [https://www.adept.ai/](https://www.adept.ai/)
  16. [https://www.multi-on.com/](https://www.multi-on.com/)
  17. [https://www.lindyai.com/](https://www.lindyai.com/)
  18. [https://www.tome.app/](https://www.tome.app/)
  19. [https://www.asana.com/product/ai](https://www.asana.com/product/ai)
  20. [https://monday.com/ai](https://monday.com/ai)
  21. [https://www.wix.com/studio/ai](https://www.wix.com/studio/ai)
  22. [https://www.weebly.com/](https://www.weebly.com/)
  23. [https://mailchimp.com/features/ai/](https://mailchimp.com/features/ai/)
  24. [https://www.gorgias.com/](https://www.gorgias.com/)
  25. [https://www.klaviyo.com/](https://www.klaviyo.com/)
  26. [https://www.freshworks.com/ai/](https://www.freshworks.com/ai/)
  27. [https://www.zoho.com/zia/](https://www.zoho.com/zia/)
  28. [https://www.airtable.com/ai](https://www.airtable.com/ai)
  29. [https://clickup.com/ai](https://clickup.com/ai)
  30. [https://coda.io/product/ai](https://coda.io/product/ai)
  31. [https://www.typeform.com/ai/](https://www.typeform.com/ai/)
  32. [https://calendly.com/](https://calendly.com/)
  33. [https://acuityscheduling.com/](https://acuityscheduling.com/)
  34. [https://www.vagaro.com/](https://www.vagaro.com/)
  35. [https://www.mindbodyonline.com/](https://www.mindbodyonline.com/)
  36. [https://www.booksy.com/](https://www.booksy.com/)
  37. [https://squareup.com/us/en/appointments](https://squareup.com/us/en/appointments)
  38. [https://www.honeybook.com/](https://www.honeybook.com/)
  39. [https://www.dubsado.com/](https://www.dubsado.com/)
  40. [https://www.hellobonzai.com/](https://www.hellobonzai.com/)
  41. [https://www.jobber.com/](https://www.jobber.com/)
  42. [https://www.housecallpro.com/](https://www.housecallpro.com/)
  43. [https://www.servicetitan.com/](https://www.servicetitan.com/)
  44. [https://www.thryv.com/](https://www.thryv.com/)
  45. [https://www.podia.com/](https://www.podia.com/)
  46. [https://www.kajabi.com/](https://www.kajabi.com/)
  47. [https://teachable.com/](https://teachable.com/)
  48. [https://www.patreon.com/](https://www.patreon.com/)
  49. [https://gumroad.com/](https://gumroad.com/)
  50. [https://www.buymeacoffee.com/](https://www.buymeacoffee.com/)
  51. [https://linktr.ee/](https://linktr.ee/)

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
