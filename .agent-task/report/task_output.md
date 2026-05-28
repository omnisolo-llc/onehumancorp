issue_title: "Invisible Multi-Lingual Translation Mesh"
issue_description: |
  ## Title
  Invisible Multi-Lingual Translation Mesh

  ## Problem Statement
  Small business owners like Fatima, who may have limited English proficiency, struggle to operate the platform effectively. Additionally, they miss out on potential customers who speak different languages. The platform lacks an invisible, real-time translation mesh that allows users to operate the app in their native language while seamlessly interacting with customers in their preferred language.

  ## Research Report
  * **Competitor Analysis:**
    * **Shopify:** Requires third-party apps for multi-lingual support, which often have clunky integrations and additional costs.
    * **Wix/Squarespace:** Basic multi-lingual features, but require manual translation for most content and lack real-time conversational translation.
  * **Findings:** There is a significant market gap for a platform that seamlessly bridges language barriers for both the business owner and their customers. The translation needs to be ubiquitous, covering UI, product descriptions, customer support (AI/inbox), and operational tasks (e.g., printing order tickets).

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Mobile/Desktop UI] --> B{Edge Gateway}
      B --> C[Translation Agent]
      C --> D{KAIROS Orchestrator}
      D --> E[Multi-Lingual Content DB]
      D --> F[AI Inbox Handler]
      D --> G[Storefront Builder]
      F --> C
      G --> C
      E --> C
  ```

  ### UI Wireframes & Screen Flow (375px)
  *   **Onboarding:** The app detects device language or prompts for preferred language during the initial setup.
  *   **Dashboard:** All cards, metrics, and actionable buttons are presented in the user's selected language.
  *   **Inbox:** Incoming messages from customers (e.g., in English) are automatically translated to the owner's language (e.g., Arabic) with a subtle "Translated from English" tag. The owner's reply is translated back to English before sending.
  *   **Settings -> Advanced:** Options to fine-tune AI translation models or specify preferred languages for different regions.

  ### Mobile UX Flow
  1.  **Zero-Configuration Setup:** Language is inferred seamlessly.
  2.  **Omnichannel Translation:** Notifications, emails, and SMS are localized.
  3.  **Real-Time Interaction:** Chat and order updates happen instantly without explicit "translate" buttons.

  ### AI Agent Integration Points
  *   **Translation Agent:** A dedicated background agent intercepts text payloads (UI strings, product descriptions, chat messages) and provides context-aware translations.
  *   **Operations Agent:** Uses translated context to manage inventory or generate reports.

  ### Key Design Decisions
  *   **Ubiquity:** Translation must occur at the edge or via the orchestration layer before reaching the client to ensure performance.
  *   **Context-Awareness:** The translation agent must understand business context (e.g., distinguishing between "cart" as a physical object and "cart" in e-commerce).
  *   **Fallback Mechanism:** In case of translation failure, display the original text with a warning icon, avoiding complete system breakdown.

  ## Implementation Prompt
  Implement a robust multi-lingual translation mesh. The outcome should allow a user like Fatima to run her business entirely in Arabic, while her English-speaking customers view the storefront, place orders, and receive notifications in English. The translation should be invisible, real-time, and require zero manual configuration. The solution must integrate with the existing AI departments for handling customer inquiries and updating content.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []