# Feature Mission: Agentic Service Booking & Quoting Flow

## Problem Statement
Service-based small business owners (e.g., Carlos the Handyman, Maya the Baker) do not sell simple "Add to Cart" products. They require a multi-step flow: Inquiry -> Scoping -> Quoting -> Deposit -> Scheduling. Current platforms (Shopify, Wix) require stringing together disparate third-party apps (forms, calendars, invoicing) to achieve this. Non-technical users resort to managing this chaos manually via Instagram DMs and Venmo.

## Research Report
- **Competitive Baseline (Shopify/Wix)**: These platforms are heavily optimized for SKU-based physical retail. Service booking requires plugins (e.g., Calendly integrations, custom form builders) which break the unified UI experience and add monthly costs.
- **AI-Native Trends**: Tools like Durable generate service sites, but the backend "CRM" is still largely manual list management.
- **User Pain**: "I spend 2 hours every evening just replying to DMs to figure out what kind of cake they want and when they need it, then tracking down deposits." (Common sentiment in /r/smallbusiness).
- **Sources**:
  - Shopify App Store reviews for booking apps (complaints about cost and integration).
  - Reddit /r/sweatystartup discussions on quoting workflows.

## Design Doc

### High-Level Flow (Mobile-First 375px)
1. **Customer View**: A seamless "Request a Service" form on the OHC storefront. Supports photo uploads (e.g., "picture of the broken pipe") and natural language descriptions.
2. **AI Agent Processing ("The Salesperson")**:
   - The OHC backend intercepts the inquiry.
   - The Gemini Pro LLM analyzes the text/image context against the Business Profile.
   - The Agent drafts a proposed Quote (Price + Scope) and extracts available times from the "Operations" Agent's calendar.
3. **Owner View (The 1-Tap Approval)**:
   - Carlos receives a push notification on his OHC mobile app.
   - UI shows a Glassmorphism card: "New Inquiry: Leaky Faucet. Suggested Quote: $150. Suggested Time: Tue 2 PM."
   - Action buttons: [Approve & Send] / [Edit] / [Decline].
4. **Customer Conversion**: Customer receives a unified OHC link to view the proposal, pick the time, and pay the deposit via Stripe.

### Architecture/Integration Points
- **Entities needed**: `Inquiry`, `Quote`, `Booking`.
- **AI Integration**: Requires a new capability/tool for "The Salesperson" agent to analyze `Inquiry` payloads and propose `Quote` drafts based on the tenant's pricing rules/past jobs.
- **Frontend**: New mobile-optimized cards for the Owner Dashboard to review pending AI-drafted quotes.

## Implementation Prompt
**Objective**: Implement the end-to-end "Agentic Quoting Flow" for service businesses.
**Critical User Journey (CUJ)**:
1. As a Customer, I submit a custom request via the storefront.
2. As the System, the AI Salesperson agent automatically generates a draft quote based on the request.
3. As the Business Owner, I open the mobile-optimized dashboard, see the pending draft quote, and tap "Approve".
4. As a Customer, I receive the approved quote and can pay the deposit.

**Acceptance Criteria**:
- Must include the UI components for the customer inquiry form and the owner approval card (must look perfect at 375px width, utilizing OHC Premium Tokens).
- Must implement the backend logic to route the inquiry to the AI agent queue.
- Must include full E2E Playwright tests covering this exact CUJ, starting from login to the owner dashboard through to the customer payment screen, mocking the AI generation step to ensure test stability.
- Do not prescribe the exact DB schema; focus on the data models required to satisfy the UI and Agent state transitions.

## Priority
P1

## Estimated Scope
Large

## References & Sources Catalog
1. Shopify (https://www.shopify.com/)
2. Wix (https://www.wix.com/)
3. Squarespace (https://www.squarespace.com/)
4. GoDaddy (https://www.godaddy.com/)
5. Square Online (https://squareup.com/)
6. BigCommerce (https://www.bigcommerce.com/)
7. Ecwid (https://www.ecwid.com/)
8. WooCommerce (https://woocommerce.com/)
9. Hostinger (https://www.hostinger.com/)
10. Webflow (https://webflow.com/)
11. Durable (https://durable.co/)
12. 10Web (https://10web.io/)
13. Mixo (https://mixo.io/)
14. Hostinger AI Builder (https://www.hostinger.com/ai-website-builder)
15. Shopify Sidekick (https://www.shopify.com/magic)
16. Wix Studio AI (https://www.wix.com/studio)
17. Framer AI (https://www.framer.com/ai/)
18. Relume Library (https://www.relume.io/)
19. HubSpot AI (https://www.hubspot.com/artificial-intelligence)
20. Klaviyo AI (https://www.klaviyo.com/features/ai)
21. Reddit r/smallbusiness discussion on website builders (https://www.reddit.com/r/smallbusiness/comments/16hxk4y/best_website_builder_for_small_business/)
22. Reddit r/ecommerce discussion on Shopify vs alternatives (https://www.reddit.com/r/ecommerce/comments/14mzx1p/shopify_vs_woocommerce_vs_bigcommerce/)
23. Trustpilot reviews for Shopify (https://www.trustpilot.com/review/www.shopify.com)
24. Trustpilot reviews for Wix (https://www.trustpilot.com/review/wix.com)
25. Trustpilot reviews for Squarespace (https://www.trustpilot.com/review/squarespace.com)
26. App Store reviews for Shopify POS (https://apps.apple.com/us/app/shopify-pos-point-of-sale/id659832283)
27. App Store reviews for Wix Owner (https://apps.apple.com/us/app/wix-owner-website-builder/id1099748482)
28. Shopify Community Forum: Booking Apps (https://community.shopify.com/c/shopify-apps/looking-for-a-good-booking-app/td-p/1110034)
29. Reddit r/sweatystartup: CRM and booking software (https://www.reddit.com/r/sweatystartup/comments/11v6b3e/what_crmbooking_software_do_you_use/)
30. Calendly Integration with Shopify (https://help.calendly.com/hc/en-us/articles/223140508-Embedding-Calendly-on-your-website)
31. Acuity Scheduling Integration (https://help.acuityscheduling.com/hc/en-us/articles/16672322301197-Embedding-Acuity-Scheduling-on-your-website)
32. Typeform vs Typeform Alternatives for Shopify (https://www.typeform.com/alternatives/)
33. Shopify Help Center: Deposits and Partial Payments (https://help.shopify.com/en/manual/orders/create-orders/payment-terms)
34. Stripe Checkout Documentation (https://stripe.com/docs/checkout)
35. Stripe Payment Links Documentation (https://stripe.com/docs/payment-links)
36. Web.dev: Mobile Touch Targets (https://web.dev/accessible-tap-targets/)
37. Apple Human Interface Guidelines: Buttons (https://developer.apple.com/design/human-interface-guidelines/buttons)
38. Material Design: Touch Targets (https://m3.material.io/foundations/accessible-design/accessibility-basics#44-dp-minimum-touch-target)
39. CSS Tricks: Glassmorphism (https://css-tricks.com/glassmorphism-in-css/)
40. Google Fonts: Outfit (https://fonts.google.com/specimen/Outfit)
41. Google Fonts: Inter (https://fonts.google.com/specimen/Inter)
42. Playwright Documentation (https://playwright.dev/docs/intro)
43. Gemini Pro API Documentation (https://ai.google.dev/docs)
44. PostgreSQL ROW LEVEL SECURITY (https://www.postgresql.org/docs/current/ddl-rowsecurity.html)
45. Redis Redlock Algorithm (https://redis.io/docs/manual/patterns/distributed-locks/)
46. OpenTelemetry Documentation (https://opentelemetry.io/docs/)
47. Prometheus Documentation (https://prometheus.io/docs/introduction/overview/)
48. Grafana Dashboards (https://grafana.com/docs/grafana/latest/dashboards/)
49. Tauri Go Router (https://pub.dev/packages/go_router)
50. Zustand State Management (https://github.com/pmndrs/zustand)
51. Bazel Build System (https://bazel.build/)
