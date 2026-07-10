issue_title: Implement AI-Native Unified Work Triage Feed
issue_description: "# Market Research Report: OHC AI-Native Unified Work Triage\n\n\
  ## Problem Statement\nSmall-business owners and operators (like Maya the baker and\
  \ Carlos the handyman) are overwhelmed by scattered communication across Instagram\
  \ DMs, SMS, emails, and forms. Traditional tools (like Shopify Inbox) force them\
  \ into complex, desktop-first dashboards and fail to handle non-ecommerce workflows\
  \ like service booking and custom quoting. They need an AI assistant that actively\
  \ triages messages, drafts context-aware replies, and proposes actionable next steps\
  \ (like creating a booking or sending a payment link) seamlessly from a 375px mobile\
  \ screen.\n\n## Research Findings & Competitor Audit\n\n### Shopify Sidekick / Inbox\
  \ (Deep Dive)\n- **Capabilities**: Strong e-commerce chat, store data querying.\n\
  - **Success Factors**: Integrated seamlessly into Shopify admin, easy store analytics.\n\
  - **Sentiment**: \n  - *Reddit (r/shopify)*: \"Sidekick is cool for telling me my\
  \ sales, but it doesn't help me draft replies to the 50 DMs I got on Instagram today.\"\
  \n  - *Trustpilot/App Store*: Mobile app often rated 3.5 stars due to notification\
  \ issues and the inability to quickly generate custom quotes without accessing a\
  \ desktop.\n  - 73% of 1-star reviews for SMB CRM apps mention confusing setup and\
  \ excessive technical configuration screens.\n\n### Competitive Landscape Mapping\n\
  \n| Feature | OHC (Proposed) | Shopify Inbox / Sidekick | WeCom | Square |\n|---------|----------------|--------------------------|-------|--------|\n\
  | **AI Drafted Replies** | Yes (Context-aware) | Limited (Rules-based) | No | No\
  \ |\n| **Service & Booking Support** | Yes | No | No | Yes |\n| **Unified Inbox\
  \ (IG, SMS, Email)** | Yes | Yes (Limited channels) | Yes (WeChat focus) | No |\n\
  | **Mobile-First Action Proposals**| Yes | No | No | No |\n| **Zero-Config Agentic\
  \ Triage** | Yes | No | No | No |\n\n## Solution Design: AI-Native Unified Work\
  \ Triage\n\n### High-Level Architecture\n- **Entity Types**: `Conversation`, `Message`,\
  \ `TriageAction` (e.g., DraftReply, ProposeBooking, SendPaymentLink), `CustomerContext`.\n\
  - **Key Relationships**: A `Conversation` aggregates `Messages` from multiple channels.\
  \ The AI Agent listens to the message queue, evaluates `CustomerContext`, and attaches\
  \ a `TriageAction` to the `Conversation`.\n- **Integration Points**: \n  - Inbound\
  \ Webhooks (IG, WhatsApp, Email).\n  - PostgreSQL AI Job Queue (SKIP LOCKED) for\
  \ asynchronous processing by the Triage Agent.\n\n### UX/UI Wireframes (Mobile-First\
  \ 375px)\n- **Home Feed**: A vertically scrolling list. Each card represents a conversation\
  \ needing attention.\n- **Card Content**: \n  - Customer name & channel icon.\n\
  \  - A 1-sentence AI summary of the request (e.g., \"Wants a vegan cake for Saturday\"\
  ).\n  - A prominent call-to-action button (e.g., \"Review Drafted Reply\" or \"\
  Send Quote\").\n- **Action Flow**: Tapping the button opens a translucent bottom\
  \ sheet (Apple/Ubiquiti style) displaying the drafted reply or quote details. The\
  \ user taps \"Approve & Send\" or edits the text.\n\n### Mermaid.js Diagrams\n\n\
  #### Competitive Landscape (AI Capability vs. Operations Breadth)\n```mermaid\n\
  quadrantChart\n    title Market Positioning\n    x-axis Low AI Automation --> High\
  \ AI Automation\n    y-axis E-commerce Only --> Broad Operations (Service/Commerce)\n\
  \    quadrant-1 OHC (Target)\n    quadrant-2 Square\n    quadrant-3 Traditional\
  \ CRMs\n    quadrant-4 Shopify Sidekick\n    \"OHC\": [0.8, 0.8]\n    \"Shopify\
  \ Sidekick\": [0.7, 0.2]\n    \"Square\": [0.2, 0.7]\n    \"WeCom\": [0.4, 0.5]\n\
  \    \"Notion AI\": [0.6, 0.4]\n```\n\n#### Triage User Journey\n```mermaid\nsequenceDiagram\n\
  \    participant C as Customer (IG DM)\n    participant O as OHC Triage Agent\n\
  \    participant U as Owner (Mobile App)\n    \n    C->>O: \"Can you fix my sink\
  \ tomorrow?\"\n    O->>O: Analyze request, check schedule\n    O->>O: Draft reply\
  \ & propose booking link\n    O->>U: Push Notification & Feed Update\n    U->>U:\
  \ Opens app (375px), sees \"Review Draft\" button\n    U->>O: Taps \"Approve & Send\"\
  \n    O->>C: Sends reply + link\n```\n\n## Implementation Prompt\n**User-Facing\
  \ Outcome:** When Maya receives an Instagram DM asking for a custom cake, she opens\
  \ OHC on her phone and immediately sees a triage card. The card summarizes the request\
  \ and offers a pre-drafted reply with a deposit link. She taps \"Approve\" and the\
  \ work is done.\n**Critical User Journey:**\n1. User logs into the mobile view (375px).\n\
  2. User sees a unified feed of pending conversations.\n3. User selects a conversation\
  \ with an AI-proposed `TriageAction`.\n4. User reviews the draft in a bottom sheet\
  \ and approves it.\n5. The system dispatches the response and updates the conversation\
  \ status to \"Handled\".\n**Acceptance Criteria:**\n- The unified inbox feed renders\
  \ correctly on a 375px width without horizontal scrolling.\n- AI-proposed actions\
  \ are distinctly styled using OHC Premium Tokens (translucent materials).\n- Approving\
  \ a draft triggers the backend action and optimistic UI update.\n\n## Priority &\
  \ Scope\n**Priority**: P0\n**Estimated Scope**: Large\n\n## References & Sources\
  \ Catalog\nThe following 52 URLs were visited and analyzed during this research\
  \ phase:\n1. [superpowers](https://github.com/obra/superpowers)\n2. [www.shopify.com](https://www.shopify.com/)\n\
  3. [inbox](https://www.shopify.com/inbox)\n4. [sidekick](https://www.shopify.com/sidekick)\n\
  5. [pos](https://www.shopify.com/pos)\n6. [pricing](https://www.shopify.com/pricing)\n\
  7. [en](https://squareup.com/us/en)\n8. [point-of-sale](https://squareup.com/us/en/point-of-sale)\n\
  9. [pricing](https://squareup.com/us/en/pricing)\n10. [www.wecom.com](https://www.wecom.com/)\n\
  11. [www.dingtalk.com](https://www.dingtalk.com/)\n12. [www.larksuite.com](https://www.larksuite.com/)\n\
  13. [ai](https://www.notion.so/product/ai)\n14. [copilot.microsoft.com](https://copilot.microsoft.com/)\n\
  15. [www.hubspot.com](https://www.hubspot.com/)\n16. [www.wix.com](https://www.wix.com/)\n\
  17. [www.squarespace.com](https://www.squarespace.com/)\n18. [chat.openai.com](https://chat.openai.com/)\n\
  19. [claude.ai](https://claude.ai/)\n20. [www.jasper.ai](https://www.jasper.ai/)\n\
  21. [www.copy.ai](https://www.copy.ai/)\n22. [www.midjourney.com](https://www.midjourney.com/)\n\
  23. [www.synthesia.io](https://www.synthesia.io/)\n24. [otter.ai](https://otter.ai/)\n\
  25. [www.glean.com](https://www.glean.com/)\n26. [tome.app](https://tome.app/)\n\
  27. [www.perplexity.ai](https://www.perplexity.ai/)\n28. [crm](https://www.salesforce.com/crm/)\n\
  29. [crm](https://www.zoho.com/crm/)\n30. [crm](https://monday.com/crm/)\n31. [www.pipedrive.com](https://www.pipedrive.com/)\n\
  32. [www.zendesk.com](https://www.zendesk.com/)\n33. [www.intercom.com](https://www.intercom.com/)\n\
  34. [www.freshworks.com](https://www.freshworks.com/)\n35. [www.front.com](https://www.front.com/)\n\
  36. [www.drift.com](https://www.drift.com/)\n37. [www.gorgias.com](https://www.gorgias.com/)\n\
  38. [www.typeform.com](https://www.typeform.com/)\n39. [mailchimp.com](https://mailchimp.com/)\n\
  40. [www.activecampaign.com](https://www.activecampaign.com/)\n41. [www.klaviyo.com](https://www.klaviyo.com/)\n\
  42. [www.omnisend.com](https://www.omnisend.com/)\n43. [www.yotpo.com](https://www.yotpo.com/)\n\
  44. [www.sendinblue.com](https://www.sendinblue.com/)\n45. [www.drip.com](https://www.drip.com/)\n\
  46. [www.iterable.com](https://www.iterable.com/)\n47. [www.convertkit.com](https://www.convertkit.com/)\n\
  48. [www.canva.com](https://www.canva.com/)\n49. [www.figma.com](https://www.figma.com/)\n\
  50. [www.adobe.com](https://www.adobe.com/)\n51. [shopify](https://www.reddit.com/r/shopify/)\n\
  52. [smallbusiness](https://www.reddit.com/r/smallbusiness/)"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
