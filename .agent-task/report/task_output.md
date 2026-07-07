issue_title: "Agentic Voice-to-Task Operations Intake Pipeline"
issue_description: |
  ## Title: Agentic Voice-to-Task Operations Intake Pipeline

  ## Problem Statement
  Small business owners who work in the field (e.g., Carlos the Handyman, Maya the Baker when baking) cannot easily pull out their phone, navigate a UI, and type out a task, reminder, or customer note. Their hands are full, dirty, or they are driving. Existing platforms require them to either write things on paper, send a text to themselves, or navigate a complex mobile app to add a simple CRM note or task.

  ## Research Report
  - **Market Context**: Traditional platforms (Shopify, Wix, even specialized tools like Jobber) rely heavily on keyboard input for task creation and CRM updates. While some offer voice-to-text on the OS level, it's just transcription, not intelligent interpretation.
  - **The OHC Opportunity**: By introducing an "Agentic Voice-to-Task Intake Pipeline", owners can simply tap a microphone button (or use a lock-screen widget) and speak: "Remind me to order more caulk for the Smith job on Tuesday, and send them a quote for the bathroom remodel." The Operations Agent will parse this, create a calendar reminder, add a task, and the Sales Agent will draft a quote for the Smith customer.
  - **Competitor Gaps**:
    - *Jobber*: Requires navigating through several menus to add a task or note.
    - *Siri/Google Assistant*: Good for personal reminders, but not integrated into the business platform's CRM or multi-agent workflows.

  ## Design Doc
  ### Architecture
  - **Audio Intake (Mobile/PWA)**: A prominent, easily accessible microphone button on the mobile UI (375px first). It captures audio via the Web Audio API or Flutter native equivalent.
  - **Transcription Service**: Integration with Whisper API (or similar lightweight on-device transcription) to convert audio to text.
  - **AI Triage Agent (The Orchestrator)**: Parses the transcribed text to identify intent, entities (customers, dates, products, tasks), and routes them to the appropriate agent departments (Operations for tasks, Sales for quotes, CRM for notes).
  - **Data Model (PostgreSQL)**:
    - `VoiceIntakeEvent`: Stores the raw transcription, timestamp, and processing status.
    - Links to `Tasks`, `Customers`, `Quotes` generated from the event.

  ### Mobile UX Flow (375px)
  1. **Intake**: A floating action button (FAB) with a microphone icon is available on the main OHC dashboard.
  2. **Recording**: When tapped, a translucent glass modal appears indicating active listening, with a visualization of the audio wave.
  3. **Processing**: Upon release, the modal shows a "Processing..." state (Agent Triage).
  4. **Approval Card**: The feed immediately shows a card: "Based on your audio, I've created a reminder for Tuesday and drafted a quote for Smith. [Approve & Send]".

  ### AI Agent Integration
  - **Triage Agent**: Acts as the central router for transcribed text.
  - **Operations Agent**: Creates tasks and calendar events based on extracted intents.
  - **Customer Success Agent**: Drafts communications (quotes, emails) based on instructions in the audio.

  ## Implementation Prompt
  **Feature Name**: Agentic Voice-to-Task Intake Pipeline
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos can tap a button and dictate a complex multi-step request (task + CRM update + quote generation), and the OHC agents will automatically break it down into actionable cards in his feed.

  **Next Actions**:
  1. Implement the Flutter mobile UI for audio recording and the prominent FAB on the dashboard (adhering to 375px constraints).
  2. Integrate a transcription service (e.g., Whisper) to convert audio to text.
  3. Develop the `Triage Agent` logic to parse the transcription, extract intents, and orchestrate actions across the `Operations Agent` and `Customer Success Agent`.
  4. Ensure the resulting actions are presented as single-tap "Approval Cards" in the user's unified feed.
  5. Create Playwright E2E tests simulating the voice intake flow (using a mock transcription response) to verify the correct creation of tasks and drafts.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
