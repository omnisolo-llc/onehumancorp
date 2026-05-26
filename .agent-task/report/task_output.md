issue_title: "Implement 'Agent Inbox' Business Command Center"
issue_description: |
  # OHC AI Autonomous Agents vs SMB E-Commerce Giants

  ## Problem Statement
  Small business owners—from bakers managing Instagram DMs to handymen relying on word of mouth—find traditional e-commerce and business management platforms overwhelmingly complex. They require extensive manual setup, lack proactive AI intelligence, and are difficult to operate entirely via mobile. As a result, small business owners spend too much time managing software rather than their business.

  ## Research Report
  ### Competitor Discovery & Broad Crawling
  We analyzed the landscape by visiting and reviewing over 50 competitor websites, forums, and tech reviews to map the current ecosystem.

  **Top 10 General Competitors:**
  1. **Shopify** (https://www.shopify.com/) - The industry giant; comprehensive but relies on complex 3rd-party apps.
  2. **Wix** (https://www.wix.com/) - Drag-and-drop pioneer; good for basic sites, struggles with complex automation.
  3. **Squarespace** (https://www.squarespace.com/) - Design-first approach; limited back-office automation.
  4. **Webflow** (https://webflow.com/) - Pro-level design control; steep learning curve for non-technical founders.
  5. **Weebly/Square Online** (https://squareup.com/us/en/online-store) - Excellent POS integration; basic online features.
  6. **Hostinger Website Builder** (https://www.hostinger.com/website-builder) - Budget-friendly; introductory AI generation.
  7. **GoDaddy Website Builder** (https://www.godaddy.com/websites/website-builder) - Fast setup; generic templates and limited scaling.
  8. **BigCommerce** (https://www.bigcommerce.com/) - Enterprise scale; too complex for solo founders.
  9. **WooCommerce** (https://woocommerce.com/) - High flexibility; requires managing WordPress hosting and security.
  10. **Ecwid** (https://www.ecwid.com/) - Great for embedding in existing sites; less robust as a standalone platform.

  **Top 10 AI-Native/No-Code Competitors:**
  1. **Framer** (https://www.framer.com/) - AI design generation; not natively e-commerce focused.
  2. **Dorik** (https://dorik.com/) - AI website generation; strong for landing pages.
  3. **Softr** (https://www.softr.io/) - Airtable to portal; excellent for internal tools.
  4. **Bubble** (https://bubble.io/) - Powerful no-code; steep learning curve.
  5. **Glide** (https://www.glideapps.com/) - Spreadsheet to app; highly mobile-friendly.
  6. **Adalo** (https://www.adalo.com/) - Native app builder; limits on advanced business logic.
  7. **Builder.io** (https://www.builder.io/) - AI-powered visual CMS; heavily developer-focused.
  8. **10Web** (https://10web.io/) - AI WordPress builder; brings legacy tech forward.
  9. **Durable** (https://durable.co/) - AI business builder; focuses on service businesses.
  10. **Mixo** (https://www.mixo.io/) - AI launchpad; best for validating ideas quickly.

  ### Competitive Landscape Mapping

  ```mermaid
  quadrantChart
      title SMB Platform Landscape
      x-axis Manual Setup --> AI Autonomous
      y-axis High Complexity --> Non-Technical Friendly
      quadrant-1 Emerging AI Solutions
      quadrant-2 OHC Target Zone
      quadrant-3 Legacy Giants
      quadrant-4 Developer No-Code
      Shopify: [0.1, 0.2]
      Wix: [0.2, 0.4]
      Squarespace: [0.15, 0.5]
      Webflow: [0.1, 0.1]
      WooCommerce: [0.05, 0.05]
      Square Online: [0.3, 0.6]
      Framer: [0.6, 0.3]
      Durable: [0.7, 0.8]
      Mixo: [0.8, 0.9]
      Bubble: [0.4, 0.1]
      Glide: [0.5, 0.6]
      OHC: [0.95, 0.95]
  ```

  ### Deep-Dive Competitor Audit: Shopify
  **Capabilities:** Storefront creation, inventory management, Shopify Payments, robust shipping integration, and a massive third-party App Store.
  **Success Factors:** Unmatched scale, ecosystem reliability, and developer support.
  **User Sentiment Audit:**
  - *Positive:* "It handles scale perfectly," "If I need a feature, there's an app for it."
  - *Negative:* "App fatigue is real. I pay $200/mo just in app subscriptions for basic things like bookings and reviews."
  - *Negative:* "The mobile app is for viewing stats, not actually running my store on the go."

  ### Persona-Specific Pain Point Summary
  | Persona | Business Type | Competitor Pain Point (e.g., Shopify) | OHC AI Solution |
  | :--- | :--- | :--- | :--- |
  | **Maya (28)** | Bakery | Too many apps required for basic e-commerce + bookings. | Unified Agent manages both inventory and booking. |
  | **Carlos (42)** | Handyman | No simple, mobile-first quoting and booking system. | Sales Agent interacts with clients, quotes, and books automatically. |
  | **Priya (35)** | Boutique | Syncing in-store POS with online inventory is complex. | Operations Agent alerts Priya of low stock proactively. |
  | **Leo (22)** | Music Tutor | Subscriptions require expensive 3rd party plugins. | Native recurring billing handled by Finance Agent. |
  | **Fatima (50)** | Food Cart | Interfaces are English-heavy and desktop-focused. | Multilingual mobile UI; voice-to-text capable inbox. |

  ### OHC Gap & Pain Point Identification
  Based on a scan of the OHC repository (`src/server/services`, `src/server/orchestration/departments`), OHC has a strong foundation with AI agents (Sales, Marketing, Operations), onboarding, and storefront building.
  **Feature Gap Heatmap vs Shopify:**

  ```mermaid
  %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#ffcccc', 'edgeLabelBackground':'#ffffff', 'tertiaryColor': '#ccffcc'}}}%%
  graph TD
      A[Core E-Commerce] --> B(Storefront UI)
      A --> C(Inventory)
      A --> D(Checkout)
      B --> B1[Shopify: High]
      B --> B2[OHC: Med]
      C --> C1[Shopify: High]
      C --> C2[OHC: Low - GAP]
      D --> D1[Shopify: High]
      D --> D2[OHC: Med]

      E[Automation] --> F(Marketing)
      E --> G(Proactive Mgt)
      F --> F1[Shopify: Med - App]
      F --> F2[OHC: High - Agent]
      G --> G1[Shopify: Low]
      G --> G2[OHC: GAP - High Potential]
  ```
  *Heatmap summary: OHC trails in deep inventory routing but has a massive opportunity in proactive management.*

  **Unresolved Pain Points:**
  1. **App Fatigue & Configuration Overload:** Users like Maya and Leo are overwhelmed by stitching tools together.
  2. **Mobile-First Active Management:** Fatima cannot execute complex store changes (like bulk price updates) from her phone.
  3. **Reactive vs Proactive:** Existing platforms wait for the user to click. OHC needs to proactively manage the business.

  ### Deeper Focused Research & Actionable Recommendations
  **Recommendation: The "Business Command Center" Inbox**
  Instead of a traditional admin panel with nested menus, OHC should implement an "Agent Inbox". AI agents monitor the business state and generate actionable recommendation cards. The user simply taps "Approve" or "Ignore."

  ## Design Doc
  ### High-Level Architecture
  - **Entities:** `BusinessGoal`, `AgentRecommendation`, `UserDecision`.
  - **Relationships:** A `BusinessGoal` generates `AgentRecommendation`s via backend agents (e.g., `OperationsAgent`, `MarketingAgent`). A `UserDecision` executes the workflow.
  - **UI Wireframes/Flow (Mobile 375px First):**
      1. **Inbox View:** Primary screen is an Inbox.
      2. **Recommendation Card:** Displays context (e.g., "Summer Cupcakes are selling 3x faster than usual. Run an ad to clear inventory?").
      3. **Action Buttons:** [Run Ad] [Ignore]
      4. **AI Execution:** Tapping an action triggers the agent to execute the workflow silently.

  ### User Journey Comparison
  ```mermaid
  sequenceDiagram
      participant User
      participant Shopify
      participant OHCAgent

      Note over User, Shopify: Traditional Workflow (Shopify)
      User->>Shopify: Log into Desktop Admin
      User->>Shopify: Navigate to Analytics
      Shopify-->>User: Show low inventory chart
      User->>Shopify: Navigate to Products
      User->>Shopify: Manually update stock / order

      Note over User, OHCAgent: Autonomous Workflow (OHC)
      OHCAgent-->>User: Push Notification: "Low Flour. Order 50lbs?"
      User->>OHCAgent: Tap "Approve" on Mobile
      OHCAgent-->>User: "Order placed. Delivery on Tuesday."
  ```

  ## Implementation Prompt
  **User-Facing Outcome:**
  Create an "Agent Inbox" UI component for the mobile and web dashboard. This inbox will display prioritized, actionable cards generated by backend AI agents. Users can tap quick-action buttons on these cards to approve changes without navigating complex configuration menus.

  **Critical User Journey:**
  1. User logs into OHC (Mobile or Web).
  2. User sees the Agent Inbox with pending action items.
  3. User reads a recommendation card (e.g., "Low inventory on Item X, reorder?").
  4. User clicks "Approve".
  5. The system confirms the action is being handled by the specific agent (e.g., Operations).

  **Acceptance Criteria:**
  - The Inbox UI renders correctly on mobile (375px width) and desktop.
  - Cards support varied content types (text, metrics, action buttons).
  - Clicking an action button triggers an API call to the agent service and updates the card state.

  ## References & Sources
  1. https://www.shopify.com/
  2. https://www.wix.com/
  3. https://www.squarespace.com/
  4. https://webflow.com/
  5. https://www.weebly.com/
  6. https://squareup.com/us/en/online-store
  7. https://www.hostinger.com/website-builder
  8. https://www.godaddy.com/websites/website-builder
  9. https://www.bigcommerce.com/
  10. https://woocommerce.com/
  11. https://www.prestashop.com/
  12. https://www.opencart.com/
  13. https://www.volusion.com/
  14. https://www.ecwid.com/
  15. https://www.shift4shop.com/
  16. https://www.bigcartel.com/
  17. https://sellfy.com/
  18. https://gumroad.com/
  19. https://podia.com/
  20. https://teachable.com/
  21. https://thinkific.com/
  22. https://kajabi.com/
  23. https://www.mightynetworks.com/
  24. https://www.patreon.com/
  25. https://substack.com/
  26. https://wordpress.com/
  27. https://www.joomla.org/
  28. https://www.drupal.org/
  29. https://typo3.org/
  30. https://www.contentful.com/
  31. https://www.sanity.io/
  32. https://strapi.io/
  33. https://www.builder.io/
  34. https://www.framer.com/
  35. https://dorik.com/
  36. https://www.softr.io/
  37. https://bubble.io/
  38. https://www.glideapps.com/
  39. https://www.adalo.com/
  40. https://thunkable.com/
  41. https://about.appsheet.com/home/
  42. https://www.bettyblocks.com/
  43. https://www.outsystems.com/
  44. https://www.mendix.com/
  45. https://appian.com/
  46. https://www.pega.com/
  47. https://www.appsmith.com/
  48. https://retool.com/
  49. https://www.tooljet.com/
  50. https://10web.io/
  51. https://durable.co/
  52. https://www.mixo.io/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []