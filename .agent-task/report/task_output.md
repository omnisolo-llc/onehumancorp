issue_title: "[Feature] Autonomous Hyperlocal Growth & Ad Buying Engine"
issue_description: |
  **Problem Statement:**
  Small business owners (like Maya the baker or Carlos the handyman) know they need to "run ads" to get local customers, but ad platforms (Meta, Google) are overwhelmingly complex. They don't understand "lookalike audiences," "pixels," or how to design ad creative. They just want to spend $50/week and get 3 new customers. The current platform lacks an integrated, zero-touch way to turn a slice of revenue into local customer acquisition.

  **Research Report:**
  - **Competitor Analysis:** Shopify relies on third-party apps or manual Meta integration. Wix and Squarespace have basic integrations but require user setup. None offer true "zero-touch" local ad generation.
  - **Pain Points:** SMBs abandon campaigns due to complicated setup and waste money on poor targeting.
  - **Opportunity:** OHC already holds the product catalog, pricing, and customer CRM data. We can use AI to generate creative (images/copy), automatically bid on hyperlocal radiuses, and track ROAS directly to the booking/sale, completely invisibly.

  **Design Doc:**
  - **Architecture Diagram:**
    ```mermaid
    graph TD
        A[Small Business Owner] -->|Sets Weekly Budget| B(Growth Agent)
        B --> C{Creative Generator}
        C -->|Uses Catalog Data| D[Meta/Google Ad APIs]
        B --> E{Audience Modeler}
        E -->|Uses OHC CRM Data| D
        D --> F[Hyperlocal Customers]
        F --> G[OHC Storefront/Booking]
        G -->|Conversion Data| H(Attribution Engine)
        H -->|Updates Model| B
    ```
  - **UI Wireframes (375px first):**
    - Dashboard Card: "Grow your business".
    - Screen 1: Budget Selection ("How much do you want to invest this week? [ $50 ] [ $100 ] [ Custom ]").
    - Screen 2: Targeting ("Where do you want to find customers? [ Local 5-mile radius ] [ Online everywhere ]").
    - Screen 3: Preview & Confirm (Shows AI-generated ad creatives in Instagram story and Google Search formats).
  - **Mobile UX flow:** 3 taps (Tap "Grow", select budget, confirm).
  - **AI Agent Integration Points:**
    - *Marketing Agent:* Generates ad copy/variants, interfaces with Ad Network APIs.
    - *Finance Agent:* Ensures budget constraints, deducts ad spend from OHC wallet.
    - *Operations Agent:* Pauses ads if inventory is out of stock or calendar is fully booked.
  - **Key Design Decisions:**
    - Abstract away all ad network concepts (CPA, CPC, Pixels).
    - Use OHC's unified ledger to fund campaigns directly from the business's balance.
    - Tightly couple ads with inventory/calendar to prevent advertising sold-out items or unavailable times.

  **Estimated Scope:** Large

  **Implementation Prompt:**
  Build the `HyperlocalGrowthEngine`. The system must allow a user to allocate a budget (e.g., $50) and select a goal (e.g., "more local bookings"). The engine should autonomously generate ad creatives from the user's catalog, create and manage campaigns via Meta/Google APIs, and track conversions. The UI should consist of a simple budget slider and performance summary card. No ad-tech jargon is permitted in the user interface.

  **Acceptance Criteria:**
  1. User can start a campaign in 3 taps on mobile.
  2. AI generates at least 3 ad variants from catalog data.
  3. System automatically pauses campaigns if the target item goes out of stock or calendar fills up.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
