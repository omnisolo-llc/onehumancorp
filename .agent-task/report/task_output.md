issue_title: "Implement 'Today's Action Feed' Command Center UI"
issue_description: |
  # Mission Queue Protocol: 'Today's Action Feed' Command Center UI

  ## Problem Statement
  Small business owners (like Maya the baker or Carlos the handyman) are currently overwhelmed by scattered work demand. They check Instagram DMs for inquiries, a notebook or separate scheduling app for bookings, Stripe for payments, and sticky notes for reminders. Existing solutions like Shopify or HubSpot are too complex and feel like admin portals rather than assistive interfaces. Owners need a single, unified "Command Center" feed that tells them exactly what needs attention *today*, why it matters, and what to do next.

  ## Research Report
  ### Market Mapping & Competitor Discovery
  - **Tencent Workbuddy & WeCom**: Excels at merging chat, internal tasks, and approval flows into a single daily feed. Highly successful due to its chat-first interface which owners are already familiar with.
  - **Shopify Sidekick**: AI copilot that acts on commands but lacks a proactive, unified feed of prioritized daily work actions (it waits for the user to ask).
  - **Notion AI**: Good for knowledge synthesis, but poor for transactional, operational daily triage.
  - **Chatwoot**: Provides an omnichannel inbox, but lacks the concept of operational tasks, bookings, and revenue actions intermingled with messages.

  ### Deep-Dive Competitor Audit: WeCom (Tencent)
  - **Capabilities**: Unifies customer messages (WeChat), internal tasks, approvals, and daily summaries into a single feed.
  - **Success Factors**: Zero-learning-curve interface. The owner opens the app and immediately sees a chronologically and priority-sorted list of actionable items.
  - **User Sentiment**: Reviews heavily praise the fact that nothing falls through the cracks. "I don't need to check 5 apps; my morning starts with the WeCom feed."

  ### OHC Gap & Pain Point Identification
  - **Current State**: OHC currently lacks a centralized, unified feed. Agent actions, customer messages, and operations tasks are siloed or hard to triage simultaneously.
  - **The Gap**: When an owner logs in, they are not immediately presented with a prioritized list of actionable items (e.g., "Maya, you have 3 pending cake orders and 1 deposit missing").

  ## Design Doc
  ### High-Level Architecture
  - **Entity Types**: `ActionItem` (polymorphic entity representing a Message, Booking, Payment Request, or Agent Draft).
  - **Key Relationships**: Links to `Tenant`, `Customer`, and `AgentDraft`.
  - **Integration Points**: Backend AI Job Queue (to generate actionable summaries) and the existing omnichannel chat/messaging tables.

  ### Mobile UX Flow (375px first)
  1. **Home Shell (`AppShell`)**: Replaces the generic dashboard with the "Today's Action Feed".
  2. **Feed List**: A vertical scroll of `ActionItem` cards. Each card uses macOS translucent glass styling and robust typography.
  3. **Action Card Anatomy**:
     - **Context**: "New DM from @sarah_bakes"
     - **Summary**: "Asking about availability for a vegan wedding cake next Saturday."
     - **Agent Draft / Recommendation**: AI-generated reply preview or "Approve & Send Quote" button.
  4. **Interactive Action**: Tapping "Approve" immediately dispatches the action and dismisses the card with a satisfying, fast animation.

  ## Implementation Prompt
  **Critical User Journey (CUJ):**
  1. User (Owner) opens the OHC mobile app or Next.js PWA.
  2. The initial screen (AppShell) displays the "Today's Action Feed".
  3. The user sees a prioritized list of tasks (e.g., "Review draft reply to Customer X", "Approve estimate for Customer Y").
  4. The user clicks "Approve & Send" on a draft reply. The item is marked done and removed from the feed.

  **Acceptance Criteria:**
  - Build the `Today's Action Feed` UI within the Next.js/Flutter shell.
  - Cards must render unified inbox items, pending agent drafts, and operational alerts.
  - Include an "Approve & Send" or "Approve" primary action button on agent-drafted cards.
  - Ensure 100% responsiveness down to 375px width (no horizontal scrolling).
  - The UI must use the OHC Premium Token library (translucent materials, clean spacing).
  - *No database schemas or exact API contracts are prescribed here; implementers will design the optimal data flow.*
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
