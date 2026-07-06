issue_title: "Integration: Twilio WhatsApp Business API for OHC Customers"
issue_description: |
  **Title**: Add Support for Twilio WhatsApp Business API Integration

  **Problem Statement**:
  For small business owners like Maya (Home Baker) and Fatima (Food Cart Operator), WhatsApp is the primary channel for receiving orders, answering customer questions, and building relationships. Currently, coordinating these messages manually leads to missed inquiries, mixed contexts, and slow response times. Owners need a way to manage WhatsApp conversations directly within the OHC unified work feed alongside their other tasks and bookings, without needing technical expertise to set it up.

  **Research Report**:
  - **Ecosystem Scraping & Community Mining**: WhatsApp is overwhelmingly the preferred communication method for small businesses in LATAM, EMEA, and parts of APAC. Competitors like Shopify, Wix, and HubSpot all offer WhatsApp integrations. Small business owners frequently complain on Reddit (r/smallbusiness) about missing messages when switching between personal and business WhatsApp apps.
  - **Tool Evaluated**: Twilio WhatsApp Business API.
  - **Capabilities**: Twilio provides a robust, well-documented API for sending and receiving WhatsApp messages. It handles the complexities of WhatsApp's approval processes and template messages. Webhooks allow real-time incoming message capture.
  - **SaaS Viability**: Twilio offers a pay-as-you-go pricing model, which is highly accessible for small businesses. It's multi-tenant friendly (each OHC tenant can connect their own Twilio account or OHC can act as an ISV).
  - **User-First Value Mapping**: By integrating Twilio, the OHC Customer & Relationship Assistant can automatically draft replies to WhatsApp inquiries, maintaining customer context from previous interactions, and the Work Triage system can unify these messages into the owner's daily feed.

  **Design Doc**:
  - **Integration Point**: Add a "WhatsApp Setup" option in the OHC Settings -> Integrations panel.
  - **Trigger**: Incoming webhooks from Twilio will trigger the Work Triage agent to parse the message and add it to the unified feed.
  - **Action**: The Customer & Relationship Assistant will use the tenant's context to draft a reply. When the owner approves, an API call is made to Twilio to send the WhatsApp message.
  - **UI/UX**:
    - A simple OAuth or API Key setup screen.
    - Incoming messages appear in the unified feed with a WhatsApp icon.
    - The reply interface includes a field to approve/edit the AI-drafted response.

  **Implementation Prompt**:
  Implement the Twilio WhatsApp Business API integration. The owner should be able to connect their Twilio account with simple credentials in the Settings. Once connected, incoming WhatsApp messages should automatically appear in the Work Triage feed. The AI Customer Assistant should generate draft responses for these messages based on past customer interactions and business rules. The owner must be able to review, edit, and send the reply directly from the OHC interface, and the message must successfully arrive on the customer's WhatsApp. Ensure all external network calls are robust against transient failures and that no sensitive credentials are leaked.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
