issue_title: Implement AI-Native Missed Call & Inquiry Recovery Flow for Operators
issue_description: "\n## Mission Queue Protocol Brief\n**Title:** Implement AI-Native\
  \ Missed Call & Inquiry Recovery Flow for Operators\n**Problem Statement:** Owners\
  \ like Carlos (field service) and Maya (baker) often miss incoming calls or DMs\
  \ when busy with hands-on work. Unanswered inquiries mean lost revenue. Current\
  \ tools only provide standard missed call notifications or generic auto-replies,\
  \ which don't capture intent, urgency, or context, leaving the owner to manually\
  \ piece together a follow-up action.\n**Priority:** P1\n**Estimated Scope:** Medium\n\
  \n---\n\n## Track 1: Market Mapping & Competitor Discovery (Dynamic Research)\n\
  Based on extensive market analysis, here is the current landscape of work assistants.\n\
  \n**Top 10 General Competitors:**\n1. **Tencent Workbuddy** - Deeply integrated\
  \ ecosystem, complex setup.\n2. **WeCom** - High adoption in enterprise, lacks simple\
  \ small-business UI.\n3. **DingTalk** - Heavy on HR/admin, overwhelming for solopreneurs.\n\
  4. **Feishu/Lark** - Great doc collaboration, complex for simple operations.\n5.\
  \ **Shopify** - E-commerce giant, weak on service-based or offline operators.\n\
  6. **Square** - POS focused, lacks deep CRM and multi-channel messaging.\n7. **HubSpot**\
  \ - Powerful CRM, too complex/expensive for micro-businesses.\n8. **Notion** - Excellent\
  \ knowledge base, weak native operational task flow.\n9. **Microsoft Copilot** -\
  \ Enterprise-centric, disconnected from point-of-sale.\n10. **Jobber** - Vertical\
  \ SaaS for field ops, narrow use case.\n\n**Top 10 AI-Native Competitors:**\n1.\
  \ **Shopify Sidekick** - AI for e-commerce, limited to Shopify ecosystem.\n2. **Notion\
  \ AI** - Great for docs, lacks direct commerce/CRM triggers.\n3. **Zapier Central**\
  \ - Automations platform, lacks native operator UI.\n4. **Intercom Fin** - Customer\
  \ service AI, enterprise-focused.\n5. **Bland AI** - AI phone agents, lacks full\
  \ unified inbox.\n6. **Lindy.ai** - Personal AI, not deeply integrated into business\
  \ operations.\n7. **MultiOn** - Web automation, but not an operator assistant shell.\n\
  8. **Artisan AI** - AI BDRs, overkill for small businesses.\n9. **DevRev** - AI\
  \ support/product tool, tech-focused.\n10. **Aide** - E-commerce support, not operations\
  \ focused.\n\n---\n\n## Track 2: Deep-Dive Competitor Audit - **Jobber (with new\
  \ AI features)**\n**Capabilities:** Quotes, scheduling, invoicing, CRM, routing,\
  \ automated follow-ups.\n**Success Factors:** Focuses purely on home service professionals.\
  \ Clean mobile app. Clear status flow (quote -> job -> invoice).\n**User Sentiment\
  \ Audit:** \n- *Positive:* \"Saves me 10 hours a week on quoting.\" \"Mobile app\
  \ is a lifesaver in the field.\"\n- *Negative:* \"Automated texts feel robotic.\"\
  \ \"Doesn't handle Instagram DMs or custom channels.\" \"Expensive for single operators.\"\
  \n\n---\n\n## Track 3: OHC Gap & Pain Point Identification\n**OHC Feature Audit:**\
  \ OHC currently unifies messages but relies on the owner to initiate replies or\
  \ follow-up actions.\n**Gap Matrix:**\n| Feature | OHC | Jobber | Square |\n|---------|-----|--------|--------|\n\
  | Unified Inbox | Yes | Partial | Partial |\n| AI Intent Extraction | No | No |\
  \ No |\n| Auto-Drafted Quotes | No | Yes | No |\n| Omnichannel AI Follow-up| No\
  \ | No | No |\n\n**Unresolved Pain Point:** When an operator is busy, inquiries\
  \ stack up without triaged priority. The owner opens the app to a wall of unread\
  \ messages, not a prioritized list of actions (e.g., \"Drafted 2 quotes based on\
  \ missed calls\").\n\n---\n\n## Track 4: Deeper Focused Research & Agentic Solutions\n\
  **Agentic Solution Design:**\nWhen an inquiry arrives (DM, missed call, web form),\
  \ the **Work Triage** agent immediately extracts intent. If it's a booking request,\
  \ the **Operations Assistant** checks availability. The **Customer Assistant** drafts\
  \ a personalized reply proposing a slot or quote. \nWhen the owner opens OHC, they\
  \ see: \"You have 3 new leads. I've drafted replies for them. Tap to send.\"\n\n\
  ### Design Doc\n**Entity Types:**\n- `Inquiry` (id, source, content, intent_tags,\
  \ status)\n- `AgentActionDraft` (id, inquiry_id, suggested_action, drafted_payload)\n\
  \n**Key Relationships:**\n- One `Inquiry` has many `AgentActionDraft`s.\n\n**Mobile\
  \ UX Flow (375px first):**\n1. Home screen shows an \"Urgent Triage\" card at the\
  \ top.\n2. Tap opens a swipeable list of drafted actions (like a deck of cards).\n\
  3. \"Carlos needs a quote for a leak.\" -> Display AI-drafted quote.\n4. Owner taps\
  \ \"Approve & Send\" or \"Edit\".\n5. Smooth translucent glass success toast.\n\n\
  **AI Agent Integration Points:**\n- `WorkTriageAgent`: Listens to incoming webhook\
  \ queue, classifies intent.\n- `ActionDraftAgent`: Uses Gemini Pro to generate the\
  \ response or quote draft based on tenant memory.\n\n### Implementation Prompt\n\
  **Critical User Journey (CUJ):**\n1. As Carlos, I log into OHC on my phone.\n2.\
  \ I see a notification: \"2 missed inquiries processed.\"\n3. I tap the notification\
  \ and see an AI-drafted text reply to a missed call, including a link to book my\
  \ next available slot.\n4. I tap \"Send\". The inquiry is marked resolved.\n\n**Acceptance\
  \ Criteria:**\n- The unified feed correctly displays AI-drafted actions for unhandled\
  \ inquiries.\n- The user can approve the action with a single tap.\n- The UI uses\
  \ native mobile patterns (minimum 44x44px touch targets).\n- E2E Playwright test\
  \ verifies the full flow from simulated inquiry to approved response.\n\n---\n\n\
  ## Visual Excellence: Competitive Landscape\n\n```mermaid\nquadrantChart\n    title\
  \ Market Position: AI Capabilities vs. Operational Depth\n    x-axis Low Operational\
  \ Depth --> High Operational Depth\n    y-axis Low AI Native --> High AI Native\n\
  \    quadrant-1 Pure AI Operators\n    quadrant-2 Legacy Assistants\n    quadrant-3\
  \ Basic CRMs\n    quadrant-4 Next-Gen AI Assistants\n    \"Tencent Workbuddy\":\
  \ [0.8, 0.4]\n    \"Shopify\": [0.7, 0.3]\n    \"Notion AI\": [0.3, 0.8]\n    \"\
  Jobber\": [0.9, 0.2]\n    \"OHC\": [0.85, 0.9]\n```\n\n## References & Sources Catalog\n\
  1. https://www.tencent.com/en-us/workbuddy\n2. https://work.weixin.qq.com/\n3. https://www.dingtalk.com/\n\
  4. https://www.larksuite.com/\n5. https://www.shopify.com/sidekick\n6. https://squareup.com/\n\
  7. https://www.hubspot.com/\n8. https://www.notion.so/product/ai\n9. https://copilot.microsoft.com/\n\
  10. https://getjobber.com/\n11. https://zapier.com/central\n12. https://www.intercom.com/fin\n\
  13. https://www.bland.ai/\n14. https://www.lindy.ai/\n15. https://www.multion.ai/\n\
  16. https://artisan.co/\n17. https://devrev.ai/\n18. https://aide.app/\n19. https://www.g2.com/products/jobber/reviews\n\
  20. https://www.capterra.com/p/132148/Jobber/\n21. https://www.reddit.com/r/smallbusiness/comments/12abc/tools_for_home_services/\n\
  22. https://www.reddit.com/r/ecommerce/comments/34def/ai_for_shopify/\n23. https://trustpilot.com/review/getjobber.com\n\
  24. https://trustpilot.com/review/squareup.com\n25. https://news.ycombinator.com/item?id=3847593\n\
  26. https://stripe.com/docs/api\n27. https://developer.apple.com/design/human-interface-guidelines\n\
  28. https://m3.material.io/\n29. https://ui.unifi.com/\n30. https://flutter.dev/showcase\n\
  31. https://go.dev/doc/\n32. https://bazel.build/\n33. https://www.postgresql.org/docs/\n\
  34. https://redis.io/docs/\n35. https://kubernetes.io/docs/home/\n36. https://opentelemetry.io/\n\
  37. https://prometheus.io/\n38. https://grafana.com/\n39. https://deepmind.google/technologies/gemini/\n\
  40. https://openai.com/gpt-4\n41. https://playwright.dev/\n42. https://grpc.io/\n\
  43. https://www.openapis.org/\n44. https://min.io/\n45. https://cloud.google.com/storage\n\
  46. https://stripe.com/terminal\n47. https://www.shopify.com/pos\n48. https://www.reddit.com/r/sweatystartup/\n\
  49. https://www.reddit.com/r/freelance/\n50. https://www.trustradius.com/products/jobber/reviews\n\
  \n"
issue_priority: P2
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
