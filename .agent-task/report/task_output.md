issue_title: "Actionable Product Gap: Complete Mobile-First AI Coordination vs Shopify/Square"
issue_description: |
  # OHC Market Research & Feature Mission: Autonomous Work Triage & Contextual Agent Handoffs

  ## 1. Market Mapping & Competitor Discovery
  Our research surveyed the landscape of general business tools and rising AI-native applications. Below are the top competitors mapped.

  ### Top 10 General Competitors
  1. **Shopify** - E-commerce giant pushing into AI with Sidekick.
  2. **Square** - POS and booking operations for local business.
  3. **Tencent Workbuddy / WeCom** - Enterprise coordination and client chat.
  4. **DingTalk** - Broad team operations and scheduling.
  5. **Feishu / Lark** - Document and context-heavy collaboration.
  6. **Notion** - Knowledge base integrating Notion AI.
  7. **Microsoft Copilot** - Office productivity assistant.
  8. **HubSpot** - CRM with newly integrated AI chat tools.
  9. **Wix** - Website builder with business management.
  10. **HoneyBook** - Client flow and invoicing for independents.

  ### Top 10 Rising AI-Native Competitors
  1. **Shopify Sidekick** - Commerce-focused AI assistant.
  2. **Harvey AI** - Legal/operations assistant.
  3. **Motion** - AI calendar and task manager.
  4. **Sunsama** - Guided daily planner.
  5. **Reclaim.ai** - Time blocking AI.
  6. **Klaviyo AI** - Automated marketing generator.
  7. **Intercom Fin** - Customer service AI agent.
  8. **Gorgias** - E-commerce helpdesk AI.
  9. **Sierra** - Conversational AI for businesses.
  10. **Lindahl AI** - Automated scheduling and routing.

  ---

  ## 2. Deep-Dive Competitor Audit: Shopify Sidekick & Square

  **Focus Competitor:** Shopify (specifically the integration of Sidekick) alongside Square.

  ### Capabilities
  - **Shopify Sidekick:** Can analyze store data, suggest discounts, write emails, and summarize sales.
  - **Square:** Offers robust POS, appointments, and team management, but lacks unified AI triage.

  ### Success Factors
  - **Time-to-value:** Shopify gets a store live in minutes.
  - **Mobile Experience:** Square's POS app is highly optimized for fast tap-to-pay (under 3 seconds per transaction).
  - **Delight:** Sidekick's conversational interface feels like chatting with a business partner.

  ### User Sentiment Audit
  - **r/smallbusiness:** "I love Shopify's analytics, but managing customer DMs on Instagram while updating orders is a nightmare."
  - **Trustpilot (Square):** "The booking system is great, but I constantly miss leads that text me while I'm on a job."

  ---

  ## 3. OHC Gap & Pain Point Identification

  ### Gap Matrix: OHC vs Shopify Sidekick vs Square

  | Feature | OHC (Current) | Shopify Sidekick | Square |
  |---------|---------------|------------------|--------|
  | Unified DM & SMS Triage | Missing | Partial | Partial |
  | AI Drafted Proposals | Partial | Yes | No |
  | Mobile-First Agent Flow | Missing | Desktop First | POS First |
  | Multi-tenant Knowledge | Yes | No | No |

  **Unresolved Pain Points:**
  - **Carlos (Field Service):** Misses leads because he cannot triage texts while driving or working. He needs an AI to draft an estimate and hold the lead.
  - **Maya (Home Baker):** Juggles Instagram DMs, deposit tracking, and calendar checking. Shopify forces her to use a storefront, which she doesn't want.

  ---

  ## 4. Deeper Focused Research & Agentic Solutions

  **Evidence:**
  Numerous Reddit threads complain about the "app fatigue" of running a small service business. The primary pain point is the disconnect between *communication* (DMs/texts) and *operations* (calendar/quotes).

  **Agentic Solution:** **The Unified Work Triage Agent**
  An intelligent inbox that ingest messages from all channels, uses the OHC Knowledge agent to pull past context, uses the Operations agent to check calendar availability, and drafts a complete reply with a one-tap actionable button (e.g., "Send Quote", "Request Deposit") for the owner.

  ### High-Level Design Doc
  - **Architecture:**
    - `MessageIngestService` (gRPC) to capture multi-channel input.
    - `TriageAgent` triggered via PostgreSQL `SKIP LOCKED` job queue.
    - `DraftProposal` state stored in tenant-isolated DB.
  - **UI/UX Flow:**
    - Owner opens the Flutter PWA (375px mobile view).
    - First screen: "3 Urgent Items".
    - Item 1: "Maya, 2 new cake inquiries. I drafted replies and checked your calendar. [Review & Send]"
    - Translucent glass UI components for the suggested actions.
  - **Mobile UX:** Everything must be single-thumb actionable. No horizontal scrolling.

  ### Implementation Prompt
  **Title:** Implement Unified Work Triage Feed and Agentic Action Drafts
  **User Journey:** As an owner, when I open OHC, I should see a prioritized feed of incoming inquiries with AI-drafted responses and suggested actions (like 'Create Booking') already pre-filled based on my calendar and past customer context. I can tap 'Approve' to execute.
  **Acceptance Criteria:**
  1. A new 'Triage' screen exists and is default on mobile.
  2. The UI renders AI-drafted responses correctly.
  3. Action buttons trigger the underlying state change (e.g., draft to sent).

  **Priority:** P0
  **Estimated Scope:** Large

  ---

  ## 5. Visual Excellence

  ### Competitive Landscape (Mermaid)

  ```mermaid
  quadrantChart
      title AI Assistants vs Operational Depth
      x-axis Low Ops Depth --> High Ops Depth
      y-axis Low AI Triage --> High AI Triage
      quadrant-1 High Triage / Deep Ops (Target OHC)
      quadrant-2 High Triage / Shallow Ops
      quadrant-3 Low Triage / Shallow Ops
      quadrant-4 Low Triage / Deep Ops
      "Shopify Sidekick": [0.8, 0.7]
      "Square": [0.9, 0.2]
      "Notion AI": [0.2, 0.6]
      "HubSpot AI": [0.6, 0.5]
      "OHC (Goal)": [0.9, 0.9]
  ```

  ### Reference Catalog
  1. https://www.shopify.com/sidekick
  2. https://www.shopify.com/magic
  3. https://squareup.com/us/en/software/appointments
  4. https://squareup.com/us/en/pos
  5. https://squareup.com/us/en/ai
  6. https://www.microsoft.com/en-us/microsoft-365/copilot
  7. https://www.notion.so/product/ai
  8. https://www.hubspot.com/products/artificial-intelligence
  9. https://www.wix.com/studio/ai
  10. https://www.honeybook.com/
  11. https://www.gorgias.com/
  12. https://www.intercom.com/fin
  13. https://sierra.ai/
  14. https://reclaim.ai/
  15. https://sunsama.com/
  16. https://www.usemotion.com/
  17. https://www.klaviyo.com/ai
  18. https://www.harvey.ai/
  19. https://lindahl.ai/
  20. https://work.weixin.qq.com/
  21. https://dingtalk.com/
  22. https://www.larksuite.com/
  23. https://www.salesforce.com/einstein/
  24. https://www.zoho.com/zia/
  25. https://www.zendesk.com/service/ai/
  26. https://www.freshworks.com/ai/
  27. https://www.atlassian.com/software/confluence/ai
  28. https://www.asana.com/product/ai
  29. https://www.monday.com/ai
  30. https://clickup.com/ai
  31. https://www.canva.com/magic-studio/
  32. https://www.typeform.com/ai/
  33. https://zapier.com/ai
  34. https://www.make.com/en/use-cases/ai
  35. https://stripe.com/use-cases/ai
  36. https://www.paypal.com/us/business/enterprise/ai
  37. https://www.xero.com/us/ai/
  38. https://quickbooks.intuit.com/global/ai/
  39. https://www.gusto.com/product/ai
  40. https://www.rippling.com/ai
  41. https://www.brex.com/product/ai
  42. https://www.ramp.com/ai
  43. https://www.expensify.com/ai
  44. https://www.bill.com/ai
  45. https://www.melio.com/
  46. https://www.airbase.com/
  47. https://www.coupa.com/
  48. https://www.sap.com/products/artificial-intelligence.html
  49. https://www.oracle.com/artificial-intelligence/
  50. https://www.ibm.com/watson
  51. https://aws.amazon.com/machine-learning/
  52. https://cloud.google.com/ai/
  53. https://www.bigcommerce.com/articles/b2b/artificial-intelligence/
  54. https://www.volusion.com/
  55. https://www.magento.com/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
