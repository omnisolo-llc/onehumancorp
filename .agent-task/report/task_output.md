issue_title: "Reserve Capacity: Google Workspace Calendar Integration"
issue_description: |
  **Title**: Reserve Capacity: Google Workspace Calendar Integration

  **Problem Statement**: Small business owners like service providers, consultants, and field operators need to reserve capacity (e.g., booking appointments, deposits, buffers) without double-booking or overselling. They currently juggle manual calendar checks and disparate scheduling tools. They need an integrated solution where the AI team can verify availability, reserve slots, handle timezones, and process cancellations automatically, directly syncing with their primary calendar.

  **Research Report**:
  - **Strategy**: Direct integration with Google Workspace Calendar API to manage capacity reservation.
  - **Target Persona**: Nora (solo web/design/marketing services), consultants, field service operators.
  - **Advantages**: Google Workspace is explicitly mentioned as a candidate hypothesis in the RESEARCH.md. It is ubiquitous among small business owners. The API provides robust support for checking free/busy times, creating events, and managing cancellations. This enables the AI to confirm reservations without double-booking and release capacity upon cancellation.
  - **Risks**: Requires handling Google's OAuth scopes and review requirements. Requires robust webhook handling for changes in the calendar state to keep OHC in sync.
  - **Pricing**: Google Workspace APIs are typically included with the Workspace subscription, but usage limits and quotas apply.
  - **Compatibility**: Cloud (via webhooks/OAuth).
  - **Ease of Use**: Non-technical owners simply authenticate with Google Workspace. The system then automatically handles capacity reservation.

  **Design Doc**:
  - The business owner navigates to the Integrations dashboard and clicks "Connect Google Calendar".
  - The owner authenticates via the Google OAuth flow, granting necessary calendar read/write permissions.
  - OHC stores the integration token securely.
  - When an AI agent (e.g., Operations or Sales) receives a booking request, it queries the Google Calendar API to check for available capacity (respecting buffers and timezones).
  - Upon confirmation, the agent creates an event in the Google Calendar to reserve the capacity.
  - Changes or cancellations via OHC or directly in Google Calendar (via webhook sync) automatically update the agent's understanding of available capacity.

  **Implementation Prompt**:
  Implement a direct Google Workspace Calendar integration to support the "Reserve capacity" business responsibility. Create the OAuth flow for user authentication, implement the API calls necessary for agents to check free/busy times and create/cancel events, and set up webhooks to keep OHC's internal state synchronized with the external calendar. Ensure all external actions are properly logged and authorized.
  - **Acceptance Criteria**: Owner can connect their Google Workspace Calendar. Agents can read availability and create reservations that appear in the owner's Google Calendar. Cancellations release the capacity.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
