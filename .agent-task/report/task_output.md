issue_title: "Native Google Business Profile Integration for Automated Customer Conversations and Reputation"
issue_description: |
  # Title
  Native Google Business Profile API Integration for Automated Customer Conversations and Reputation Management

  ## Problem Statement
  Small business owners (like Carlos, the Auto Repair Shop Owner, or Leo, the Handyman) get many of their leads and questions directly through Google Search and Google Maps. When a potential customer sends a message or leaves a review via Google Business, these owners often miss it because they are busy working on a job. They have to manually check the Google Business app, leading to slow response times, lost leads, and unmanaged public reputation. They need these interactions to flow directly into OHC so an AI agent can handle initial inquiries instantly, book appointments, and draft review responses automatically.

  ## Research Report
  - **Strategy**: Direct integration with Google Business Profile APIs (specifically the Google Business Messages and Reviews APIs).
  - **Target Persona**: Carlos (Auto Repair Shop Owner), Leo (Handyman), and any local service or retail business.
  - **Advantages**: Google Search and Maps are the primary discovery channels for local businesses. Integrating with it captures high-intent customers at the exact moment of search. Native integration ensures we own the AI interaction flow without relying on third-party middleware.
  - **Risks**: Google's API approval process for Business Messages can be strict. Managing API quotas and adhering to Google's response time SLAs (merchants must respond within 24 hours to keep the messaging feature active).
  - **Pricing**: Google Business Profile APIs are generally free to use for merchants managing their own profiles, though subject to quotas.
  - **Ease of Use**: Once connected via OAuth, it is fully invisible. The business owner just sees messages and reviews pop up in their OHC inbox, with AI-suggested responses ready to go.
  - **Compatibility**: Cloud (Webhooks/OAuth). Standalone (Requires a cloud proxy for webhooks and OAuth redirects).

  ## Design Doc
  - **Integration with OHC**:
      - User connects their Google Business Profile Account in the "Operations" settings using a secure OAuth flow.
      - OHC registers webhooks/pubsub to receive incoming Google Business Messages and new Reviews in real time.
      - The "Ambassador" AI agent analyzes incoming Google messages and drafts/sends a response (e.g., sharing business hours, booking links, or answering FAQs) based on the business profile.
      - The AI agent also drafts polite, context-aware responses to new Google Reviews.
      - All conversations and reviews are surfaced in the OHC unified "Customer Inbox" screen.
  - **User View**: A unified thread showing Google messages alongside WhatsApp/SMS, with AI-drafted replies ready for approval or auto-send. A separate "Reputation" tab for approving AI-drafted review responses.

  ## Implementation Prompt
  Build a native integration for the Google Business Profile APIs (Messages and Reviews). Implement the secure OAuth flow for merchants to connect their accounts. Handle incoming message and review webhooks/pubsub events, and implement outbound API calls for sending messages and replying to reviews. Ensure the "Ambassador" AI agent can participate in Google Business threads by drafting and sending replies, and generate suggested responses for reviews. Normalize Google Business interactions into the OHC unified inbox schema.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
