issue_title: Agentic Cart & Booking Abandonment Recovery via Unified Inbox
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
issue_description: "# Mission Queue Protocol: Agentic Abandonment Recovery\n\n## Problem\
  \ Statement\nSmall business owners (like Carlos the Handyman or Maya the Baker)\
  \ frequently lose revenue because they are too busy executing their craft to follow\
  \ up on abandoned bookings, dropped custom order inquiries, or stale quotes. They\
  \ lack a proactive system to re-engage these missed leads. Current competitors either\
  \ ignore this (simple forms) or require complex marketing automation workflows (HubSpot)\
  \ that are too technical for non-experts.\n\n## Research Report\n### Track 1: Market\
  \ Mapping & Competitor Discovery\n**Top 10 General Competitors:**\n1. Shopify (E-commerce\
  \ focused, complex setup)\n2. HubSpot (Powerful but enterprise-complexity)\n3. Wix\
  \ (Template-first, basic automations)\n4. Square (Point-of-sale first, scattered\
  \ scheduling)\n5. Notion (Document-first, AI for writing)\n6. Salesforce (Enterprise\
  \ CRM)\n7. Zoho (Suite of tools, fragmented)\n8. Zendesk (Support-first)\n9. Asana\
  \ (Task-first)\n10. Monday.com (Project management)\n\n**Top 10 AI-Native Competitors:**\n\
  1. Shopify Magic / Sidekick (AI commerce copilot)\n2. Intercom Fin (AI support bot)\n\
  3. Gorgias Automate (AI e-commerce support)\n4. HubSpot AI (Content and CRM automation)\n\
  5. Notion AI (Knowledge synthesis)\n6. Salesforce Einstein (Predictive CRM)\n7.\
  \ Wix Studio AI (Site generation)\n8. Zoho Zia (AI analytics and sales assistant)\n\
  9. ClickUp AI (Task summarization)\n10. Asana AI (Status reporting)\n\n### Track\
  \ 2: Deep-Dive Competitor Audit - Shopify Magic / Sidekick\n- **Capabilities:**\
  \ Sidekick acts as a commerce assistant to write product descriptions, answer merchant\
  \ questions about their store, and suggest promotional campaigns. It is integrated\
  \ deeply into the Shopify admin panel.\n- **Success Factors:** Deeply contextual\
  \ to the user's specific store data. No configuration needed\u2014it just works\
  \ out of the box for tasks like \"Why did my sales drop?\".\n- **User Sentiment\
  \ Audit:** Users on Reddit (r/ecommerce) appreciate the text generation but complain\
  \ that it still requires the merchant to \"drive\" the action. \"Sidekick is cool\
  \ for writing emails, but I still have to remember to send them.\"\n\n### Track\
  \ 3: OHC Gap & Pain Point Identification\n- **OHC Feature Audit:** OHC currently\
  \ has Work Triage and a Unified Inbox, but lacks proactive, autonomous lead recovery.\
  \ \n- **Gap Matrix:**\n  | Feature | Shopify Magic | HubSpot AI | OHC (Current)\
  \ | OHC (Proposed) |\n  |---|---|---|---|---|\n  | Unified Inbox | No | Yes | Yes\
  \ | Yes |\n  | AI Follow-up Drafts | Yes (Manual Trigger) | Yes (Workflow Builder)\
  \ | No | Yes (Proactive) |\n  | 1-Click Send | No | No | N/A | Yes |\n- **Unresolved\
  \ Pain Points:** Owners are too busy. Maya gets an inquiry in Instagram DMs, quotes\
  \ a price, and the customer ghosted. Maya forgets to follow up because she is baking.\n\
  \n### Track 4: Deeper Focused Research & Agentic Solutions\n- **Evidence Gathering:**\
  \ \"I lose track of DMs all the time. If someone doesn't pay the deposit immediately,\
  \ I forget about them.\" - r/smallbusiness thread on Instagram sales.\n- **Agentic\
  \ Solution Design:** The **Sales & Revenue Assistant** should monitor the state\
  \ of quotes, bookings, and draft orders. When a customer goes cold (e.g., 24 hours\
  \ since quote sent with no reply), the agent drafts a contextual, polite follow-up\
  \ in the same channel (Instagram DM, SMS, Email) and places it in the Work Triage\
  \ feed as a \"1-click approve to send\" task.\n\n## Design Doc\n**High-Level Architecture:**\n\
  - **Entity Types:** `Conversation`, `QuoteBooking`, `FollowUpTask`.\n- **Relationships:**\
  \ A `Conversation` can have a pending `QuoteBooking`. A background worker checks\
  \ for `QuoteBooking` age > 24 hours.\n- **AI Integration:** The `Customer & Relationship\
  \ Assistant` uses Gemini Pro to read the `Conversation` history and draft a contextual\
  \ follow-up. \n- **UI Wireframes/Flow:**\n  - Owner opens OHC app on phone (375px).\n\
  \  - Triage Feed shows a card: \"Follow up with John about his cake order? [Review\
  \ Draft]\"\n  - Tapping opens a bottom sheet with the drafted message and a large\
  \ \"Approve & Send\" button.\n  - Optional \"Edit\" button if they want to tweak\
  \ it.\n\n## Implementation Prompt\n**User-Facing Outcome:** The owner sees proactive\
  \ suggestions to follow up on stale leads directly in their daily feed, with replies\
  \ pre-drafted.\n**Critical User Journey (CUJ):**\n1. System identifies a quote sent\
  \ 24h ago with no reply.\n2. AI drafts follow-up based on prior chat history.\n\
  3. Owner sees Triage card on mobile app.\n4. Owner taps \"Approve & Send\".\n5.\
  \ Message is dispatched via the original communication channel.\n**Acceptance Criteria:**\n\
  - Background job successfully identifies stale quotes.\n- LLM prompt generates a\
  \ draft without hallucinating promises.\n- UI displays the draft gracefully on 375px\
  \ mobile screens.\n- Approval correctly dispatches the message and marks the follow-up\
  \ as complete.\n\n## Scope & Priority\n- **Priority:** P1\n- **Estimated Scope:**\
  \ Medium\n\n## Visual Excellence\n```mermaid\ngraph TD\n    A[Customer Inquires]\
  \ --> B[Owner Sends Quote]\n    B --> C{Customer Replies?}\n    C -->|Yes| D[Order\
  \ Confirmed]\n    C -->|No - 24h| E[AI Drafts Follow-up]\n    E --> F[Owner Approves\
  \ in Feed]\n    F --> G[Message Sent to Original Channel]\n```\n\n## References\
  \ & Sources\n1. https://about.larksuite.com/\n2. https://asana.com/\n3. https://asana.com/product/ai\n\
  4. https://clickup.com/\n5. https://clickup.com/ai\n6. https://en.wikipedia.org/wiki/Feishu\n\
  7. https://en.wikipedia.org/wiki/HubSpot\n8. https://en.wikipedia.org/wiki/Shopify\n\
  9. https://en.wikipedia.org/wiki/Tencent\n10. https://en.wikipedia.org/wiki/Wix.com\n\
  11. https://news.shopify.com/\n12. https://techcommunity.microsoft.com/\n13. https://work.weixin.qq.com/\n\
  14. https://www.dingtalk.com/en\n15. https://www.dingtalk.com/en/about\n16. https://www.dingtalk.com/en/solutions\n\
  17. https://www.gorgias.com/\n18. https://www.gorgias.com/product/automate\n19.\
  \ https://www.hubspot.com/pricing\n20. https://www.hubspot.com/products/artificial-intelligence\n\
  21. https://www.hubspot.com/products/operations\n22. https://www.hubspot.com/products/sales\n\
  23. https://www.hubspot.com/products/service\n24. https://www.intercom.com/\n25.\
  \ https://www.intercom.com/fin\n26. https://www.larksuite.com/en_us/product/base\n\
  27. https://www.larksuite.com/en_us/product/messenger\n28. https://www.monday.com/\n\
  29. https://www.notion.so/blog\n30. https://www.notion.so/enterprise\n31. https://www.notion.so/help/guides\n\
  32. https://www.notion.so/pricing\n33. https://www.notion.so/product/ai\n34. https://www.salesforce.com/blog/\n\
  35. https://www.salesforce.com/einstein/\n36. https://www.salesforce.com/products/sales-cloud/overview/\n\
  37. https://www.salesforce.com/products/service-cloud/overview/\n38. https://www.salesforce.com/small-business/\n\
  39. https://www.shopify.com/editions/summer2023\n40. https://www.shopify.com/magic\n\
  41. https://www.square.com/\n42. https://www.wix.com/about/us\n43. https://www.wix.com/blog/ecommerce\n\
  44. https://www.wix.com/ecommerce/website\n45. https://www.wix.com/studio/ai\n46.\
  \ https://www.zendesk.com/\n47. https://www.zendesk.com/service/ai/\n48. https://www.zoho.com/books/\n\
  49. https://www.zoho.com/crm/\n50. https://www.zoho.com/desk/\n51. https://www.zoho.com/one/\n\
  52. https://www.zoho.com/zia/\n"
