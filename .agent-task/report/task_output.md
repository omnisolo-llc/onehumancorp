issue_title: "Implement The Ambassador Agent - Native Social Inbox Auto-Responder"
issue_description: |
  # The Ambassador Agent Issue Brief

  ## Target Persona: Maya (Home Baker)

  ## Problem Statement
  Solopreneurs like Maya miss critical sales because they are unable to monitor social media DMs (Instagram/WhatsApp) while running physical operations like baking or deliveries. Existing solutions require complex logic builders (e.g., ManyChat) which are too technical for the OHC target audience.

  ## Research Report
  Our research across the e-commerce platform landscape reveals two distinct categories: Legacy giants like Shopify and Wix, and AI-native emerging players.
  - **Shopify**: Excellent ecosystem but highly complex setup. The "App Tax" is a major pain point.
  - **Wix**: Popular visual builder but disjointed e-commerce.
  - **The "Sidekick" Limitation**: Current AI implementations in legacy platforms are mostly reactive chatbots. Users have to know what to ask rather than the AI proactively managing the store.
  - **OHC Opportunity**: Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response.

  ## Architecture & Design Flow
  - **Data Ingestion**: Webhooks connected to Instagram Graph API.
  - **Processing Layer**: LLM intent classification (Is this a pricing inquiry, availability check, or general support?).
  - **Context Generation**: RAG pipeline retrieving Maya's inventory count, store policies, and FAQ embeddings.
  - **Draft Generation**: Agent generates a contextually accurate reply.
  - **Mobile UX**: Pushes a notification to Maya. The OHC mobile app displays a 375px card showing the drafted message, with "Approve & Send", "Edit", and "Discard" actions.
  - **Mobile UX Flow**:
    1. Maya logs into the OHC mobile web app (375px view).
    2. Maya connects her Instagram Business account via the Integrations tab.
    3. A customer DMs Maya on Instagram: "Do you have vegan chocolate cake available for Saturday?"
    4. The Ambassador Agent queries Maya's OHC inventory, confirms availability, and drafts: "Yes we do! We have 3 left for this Saturday. Would you like me to send a booking link?"
    5. Maya receives a push notification on her phone: "Agent drafted a reply to @customer. Tap to review."
    6. Maya taps the notification, sees the draft, and clicks "Approve". The message is sent.

  ## Implementation Prompt
  - Integrate Instagram Graph API for message receiving/sending.
  - Implement intent classification using Gemini Pro.
  - Implement RAG retrieval for context building.
  - Build the mobile-first (375px) notification card UX for approval.
  - Do NOT prescribe database schemas here. Focus on the seamless connection between the webhook, the LLM, and the user's mobile feed.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
