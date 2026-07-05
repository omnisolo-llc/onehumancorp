issue_title: "Implement AI Unified Intake Feed (Work Triage)"
issue_description: |
  # Research Report: AI Unified Intake Feed

  ## Problem Statement
  Owners and operators face a disjointed experience when receiving inbound demand. Messages arrive via Instagram DMs, SMS, WhatsApp, and email, while bookings and payments land in separate portals. Maya (baker), for instance, has to manually check three different apps to figure out which cake orders need attention today. This creates context switching, missed leads, and anxiety.

  ## Market Mapping & Competitor Discovery
  We researched general competitors (Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, Shopify, Square, HubSpot, Notion, Microsoft Copilot) and AI-native competitors (e.g., AI work assistants, AI commerce copilots, autonomous operations managers, AI CRM assistants).

  ## Deep-Dive Competitor Audit: Tencent Workbuddy
  Tencent Workbuddy's core success lies in its unified feed. It aggregates messages, approvals, and tasks into a single actionable timeline. Users praise its ability to reduce context switching, allowing them to handle an inquiry, generate a quote, and send a payment link from one screen.

  ## OHC Gap & Pain Point Identification
  OHC currently lacks a unified feed. We have separate views for messages, tasks, and bookings, forcing the owner to manually piece together their day. Unresolved pain points include:
  - "I forget to reply to Instagram DMs when I'm busy with bookings."
  - "I don't know what's urgent when I open the app."

  ## Agentic Solution Design
  The AI Unified Intake Feed will act as the single entry point for the owner.
  - **Work Triage:** Unifies messages, tasks, bookings, payments, customer requests, and system alerts into a prioritized owner feed.
  - **AI Agent:** Explains why something matters and proposes the next action (e.g., "Maya, this DM is about a cake for tomorrow. Draft a reply with a deposit link?").

  ## Implementation Prompt
  - **Outcome:** The owner logs in and sees a prioritized feed of items needing attention (Work Triage).
  - **CUJ:** Maya opens OHC -> Sees 3 new DMs grouped with a pending booking -> Clicks "Draft Reply" -> AI agent drafts a response with a payment link -> Maya approves and sends.
  - **Acceptance Criteria:** Work items from different sources (messages, bookings, alerts) are aggregated into a single feed. Each item shows a reason why it matters and an AI-proposed next action.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
