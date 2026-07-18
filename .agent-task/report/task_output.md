issue_title: "Integrate WhatsApp Business API (via Twilio) for Omnichannel Work Triage"
issue_description: |
  ## Title
  Integrate WhatsApp Business API (via Twilio) for Omnichannel Work Triage

  ## Problem Statement
  For owners like Maya (Home Baker) and Fatima (Food Cart Operator), customer inquiries and orders don't come through formal web forms—they happen natively in chat apps, especially WhatsApp. Currently, these owners have to constantly switch between their personal/business WhatsApp app and OHC to track orders, quote prices, and schedule pickups. This leads to missed messages, forgotten context, and delayed responses. They need OHC to act as an invisible proxy that intercepts WhatsApp demand, organizes it in their Work Triage feed, and allows the Customer Assistant AI to draft and send replies seamlessly back to the customer's WhatsApp, all without the owner ever needing to configure webhook endpoints or manage Twilio sender IDs directly.

  ## Research Report
  ### Market Context & Competitor Analysis
  - **Tencent Workbuddy / WeCom**: Deeply integrated with WeChat. Every customer interaction is natively captured, allowing the business owner to see a unified CRM profile for a chat handle.
  - **Shopify / Wix**: Both offer WhatsApp integration plugins (often via MessageBird or Twilio), consistently ranking among top-installed apps globally, especially in LATAM, EMEA, and APAC.
  - **Pain Points with Current Tools**: Most standalone WhatsApp CRM tools are too complex, requiring owners to build visual flowcharts for chatbots. OHC's differentiation is the *AI Work Assistant* doing the work.

  ### Twilio WhatsApp API Evaluation
  - **Capabilities**: Send/receive free-form messages within a 24-hour session window. Support for rich media (images, PDFs - e.g., for Maya's cake references or Carlos's repair photos).
  - **SaaS Viability**: Twilio provides a reliable multi-tenant friendly API. In Cloud mode, OHC can use a master Twilio account with subaccounts per tenant, or allow users to connect their own Twilio credentials.
  - **Ease of Use for Owners**: Through OHC, the owner simply clicks "Connect WhatsApp" and follows an OAuth/login flow to link their number. OHC hides the complexity of session windows and template approvals.

  ## Design Doc
  - **Trigger / User Experience**: The owner goes to the OHC Integrations panel and selects "WhatsApp". After a simple authentication flow, new WhatsApp messages automatically appear in the OHC Work Triage feed.
  - **Assistant Action**: When a customer messages via WhatsApp, OHC's Work Triage identifies the customer, links previous order history, and uses the Customer Assistant to draft a reply. The owner sees the draft in OHC, clicks "Approve", and the message is dispatched back to the customer's WhatsApp.
  - **Offline/Mobile Reality**: Notifications of urgent WhatsApp inquiries will push to the OHC Flutter app. If the owner is on the go (like Carlos or Fatima), they can quickly read the plain-language summary in OHC and tap to send the AI-generated quote.
  - **Rich Media Handling**: Customer-sent photos (e.g., a broken pipe for Carlos) are automatically attached to the OHC work request card.

  ## Implementation Prompt
  Implement a WhatsApp Business integration that connects customer WhatsApp messages directly into the OHC Work Triage feed.
  - **Acceptance Criteria 1**: An owner can link their WhatsApp account via a simple setup flow without dealing with API keys manually if using the managed cloud option.
  - **Acceptance Criteria 2**: Incoming WhatsApp messages generate or update customer profiles in OHC and appear in the Work Triage feed.
  - **Acceptance Criteria 3**: The AI Customer Assistant automatically drafts context-aware replies to WhatsApp messages, which the owner can approve and send directly from the OHC interface.
  - **Acceptance Criteria 4**: The UI must clearly indicate that a message thread is happening via WhatsApp and handle 24-hour session window warnings in plain owner-friendly language.

  Please design the necessary database models to store the connection state, backend webhook receivers to process incoming Twilio requests, and the frontend integration cards to enable the connection.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
