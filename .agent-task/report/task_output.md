issue_title: "Research and Integrate WhatsApp Cloud API vs Twilio for SMB Operators"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart) rely heavily on WhatsApp to communicate with their customers. Currently, capturing order details, confirming bookings, or answering repetitive queries via WhatsApp is a manual, context-switching process. OHC needs a reliable, user-friendly way to integrate with WhatsApp so the OHC Assistant can triage messages, draft replies, and initiate workflows directly from the owner's unified inbox.

  ## Research Report
  - **WhatsApp Cloud API (Meta):**
    - **Pros:** Direct integration with Meta, no third-party markup on messaging costs, robust template message support, and high reliability.
    - **Cons:** Setup requires a Meta Developer account and a verified business portfolio. The onboarding flow can be complex for non-technical users unless heavily abstracted.
  - **Twilio API for WhatsApp:**
    - **Pros:** Extremely developer-friendly, provides a unified API for SMS and WhatsApp, handles some of the complexity of Meta's approval processes.
    - **Cons:** Additional markup per message sent, requires a Twilio account in addition to Meta approval.

  **Conclusion:** For OHC, integrating directly with the **WhatsApp Cloud API** is the preferred path for cost efficiency, provided OHC can abstract the Meta Business verification process into a simple "Connect WhatsApp" button for the user. If rapid MVP deployment is required, Twilio is a strong fallback but introduces an unnecessary middleman cost for price-sensitive SMBs.

  ## Design Doc
  - **User Experience:** The owner sees a "Connect WhatsApp Business" card in their OHC settings. Clicking it initiates an OAuth-style flow or step-by-step guide to link their WhatsApp Business number.
  - **Trigger:** Incoming WhatsApp messages trigger a webhook to OHC.
  - **Action:** The OHC Assistant reads the message context, associates it with an existing customer or creates a new lead, and surfaces it in the "Work Triage" feed. The assistant can suggest a reply or automatically send a pre-approved response (e.g., pricing list).

  ## Implementation Prompt
  Implement the WhatsApp Business Cloud API integration.
  - Create a seamless onboarding flow in the UI for users to connect their WhatsApp Business number.
  - Establish a secure webhook endpoint to receive incoming WhatsApp messages.
  - Ensure incoming messages are correctly parsed, associated with the correct tenant, and fed into the Work Triage system.
  - Provide UI components to allow the owner to review and approve drafted WhatsApp replies.
  - Acceptance Criteria: A user can connect their WhatsApp number, receive a message in OHC, and OHC can successfully send a reply back to the user's WhatsApp.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
