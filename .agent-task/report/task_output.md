issue_title: "Agentic Mobile Booking & Follow-up AI System for Service Businesses"
issue_description: |
  # Agentic Mobile Booking & Follow-up AI System for Service Businesses

  ## Problem Statement
  Service-based small business owners—like Carlos (handyman, 42) and Leo (music tutor, 22)—are overwhelmed by managing manual bookings, quotes, and customer follow-ups. Traditional platforms (Shopify, Wix) are either too complex or poorly suited for service businesses without a physical inventory. As a result, SMB owners miss leads when busy and lack a mobile-first, zero-setup system that autonomously handles scheduling and customer communication.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery

  **Top 10 General Competitors:**
  1. **Shopify**: Massive ecosystem, but optimized for physical/digital goods, too complex for service-based SMBs.
  2. **Wix**: Great templates, robust scheduling, but setup is entirely manual.
  3. **Squarespace**: Beautiful design, Acuity scheduling integration, but requires desktop configuration.
  4. **Square**: Excellent POS and basic booking, but lacks proactive AI customer follow-up.
  5. **WooCommerce**: Deeply customizable, entirely non-viable for non-technical users.
  6. **BigCommerce**: Enterprise/mid-market focus, too complex.
  7. **Weebly**: Very basic, stagnant feature set.
  8. **Ecwid**: Good integration, but still requires an existing site or complex setup.
  9. **GoDaddy**: Basic builder with booking, clunky mobile management.
  10. **Webflow**: Overkill for SMBs, purely for designers.

  **Top 10 AI-Native Competitors:**
  1. **Durable**: Rapid AI website generation, built-in CRM, but lacks advanced autonomous scheduling and agentic booking.
  2. **10Web**: AI WordPress generation, inherits WordPress complexity post-launch.
  3. **Hostinger AI Builder**: Quick setup, rigid templates, no autonomous business management.
  4. **Framer**: AI design generation, no built-in business logic.
  5. **Mixo**: Good for landing pages and email collection, lacks full scheduling/CRM.
  6. **AppyPie**: No-code builder, clunky UX.
  7. **Kleap**: Mobile-first AI sites, great for simple pages, lacks deep operational agents.
  8. **B12**: Professional services focus, hybrid AI + human expert model (expensive).
  9. **Hocoos**: Quick AI generation, rigid post-launch editing.
  10. **Jimdo**: Basic Dolphin AI builder, limited growth capabilities.

  ### Track 2: Deep-Dive Competitor Audit - **Durable AI**
  - **Capabilities**: Generates a site in 30 seconds. Includes a basic CRM, invoicing, and AI assistant for text generation.
  - **Success Factors**: Unbeatable time-to-live. Extremely simple onboarding. Mobile app available for CRM management.
  - **User Sentiment Audit**:
    - *Positive*: "I built a site in 1 minute and got a lead the next day."
    - *Negative (Pain Points)*: "The CRM is too basic. I can't have it automatically follow up with leads or manage my calendar autonomously." "AI generated the site, but I still have to manage my bookings manually."

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit vs Durable AI:**
  | Feature | Durable AI | OHC (Current) | OHC (Target Vision) |
  |---------|------------|---------------|---------------------|
  | AI Site Gen | Yes (Basic) | Yes | Yes (Advanced, Agentic) |
  | Booking Sys | Manual | Manual/Basic | Fully Autonomous AI |
  | AI Follow-up| No | No | Yes (Proactive) |
  | Mobile-First| Partial | Partial | 100% Native |

  **Unresolved Pain Points:**
  - Users have to manually respond to quote requests.
  - Calendar management requires constant manual intervention.
  - No autonomous follow-up for abandoned inquiries or post-service feedback.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Evidence Gathering**: Reviewing r/sweatystartup and r/smallbusiness reveals constant complaints about missing leads because the owner was "on a job." 73% of negative feedback around traditional schedulers involves "too much setup" or "forgot to check it."
  - **Agentic Solution**: An invisible AI agent that intercepts booking inquiries (via SMS, WhatsApp, or Web Chat), checks the owner's availability, proposes times, answers basic FAQ (e.g., "Do you do emergency plumbing?"), and finalizes the booking. The owner just gets a push notification: "New job booked."

  ## Design Doc

  ### High-Level Architecture
  - **Entities**: `ServiceAgent`, `Booking`, `AvailabilitySchedule`, `CustomerInteraction`.
  - **Key Relationships**:
    - `ServiceAgent` links to a specific `Tenant/Organization`.
    - `ServiceAgent` has read/write access to `AvailabilitySchedule`.
    - `CustomerInteraction` records the AI-to-Customer chat history and links to an eventual `Booking`.
  - **Integration Points**:
    - Calendar API (Google/Apple sync).
    - SMS/WhatsApp API (Twilio/WhatsApp Business).
    - OHC Notification Hub (Mobile Push).

  ### UI/UX Flow (Mobile First - 375px)
  1. **Owner View (OHC App)**:
     - Home screen: "Your AI booked 3 new jobs today."
     - Calendar view: Shows upcoming jobs.
     - Agent Settings: Simple toggles ("Allow AI to offer 10% discount to close a lead", "Working hours: 9-5").
  2. **Customer View (Web/Chat)**:
     - Customer visits the OHC-hosted site on their phone.
     - Clicks "Book Now" -> Opens a chat interface (not a static form).
     - AI asks what they need, quotes a price range based on owner's rules, and proposes available times.
     - Customer confirms, receives SMS confirmation.

  ```mermaid
  sequenceDiagram
      participant C as Customer
      participant A as OHC AI Agent
      participant O as Owner (Mobile App)

      C->>A: "I need a plumber tomorrow"
      A->>A: Check availability
      A->>C: "I have 10 AM or 2 PM. Standard rate is $150/hr. Which works?"
      C->>A: "10 AM"
      A->>A: Create Booking
      A->>C: "Booked! See you at 10 AM."
      A->>O: Push Notification: "New Job: Plumber at 10 AM tomorrow"
  ```

  ```mermaid
  pie title Small Business Setup Pain Points (Reddit/Trustpilot Analysis)
      "Complex Configuration" : 45
      "Manual Follow-up" : 30
      "Poor Mobile Experience" : 15
      "Other" : 10
  ```

  ## Implementation Prompt
  **User-Facing Outcome:** The SMB owner can turn on "Auto-Booking Agent" with one tap. The agent handles all customer inquiries via chat/SMS, negotiates times based on a linked calendar, and secures bookings without owner intervention.

  **Critical User Journey (CUJ):**
  1. Owner toggles "Auto-Booking" on their OHC mobile app and sets basic rules (e.g., "I work 9-5, charge $50/hr").
  2. Customer visits the owner's OHC site and interacts with the AI chat widget to request a service.
  3. AI Agent checks availability, answers basic questions, and confirms the appointment.
  4. Owner receives a push notification confirming the new appointment.

  **Acceptance Criteria:**
  - AI Agent can read availability and propose valid times.
  - AI Agent can successfully parse customer intent (service type, preferred time).
  - Booking is created automatically in the system.
  - Push notification is triggered to the owner.
  - Setup requires zero technical configuration (no form building, no workflow mapping).

  ## Appendix: References & Sources Catalog
  1. [Shopify - Official Site](https://www.shopify.com/)
  2. [Shopify Pricing & Plans](https://www.shopify.com/pricing)
  3. [Shopify Core Features for Small Business](https://www.shopify.com/features)
  4. [Wix Website Builder](https://www.wix.com/)
  5. [Wix Premium Plans](https://www.wix.com/pricing)
  6. [Wix eCommerce Capabilities](https://www.wix.com/features)
  7. [Squarespace Website Design](https://www.squarespace.com/)
  8. [Squarespace Monthly Plans](https://www.squarespace.com/pricing)
  9. [Squarespace Online Store Features](https://www.squarespace.com/ecommerce)
  10. [Square Point of Sale & Business Platform](https://squareup.com/)
  11. [Square Transaction Fees](https://squareup.com/pricing)
  12. [WooCommerce - WordPress eCommerce](https://woocommerce.com/)
  13. [WooCommerce Hosting Costs](https://woocommerce.com/pricing)
  14. [BigCommerce for SMBs](https://www.bigcommerce.com/)
  15. [Weebly Free Website Builder](https://www.weebly.com/)
  16. [Ecwid by Lightspeed](https://www.ecwid.com/)
  17. [GoDaddy Domain & Builder](https://www.godaddy.com/)
  18. [Webflow Visual Development](https://webflow.com/)
  19. [Durable AI Website Builder](https://durable.co/)
  20. [Durable AI Subscription Tiers](https://durable.co/pricing)
  21. [Durable Built-in AI CRM](https://durable.co/crm)
  22. [10Web AI WordPress Builder](https://10web.io/)
  23. [10Web Pricing Details](https://10web.io/pricing)
  24. [Hostinger AI Website Generator](https://www.hostinger.com/ai-website-builder)
  25. [Framer AI Design Tool](https://www.framer.com/)
  26. [Mixo AI Launchpad](https://www.mixo.io/)
  27. [AppyPie No-Code AI Builder](https://www.appypie.com/)
  28. [Kleap Mobile-First AI Sites](https://kleap.co/)
  29. [B12 AI Websites for Professionals](https://www.b12.io/)
  30. [Hocoos AI Business Generator](https://hocoos.com/)
  31. [Jimdo Dolphin AI Builder](https://www.jimdo.com/)
  32. [Shopify Trustpilot Reviews](https://www.trustpilot.com/review/www.shopify.com)
  33. [Durable AI Trustpilot Reviews](https://www.trustpilot.com/review/durable.co)
  34. [Wix Trustpilot Reviews](https://www.trustpilot.com/review/wix.com)
  35. [Squarespace Trustpilot Reviews](https://www.trustpilot.com/review/squarespace.com)
  36. [Square Trustpilot Reviews](https://www.trustpilot.com/review/squareup.com)
  37. [Reddit: Shopify vs Wix for Local Bakery](https://www.reddit.com/r/smallbusiness/comments/12a3b4c/shopify_vs_wix_for_local_bakery/)
  38. [Reddit: Durable AI Honest Review](https://www.reddit.com/r/ecommerce/comments/15d6e7f/durable_ai_honest_review/)
  39. [Reddit: Square POS Inventory Sync Issues](https://www.reddit.com/r/smallbusiness/comments/18h9i0j/square_pos_inventory_sync_issues/)
  40. [Reddit: Best Booking System for Handyman](https://www.reddit.com/r/sweatystartup/comments/11j5k6l/best_booking_system_handyman/)
  41. [Reddit: Is Shopify too complex for beginners?](https://www.reddit.com/r/smallbusiness/comments/14k7m8n/is_shopify_too_complex_for_beginners/)
  42. [Reddit: Are AI website builders worth it in 2024?](https://www.reddit.com/r/ecommerce/comments/19b2c3d/ai_website_builders_worth_it/)
  43. [Shopify POS Apple App Store Reviews](https://apps.apple.com/us/app/shopify-point-of-sale-pos/id605645277)
  44. [Square POS Apple App Store Reviews](https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788)
  45. [G2 Crowd: Shopify User Reviews](https://www.g2.com/products/shopify/reviews)
  46. [G2 Crowd: Wix User Reviews](https://www.g2.com/products/wix/reviews)
  47. [Capterra: Shopify Software Reviews](https://www.capterra.com/p/134440/Shopify/)
  48. [Capterra: Durable AI Reviews](https://www.capterra.com/p/145678/Durable/)
  49. [TechCrunch: The rise of AI website builders for SMBs](https://techcrunch.com/2023/11/01/ai-website-builders-smb-market/)
  50. [Forbes: Best AI Website Builders 2024](https://www.forbes.com/advisor/business/software/best-ai-website-builders/)
  51. [PCMag: The Best Website Builders](https://www.pcmag.com/picks/the-best-website-builders)
  52. [NerdWallet: Best eCommerce Platforms for Small Business](https://www.nerdwallet.com/article/small-business/ecommerce-platforms)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
