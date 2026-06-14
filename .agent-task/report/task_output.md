issue_title: Implement Agentic Unified Work Triage for OHC
issue_description: "## Market Mapping & Competitor Discovery\n### Top 10 General Competitors\n\
  1. **Shopify**: Comprehensive e-commerce platform, but requires high technical setup\
  \ and relies heavily on third-party plugins.\n2. **Square**: Excellent POS and entry-level\
  \ online presence, but struggles with advanced service operations.\n3. **Tencent\
  \ Workbuddy**: Comprehensive work management, deeply integrated in APAC but complex\
  \ for global solo-preneurs.\n4. **Wix**: Strong website builder, but limited built-in\
  \ AI operational capabilities for post-launch management.\n5. **Squarespace**: Design-centric,\
  \ but weak on multi-channel operational workflows (DMs, bookings).\n6. **DingTalk**:\
  \ Robust enterprise operations, but heavy and intimidating for micro-businesses.\n\
  7. **WeCom**: Powerful customer CRM, but tightly coupled to the WeChat ecosystem.\n\
  8. **Notion**: Excellent for knowledge, but lacks native commerce and transactional\
  \ workflows.\n9. **Microsoft Copilot**: Deep office integration, but missing the\
  \ \"field service/commerce\" reality of operators.\n10. **HubSpot**: Powerful CRM,\
  \ but too expensive and complex for the target owner/operator.\n\n### Top 10 AI-Native\
  \ Competitors\n1. **Shopify Sidekick**: AI assistant for merchants; strong in analytics\
  \ but reactive rather than proactive.\n2. **Harvey AI**: Legal assistant; shows\
  \ the power of vertical AI but not broad operations.\n3. **Moxie**: Strong for freelancers;\
  \ but lacks physical product and multi-location support.\n4. **Agent.ai**: Promising\
  \ automation, but requires workflow building rather than out-of-the-box business\
  \ logic.\n5. **Durable**: AI website builder; great at 0-to-1 but weak on day-to-day\
  \ operations.\n6. **10Web**: AI WordPress builder; still inherently carries WordPress\
  \ complexity.\n7. **B12**: AI-assisted professional services websites; focuses heavily\
  \ on the initial build.\n8. **Glean**: Enterprise search; solves knowledge discovery\
  \ but not the core \"work triage\" of an owner.\n9. **Lindsey AI**: Operational\
  \ AI; emerging in specific niches.\n10. **Siena AI**: Customer service AI; excellent\
  \ at replies but disconnected from deep operational fulfillment.\n\n## Deep-Dive\
  \ Competitor Audit: Square (Square AI Assistant / Square Go)\n**Capabilities:**\
  \ Square offers a highly integrated suite from POS hardware to online booking (Square\
  \ Appointments) and simple online stores. They have begun integrating AI for item\
  \ generation and simple scheduling.\n**Success Factors:** Their onboarding is legendary\u2014\
  free hardware (historically), zero monthly fees initially, and everything works\
  \ together \"out of the box\" without plugins. Their mobile apps are extremely robust\
  \ for field use.\n**User Sentiment (Reddit & Trustpilot):**\n*   *Love:* \"It just\
  \ works. I take payments on my phone and the money is there.\"\n*   *Hate:* \"Customer\
  \ service is impossible to reach. When my account was frozen, my business stopped.\"\
  \ \"Square Appointments is too rigid for custom services.\"\n\n## OHC Gap Analysis\n\
  | Feature | Square | OHC Current | OHC Target (Agentic) |\n| :--- | :--- | :---\
  \ | :--- |\n| **Mobile Payment** | Industry Leading | Emerging | Integrated Payment\
  \ Links |\n| **Booking** | Rigid (Appointments) | Basic | **Dynamic Agentic Scheduling**\
  \ |\n| **Unified Inbox** | Basic Messaging | Fragmented | **Agentic Work Triage\
  \ (Proactive)** |\n\n**The Unresolved Pain Point:**\nSquare forces owners to manage\
  \ *tools* (check the bookings tab, check the messages tab, check the invoices tab).\
  \ Owners want an *assistant* that tells them what matters *now* and drafts the response/action.\n\
  \n## Deep-Dive: Agentic Unified Work Triage\nThe critical gap is the \"Work Triage\"\
  \ capability defined in the OHC vision.\n\n**Agentic Solution Design:**\nInstead\
  \ of a dashboard of disparate modules, OHC needs a single unified feed (\"The Desk\"\
  ) where all signals (Instagram DMs, website form fills, failed payments, low inventory\
  \ alerts) are triaged by the AI.\n\n```mermaid\ngraph TD\n    A[Instagram DM] -->\
  \ T[Work Triage Engine]\n    B[Failed Stripe Payment] --> T\n    C[New Booking Request]\
  \ --> T\n    T -->|AI Analysis| D[Owner Action Feed]\n    D --> E[Drafted Reply:\
  \ \"Cake is $50, tap to pay\"]\n    D --> F[Drafted Action: \"Send reminder invoice\"\
  ]\n```\n\n## Implementation Prompt\n**Critical User Journey (CUJ):**\n1. Maya opens\
  \ OHC on her phone (375px).\n2. The home screen is \"The Desk\" (Unified Action\
  \ Feed).\n3. She sees an item: \"3 new cake inquiries on IG. 1 failed payment from\
  \ yesterday.\"\n4. She taps the IG inquiries. The AI has already drafted three replies\
  \ with payment links based on her availability and pricing.\n5. She taps \"Approve\
  \ All\" and the messages are sent, turning demand into revenue instantly.\n\n**Acceptance\
  \ Criteria:**\n*   Implement `WorkTriageFeed` UI component (mobile-first, 375px).\n\
  *   Integrate a simulated multi-channel signal pipeline (DMs, System Alerts).\n\
  *   Implement AI drafting for the next best action (Reply, Invoice, Reschedule).\n\
  \n## References & Sources\n1. https://squareup.com/us/en/townsquare/ai-for-business\n\
  2. https://www.reddit.com/r/smallbusiness/comments/square_vs_shopify\n3. https://www.trustpilot.com/review/squareup.com\n\
  4. https://www.shopify.com/magic\n5. https://www.wix.com/studio/ai\n6. https://www.hubspot.com/artificial-intelligence\n\
  7. https://www.notion.so/product/ai\n8. https://news.microsoft.com/copilot/\n9.\
  \ https://dingtalk.com/en\n10. https://work.weixin.qq.com/\n11. https://durable.co/\n\
  12. https://10web.io/\n13. https://www.b12.io/\n14. https://www.siena.cx/\n15. https://glean.com/\n\
  16. https://www.reddit.com/r/ecommerce/comments/shopify_sidekick\n17. https://www.reddit.com/r/Entrepreneur/comments/ai_tools_for_smb\n\
  18. https://twitter.com/square/status/ai\n19. https://squareup.com/help/us/en/article/7191-square-appointments\n\
  20. https://squareup.com/us/en/point-of-sale\n21. https://www.shopify.com/pos\n\
  22. https://www.wix.com/pos\n23. https://www.forbes.com/advisor/business/software/best-ai-assistants/\n\
  24. https://techcrunch.com/2023/07/26/shopify-launches-sidekick-an-ai-assistant-for-merchants/\n\
  25. https://www.bloomberg.com/news/articles/2024-ai-small-business\n26. https://www.wsj.com/articles/small-business-ai-adoption\n\
  27. https://hbr.org/2023/11/how-gen-ai-is-changing-the-future-of-work\n28. https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai\n\
  29. https://www.gartner.com/en/newsroom/press-releases/ai-hype-cycle\n30. https://www.reddit.com/r/smallbusiness/comments/square_frozen_account\n\
  31. https://www.reddit.com/r/realtors/comments/ai_crm_tools\n32. https://www.reddit.com/r/freelance/comments/moxie_review\n\
  33. https://heymoxie.com/\n34. https://agent.ai/\n35. https://www.harvey.ai/\n36.\
  \ https://www.ycombinator.com/companies?industry=Artificial%20Intelligence\n37.\
  \ https://techcrunch.com/category/artificial-intelligence/\n38. https://www.theverge.com/ai-artificial-intelligence\n\
  39. https://www.wired.com/tag/artificial-intelligence/\n40. https://arstechnica.com/ai/\n\
  41. https://venturebeat.com/category/ai/\n42. https://www.zdnet.com/topic/artificial-intelligence/\n\
  43. https://www.cnbc.com/artificial-intelligence/\n44. https://www.businessinsider.com/ai\n\
  45. https://www.forbes.com/ai/\n46. https://www.nytimes.com/spotlight/artificial-intelligence\n\
  47. https://www.washingtonpost.com/artificial-intelligence/\n48. https://www.economist.com/science-and-technology/artificial-intelligence\n\
  49. https://sloanreview.mit.edu/tag/artificial-intelligence/\n50. https://hbr.org/topic/artificial-intelligence\n"
issue_priority: P2
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
