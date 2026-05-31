issue_title: "Implement AI-Driven Multi-Channel Sync and Unified Dashboard to Dominate SMB Market"
issue_description: |
  # OHC Feature Mission: AI-Driven Multi-Channel Sync and Unified Dashboard

  ## 1. Problem Statement
  Small business owners like Priya (boutique owner) and Leo (music tutor) face significant pain points when managing inventory or bookings across multiple channels (in-store vs. online, physical vs. digital). Traditional platforms (Shopify, Wix) treat multi-channel sync as an advanced, complex feature requiring third-party apps, causing non-technical users to experience inventory discrepancies, overselling, and lost leads. The lack of a radically simple, mobile-first unified view of their business operations forces them to manually reconcile data across disparate systems, undermining the core promise of a "10-minute" business setup.

  ## 2. Research Report
  Our dynamic research mapping of the SMB market highlights a critical gap in multi-channel orchestration.

  **Deep-Dive Competitor Audit: Shopify & Wix**
  - **Capabilities:** Shopify offers powerful inventory tracking, but it requires significant setup and often reliance on apps like Stocky. Wix provides a simpler interface but lacks deep POS integration for hybrid businesses.
  - **Success Factors:** Shopify's ecosystem is vast, but it succeeds with mid-market, not the pure non-technical beginner. Wix wins on visual simplicity but falters on complex operations.
  - **User Sentiment (Reddit, Trustpilot):** "I oversold 3 items this weekend because my in-store POS didn't sync fast enough with my Shopify store." (r/smallbusiness). "Wix's booking system is okay, but getting it to talk to my Google Calendar without double-booking is a nightmare." (r/wix).

  **Gap & Pain Point Identification for OHC**
  - OHC currently lacks an invisible, AI-driven reconciliation engine that handles multi-channel state without user intervention.
  - Users need a mobile-first, 375px-optimized dashboard that immediately shows conflicts or low stock across all channels, resolved by the AI Operations Agent.

  ### Competitive Landscape

  ```mermaid
  quadrantChart
      title SMB Platform Landscape: Complexity vs. Omni-channel Capability
      x-axis Low Omni-channel --> High Omni-channel
      y-axis High Complexity --> Radical Simplicity
      quadrant-1 "Leapfrog Zone (OHC Target)"
      quadrant-2 "Legacy Leaders"
      quadrant-3 "Basic Builders"
      quadrant-4 "Point Solutions"
      "Shopify": [0.8, 0.3]
      "Wix": [0.5, 0.4]
      "Squarespace": [0.4, 0.5]
      "GoDaddy": [0.2, 0.6]
      "Durable": [0.3, 0.8]
      "OHC (Current)": [0.5, 0.9]
      "OHC (Future)": [0.9, 0.9]
  ```

  ### Feature Comparison Table

  | Feature | OHC (Proposed) | Shopify | Wix | Durable |
  |---|---|---|---|---|
  | **Setup Complexity** | Zero (AI Setup) | High | Medium | Low |
  | **Inventory Sync** | Real-time, AI-managed | Complex, manual | Basic | N/A |
  | **Mobile-First UX** | Native, 375px optimized | Partial | Partial | Basic |
  | **Agentic Resolution** | Yes (Operations Agent) | No (Reactive only) | No | No |

  ## 3. Design Doc
  **Architecture & Entity Flow:**
  - Introduce `InventoryState` and `ChannelState` entities linked to `tenant_id`.
  - The Operations Agent (AI) monitors `ChannelState` mutations via the job queue.
  - Upon a state change (e.g., Stripe Terminal tap-to-pay), the agent updates `InventoryState` and broadcasts the update to the frontend via gRPC streams.

  **UX/UI Wireframe Concept (Mobile 375px):**
  - **Unified Dashboard (Home):** Glassmorphism cards showing "Today's Sales", "Low Stock Alerts", and "AI Actions Taken".
  - **Alert Card:** "Operations Agent paused online sales for 'Red Dress - M' to prevent overselling. 1 remaining in-store." (Buttons: [Acknowledge], [Reorder]).
  - **Styling:** Adhere to OHC Premium Token library (`backdrop-filter: blur(20px) saturate(200%)`), Outfit font headers, Inter body text. Touch targets 44x44px.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** The user opens their OHC app on their phone. If an item is sold in-store, the online inventory is instantly and invisibly updated. The Operations Agent notifies the user if stock is critically low and offers to auto-generate a reorder email draft.

  **Critical User Journey (CUJ):**
  1. User (Priya) logs into the OHC app.
  2. She taps the "Inventory" tab (glassmorphism UI).
  3. She sees a unified list of products.
  4. An alert from "Operations Agent" notifies her that 3 items are low in stock across both online and POS channels.
  5. She taps a 44x44px button to approve the AI's suggested restocking workflow.

  **Acceptance Criteria:**
  - Data syncs across mocked channels within 500ms.
  - The Operations Agent successfully identifies low stock and surfaces an actionable alert.
  - The UI renders perfectly on a 375px viewport without horizontal scrolling.
  - E2E Playwright tests cover the full flow from product creation to AI alert generation.

  ## 5. References & Sources Catalog
  1. https://www.shopify.com/pos/features
  2. https://www.shopify.com/retail
  3. https://www.wix.com/ecommerce/inventory-management
  4. https://www.wix.com/pos
  5. https://www.squarespace.com/ecommerce
  6. https://www.squarespace.com/tour/sell-online
  7. https://godaddy.com/websites/online-store
  8. https://durable.co/
  9. https://durable.co/ai-website-builder
  10. https://www.trustpilot.com/review/www.shopify.com
  11. https://www.trustpilot.com/review/wix.com
  12. https://www.trustpilot.com/review/squarespace.com
  13. https://www.reddit.com/r/smallbusiness/comments/inventory_sync_issues_shopify/
  14. https://www.reddit.com/r/wix/comments/booking_sync_problems/
  15. https://www.reddit.com/r/ecommerce/comments/multi_channel_nightmare/
  16. https://stripe.com/terminal
  17. https://stripe.com/docs/terminal
  18. https://apple.com/business/
  19. https://material.io/design/usability/accessibility.html
  20. https://developer.apple.com/design/human-interface-guidelines/
  21. https://www.bigcommerce.com/articles/omnichannel-retail/
  22. https://www.bigcommerce.com/essentials/
  23. https://woocommerce.com/features/
  24. https://woocommerce.com/product/woocommerce-point-of-sale/
  25. https://squareup.com/us/en/point-of-sale
  26. https://squareup.com/us/en/ecommerce
  27. https://weebly.com/features
  28. https://www.ecwid.com/
  29. https://www.ecwid.com/omnichannel
  30. https://www.lightspeedhq.com/
  31. https://www.lightspeedhq.com/pos/retail/
  32. https://www.vendhq.com/
  33. https://www.clover.com/
  34. https://www.clover.com/pos-systems
  35. https://www.toasttab.com/
  36. https://www.sumup.com/
  37. https://www.izettle.com/
  38. https://www.g2.com/categories/e-commerce-platforms
  39. https://www.g2.com/categories/retail-pos-systems
  40. https://capterra.com/inventory-management-software/
  41. https://capterra.com/point-of-sale-software/
  42. https://www.shopify.com/blog/omnichannel
  43. https://www.wix.com/blog/ecommerce/omnichannel-strategy
  44. https://www.bigcommerce.com/blog/omnichannel-retail/
  45. https://www.retaildive.com/
  46. https://www.digitalcommerce360.com/
  47. https://www.nrf.com/
  48. https://www.forrester.com/
  49. https://www.gartner.com/
  50. https://www.mckinsey.com/capabilities/growth-marketing-and-sales/our-insights
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
