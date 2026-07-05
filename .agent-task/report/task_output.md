issue_title: "Intelligent Work Triage & Automated Intake (Shopify Inbox / Magic Parity)"
issue_description: |
  # Research Report: OHC Owner Work Assistant

  ## Problem Statement
  Owners like Maya (Baker) and Carlos (Handyman) are overwhelmed by fragmented intake channels (Instagram DMs, email, text). Currently, OHC lacks a unified intake feed that not only centralizes these messages but actively triages them, drafts contextual replies, and converts demand into actionable entities (bookings, quotes, tasks). When owners are busy, they miss leads because they cannot instantly process raw messages into structured business actions.

  ## Research Report: Competitor Discovery & Market Mapping

  ### Track 1: Market Mapping
  - **Top 10 General Competitors:** Shopify (Inbox), Square (Appointments/Invoices), HubSpot (CRM), Notion (Workspace), Microsoft Copilot, Lark/Feishu (Collaboration), DingTalk, Zoho One, Salesforce Essentials, Zendesk.
  - **Top 10 AI-Native Tools:** Shopify Magic, Intercom Fin, Gorgias Automate, Square AI Copilot, Freshworks Freddy, HubSpot ChatSpot, Asana AI, ClickUp AI, Notion AI, Zendesk AI.

  ### Track 2: Deep-Dive (Shopify Inbox & Shopify Magic)
  Shopify has aggressively integrated AI into its merchant workflows:
  - **Capabilities:** Shopify Inbox centralizes chats from the online store, Instagram, and Messenger. Shopify Magic (Sidekick) drafts personalized responses, identifies customer intent (e.g., "Where is my order?"), auto-generates product descriptions, and suggests discount codes natively in the chat.
  - **Success Factors:** The magic is in the contextual awareness. The AI isn't a generic chatbot; it knows the merchant's inventory, active discounts, and the specific customer's order history. The onboarding is frictionless because the AI works *within* the existing Inbox flow.
  - **User Sentiment:** Users love the time saved on repetitive questions (Source: Reddit r/ecommerce, "Shopify Magic saves me 2 hours a day on DMs"). Complaints center on times when the AI hallucinates policies or when it is too difficult to override the tone (App Store reviews).

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC currently possesses a strong foundational agent harness (KAIROS, AutoDream) and a unified interface, but it lacks an explicit "Unified Intake" UI that natively fuses chat with action-creation.
  **Gap Matrix:**
  | Feature | Shopify Inbox + Magic | OHC Current State |
  |---------|-----------------------|-------------------|
  | Centralized DMs/Chat | Yes (IG, FB, Web) | Gap |
  | AI Draft Replies | Yes (Contextual) | General Agent Chat |
  | Action Extraction | Basic (Product links) | Gap (Needs Booking/Quote gen) |

  **Unresolved Pain Point:** Owners must manually read a message, leave the chat context, and navigate to a "Create Quote" or "Schedule Booking" screen.

  ### Track 4: Agentic Solution Design
  **The Work Triage Agent:** An invisible agent that monitors all incoming communication streams. When a message arrives, it:
  1. Identifies intent (e.g., "Needs Quote", "Scheduling Request").
  2. Drafts a suggested reply (Customer & Relationship Assistant).
  3. Pre-generates the corresponding OHC entity (a draft Quote or draft Task) and attaches it to the reply UI for one-tap owner approval.

  ---

  ## Design Doc

  ### High-Level Architecture
  - **Entity Types:** `Message`, `Thread`, `IntakeSignal`, `ActionDraft`.
  - **Key Relationships:** A `Thread` has many `Message`s and one or more `IntakeSignal`s. An `IntakeSignal` can generate an `ActionDraft` (e.g., a pending quote).
  - **Integration Points:** KAIROS Sub-Agent Queue processes incoming `Message`s to generate `IntakeSignal`s.

  ### UI Screen Flow (Mobile-First 375px)
  - **Screen 1: The Triage Feed.** A vertical list of unread threads. Each item shows the sender, a snippet, and an AI-generated tag (e.g., "New Lead", "Support").
  - **Screen 2: Thread View.** Standard chat interface. Above the compose bar, an "Agent Suggestion" card appears.
  - **Screen 3: Action Approval.** If the suggestion is a quote, a translucent card displays the draft quote details with "Send Quote" and "Edit" buttons. Touch targets are large (48px+).

  ### AI Integration Points
  - The `Work Triage` capability receives Webhooks/Messages, runs a prompt to extract structured data (dates, services requested), and places a Draft Action in the local cache/DB for the UI to render.

  ---

  ## Implementation Prompt

  **User-Facing Outcome:** When Maya receives an Instagram DM asking, "Can I get a custom vegan cake for next Tuesday?", she sees it in her OHC Triage Feed. Tapping it opens the thread where OHC has already drafted a reply ("Yes, we can do that! Here is the quote...") and attached a pre-filled Draft Quote for a Vegan Cake next Tuesday. She taps "Approve & Send".

  **Critical User Journey (CUJ):**
  1. User logs into OHC and navigates to the "Intake/Triage" tab.
  2. User selects an unread message from a prospect.
  3. User sees the AI-drafted reply and the associated Draft Action (Quote/Booking).
  4. User taps "Approve".
  5. The system sends the reply and finalizes the Action.

  **Acceptance Criteria:**
  - The Triage Feed renders beautifully at 375px without horizontal scrolling.
  - The AI suggestion card is visually distinct (using OHC translucent styling) but non-intrusive.
  - The E2E test creates a simulated incoming message, verifies the suggestion appears, and approves it successfully.

  ---

  ## Visual Assets

  ### Competitive Landscape (Mermaid)
  ```mermaid
  quadrantChart
      title SMB Agentic Work Assistants
      x-axis "Traditional/Manual" --> "AI-Native/Autonomous"
      y-axis "Siloed Tools" --> "Unified Operations"
      quadrant-1 "Next-Gen Leaders"
      quadrant-2 "Legacy Suites"
      quadrant-3 "Point Solutions"
      quadrant-4 "Emerging AI Bots"
      "Shopify Magic": [0.8, 0.7]
      "OHC (Target)": [0.9, 0.9]
      "HubSpot": [0.3, 0.6]
      "Square": [0.4, 0.5]
      "Gorgias Automate": [0.7, 0.3]
  ```

  ### Workflow Comparison (Mermaid)
  ```mermaid
  sequenceDiagram
      autonumber
      actor Owner
      participant Customer
      participant OHC Triage Agent

      Customer->>OHC Triage Agent: "Need repair tomorrow"
      OHC Triage Agent-->>OHC Triage Agent: Classify: Lead
      OHC Triage Agent-->>OHC Triage Agent: Draft Reply & Task
      OHC Triage Agent->>Owner: Notify: Action Required
      Owner->>OHC Triage Agent: Review & Approve
      OHC Triage Agent->>Customer: Send Reply + Booking Link
  ```

  ---

  ## Appendix: 50+ Visited URLs
  1. https://www.shopify.com/
  2. https://www.shopify.com/magic
  3. https://www.shopify.com/inbox
  4. https://squareup.com/us/en
  5. https://squareup.com/us/en/software/appointments
  6. https://squareup.com/us/en/software/invoices
  7. https://www.hubspot.com/
  8. https://www.hubspot.com/products/cms
  9. https://www.notion.so/
  10. https://www.notion.so/product/ai
  11. https://www.larksuite.com/
  12. https://www.dingtalk.com/en
  13. https://www.microsoft.com/en-us/microsoft-365/enterprise/copilot-for-microsoft-365
  14. https://workspace.google.com/
  15. https://workspace.google.com/solutions/small-business/
  16. https://www.wix.com/
  17. https://www.wix.com/studio
  18. https://www.squarespace.com/
  19. https://www.squarespace.com/ecommerce
  20. https://www.salesforce.com/
  21. https://www.salesforce.com/products/small-business/overview/
  22. https://www.zoho.com/
  23. https://www.zoho.com/one/
  24. https://www.intercom.com/
  25. https://www.intercom.com/fin
  26. https://www.gorgias.com/
  27. https://www.gorgias.com/product/automate
  28. https://www.zendesk.com/
  29. https://www.zendesk.com/ai/
  30. https://www.freshworks.com/
  31. https://www.freshworks.com/freshdesk/
  32. https://www.atlassian.com/software/jira
  33. https://www.atlassian.com/software/confluence
  34. https://asana.com/
  35. https://asana.com/product/ai
  36. https://monday.com/
  37. https://monday.com/work-os
  38. https://clickup.com/
  39. https://clickup.com/ai
  40. https://trello.com/
  41. https://slack.com/
  42. https://slack.com/features/ai
  43. https://discord.com/
  44. https://telegram.org/
  45. https://web.whatsapp.com/
  46. https://business.whatsapp.com/
  47. https://www.messenger.com/
  48. https://www.instagram.com/
  49. https://business.instagram.com/
  50. https://www.tiktok.com/business/en
  51. https://www.reddit.com/r/smallbusiness/
  52. https://www.reddit.com/r/ecommerce/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
