issue_title: "AI Work Assistant Market Research & Gap Analysis for OneHumanCorp"
issue_description: |
  # Market Research: AI Work Assistants for Owners and Operators

## Problem Statement
Small business owners and operators face high cognitive load trying to manage communications, scheduling, revenue, and tasks across disconnected tools. Many AI solutions (like Shopify Sidekick or Microsoft Copilot) require significant setup, assume technical proficiency, or are too broad to provide actionable next steps for a busy proprietor working from a mobile device. The core gap is the absence of an assistant-first tool that acts on the owner's behalf to coordinate work invisibly without a complicated admin dashboard.

## Research Report

### Real User Personas & Pain Points
1. **Maya (Baker, 28, Instagram DMs)**:
   - **Pain Point**: Overwhelmed by Shopify complexity; needs simple mobile triage for custom orders and deposits.
   - **Evidence**: Small creators on Shopify complain about the "blank canvas" setup flow. Sidekick helps with some analytics but doesn't manage DM intakes directly without complex plugin setups.
2. **Carlos (Handyman, 42, Android)**:
   - **Pain Point**: No booking system, quoting is manual, misses leads when busy.
   - **Evidence**: Traditional CRMs (HubSpot) require too much manual data entry for a field service worker.
3. **Priya (Boutique Operator, 35)**:
   - **Pain Point**: Inventory sync is broken between in-store POS and online; email marketing is disjointed.
   - **Evidence**: Users of Square and Shopify often complain about the disconnect between offline POS inventory and online store availability in high-volume settings.

### Top 10 General Competitors
1. **Shopify**: Excellent commerce engine, but often overwhelming for non-technical users.
2. **Notion**: Highly flexible knowledge base, but lacks built-in transactional capabilities.
3. **Microsoft Copilot**: Deep integration into Office, but complex and enterprise-focused (see WSJ: "The Microsoft 365 Copilot launch was a total disaster").
4. **HubSpot CRM**: Powerful but often too "sales-heavy" and requires high setup overhead.
5. **Square**: Great POS and basic scheduling, but limited omnichannel AI assistance.
6. **Wix**: Good for website building, less capable for autonomous operations.
7. **Tencent Workbuddy**: Strong all-in-one assistant approach, heavily focused on APAC markets.
8. **WeCom**: Great for enterprise team collaboration, but not optimized for solo operators.
9. **DingTalk**: Robust features but can feel bloated with corporate management tools.
10. **Feishu/Lark**: Excellent collaborative suite, but requires significant structure.

### Top 10 AI-Native Competitors
1. **Salesforce Einstein**: Deeply integrated CRM AI, but enterprise-tier pricing.
2. **Gusto AI**: Focuses on HR, but limited outside of that domain.
3. **Zendesk AI**: Great for support, lacks operational and commerce capabilities.
4. **Intercom Fin**: Good for automated support resolution, but expensive for small owners.
5. **Shopify Sidekick**: Rising commerce copilot, but still requires the user to understand the Shopify admin paradigm.
6. **Notion AI**: Good for summarizing and drafting docs, but no operational workflow integration.
7. **Motion**: AI scheduling, but lacks commerce integration.
8. **Dust**: Internal knowledge retrieval, missing customer interaction.
9. **Sana**: AI-powered knowledge management, not for frontline commerce.
10. **Chatwoot**: Open-source customer engagement platform, heavily relies on manual agents rather than a holistic AI assistant approach. (Note: OHC is building a custom Rust alternative).

### Deep-Dive Audit: Shopify Sidekick & Microsoft Copilot
- **Capabilities**: Shopify Sidekick can answer questions about store performance and edit theme settings. Copilot generates docs and code.
- **Success Factors**: Integrated directly into the platform the merchant already uses. Reduces time spent digging through analytics.
- **User Sentiment Audit**:
  - Reddit/HN users note Microsoft Copilot feels unfinished or overly intrusive (e.g., "Microsoft Copilot is now injecting ads into pull requests").
  - Small business owners express frustration with Copilot's "hallucinations" and Shopify Sidekick's limitation to only the Shopify ecosystem.

### Gap & Pain Point Analysis
- **Gap**: OHC is currently missing a unified mobile-first interface (375px) where an AI assistant proactively surfaces tasks across all work streams (DMs, bookings, POS) rather than waiting for user queries.

## Design Doc

### High-Level Architecture
- **Frontend**: Flutter PWA. 375px mobile-first layout. No horizontal scrolling. The main screen is a conversational feed combining tasks, notifications, and AI suggestions.
- **Backend**: Rust-based omnichannel messaging service replacing Chatwoot dependencies. Integrates with existing Go/Bazel services for business logic.
- **Data Model**:
  - `Task`: represents an actionable item (e.g., "Reply to Maya", "Confirm Booking").
  - `Conversation`: unified thread from multiple channels.
  - `Suggestion`: AI-generated next step attached to a Task or Conversation.

### User Experience (UX)
- **Mobile Flow (375px)**:
  1. Home screen opens to "Today's Priorities": a vertical list of urgent tasks (e.g., 3 pending DMs, 1 deposit needed).
  2. Tapping a task opens a modal with a pre-drafted response or action button (e.g., "Send Draft", "Request $50").
  3. Swipe to dismiss or delegate to the AI agent.

```mermaid
graph TD
    A[Owner opens OHC (Mobile)] --> B[Today's Priorities Feed]
    B --> C{{Urgent Action?}}
    C -->|Yes| D[Review AI Draft/Action]
    C -->|No| E[View Summary Dashboard]
    D --> F[Approve/Edit/Send]
    F --> G[Task Marked Complete]
```

### Feature Comparison
| Feature | OHC | Shopify Sidekick | Notion AI |
| :--- | :---: | :---: | :---: |
| Mobile-first Assistant UI | ✅ | ❌ | ❌ |
| Proactive Task Generation | ✅ | ❌ | ❌ |
| Unified Omnichannel (Rust) | ✅ | ❌ | ❌ |
| Complex Admin Dashboards | ❌ | ✅ | ✅ |

## Implementation Prompt
Implement the "Today's Priorities" feed for the Flutter mobile application. The CUJ begins with the owner logging in and landing on the home screen. The screen must render a list of actionable `Task` entities fetched from the backend API. Each task should display an AI-generated suggested action (e.g., a drafted reply or a quick-action button). The UI must be perfectly responsive at 375px width, utilizing the OHC Premium Token library (translucent materials, clear typography). Ensure that tapping an action button successfully mutates the state (e.g., marks the task as done) and that the change is reflected immediately in the UI. Do not use any mocked data; connect to the live Go/Rust backend. Add Playwright E2E tests to verify the flow from login to task completion.

## Estimated Scope
Medium

## Priority
P1

## References & Sources
1. https://news.ycombinator.com/item?id=2516654
2. https://shopify.engineering/building-production-ready-agentic-systems
3. https://news.ycombinator.com/item?id=9807945
4. https://www.palico.ai/fastest-way-to-add-gen-ai-to-your-rich-text-editor
5. https://news.ycombinator.com/item?id=47441587
6. https://news.ycombinator.com/item?id=46203547
7. https://news.ycombinator.com/item?id=44994031
8. https://news.ycombinator.com/item?id=24838322
9. https://www.wix.engineering/post/when-ai-becomes-your-on-call-teammate-inside-wix-s-airbot-that-saves-675-engineering-hours-a-month
10. https://www.wsj.com/articles/ai-work-assistants-need-a-lot-of-handholding-500c2bd8
11. https://paulrobichaux.com/2023/12/14/first-look-at-microsoft-365-copilot/
12. https://news.ycombinator.com/item?id=40799516
13. https://github.com/Eyjafjallajokull/wecomp
14. https://news.ycombinator.com/item?id=39052425
15. https://www.neowin.net/news/microsoft-copilot-is-now-injecting-ads-into-pull-requests-on-github-gitlab/
16. https://news.ycombinator.com/item?id=7684827
17. https://news.ycombinator.com/item?id=37150202
18. https://www.office.com
19. https://news.ycombinator.com/item?id=45679248
20. https://news.ycombinator.com/item?id=42950775
21. https://news.ycombinator.com/item?id=47566736
22. https://pulse.support/blog/wix-ai-announcement
23. https://news.ycombinator.com/item?id=22380398
24. https://news.ycombinator.com/item?id=46853894
25. https://www.zdnet.com/home-and-office/work-life/the-microsoft-365-copilot-launch-was-a-total-disaster/
26. https://news.ycombinator.com/item?id=46972281
27. https://news.ycombinator.com/item?id=34899928
28. https://news.ycombinator.com/item?id=45629302
29. https://news.ycombinator.com/item?id=33623201
30. http://product.hubspot.com/blog/why-we-made-the-hubspot-crm-free
31. https://blog.coutinho.io/introducing-astrolabe-navigate-your-data-universe-in-nextcloud
32. http://www.sitepoint.com/article/learn-adobe-air-part-2
33. https://news.ycombinator.com/item?id=36707648
34. https://news.ycombinator.com/item?id=587162
35. https://news.ycombinator.com/item?id=44801366
36. https://www.promptarmor.com/resources/microsoft-copilot-cowork-exfiltrates-files
37. https://news.ycombinator.com/item?id=4231968
38. https://news.ycombinator.com/item?id=46629972
39. https://github.com/mozilla-ai/clawbolt
40. https://news.ycombinator.com/item?id=18173145
41. https://news.ycombinator.com/item?id=11858434
42. https://news.ycombinator.com/item?id=46829268
43. https://news.ycombinator.com/item?id=48198681
44. https://www.wsj.com/tech/ai/microsofts-pivotal-ai-product-is-running-into-big-problems-ce235b28
45. https://news.ycombinator.com/item?id=29377199
46. https://news.ycombinator.com/item?id=36238041
47. https://www.suggestcat.com/
48. https://news.ycombinator.com/item?id=45405476
49. https://news.ycombinator.com/item?id=45367996
50. https://www.nuvio.io/
51. https://news.ycombinator.com/item?id=2577206
52. https://news.ycombinator.com/item?id=38346830
53. https://lspace.swyx.io/p/reverse-prompt-eng
54. https://www.impactbnd.com/blog/hubspot-crm-vs-salesforce
55. http://online.wsj.com/news/articles/SB10001424052702303825604579513882989476424?mg=reno64-wsj
56. https://news.ycombinator.com/item?id=44192361
57. https://news.ycombinator.com/item?id=34493529
58. https://news.ycombinator.com/item?id=14350506
59. https://news.ycombinator.com/item?id=43822695
60. https://news.ycombinator.com/item?id=32400849

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
