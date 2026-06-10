issue_title: "Implement 'Actionable Empty States' & Contextual Onboarding for New Tenants"
issue_description: |
  # Research Report: "Actionable Empty States" & Contextual Onboarding for New Tenants

  ## Problem Statement
  Small business owners and independent operators (like Maya the baker or Carlos the handyman) who first sign up for an operational tool face "blank page anxiety". When they log into a system that asks them to "add an order" or "create a contact" without showing them *how* or *why*, they abandon the setup. They do not have time to explore empty dashboards. The product fails the "Time to Live Store" test because it demands configuration before it shows value.

  ## Research Findings & Competitive Audit

  **Methodology:**
  Conducted research analyzing 54 distinct URLs, including competitor homepages, onboarding flows, product documentation, and feature landing pages across major small-business SaaS, AI companions, and enterprise collaboration tools.

  **General Competitors Analyzed (Top 10):**
  1. Shopify (POS & Inbox)
  2. Square (Appointments & Payments)
  3. HubSpot
  4. Wix / Squarespace
  5. Honeybook / Dubsado
  6. Jobber / Housecall Pro
  7. ServiceTitan
  8. Clover / Lightspeed
  9. Lark / DingTalk / WeCom (Tencent Workbuddy proxies)
  10. Stripe (Terminal & Payment Links)

  **AI-Native & Productivity Competitors Analyzed (Top 10):**
  1. Shopify Sidekick
  2. HubSpot AI
  3. Notion AI
  4. Microsoft Copilot
  5. Slack AI
  6. Salesforce Agentforce
  7. Zendesk AI
  8. Intercom Fin
  9. Asana / ClickUp AI
  10. Anthropic / Cohere / OpenAI

  **Deep Dive Audit: Shopify (Core + Sidekick)**
  *   **Capabilities:** Shopify gets users selling fast by providing pre-populated demo data, actionable setup checklists ("Add your first product", "Customize your theme"), and integrated sales channels. Sidekick acts as a conversational assistant to configure store settings, generate reports, and write product descriptions.
  *   **Success Factors:** The "Time to Live Store" is incredibly short. Users are not faced with a blank void; they are guided through a prioritized checklist. It feels less like setting up software and more like opening a box that's mostly assembled.
  *   **User Sentiment:**
      *   *Positive:* "It was so easy to get started. I just followed the checklist and my store was live." (Theme from community forums).
      *   *Negative (Before AI/Checklists):* "I logged in and didn't know what to do next. The dashboard was just empty charts."

  **OHC Gap Matrix & Unresolved Pain Points**

  | Feature | Shopify (Deep Dive) | OHC (Current Status) | Gap |
  | :--- | :--- | :--- | :--- |
  | **Initial Landing Experience** | Guided setup checklist, demo data | Often blank dashboards or generic "No data" messages | High: OHC lacks a guided, agent-driven onboarding flow. |
  | **Empty States** | "Add a product" button with a template | Empty table | High: OHC empty states are not actionable. |
  | **AI Assistance in Setup** | Sidekick generates descriptions, configures settings | AI is present but not explicitly driving the first 5 minutes of setup | Medium: Need to integrate the AI assistant actively into the empty state. |

  **The Core Unresolved Pain Point:**
  New users signing up for OHC (like Carlos or Fatima) land in a system that is theoretically powerful but visually empty. Without an assistant explicitly guiding them to ingest their first customer or create their first service, the cognitive load is too high.

  ## Proposed Solution: Agentic Actionable Empty States

  Instead of static "No records found" messages, every primary module (Inbox, Orders, Customers, Tasks) must feature an **Actionable Empty State** powered by the AI Assistant.

  When Maya visits the "Orders" page for the first time, she shouldn't see a blank table. She should see a glass-morphic card that says:
  > "You don't have any active orders yet. I can help you draft a custom cake package or import your existing customer list. What would you like to do?"
  > [Draft a custom offer] [Import customers] [Show me an example]

  ### Design Doc
  *   **Architecture & Components:**
      *   Create a reusable Flutter widget: `ActionableEmptyStateCard`.
      *   It must use OHC Premium Tokens (translucent glass styling, strong typography).
      *   It takes a `moduleContext` (e.g., "orders", "customers") and a list of `AgentAction` callbacks.
  *   **UX Flow (375px Mobile First):**
      1.  User taps "Customers" tab. Database returns 0 records.
      2.  Instead of an empty list, the screen renders a beautifully spaced `ActionableEmptyStateCard` taking up the upper half of the screen.
      3.  The text is friendly and conversational (Agent-persona).
      4.  Large, easily tappable (min 44x44px) primary action buttons are presented.
      5.  Tapping an action invokes the AI assistant to begin a guided flow (e.g., opening a chat sheet pre-filled with "Help me create my first service offering").
  *   **Visuals:** Premium macOS Translucent Glass standard. No horizontal scrolling.

  ```mermaid
  graph TD
      A[User navigates to Empty Module] --> B{Check Record Count}
      B -- Records > 0 --> C[Render Standard Data View]
      B -- Records == 0 --> D[Render ActionableEmptyStateCard]
      D --> E[Agent Prompt: "Let's get started..."]
      E --> F[Primary Action 1 e.g. Create Offer]
      E --> G[Primary Action 2 e.g. Import Data]
      F --> H[Invoke AI Assistant Flow]
  ```

  ### Implementation Prompt
  1.  **Objective:** Implement the `ActionableEmptyStateCard` component and integrate it into at least two primary modules (e.g., Customers and Orders) to replace existing static empty states.
  2.  **Critical User Journey (CUJ):** A newly registered owner navigates to Customers tab on mobile (375px width). Instead of a blank screen, they see a conversational card from the OHC Assistant offering to help them add their first customer or import a list. They tap "Add a customer", which immediately opens a guided creation flow or assistant chat.
  3.  **Acceptance Criteria:**
      *   The component must strictly follow OHC design guidelines (glass-morphic, proper padding, legible text).
      *   It must render perfectly on a 375px viewport without overflow.
      *   It must replace the default empty state in the selected modules.
      *   Buttons must have a minimum 44x44px touch target.
      *   Unit tests and Playwright E2E tests must verify the component renders when records are 0 and disappears when records > 0.
      *   No mocked UI data; it must trigger based on actual database counts.

  ## References & Sources Catalog
  1. https://about.meta.com/
  2. https://www.shopify.com/
  3. https://www.shopify.com/sidekick
  4. https://www.hubspot.com/
  5. https://www.hubspot.com/products/artificial-intelligence
  6. https://www.notion.so/
  7. https://www.notion.so/product/ai
  8. https://copilot.microsoft.com/
  9. https://slack.com/
  10. https://slack.com/features/ai
  11. https://www.salesforce.com/agentforce/
  12. https://www.salesforce.com/products/einstein/overview/
  13. https://www.zendesk.com/ai/
  14. https://www.intercom.com/fin
  15. https://www.wix.com/studio/ai
  16. https://asana.com/product/ai
  17. https://clickup.com/ai
  18. https://coda.io/product/ai
  19. https://www.smartsheet.com/ai
  20. https://airtable.com/platform/ai
  21. https://www.atlassian.com/software/confluence/ai
  22. https://www.atlassian.com/software/jira/ai
  23. https://miro.com/ai/
  24. https://mural.co/
  25. https://www.jasper.ai/
  26. https://copy.ai/
  27. https://www.writer.com/
  28. https://www.typeform.com/ai/
  29. https://www.gong.io/
  30. https://www.chorus.ai/
  31. https://otter.ai/
  32. https://fireflies.ai/
  33. https://www.dialpad.com/ai/
  34. https://www.ringcentral.com/ringsense.html
  35. https://support.apple.com/en-us/104995
  36. https://anthropic.com/claude
  37. https://cohere.com/
  38. https://www.larksuite.com/
  39. https://www.dingtalk.com/en
  40. https://work.weixin.qq.com/
  41. https://www.shopify.com/pos
  42. https://www.shopify.com/inbox
  43. https://stripe.com/en-us/terminal
  44. https://stripe.com/en-us/payments/payment-links
  45. https://www.wix.com/ecommerce/website
  46. https://www.squarespace.com/ecommerce-website
  47. https://www.honeybook.com/
  48. https://www.dubsado.com/
  49. https://www.fresha.com/
  50. https://www.mindbodyonline.com/
  51. https://www.housecallpro.com/
  52. https://www.servicetitan.com/
  53. https://www.lightspeedhq.com/
  54. https://www.clover.com/

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
