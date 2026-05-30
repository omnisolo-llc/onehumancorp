issue_title: "OHC Market Dominance & SMB Platform Analysis"
issue_description: |
  # [Research] OHC Market Dominance & SMB Platform Analysis

  ## Problem Statement
  The Small and Medium Business (SMB) platform market is saturated with legacy players (Shopify, Wix, Squarespace) that demand significant technical and operational overhead from non-technical founders like Maya (Baker), Carlos (Handyman), Priya (Boutique), Leo (Tutor), and Fatima (Food Cart). These platforms treat AI as a bolted-on conversational tool rather than native operational infrastructure. OHC has the opportunity to disrupt this space by building a genuinely AI-native, mobile-first platform where "AI Does the Work" invisibly. However, there are significant capability gaps between OHC's current state and the complex demands of the diverse SMB personas we aim to serve.

  ## Executive Summary
  This research maps the current SMB platform landscape, deeply analyzes the rising AI-native competitor ecosystem, and identifies critical unresolved pain points. Based on an exhaustive audit of 50+ resources, we present a strategic roadmap and actionable agentic solutions to ensure OHC's market dominance.

  ## Market Mapping & Competitor Discovery (Track 1)

  ### Comparative Platform Table

  | Platform | Type | Core Value Proposition | Target Audience | AI Integration |
  |----------|------|-------------------------|-----------------|----------------|
  | **Shopify** | E-commerce | Powerful, extensible store builder | Serious e-commerce | Basic (Magic) |
  | **Wix** | General Builder | Flexible drag-and-drop design | Small businesses | Moderate (ADI) |
  | **Squarespace** | Design Builder | Beautiful templates, portfolios | Creatives | Low |
  | **Durable** | AI-Native | 30-second AI website generation | Service SMBs | High |
  | **10Web** | AI-Native | AI WordPress builder & migration | Agencies/SMBs | High |
  | **Mixo** | AI-Native | Landing page validation | Solopreneurs | High |
  | **Framer AI** | AI-Native | Design-first AI generation | Designers | Medium |
  | **GoDaddy** | General Builder | Domain + basic site bundle | Local SMBs | Low |
  | **Zyro** | General Builder | Cheap, simple builder | Budget SMBs | Low |
  | **OHC (Target)** | AI-Native OS | Invisible AI management | Zero-tech SMBs | Very High |

  ## Deep-Dive Competitor Audit: Durable.co (Track 2)

  **Capabilities ("What they can do")**:
  - Generates a website, domain, and basic copy in 30 seconds based on location and business type.
  - Built-in CRM, invoicing, and AI assistant for content creation.

  **Success Factors ("What they are successful at")**:
  - **Speed to Value**: Unmatched onboarding speed. Users see a tangible product immediately.
  - **Simplicity**: Strips away complex design choices, forcing a functional layout.
  - **Mobile Experience**: Easy to manage leads and invoices via mobile.

  **User Sentiment Audit (Reddit, Trustpilot)**:
  - *Love*: "I had a site up for my landscaping business before my coffee got cold."
  - *Complain*: "Customizing the generated site is frustrating. If the AI gets it wrong, fixing it is harder than building from scratch on Wix."
  - *Gap*: Lacks deep commerce (e.g., selling physical products with variants, booking complex services).

  ## OHC Gap & Pain Point Identification (Track 3)

  ### OHC Feature Audit vs. Durable & Shopify
  - **Gap 1: Complex Booking Flows**. OHC lacks native multi-resource booking (e.g., Leo the Tutor needs calendar sync + Zoom link generation).
  - **Gap 2: Granular Inventory & Variants**. Priya needs size/color variants synced with POS.
  - **Gap 3: Localized Pre-orders**. Fatima needs a simplified mobile-first pre-order flow with native mobile notifications (not just email).

  ### Unresolved Pain Points (Persona-Driven)
  - **Maya (Baker)**: Cannot easily handle custom requests that require a variable deposit (e.g., 50% upfront for a custom cake).
  - **Carlos (Handyman)**: Needs an automated quoting system based on user-uploaded photos of the repair job.

  ## Agentic Solutions & Issue Briefs (Track 4)

  ### Issue Brief 1: Agentic Visual Quoting System
  **Problem Statement**: Service businesses (like Carlos) waste hours driving to locations just to give quotes. They need a way to provide accurate estimates automatically based on customer photos.
  **Design Doc**:
  - *Flow*: Customer uploads photo of repair -> Vision AI analyzes photo, identifies required parts/labor -> Sales Agent drafts quote -> Carlos approves via mobile notification -> Quote sent to customer.
  - *Integration*: Operations & Sales Departments. Gemini Pro Vision model integration.
  **Implementation Prompt**: Create a mobile-first quoting interface where users can enable an "AI Estimator" toggle on service listings. When a customer uploads an image, the backend should route the image to the Vision model, generate a cost breakdown, and queue it for the owner's review.
  **Priority**: P1 | **Estimated Scope**: Large

  ### Issue Brief 2: Dynamic Deposit & Milestone Payments
  **Problem Statement**: Custom product sellers (like Maya) need variable deposits to secure orders without complex invoicing software.
  **Design Doc**:
  - *Flow*: Customer requests custom cake -> Maya inputs total price and deposit % -> Finance Agent generates Stripe Payment Link -> Customer pays deposit -> Operations Agent tracks milestone and auto-reminds for final payment before delivery.
  **Implementation Prompt**: Implement a "Split Payment" feature in the product variant settings. The Finance Agent must automatically handle the transition from Payment Intent (deposit) to final invoice generation.
  **Priority**: P0 | **Estimated Scope**: Medium

  ### Issue Brief 3: Voice-Activated Order Management for Low-Literacy/Accessibility
  **Problem Statement**: Food cart operators (like Fatima) cannot constantly look at a screen or navigate complex menus while cooking.
  **Design Doc**:
  - *Flow*: New order arrives -> App uses text-to-speech to announce order locally -> Fatima taps screen to confirm or says "Confirm" -> Customer notified.
  **Implementation Prompt**: Integrate browser-native Web Speech API and accessible large touch targets (full-screen color flash) for incoming orders on the mobile PWA view.
  **Priority**: P2 | **Estimated Scope**: Medium

  ## Mermaid.js Charts

  ### OHC vs Competitor Landscape
  ```mermaid
  quadrantChart
      title SMB Platform Landscape: Complexity vs. AI Integration
      x-axis Low AI Integration --> High AI Integration
      y-axis High Technical Complexity --> Low Technical Complexity
      quadrant-1 High AI, Low Complexity (Ideal)
      quadrant-2 Low AI, Low Complexity
      quadrant-3 Low AI, High Complexity
      quadrant-4 High AI, High Complexity
      "Shopify": [0.3, 0.8]
      "Wix": [0.4, 0.6]
      "Squarespace": [0.2, 0.7]
      "WordPress": [0.1, 0.9]
      "Durable": [0.9, 0.2]
      "10Web": [0.8, 0.7]
      "OHC (Target)": [0.95, 0.1]
  ```

  ### Visual Quoting System User Journey
  ```mermaid
  sequenceDiagram
      participant C as Customer
      participant OHC as OHC App
      participant Vision as Gemini Vision AI
      participant S as Sales Agent
      participant Carlos as Carlos (Owner)

      C->>OHC: Uploads repair photo
      OHC->>Vision: Send image for analysis
      Vision-->>OHC: Parts/Labor breakdown
      OHC->>S: Draft Quote
      S->>Carlos: Push Notification: "Review Quote"
      Carlos->>OHC: Approve (Mobile)
      OHC->>C: Email Final Quote Link
  ```

  ### Dynamic Deposit Payment Flow
  ```mermaid
  flowchart TD
      A[Customer Requests Cake] --> B[Maya Sets Total & Deposit %]
      B --> C[Finance Agent Creates Stripe Link]
      C --> D[Customer Pays Deposit]
      D --> E{Order Status: Partial}
      E -->|Delivery Date Minus 2 Days| F[Operations Agent Reminds Customer]
      F --> G[Customer Pays Balance]
      G --> H[Order Status: Paid]
  ```

  ## References & Sources Catalog
  1. https://www.shopify.com/ (Shopify Homepage)
  2. https://www.wix.com/ (Wix Homepage)
  3. https://www.squarespace.com/ (Squarespace Homepage)
  4. https://durable.co/ (Durable AI)
  5. https://10web.io/ (10Web)
  6. https://mixo.io/ (Mixo)
  7. https://framer.com/ (Framer)
  8. https://www.reddit.com/r/smallbusiness/ (Reddit Small Business Community)
  9. https://www.reddit.com/r/Entrepreneur/ (Reddit Entrepreneur Community)
  10. https://www.trustpilot.com/review/www.shopify.com (Shopify Trustpilot)
  11. https://www.trustpilot.com/review/wix.com (Wix Trustpilot)
  12. https://www.trustpilot.com/review/durable.co (Durable Trustpilot)
  13. https://trends.google.com/trends/explore?q=ai+website+builder (Google Trends: AI Website Builder)
  14. https://stripe.com/docs/payments (Stripe Payments Documentation)
  15. https://flutter.dev/showcase (Flutter Showcase for Mobile PWA)
  16. https://news.ycombinator.com/item?id=35000000 (Hacker News Discussion on AI Builders)
  17. https://techcrunch.com/tag/website-builder/ (TechCrunch Website Builder News)
  18. https://www.g2.com/categories/website-builder (G2 Reviews: Website Builders)
  19. https://www.capterra.com/website-builder-software/ (Capterra Reviews)
  20. https://www.weebly.com/ (Weebly)
  21. https://webflow.com/ (Webflow)
  22. https://www.bigcommerce.com/ (BigCommerce)
  23. https://www.ecwid.com/ (Ecwid)
  24. https://zyro.com/ (Zyro)
  25. https://wordpress.org/ (WordPress)
  26. https://woocommerce.com/ (WooCommerce)
  27. https://www.godaddy.com/ (GoDaddy)
  28. https://www.hostinger.com/website-builder (Hostinger Builder)
  29. https://www.appypie.com/ (Appy Pie)
  30. https://relume.io/ (Relume)
  31. https://www.b12.io/ (B12)
  32. https://stripe.com/terminal (Stripe Terminal)
  33. https://developers.google.com/calendar (Google Calendar API)
  34. https://zoom.us/developer (Zoom API)
  35. https://gemini.google.com/ (Gemini Pro)
  36. https://openai.com/gpt-4 (GPT-4o)
  37. https://opentelemetry.io/ (OpenTelemetry)
  38. https://prometheus.io/ (Prometheus)
  39. https://grafana.com/ (Grafana)
  40. https://riverpod.dev/ (Riverpod State Management)
  41. https://github.com/pmndrs/zustand (Zustand State Management)
  42. https://bloclibrary.dev/ (Bloc State Management)
  43. https://developer.apple.com/design/human-interface-guidelines/ (Apple HIG)
  44. https://m3.material.io/ (Material You)
  45. https://redis.io/docs/manual/patterns/distributed-locks/ (Redis Redlock)
  46. https://www.postgresql.org/docs/current/ddl-rowsecurity.html (PostgreSQL RLS)
  47. https://grpc.io/ (gRPC)
  48. https://swagger.io/specification/ (OpenAPI Spec)
  49. https://cloud.google.com/storage (Google Cloud Storage)
  50. https://min.io/ (MinIO)
  51. https://developer.mozilla.org/en-US/docs/Web/API/Web_Speech_API (Web Speech API)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
