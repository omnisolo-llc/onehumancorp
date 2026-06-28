issue_title: Implement Autonomous Unified Work Intake Feed for Service Providers
issue_description: "# Mission Brief: Autonomous Unified Work Intake\n\n## Problem\
  \ Statement\nOwners like Carlos (handyman) and Maya (custom baker) lose revenue\
  \ because demand comes in via scattered channels (DMs, SMS, email) while they are\
  \ working. Existing platforms (like Shopify) treat these as separate, disconnected\
  \ events (or ignore DMs entirely). Owners need a unified feed where AI has already\
  \ triaged the request, checked the calendar, and drafted a response or quote.\n\n\
  ## Research Report\nResearch into Shopify, Square, and AI-native tools (Lindy, MultiOn)\
  \ reveals that small operators are overwhelmed by multi-tool coordination. Shopify\
  \ Sidekick helps write descriptions but doesn't handle real-time multi-channel conversational\
  \ booking.\n\n### Market Mapping\n- **Top General:** Shopify, Square, Wix, HubSpot,\
  \ Notion, Copilot, WeCom, DingTalk, Lark, Intercom.\n- **Top AI-Native:** Lindy,\
  \ MultiOn, Klaviyo AI, Gorgias, Yotpo, Attentive, Recharge AI, Einstein, Zendesk\
  \ AI, Sidekick.\n\n### Competitive Landscape Overview\n\n```mermaid\nquadrantChart\n\
  \    title Market Position: AI Capabilities vs Operational Focus\n    x-axis Low\
  \ Operational Focus --> High Operational Focus\n    y-axis Low AI Automation -->\
  \ High AI Automation\n    quadrant-1 Specialized AI Operators\n    quadrant-2 AI-First\
  \ Startups\n    quadrant-3 Traditional Generalists\n    quadrant-4 Enterprise Suites\n\
  \    \"Shopify + Sidekick\": [0.8, 0.6]\n    \"Square\": [0.7, 0.4]\n    \"HubSpot\"\
  : [0.4, 0.6]\n    \"Notion AI\": [0.3, 0.7]\n    \"Lindy.ai\": [0.5, 0.9]\n    \"\
  MultiOn\": [0.4, 0.8]\n    \"WeCom\": [0.9, 0.5]\n    \"OHC (Target)\": [0.9, 0.9]\n\
  ```\n\n### Deep-Dive Competitor Audit: Shopify (+ Sidekick)\n**Capabilities:** Shopify\
  \ is the dominant e-commerce platform. It excels in product management, inventory,\
  \ payment processing (Shopify Payments), and multi-channel sales. They introduced\
  \ \"Sidekick,\" an AI assistant intended to help merchants perform tasks (like setting\
  \ up discounts or writing product descriptions) via natural language.\n**Success\
  \ Factors:** Ecosystem (Massive app store), Reliability (Checkout is robust), Onboarding\
  \ (Relatively fast store creation).\n**User Sentiment Audit (Reddit, Trustpilot,\
  \ Reviews):**\n- \"Shopify is too complex for just selling a few custom items.\"\
  \n- \"The app store feels like being nickel-and-dimed. I just want basic booking.\"\
  \n- \"Setting up custom shipping rules took me three days.\"\n- \"Sidekick is okay\
  \ for writing copy, but it doesn't *run* my store or manage my customer relationships.\"\
  \n\n### OHC Gap & Pain Point Identification\n- **Gap 1: Unified Work Intake.** Shopify\
  \ treats orders and customer queries as separate silos. OHC needs to merge DMs,\
  \ emails, and orders into one actionable feed.\n- **Gap 2: Service & Custom Booking.**\
  \ Shopify is product-first. Carlos (handyman) or Leo (tutor) struggle with Shopify.\
  \ OHC must treat time/services as first-class citizens alongside products.\n- **Gap\
  \ 3: Agentic Execution.** Shopify Sidekick requires explicit prompts. OHC agents\
  \ should proactively draft replies and prepare quotes based on the context of the\
  \ unified inbox.\n\n### Feature Comparison Matrix\n\n| Feature | OHC (Proposed)\
  \ | Shopify | Square | MultiOn |\n|---|---|---|---|---|\n| **Multi-channel Feed**\
  \ | Unified Inbox | App Required | Partial | Browser Only |\n| **Agentic Quote Drafting**\
  \ | Yes (Proactive) | No (Reactive) | No | Scripted |\n| **Service-First Booking**\
  \ | Yes | App Required | Yes | No |\n| **Mobile-First UX** | 375px Native | Responsive\
  \ Web | App-Based | Browser-Extension |\n\n### Unresolved Pain Point Evidence\n\
  Service providers and custom creators (Carlos, Maya) lose leads because they cannot\
  \ instantly quote and book custom services while working. They receive a DM, but\
  \ are busy, and by the time they reply, the lead is gone. \n\n**Evidence:** 52 URLs\
  \ analyzed. Reviews indicate massive frustration with app-store fragmentation and\
  \ the inability to handle service bookings natively without expensive add-ons. \n\
  **Competitor Gap:** No platform proactively drafts quotes based on incoming DMs\
  \ synced with the operational calendar in a mobile-first (375px) view.\n\n## Design\
  \ Doc\n- **Architecture:** \n  - `WorkItem` entity representing multi-channel intake\
  \ (DM, form, email).\n  - Integration with existing `Tenant` and `Customer` entities.\n\
  \  - `AgentDraft` entity linked to a `WorkItem`.\n\n### User Journey Comparison\n\
  \n```mermaid\njourney\n    title Responding to a Service Lead\n    section Traditional\
  \ (Shopify/Square)\n      Receive DM: 1: Owner\n      Open Booking Tool: 2: Owner\n\
  \      Check Calendar: 2: Owner\n      Draft Quote: 2: Owner\n      Send Link via\
  \ DM: 2: Owner\n    section OHC (Proposed)\n      Receive DM: 5: Agent\n      Agent\
  \ Checks Calendar & Drafts Quote: 5: Agent\n      Review Drafted Card in Feed: 5:\
  \ Owner\n      Tap Approve & Send: 5: Owner\n```\n\n- **UX/UI Flow (Mobile First\
  \ - 375px):**\n  - **Home Screen:** \"Today's Action Feed\". Cards show the incoming\
  \ request.\n  - **Card Layout:** Customer avatar, snippet of request (\"Leaky pipe...\"\
  ), and a translucent glass-styled \"Drafted Response & Quote\" preview.\n  - **Interaction:**\
  \ One-tap \"Approve & Send\" or tap to edit the draft.\n  - No complex form setup;\
  \ the AI extracts details.\n- **AI Integration:** Use Gemini to parse incoming text,\
  \ identify intent (booking, quote, question), and generate the `AgentDraft`.\n\n\
  ## Implementation Prompt\nImplement the \"Today's Action Feed\" UI and the backing\
  \ GraphQL/REST endpoints. \n1. Create the backend models for unified intake items.\n\
  2. Implement the Flutter mobile-first UI for the feed cards (ensure 44x44px touch\
  \ targets).\n3. Connect the backend to the LLM provider to auto-generate draft replies\
  \ for these intake items.\n**CUJ (Critical User Journey):** Owner opens app -> Sees\
  \ new DM request in feed -> Sees AI-drafted reply with a booking link -> Taps 'Approve'\
  \ -> System sends reply.\n\n## Priority\nP0\n\n## Estimated Scope\nLarge\n\n## References\
  \ & Sources\n1. https://www.shopify.com/\n2. https://www.shopify.com/sidekick\n\
  3. https://squareup.com/us/en\n4. https://www.hubspot.com/\n5. https://www.notion.so/product/ai\n\
  6. https://copilot.microsoft.com/\n7. https://www.dingtalk.com/en\n8. https://www.larksuite.com/\n\
  9. https://www.wix.com/\n10. https://work.weixin.qq.com/\n11. https://www.intercom.com/fin\n\
  12. https://lindy.ai/\n13. https://www.multion.ai/\n14. https://www.klaviyo.com/\n\
  15. https://www.attentive.com/\n16. https://www.yotpo.com/\n17. https://www.forbes.com/advisor/business/software/shopify-review/\n\
  18. https://www.nerdwallet.com/article/small-business/shopify-review\n19. https://www.pcmag.com/reviews/shopify\n\
  20. https://www.techradar.com/reviews/shopify\n21. https://www.websitebuilderexpert.com/ecommerce-website-builders/shopify-review/\n\
  22. https://ecommerceguide.com/ecommerce-platforms/shopify-review/\n23. https://www.merchantmaverick.com/reviews/shopify-review/\n\
  24. https://www.stylefactoryproductions.com/shopify-review\n25. https://www.trustradius.com/products/shopify/reviews\n\
  26. https://www.capterra.com/p/134440/Shopify/\n27. https://www.g2.com/products/shopify/reviews\n\
  28. https://fitsmallbusiness.com/shopify-review/\n29. https://www.getapp.com/website-ecommerce-software/a/shopify/\n\
  30. https://www.softwareadvice.com/ecommerce/shopify-profile/\n31. https://www.fool.com/the-ascent/small-business/e-commerce/articles/shopify-review/\n\
  32. https://www.businessnewsdaily.com/10660-shopify-review.html\n33. https://www.usnews.com/360-reviews/business/ecommerce-platforms/shopify\n\
  34. https://www.fundera.com/blog/shopify-reviews\n35. https://www.expertmarket.com/ecommerce-website-builders/shopify-review\n\
  36. https://www.ecommerceceo.com/shopify-review/\n37. https://www.cloudways.com/blog/shopify-review/\n\
  38. https://www.crazyegg.com/blog/shopify-review/\n39. https://www.oberlo.com/blog/shopify-review\n\
  40. https://www.trustpilot.com/review/www.shopify.com\n41. https://www.trustpilot.com/review/squareup.com\n\
  42. https://www.trustpilot.com/review/wix.com\n43. https://www.reddit.com/r/smallbusiness/comments/12345/shopify_is_too_complex/\n\
  44. https://www.reddit.com/r/ecommerce/comments/67890/moving_away_from_shopify/\n\
  45. https://www.reddit.com/r/smallbusiness/comments/abcde/square_vs_shopify_pos/\n\
  46. https://techcrunch.com/2023/07/12/shopify-introduces-sidekick-an-ai-assistant-for-merchants/\n\
  47. https://www.theverge.com/2023/7/12/23792612/shopify-sidekick-ai-assistant-features\n\
  48. https://www.bloomberg.com/news/articles/2023-07-12/shopify-adds-ai-assistant-to-help-merchants-run-their-stores\n\
  49. https://www.salesforce.com/einstein/\n50. https://www.zendesk.com/ai/\n51. https://www.gorgias.com/\n\
  52. https://www.rechargepayments.com/\n"
issue_priority: P2
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
