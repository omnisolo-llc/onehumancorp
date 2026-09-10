issue_title: "Unified Mobile-First Omnichannel Inbox & AI Triage for Small Business Operators"
issue_description: |
  Title: Unified Mobile-First Omnichannel Inbox & AI Triage for Small Business Operators

  Problem Statement:
  Operators like Maya (baker) and Carlos (handyman) receive inquiries across Instagram DMs, WhatsApp, SMS, and email. Currently, they miss leads because they have to constantly switch apps on their phone while working. Traditional solutions (Zendesk, Intercom, Gorgias) are built for desktop-bound support agents, have steep learning curves, charge per-seat pricing that doesn't fit micro-businesses, and lack native AI to automatically draft quotes or booking links directly in the thread.

  Research Report:
  Competitor Analysis:
  1. Gorgias: Excellent for Shopify stores, deep e-commerce integration. Pain point: Desktop-centric, priced for higher volume, overkill for service operators like Carlos.
  2. Zendesk: Industry standard for support. Pain point: Complex setup, heavily ticketing-focused instead of conversation-focused. Mobile app is clunky.
  3. Shopify Inbox: Free for Shopify users. Pain point: Only works for Shopify. Maya and Carlos need something that works outside a strict e-commerce CMS.
  4. Intercom: Powerful AI features. Pain point: Extremely high starting costs for AI features, making it inaccessible for solo operators.
  5. Meta Business Suite: Consolidates FB/IG. Pain point: Buggy mobile notifications, doesn't integrate SMS/Email or booking systems.

  Comparative Feature Matrix (OHC vs Benchmarks):
  | Feature / Tool | OHC (Proposed) | Gorgias | Zendesk | Intercom | Meta Business Suite |
  |---|---|---|---|---|---|
  | **Mobile-First Design (375px)** | ✅ Core Focus | ⚠️ Clunky | ❌ Desktop-first | ⚠️ Okay, but complex | ⚠️ Buggy notifications |
  | **Unified IG, WA, SMS, Email** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ❌ IG/FB only |
  | **Pricing Model** | ✅ Included in tier | ❌ Volume-based | ❌ Per-seat (expensive) | ❌ Per-seat (very expensive) | ✅ Free |
  | **Native AI Drafts/Triage** | ✅ Built-in | ✅ Add-on | ✅ Add-on | ✅ Expensive add-on | ❌ No |
  | **Service Business Focus** | ✅ Yes (Carlos/Maya) | ❌ E-comm focus | ❌ Enterprise focus | ❌ Tech/SaaS focus | ❌ Generic |

  Verified Sources:
  1. Gorgias Pricing & Features: https://www.gorgias.com/pricing (Shows minimum tier and e-commerce focus)
  2. Zendesk Community Forums: https://support.zendesk.com/hc/en-us/community/topics (Highlights complexity for small teams)
  3. Shopify Inbox Overview: https://www.shopify.com/inbox (Shows tight coupling to Shopify ecosystem)
  4. Intercom Pricing: https://www.intercom.com/pricing (Highlights high starting costs for AI features)
  5. Reddit r/smallbusiness discussion on DMs: https://www.reddit.com/r/smallbusiness/ (Users complaining about missing Instagram DMs)
  6. Meta Business Suite App Reviews: https://apps.apple.com/us/app/meta-business-suite/id514643583 (Common complaints about notification reliability)

  Design Doc:
  High-level System Architecture:
  ```mermaid
  graph TD
      A[Instagram DM] --> |Webhook| D[OHC Ingress]
      B[WhatsApp] --> |Webhook| D
      C[Email/SMS] --> |Webhook| D
      D --> E[Message Normalization]
      E --> F[AI Triage Agent]
      F --> |Intent & Draft| G[Unified Inbox DB]
      G --> H[Flutter Mobile Client - 375px]
      H --> |Approve Draft| I[Egress Service]
      I --> J[Platform APIs]
  ```

  Mobile UX Flow (375px first):
  1. Home Feed: Unified list of active conversations, sorted by urgency (AI-ranked).
  2. Conversation View: Clean chat interface. At the bottom, an AI Suggestion chip floats above the keyboard (e.g., Draft Quote for $150).
  3. Action Modal: Tapping the chip opens a half-sheet to review the quote. One tap to send.
  4. Customer Context: Swiping left on the chat reveals a sidebar drawer with customer history, total spent, and upcoming bookings.

  Implementation Prompt:
  Critical User Journey:
  1. Maya receives an Instagram DM asking Do you have vegan cakes for this Saturday?
  2. The message arrives in the OHC Mobile App's unified inbox.
  3. The AI Triage agent automatically reads the message, checks Maya's availability for Saturday, and drafts a reply: Yes! We have chocolate vegan cakes. I can fit you in for Saturday. It will be $45. Shall I send a deposit link?
  4. Maya opens the notification on her 375px mobile screen, sees the draft, and taps Approve & Send.
  5. The message is sent back to the customer's Instagram DM seamlessly.

  Acceptance Criteria:
  - Unified inbox UI in Flutter matching 375px mobile constraints.
  - Native Rust adapters for at least two channels (e.g., IG and WhatsApp).
  - AI Triage agent integration to auto-generate response drafts.
  - No dummy data; use real unified inbox store.

  Priority: P0
  Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
