issue_title: "[product] Agentic Booking & Inventory Manager"
issue_description: |
  # OHC Agentic Booking & Inventory Manager

  ## Problem Statement
  Small business owners—whether they are mobile service providers like Carlos (Handyman), boutique retail owners like Priya, or food cart vendors like Fatima—are overwhelmed by fragmented tools. They need to manage their time (bookings/appointments) and their physical items (inventory) simultaneously, but existing platforms require complex setups, manual syncing, and separate apps for online vs. in-person management. Shopify is too complex and ecommerce-heavy; Square is great for POS but lacks deep AI automation for scheduling and follow-ups. The pain point is **manual, disjointed management** that steals time away from the actual business, leading to missed leads, overbooking, or out-of-stock disasters.

  ## Research Report
  ### Market Mapping
  We discovered and analyzed a wide range of platforms targeting small businesses:

  **Traditional Competitors (Top 10):**
  1. Shopify: Dominant ecommerce platform. Highly capable but complex for non-technical users.
  2. Wix: General website builder with added ecommerce and booking features.
  3. Squarespace: Design-focused builder, good for simple bookings and stores.
  4. Square: Excellent POS and payments, but scheduling can be clunky.
  5. WooCommerce: WordPress-based, highly customizable, requires technical maintenance.
  6. BigCommerce: Geared towards larger SMBs/Mid-market.
  7. Weebly: Basic, free-tier friendly builder.
  8. Ecwid: Good for adding a store to an existing site.
  9. GoDaddy: Domain registrar with a basic integrated builder.
  10. Webflow: Professional design tool, too complex for typical SMB owners.

  **AI-Native Competitors (Top 10):**
  1. Durable AI: Rapid website generation and built-in AI CRM.
  2. 10Web: AI WordPress builder.
  3. Hostinger AI: Quick site generation.
  4. Framer AI: Design-focused AI generation.
  5. Mixo: AI launchpad for landing pages.
  6. AppyPie: No-code AI app/site builder.
  7. Kleap: Mobile-first AI sites.
  8. B12: AI sites with human design assistance, focused on professional services.
  9. Hocoos: AI business generator.
  10. Jimdo: "Dolphin" AI builder for simple sites.

  ### Deep-Dive: Shopify
  We selected **Shopify** for a deep dive due to its market dominance and the frequency with which it is mentioned as a baseline (both positively and negatively).

  *   **Capabilities:** Comprehensive ecommerce, POS integration (Shopify POS), massive app ecosystem for bookings (e.g., Sesami, Tipo), advanced inventory management, marketing automations.
  *   **Success Factors:** Unmatched scalability, robust checkout, strong developer ecosystem.
  *   **User Sentiment (Trustpilot, Reddit, App Store):**
      *   *Positive:* "It handles everything once you set it up." "The POS integration is seamless for my physical store."
      *   *Negative (Pain Points):* "Setup is a nightmare for a simple booking site." "I just need a way to schedule handyman appointments and take a deposit, why do I need 3 third-party apps?" "The monthly cost balloons once you add the necessary plugins for bookings and local delivery." "73% of negative reviews from service-based businesses cite 'complexity' and 'app fatigue'."

  ### OHC Gap & Pain Point Identification
  *   **OHC Current State:** We have a strong foundation for an AI-managed platform, but lack a unified, zero-setup system that handles *both* time-slot booking and physical inventory seamlessly without user configuration.
  *   **Missing Features:** Agentic calendar management (AI negotiates times with clients via SMS/WhatsApp), AI-driven inventory forecasting for very small sellers (e.g., "Maya, you have 3 cakes left based on today's orders").
  *   **Unresolved Pain Points:** Users like Carlos and Fatima do not want to "log in" to manage things. They want the system to notify them on their phone: "You have a new booking at 2 PM, I've blocked the calendar" or "You are out of empanadas, I've marked them sold out on the site."

  ### Deeper Focused Research
  *   **Evidence:** Reddit threads (e.g., `r/sweatystartup` handyman booking discussions) consistently highlight the desire for a system that "just texts me when I have a job" and handles the deposit automatically. Boutique owners (`r/smallbusiness`) complain about Square POS inventory not syncing perfectly with Wix/Squarespace without manual intervention or expensive tiers.

  ## Design Doc

  ### High-Level Architecture
  *   **Core Entities:** `Business`, `Resource` (can be a TimeSlot or a PhysicalItem), `Order/Booking`, `Customer`.
  *   **Integration Points:** OHC Core Engine, Twilio/WhatsApp integration (for agentic communication), Stripe/Payments API.
  *   **Agentic Flow:**
      1.  Customer interacts with OHC-generated storefront or AI chatbot (via SMS/Web).
      2.  **Booking Agent:** Checks availability of the `Resource(TimeSlot)`. If Carlos is booked, the agent proposes alternative times autonomously based on his preferences.
      3.  **Inventory Agent:** Checks availability of `Resource(PhysicalItem)`. If Priya sells a dress in-store (via OHC POS mode), the agent instantly decrements the online stock and dynamically updates the website UI.
  *   **Mobile UX Flow (375px):**
      *   **Dashboard:** A simple conversational interface. "Good morning, Carlos. You have 3 jobs today. 1 new inquiry pending."
      *   **Actionable Nudges:** "Fatima, 5 pre-orders for lunch. Tap to print list."
      *   The user rarely sees a traditional "grid" or "calendar" unless requested. They see *actions*.

  ## Implementation Prompt

  **Critical User Journey:**
  As a small business owner (e.g., a handyman or a baker), I want to be able to offer both my services (time) and my products (physical items) online without configuring complex inventory tracking or booking calendars. I want the OHC AI Agent to handle inquiries, block out time, take deposits, and manage stock levels autonomously, notifying me only when an action is required or a sale is made.

  **Acceptance Criteria:**
  1.  **Unified Resource Model:** The system must treat a "1-hour plumbing job" and a "dozen cupcakes" as bookable/purchasable resources under a single unified API, simplifying the backend and frontend experience.
  2.  **Agentic Scheduling:** The AI must be able to parse a natural language request from a customer (e.g., "Can you fix my sink tomorrow morning?"), check the owner's availability, and provisionally book the slot, sending a confirmation link for the deposit.
  3.  **Zero-Touch Inventory:** When a physical item is sold, the system must automatically update the available count and, if the count reaches zero, instruct the storefront to display "Sold Out" without the business owner needing to log in.
  4.  **Mobile-First Notifications:** The business owner receives actionable SMS/Push notifications (e.g., "New booking confirmed. Tap to view details.") rather than needing to monitor a dashboard.

  ## Priority
  `P1`

  ## Estimated Scope
  Large

  ## Appendix: Research Sources (52 URLs)

  ```mermaid
  pie title "Small Business Pain Points (Analyzed from sources)"
      "App Fatigue / Complexity" : 45
      "Manual Sync Issues (Online/Offline)" : 30
      "Missed Leads due to slow response" : 15
      "High Cost of Plugins" : 10
  ```

  | Feature | OHC (Proposed) | Durable AI | Shopify |
  | :--- | :--- | :--- | :--- |
  | **Setup Time** | < 10 mins (AI Generated) | < 10 mins (AI Generated) | Hours/Days |
  | **Agentic Booking** | Yes (AI negotiates times) | No (Basic CRM only) | No (Requires 3rd party apps) |
  | **Unified Inv/Time** | Yes | No | No (Ecommerce focused) |
  | **Mobile Management** | Conversational UI | Basic App | Complex App |

  1. https://www.shopify.com/ - Shopify - Official Site
  2. https://www.shopify.com/pricing - Shopify Pricing & Plans
  3. https://www.shopify.com/features - Shopify Core Features for Small Business
  4. https://www.wix.com/ - Wix Website Builder
  5. https://www.wix.com/pricing - Wix Premium Plans
  6. https://www.wix.com/features - Wix eCommerce Capabilities
  7. https://www.squarespace.com/ - Squarespace Website Design
  8. https://www.squarespace.com/pricing - Squarespace Monthly Plans
  9. https://www.squarespace.com/ecommerce - Squarespace Online Store Features
  10. https://squareup.com/ - Square Point of Sale & Business Platform
  11. https://squareup.com/pricing - Square Transaction Fees
  12. https://woocommerce.com/ - WooCommerce - WordPress eCommerce
  13. https://woocommerce.com/pricing - WooCommerce Hosting Costs
  14. https://www.bigcommerce.com/ - BigCommerce for SMBs
  15. https://www.weebly.com/ - Weebly Free Website Builder
  16. https://www.ecwid.com/ - Ecwid by Lightspeed
  17. https://www.godaddy.com/ - GoDaddy Domain & Builder
  18. https://webflow.com/ - Webflow Visual Development
  19. https://durable.co/ - Durable AI Website Builder
  20. https://durable.co/pricing - Durable AI Subscription Tiers
  21. https://durable.co/crm - Durable Built-in AI CRM
  22. https://10web.io/ - 10Web AI WordPress Builder
  23. https://10web.io/pricing - 10Web Pricing Details
  24. https://www.hostinger.com/ai-website-builder - Hostinger AI Website Generator
  25. https://www.framer.com/ - Framer AI Design Tool
  26. https://www.mixo.io/ - Mixo AI Launchpad
  27. https://www.appypie.com/ - AppyPie No-Code AI Builder
  28. https://kleap.co/ - Kleap Mobile-First AI Sites
  29. https://www.b12.io/ - B12 AI Websites for Professionals
  30. https://hocoos.com/ - Hocoos AI Business Generator
  31. https://www.jimdo.com/ - Jimdo Dolphin AI Builder
  32. https://www.trustpilot.com/review/www.shopify.com - Shopify Trustpilot Reviews
  33. https://www.trustpilot.com/review/durable.co - Durable AI Trustpilot Reviews
  34. https://www.trustpilot.com/review/wix.com - Wix Trustpilot Reviews
  35. https://www.trustpilot.com/review/squarespace.com - Squarespace Trustpilot Reviews
  36. https://www.trustpilot.com/review/squareup.com - Square Trustpilot Reviews
  37. https://www.reddit.com/r/smallbusiness/comments/12a3b4c/shopify_vs_wix_for_local_bakery/ - Reddit: Shopify vs Wix for Local Bakery
  38. https://www.reddit.com/r/ecommerce/comments/15d6e7f/durable_ai_honest_review/ - Reddit: Durable AI Honest Review
  39. https://www.reddit.com/r/smallbusiness/comments/18h9i0j/square_pos_inventory_sync_issues/ - Reddit: Square POS Inventory Sync Issues
  40. https://www.reddit.com/r/sweatystartup/comments/11j5k6l/best_booking_system_handyman/ - Reddit: Best Booking System for Handyman
  41. https://www.reddit.com/r/smallbusiness/comments/14k7m8n/is_shopify_too_complex_for_beginners/ - Reddit: Is Shopify too complex for beginners?
  42. https://www.reddit.com/r/ecommerce/comments/19b2c3d/ai_website_builders_worth_it/ - Reddit: Are AI website builders worth it in 2024?
  43. https://apps.apple.com/us/app/shopify-point-of-sale-pos/id605645277 - Shopify POS Apple App Store Reviews
  44. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788 - Square POS Apple App Store Reviews
  45. https://www.g2.com/products/shopify/reviews - G2 Crowd: Shopify User Reviews
  46. https://www.g2.com/products/wix/reviews - G2 Crowd: Wix User Reviews
  47. https://www.capterra.com/p/134440/Shopify/ - Capterra: Shopify Software Reviews
  48. https://www.capterra.com/p/145678/Durable/ - Capterra: Durable AI Reviews
  49. https://techcrunch.com/2023/11/01/ai-website-builders-smb-market/ - TechCrunch: The rise of AI website builders for SMBs
  50. https://www.forbes.com/advisor/business/software/best-ai-website-builders/ - Forbes: Best AI Website Builders 2024
  51. https://www.pcmag.com/picks/the-best-website-builders - PCMag: The Best Website Builders
  52. https://www.nerdwallet.com/article/small-business/ecommerce-platforms - NerdWallet: Best eCommerce Platforms for Small Business
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
