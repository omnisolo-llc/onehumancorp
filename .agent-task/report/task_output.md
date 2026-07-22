issue_title: "Integrate Cal.com for Unified Omnichannel Scheduling"
issue_description: |
  ## Title: Integrate Cal.com for Unified Omnichannel Scheduling

  ### Problem Statement
  Owners and operators like Carlos (Field Service Owner) and Leo (Creator and Tutor) struggle to coordinate their time across multiple channels. When customers reach out via WhatsApp, Instagram DMs, or email to book a repair or a lesson, the owner currently has to bounce between their calendar, their chat apps, and manual notes to find a slot. This context-switching leads to double bookings, delayed responses, and lost revenue. They need a way for the OHC assistant to automatically generate bookable links or propose available times directly in the conversation flow, without needing a separate scheduling tool dashboard.

  ### Research Report
  **Market Context & Competitor Analysis:**
  Competitors like HubSpot, Square, and Wix all offer scheduling capabilities, but they often force the user into their proprietary scheduling ecosystem. Modern owners are looking for unified, friction-free scheduling that integrates directly with their communication flows.

  **Selected Tool:** **Cal.com**
  Cal.com is an open-source, developer-friendly scheduling infrastructure that provides API-first appointment booking.

  **Why Cal.com?**
  1. **Dual-Environment Support (Critical):** Cal.com supports both managed Cloud (multi-tenant via their API) and Standalone (self-hosted/local) deployments, aligning perfectly with OHC's architectural constraints.
  2. **White-Labeling:** It allows scheduling to feel native to the owner's brand rather than redirecting customers to a third-party portal.
  3. **Extensibility:** Built-in support for video conferencing (Zoom, Google Meet), payments (Stripe, which OHC uses), and calendar routing.
  4. **Ease of Use for Owners:** The owner doesn't need to learn a new tool. The OHC assistant simply syncs with their existing calendars (Google, Apple, Outlook) via Cal.com's infrastructure and generates available slots on demand.

  **Pricing & Viability:**
  Cal.com offers a generous free tier for individuals (essential for entry-level owners) and scalable team/platform pricing. The self-hosted version is free and open-source, providing ultimate flexibility for OHC's Standalone mode.

  ### Design Doc
  **Integration Flow:**
  - **Setup:** When an owner connects their calendar in OHC, behind the scenes, OHC provisions a Cal.com user (via Platform API) or maps to a self-hosted Cal.com instance.
  - **Trigger (Work Triage & Customer Assistant):** When a customer message indicates intent to meet or book a service (e.g., "Can you come fix this on Tuesday?"), the OHC assistant detects this intent.
  - **Action:** The OHC Operations Assistant securely fetches available time slots using the Cal.com API based on the owner's configured availability and real-time calendar conflicts.
  - **User Experience:** The OHC assistant drafts a reply containing natural-language proposed times and a tap-to-book link (powered by Cal.com's embed or headless UI).
  - **Resolution:** When the customer selects a time, Cal.com handles the calendar invite insertion and sends a webhook back to OHC. OHC then updates the owner's daily feed ("Work Intake") and moves the task to "Upcoming Commitments".

  ### Implementation Prompt
  **User-Facing Outcome:**
  As a non-technical owner, I want my OHC assistant to automatically offer my available times to customers in chats and emails, and instantly add confirmed bookings to my daily work feed, so I don't have to manually check my calendar and type out time slots.

  **Acceptance Criteria:**
  1. The OHC UI provides a simple "Connect Calendar" button that authorizes calendar access without exposing Cal.com configuration details.
  2. The OHC AI Assistant can generate a personalized booking link or propose three available time slots within a drafted chat/email reply.
  3. When a customer booked a slot via the generated link, the booking instantly appears in the owner's OHC "Today's Priorities" or "Upcoming Commitments" view.
  4. The solution gracefully falls back or shows a clear error state if the calendar sync is interrupted.
  5. Tested end-to-end via Playwright mimicking an owner connecting a calendar, an AI generating a link, and a simulated webhook resolving the booking.

  ### Priority
  P2

  ### Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
