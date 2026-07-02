issue_title: "Implement the Ambassador Agent: Instagram DM Auto-Responder with Intent Classification"
issue_description: |
  # OHC Agent Solutions: The Ambassador Agent Issue Brief

  ## Target Persona: Maya (Home Baker)

  ## Problem Statement
  Solopreneurs like Maya miss critical sales because they are unable to monitor social media DMs (Instagram/WhatsApp) while running physical operations like baking or deliveries. Existing solutions require complex logic builders (e.g., ManyChat) which are too technical for the OHC target audience. They need an invisible AI agent that can classify intent and draft contextually accurate responses for their approval.

  ## Research Report
  - **Shopify**: Requires installing third-party apps for social DM management, which often involve complex rules-based logic and additional monthly fees.
  - **Wix/Squarespace**: Native integrations exist but typically route to a unified inbox rather than providing autonomous, AI-driven auto-responders that understand business context.
  - **GoDaddy**: Simple inbox, but lacks proactive AI draft generation.
  - **OHC Opportunity**: By integrating directly with social APIs and utilizing Gemini Pro for intent classification and RAG against the user's data (inventory, policies), OHC can provide a zero-setup, truly autonomous "Ambassador Agent" that drafts replies invisibly and only requires a 1-tap approval from the user's mobile device.

  ## Architecture & Design Flow

  ### System Architecture
  ```mermaid
  sequenceDiagram
      participant Customer as Instagram User
      participant SocialAPI as Instagram Graph API Webhook
      participant OHCEvent as OHC Event Bus
      participant IntentEngine as LLM Intent Classifier
      participant RAG as RAG Context (Inventory/Policies)
      participant DraftEngine as LLM Draft Generator
      participant MobileApp as OHC Mobile App (375px)

      Customer->>SocialAPI: "Do you have vegan cake?"
      SocialAPI->>OHCEvent: Publish Webhook Event
      OHCEvent->>IntentEngine: Process Message
      IntentEngine->>IntentEngine: Classify Intent (Availability)
      IntentEngine->>RAG: Query "Vegan Cake Availability"
      RAG-->>IntentEngine: Return: 3 in stock
      IntentEngine->>DraftEngine: Generate Draft Reply
      DraftEngine-->>OHCEvent: Store Draft Action
      OHCEvent->>MobileApp: Push Notification
      Note right of MobileApp: Maya taps "Approve"
      MobileApp->>SocialAPI: Send Reply
      SocialAPI->>Customer: "Yes! We have 3 left."
  ```

  - **Data Ingestion**: Webhooks connected to social channels (e.g., Instagram Graph API).
  - **Processing Layer**: LLM intent classification (e.g., pricing inquiry, availability check, general support) using the configured model.
  - **Context Generation**: RAG pipeline retrieving the user's inventory count, store policies, and FAQ embeddings.
  - **Draft Generation**: Agent generates a contextually accurate reply based on the classified intent and RAG context.
  - **Mobile UX**: Pushes a notification to the user. The OHC mobile app displays a 375px optimized card in the Agent Feed showing the drafted message, with "Approve & Send", "Edit", and "Discard" actions.

  ## Implementation Prompt
  - Integrate an entry point for social messages (e.g., webhook receiver).
  - Implement intent classification to categorize incoming messages using the configured LLM provider.
  - Implement RAG retrieval for context building, pulling from inventory and policies.
  - Build the mobile-first (375px) Agent Feed notification card UX for approval.
  - Do NOT prescribe specific database schemas here. Focus on the seamless connection between the message receipt, the LLM draft generation, and the user's mobile feed approval flow.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
