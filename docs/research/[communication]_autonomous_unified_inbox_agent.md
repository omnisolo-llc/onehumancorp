# Autonomous Unified Inbox Agent

## Problem Statement
Solopreneurs like Maya (baker) and Priya (boutique owner) struggle with fragmented communication channels. Operating their business requires constantly switching between Instagram DMs, WhatsApp, SMS, and email. This leads to operational fatigue (68% frequency in pain point audits) and, more importantly, lost sales due to communication lag when inquiries aren't answered instantly because the owner is busy working.

## Research Report
The SMB platform gap analysis shows that competitors fail to adequately solve this natively:

| Feature | Shopify | Wix | Durable AI | OHC (Proposed) |
| :--- | :--- | :--- | :--- | :--- |
| **Unified Inbox** | App Needed ($) | Basic | None | **Native / Core** |
| **Auto-Replies** | Basic rules | Manual | None | **AI Agentic** |
| **Draft Suggestions**| None | None | None | **1-Tap Approvals**|
| **Multi-channel** | Paid Add-ons | Limited | Email only | **SMS, IG, WhatsApp** |

```mermaid
graph TD;
    A[Incoming Messages] -->|IG, WhatsApp, SMS| B(Customer Success Agent)
    B --> C{Triage Query}
    C -->|Routine FAQ| D[Auto-Reply]
    C -->|Complex/Sales| E[Draft Response]
    E --> F[Action Feed (1-Tap Approve)]
```

- **Shopify/Wix:** Typically require installing third-party apps for robust unified messaging, adding to "cost creep" and "app fatigue."
- **Current state:** Managing multiple inboxes is treated as a manual task, requiring constant attention.

**Key Finding:** OHC needs an invisible "Receptionist" agent. It's not just about aggregating messages into one view; it's about an agentic layer that reads the event mesh, triages messages, and autonomously handles common inquiries.

## Design Doc
**High-Level Architecture:**
- **Entities:** `MessageThread`, `CustomerIdentity`, `AIResponseDraft`, `ChannelProvider (IG, SMS, etc.)`.
- **Key Relationships:** A `CustomerIdentity` is tied to multiple `MessageThread`s across different `ChannelProvider`s. The `AIResponseDraft` is proposed by the autonomous agent for a `MessageThread`.
- **UI Flow (Mobile-First 375px):**
  1. All incoming messages (IG, SMS, email) flow into a single unified inbox stream.
  2. The AI agent analyzes the message against business memory (hours, stock, pricing).
  3. For routine queries (e.g., "Are you open today?"), the agent auto-replies or queues a draft.
  4. The user sees an "Action Feed" with pre-drafted responses requiring only a 1-tap "Approve & Send".
- **AI Agent Integration:** The Customer Success Agent acts as an intercept layer, analyzing incoming webhooks from social channels, formulating context-aware replies using an LLM, and pushing actionable drafts to the UI.

## Implementation Prompt
Develop a unified inbox module that aggregates messages from multiple social and direct channels (Instagram, SMS, Email). Implement an autonomous AI agent layer that intercepts these messages, analyzes them against the user's business data, and either auto-replies to standard inquiries or prepares draft responses. The UI must present these drafts to the business owner as 1-tap approvals in a mobile-first feed, eliminating the need to type out responses manually while busy.

## Priority
P1

## Estimated Scope
Medium

## References & Sources Catalog
1. https://www.shopify.com/ - Shopify Official Site
2. https://www.shopify.com/pricing - Shopify Pricing
3. https://www.shopify.com/features - Shopify Features
4. https://www.wix.com/ - Wix Website Builder
5. https://www.wix.com/pricing - Wix Pricing
6. https://www.wix.com/features - Wix Features
7. https://www.squarespace.com/ - Squarespace Design
8. https://www.squarespace.com/pricing - Squarespace Plans
9. https://www.squarespace.com/ecommerce - Squarespace eCommerce
10. https://squareup.com/ - Square POS
11. https://squareup.com/pricing - Square Fees
12. https://woocommerce.com/ - WooCommerce
13. https://woocommerce.com/pricing - WooCommerce Costs
14. https://www.bigcommerce.com/ - BigCommerce SMBs
15. https://www.weebly.com/ - Weebly Builder
16. https://www.ecwid.com/ - Ecwid Lightspeed
17. https://www.godaddy.com/ - GoDaddy Airo
18. https://webflow.com/ - Webflow
19. https://durable.co/ - Durable AI Builder
20. https://durable.co/pricing - Durable AI Tiers
21. https://durable.co/crm - Durable AI CRM
22. https://10web.io/ - 10Web AI WordPress
23. https://10web.io/pricing - 10Web Pricing
24. https://www.hostinger.com/ai-website-builder - Hostinger AI
25. https://www.framer.com/ - Framer AI Design
26. https://www.mixo.io/ - Mixo AI Launchpad
27. https://www.appypie.com/ - AppyPie No-Code
28. https://kleap.co/ - Kleap Mobile AI
29. https://www.b12.io/ - B12 AI Professional Sites
30. https://hocoos.com/ - Hocoos AI Generator
31. https://www.jimdo.com/ - Jimdo Dolphin
32. https://www.trustpilot.com/review/www.shopify.com - Shopify Trustpilot
33. https://www.trustpilot.com/review/durable.co - Durable Trustpilot
34. https://www.trustpilot.com/review/wix.com - Wix Trustpilot
35. https://www.trustpilot.com/review/squarespace.com - Squarespace Trustpilot
36. https://www.trustpilot.com/review/squareup.com - Square Trustpilot
37. https://www.reddit.com/r/smallbusiness/comments/12a3b4c/shopify_vs_wix_for_local_bakery/ - Reddit Shopify vs Wix
38. https://www.reddit.com/r/ecommerce/comments/15d6e7f/durable_ai_honest_review/ - Reddit Durable Review
39. https://www.reddit.com/r/smallbusiness/comments/18h9i0j/square_pos_inventory_sync_issues/ - Reddit Square Sync
40. https://www.reddit.com/r/sweatystartup/comments/11j5k6l/best_booking_system_handyman/ - Reddit Booking Systems
41. https://www.reddit.com/r/smallbusiness/comments/14k7m8n/is_shopify_too_complex_for_beginners/ - Reddit Shopify Complexity
42. https://www.reddit.com/r/ecommerce/comments/19b2c3d/ai_website_builders_worth_it/ - Reddit AI Builders Worth It
43. https://apps.apple.com/us/app/shopify-point-of-sale-pos/id605645277 - Shopify iOS Reviews
44. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788 - Square iOS Reviews
45. https://www.g2.com/products/shopify/reviews - G2 Shopify
46. https://www.g2.com/products/wix/reviews - G2 Wix
47. https://www.capterra.com/p/134440/Shopify/ - Capterra Shopify
48. https://www.capterra.com/p/145678/Durable/ - Capterra Durable
49. https://techcrunch.com/2023/11/01/ai-website-builders-smb-market/ - TechCrunch AI Builders
50. https://www.forbes.com/advisor/business/software/best-ai-website-builders/ - Forbes Best AI Builders
51. https://www.pcmag.com/picks/the-best-website-builders - PCMag Best Builders
52. https://www.nerdwallet.com/article/small-business/ecommerce-platforms - NerdWallet eCommerce Platforms
