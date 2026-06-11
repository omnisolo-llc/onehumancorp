issue_title: "Add Twilio WhatsApp Business Integration for Customer Communication"
issue_description: |
  ## Problem Statement
  Owners like Maya (Home Baker) and Carlos (Field Service) communicate with their customers daily using WhatsApp. Currently, OHC does not natively support capturing these inquiries or responding to them directly within the assistant interface. This means owners must switch between OHC and their WhatsApp mobile app, resulting in lost context, delayed responses, and a scattered workflow. Owners need a single place to handle WhatsApp messages, allowing the AI assistant to draft replies and keep track of leads without requiring technical setup.

  ## Research Report
  - **Tool Evaluated:** Twilio API for WhatsApp (Cloud)
  - **Business Need:** WhatsApp is the dominant communication channel for small businesses in many regions (LATAM, India, Europe). Supporting it is critical for work intake and customer relationships.
  - **Ease of Use:** From the owner's perspective, integration should be seamless. They authorize OHC to connect to their WhatsApp Business account (or we provision a number for them), and messages start appearing in their OHC unified feed. No API keys or webhooks to configure on their end.
  - **Pricing & Viability:** Twilio offers a pay-as-you-go model per conversation, making it scalable for multi-tenant SaaS. It supports rich media, templates (for notifications), and session-based messaging, which aligns perfectly with our need to draft replies and send service updates.
  - **Competitor Landscape:** Tools like HubSpot, Wix, and specialized CRMs natively support WhatsApp. Lacking this puts OHC at a disadvantage for businesses that rely on direct messaging for sales and support.

  ## Design Doc
  - **Work Intake & Triage:** Incoming WhatsApp messages will trigger webhook events in OHC. These will be parsed and inserted into the unified owner feed (Work Triage).
  - **Customer Relationships:** The Customer Assistant capability will process incoming messages, maintaining context based on the sender's phone number. It will generate suggested draft replies.
  - **Owner Interface:** In the OHC Flutter shell, WhatsApp messages will appear alongside other inquiries. The owner can review the AI-drafted reply, edit it if needed, and send it back—all without leaving OHC.
  - **System Integration:** We will implement a backend service in Go to handle Twilio webhooks, verify signatures, and enqueue tasks for the AI agents. Outbound messages will be routed through the Twilio API. We will use PostgreSQL for storing message threads and Redis for rate-limiting and temporary state.

  ## Implementation Prompt
  Implement the Twilio WhatsApp integration. Create a robust Go service that securely receives Twilio webhooks, parses incoming messages (text and media), and routes them to the AI Job Queue for triage and draft generation. Ensure that the system handles webhook signature verification and implements retries for transient API failures. On the frontend, update the unified feed to display WhatsApp messages distinctly and provide an interface for the owner to review, edit, and send drafted replies. The user experience must be frictionless: the owner simply sees WhatsApp messages in their feed and can respond with one tap, while the AI handles the context and drafting behind the scenes.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
