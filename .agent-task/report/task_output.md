issue_title: "Integrate Cal.com for Autonomous Assistant Scheduling"
issue_description: |
  **Mission Queue Protocol Brief: Cal.com Integration**

  **Title**: Integrate Cal.com for Autonomous Assistant Scheduling

  **Problem Statement**:
  Small-business owner/operators like Leo (Creator and Tutor) and Carlos (Field Service Owner) spend too much time coordinating meeting times, adjusting lessons, and capturing booking details from DMs or inquiries. They need OHC's Operations and Work Triage capabilities to autonomously schedule appointments, coordinate reschedules, and provide real-time visibility into their calendar commitments. Current manual or traditional booking tools force the owner to step out of their workflow and copy/paste links.

  **Research Report**:
  - **Tool Evaluated**: Cal.com
  - **Relevance**: Cal.com is an open-source, highly capable scheduling infrastructure. It directly addresses the booking and coordination needs of independent operators.
  - **Usability for Non-Technical Users**: Cal.com provides a very clean, modern UX. More importantly, its API allows OHC to abstract away the scheduling complexity, surfacing simple "Availability" and "Booking" conversational prompts to the owner and end-customers through the Assistant interface.
  - **Pricing/SaaS Viability**: Cal.com offers a robust free tier for individuals (perfect for standalone/local use cases) and scalable enterprise pricing for the multi-tenant SaaS cloud model. It supports both Cloud and Self-hosted/Standalone operation modes.
  - **Technical Capabilities**: It features a rich REST API and webhooks, supporting event types, availability schedules, routing, and dynamic meeting links (e.g., Zoom, Google Meet integrations under the hood). Webhook reliability is strong.

  **Design Doc**:
  - **Integration Point**: The OHC Operations Assistant and Customer & Relationship Assistant will integrate with Cal.com.
  - **Trigger**: When the Work Triage system detects an intent to schedule (e.g., "Can we meet next Tuesday?" via Instagram DM), the Assistant queries the Cal.com API for the owner's availability based on the appropriate Event Type.
  - **Action**: The Assistant drafts a reply with proposed times or a direct booking link generated via Cal.com. Upon booking confirmation, a Cal.com webhook notifies OHC. OHC then creates a task/calendar event in the owner's daily feed and optionally triggers the Sales Assistant if a deposit is required.
  - **User Visibility**: The owner sees the proposed booking in their feed, can approve the draft reply, and sees confirmed bookings grouped in their daily schedule. The complex routing and calendar sync are hidden; the owner only interacts with the scheduling outcome.

  **Implementation Prompt**:
  Implement the Cal.com integration within the OHC Operations Assistant module. The outcome should allow the Assistant to autonomously read an owner's availability and draft booking proposals for customers.
  - Define an integration config for Cal.com API keys and webhook secrets per tenant.
  - Create a Tool for the LLM to query availability slots given a date range.
  - Create a Tool for the LLM to generate a booking link for a specific event type.
  - Implement a webhook handler to receive `booking.created` and `booking.rescheduled` events from Cal.com, mapping them to OHC tasks visible in the owner's daily feed.
  - Acceptance Criteria: A user (like Leo) can have the Assistant successfully negotiate a lesson time with a student via chat, resulting in a confirmed Cal.com booking that appears in the OHC timeline.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []