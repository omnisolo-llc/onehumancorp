issue_title: "[architecture] Unified Multimodal Autonomous Customer Support Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by customer inquiries coming from multiple fragmented channels: Instagram DMs, WhatsApp, SMS, and website chat. Responding to these manually takes hours out of their day, often leading to delayed responses, lost sales, and poor customer satisfaction. Existing solutions either require the owner to monitor a complex unified inbox manually or rely on rigid, rule-based chatbots that fail to handle nuanced inquiries (e.g., "Can you make the cake vegan but also nut-free?"). OHC needs an intelligent, multimodal "Customer Success Ambassador" agent that can autonomously understand and resolve complex customer inquiries across all channels, escalating to the owner only when necessary.

  ## Research Report
  - **Findings:** Studies show that 42% of consumers expect a response on social media within 60 minutes. For SMBs, achieving this is nearly impossible without dedicated staff. Modern LLMs (like Gemini Pro) are highly capable of understanding intent and generating empathetic, accurate responses when grounded with the right business context (inventory, policies, past interactions).
  - **Competitive Comparison:**
    - *Shopify Inbox:* Consolidates messages but relies heavily on the owner to respond or uses very basic automated replies (e.g., order status).
    - *Zendesk/Intercom:* Powerful but far too complex and expensive for a solopreneur. Designed for support teams, not a single owner on a mobile device.
    - *Gorgias:* Good e-commerce focus, but still requires significant manual setup and rule creation.
  - **Opportunity:** OHC can provide a true "Zero-Touch" support experience. The Ambassador agent will autonomously handle 80% of routine inquiries (order status, basic product questions, scheduling changes) across all channels, presenting the owner with a simple daily summary of handled interactions and explicit approval requests for high-stakes edge cases.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
    IG[Instagram DMs] -->|Webhook| Gateway[Omnichannel API Gateway]
    WA[WhatsApp] -->|Webhook| Gateway
    SMS[Twilio SMS] -->|Webhook| Gateway
    Web[Website Chat] -->|WebSocket| Gateway
    Gateway -->|Normalize Event| MessageQueue[(Event Bus)]
    MessageQueue -->|Consume| CSAgent[Customer Success Ambassador Agent]
    CSAgent -->|Retrieve Context| VectorDB[(pgvector Memory - Customer History & Policies)]
    CSAgent -->|Check Inventory/Status| OpsAgent[Operations Agent]
    CSAgent -->|Analyze Intent & Draft Response| LLM[Gemini Pro]
    LLM -->|Draft Ready| CSAgent
    CSAgent -->|Evaluate Confidence Score| ConfidenceEngine[Confidence Engine]
    ConfidenceEngine -->|Score > 90% (Auto-Reply)| Gateway
    ConfidenceEngine -->|Score < 90% (Escalate)| Escalate[Escalation Queue]
    Escalate -->|Push Notification| OwnerApp[OHC Mobile App (375px)]
    OwnerApp -->|1-Tap Approve/Edit| Gateway
  ```

  ### UI Wireframes (375px Mobile First)
  - **Omnichannel Inbox:** A clean, UniFi-style list view. Messages handled autonomously by the AI have a subtle "✨ Handled" badge. Escalated messages have a prominent "Action Required" badge.
  - **Message Detail (Escalation):** Shows the customer's message, the AI's drafted response in a glassmorphism card, and two large touch targets (44x44px): "Approve & Send" and "Edit Draft".
  - **Daily Summary Card:** "Your Ambassador agent handled 12 inquiries today, saving you approximately 45 minutes."

  ### AI Agent Integration Points
  - **Customer Success Ambassador ("The Ambassador"):** The primary actor for processing inbound messages, maintaining conversational context, and generating responses based on the business's unique tone of voice (stored in the system prompt).
  - **Operations Agent ("The Manager"):** Queried by the Ambassador to check real-time inventory ("Yes, we have 2 vegan cakes left!") or order status.

  ### Key Design Decisions
  - **Confidence-Based Routing:** The AI only auto-replies if it has high confidence in its answer based on the grounding data. Otherwise, it drafts a response for the owner's review.
  - **Tone Matching:** The system prompt for the Ambassador agent dynamically adjusts based on the owner's preferences (e.g., professional vs. casual/friendly).
  - **Multimodal Inputs:** The system must be capable of handling images (e.g., a customer sending a photo of a cake they want replicated), utilizing vision models to interpret the request.

  ## Implementation Prompt
  Implement the Unified Multimodal Autonomous Customer Support Engine. Build the Omnichannel API Gateway to receive messages from various sources (simulated for testing). The Customer Success Ambassador agent must process these messages, retrieve relevant context from pgvector, and use the LLM to generate a response. Implement the Confidence Engine to determine if the message should be auto-replied or escalated to the owner. Build the mobile-first (375px) UI for the owner to review escalated drafts and view daily summaries. Ensure the architecture supports image inputs (vision model integration). Write comprehensive Playwright tests simulating inbound messages and the resulting AI actions/escalations.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
