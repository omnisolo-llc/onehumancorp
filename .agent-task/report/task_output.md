issue_title: "Implement Shopify Sidekick Integration and Workflow Automation"
issue_description: |
  ## Issue Title: Implement Shopify Sidekick Integration and Workflow Automation

  ## Problem Statement
  Small business owners often struggle with the technical complexities of setting up and managing an online store. The process of picking a theme, writing product descriptions, editing photos, and optimizing for SEO can be overwhelming. They need an AI assistant that can take actionable steps directly within their platform to configure settings, create content, and analyze performance. Shopify Sidekick has set a high standard for what an AI commerce assistant should be, deeply integrating with the store's data and workflows. OHC currently lacks this level of actionable AI integration for e-commerce, creating a gap for users who want to transition online seamlessly.

  ## Research Report
  Our competitive analysis focused on Shopify Sidekick as a leading example of an AI commerce assistant. Sidekick differentiates itself by having direct access to the store's backend data and the ability to execute workflows on behalf of the user.

  **Key Success Factors of Shopify Sidekick:**
  *   **Actionable Prompts:** Users can ask Sidekick to "help me pick a theme," "add product photos," "customize my design," and "analyze my pricing."
  *   **Persona-Driven Workflows:** Sidekick utilizes different personas (Designer, Photo Editor, Writer, Tech Support, Marketer) to handle specific tasks.
  *   **Deep Integration:** Sidekick isn't just a chatbot; it can write ShopifyQL queries, configure shipping, create customers/companies, and set up discount codes.
  *   **Context Awareness:** It knows the user's business context, reducing the need for repetitive information gathering.

  **OHC Gap Analysis:**
  While OHC aims to be an owner-centric work assistant, it lacks the deep, action-oriented integration seen in Sidekick, particularly for e-commerce setup and management tasks. OHC needs to move beyond simple question-answering and enable agents to propose and execute complex workflows (e.g., store setup, product creation, marketing campaigns) directly within the OHC platform.

  ### Competitive Landscape & Journey Comparisons

  ```mermaid
  quadrantChart
      title AI Assistant Landscape
      x-axis "Passive Answering" --> "Actionable Workflows"
      y-axis "Enterprise Complexity" --> "SMB Simplicity"
      quadrant-1 "Actionable SMB Native"
      quadrant-2 "Simple but Passive"
      quadrant-3 "Enterprise Passive"
      quadrant-4 "Complex Actionable"
      "Shopify Sidekick": [0.8, 0.9]
      "OHC Current": [0.2, 0.8]
      "OHC Target": [0.9, 0.9]
      "Copilot": [0.7, 0.2]
      "ChatGPT": [0.1, 0.5]
  ```

  ```mermaid
  sequenceDiagram
      title User Journey Comparison: OHC vs Shopify Sidekick
      actor User
      participant OHC as OHC Current
      participant Sidekick as Shopify Sidekick
      participant Target as OHC Target (Agentic)

      User->>OHC: How do I add a product?
      OHC-->>User: [Text Instructions to go to Settings]
      User->>OHC: *Manual setup required*

      User->>Sidekick: Help me add a product
      Sidekick-->>User: [Asks for details, generates draft, shows preview]
      User->>Sidekick: *Approves preview*
      Sidekick->>Sidekick: *Automatically creates product*

      User->>Target: Add a new chocolate cake
      Target-->>User: [Proposed Product Card with generated details]
      User->>Target: *Clicks Approve*
      Target->>Target: *Automatically creates product*
  ```

  ### Feature Gap Matrix

  | Feature Category | Shopify Sidekick | OHC Current | OHC Target (Agentic) |
  | :--- | :--- | :--- | :--- |
  | **Context Awareness** | High (Store Data, Settings) | Medium (Chat History) | High (Tenant Data, Active Tasks) |
  | **Action Execution** | Yes (Creates products, discounts) | No (Text guidance only) | Yes (Proposed Actions feed) |
  | **Persona Workflows** | Yes (Designer, Marketer, etc.) | No (Single Assistant) | Yes (Dynamic Sub-agents) |
  | **Data Queries** | Yes (ShopifyQL integration) | Basic (Retrieval only) | Advanced (Analytics integration) |
  | **Approval Flow** | Implicit/UI based | N/A | Explicit "Proposed Action" Cards |

  ### Persona Pain Points

  *   **Maya (Home Baker):** Wants to quickly add a new cake flavor. Pain: Currently has to navigate through complex product setup forms instead of just telling the assistant what she wants.
  *   **Carlos (Field Service):** Wants to quickly generate an invoice from a chat. Pain: Must switch contexts and manually enter data.
  *   **Priya (Boutique):** Wants to launch a flash sale. Pain: Cannot orchestrate marketing, discounts, and inventory from a single natural language command.

  **Sources:**
  1. https://www.shopify.com/sidekick
  2. https://squareup.com/us/en/ai
  3. https://www.hubspot.com/artificial-intelligence
  4. https://www.notion.so/product/ai
  5. https://www.dingtalk.com/en
  6. https://www.salesforce.com/artificial-intelligence/
  7. https://www.zoho.com/zia/
  8. https://asana.com/product/ai
  9. https://clickup.com/ai
  10. https://www.intercom.com/fin
  11. https://www.zendesk.com/service/ai/
  12. https://www.intuit.com/intuitassist/
  13. https://www.klaviyo.com/features/ai
  14. https://gemini.google.com/
  15. https://www.typeform.com/ai/
  16. https://www.slack.com/features/ai
  17. https://www.smartsheet.com/ai
  18. https://www.miro.com/ai/
  19. https://www.airtable.com/platform/ai
  20. https://www.basecamp.com/
  21. https://www.trello.com/
  22. https://www.jira.com/
  23. https://www.confluence.com/
  24. https://www.github.com/features/copilot
  25. https://www.figma.com/ai/
  26. https://www.framer.com/ai/
  27. https://www.webflow.com/ai
  28. https://www.wix.com/studio/ai
  29. https://www.wordpress.com/ai/
  30. https://www.shopify.com/magic
  31. https://news.shopify.com/
  32. https://community.shopify.com/
  33. https://help.shopify.com/en
  34. https://apps.shopify.com/
  35. https://themes.shopify.com/
  36. https://shopify.dev/docs
  37. https://www.shopify.com/pricing
  38. https://www.shopify.com/enterprise
  39. https://www.shopify.com/plus/solutions/b2b-ecommerce
  40. https://www.shopify.com/international
  41. https://www.shopify.com/markets
  42. https://www.shopify.com/marketing
  43. https://www.shopify.com/discounts
  44. https://www.shopify.com/analytics
  45. https://www.shopify.com/orders
  46. https://www.shopify.com/shipping
  47. https://www.shopify.com/finance
  48. https://www.shopify.com/flow
  49. https://www.shopify.com/mobile
  50. https://www.shopify.com/checkout
  51. https://www.shopify.com/payments

  ## Design Doc
  **Goal:** Enable OHC agents to execute multi-step workflows on behalf of the user, similar to Shopify Sidekick, specifically focusing on initial setup and product creation.

  **Agentic Workflow Architecture:**
  1.  **Intent Parsing:** Enhance the main Assistant Agent to accurately parse intent for complex tasks (e.g., "Set up my online store," "Add a new cake to my menu").
  2.  **Workflow Orchestration:** Introduce a mechanism for the Assistant Agent to orchestrate multi-step workflows. This involves breaking down a high-level intent into discrete tasks (e.g., generate a description, suggest a price, draft a social post) and routing them to specialized sub-agents or tool calls.
  3.  **Approval Flow:** Crucially, any action that modifies the user's data (creating a product, changing a setting) MUST be presented as a proposed action in the feed for the user to approve before execution. This aligns with the "Owner Clarity" and "AI Does Useful Work" core values.
  4.  **UI/UX Updates:** The Assistant interface must support rendering proposed actions clearly. When an agent suggests creating a product, the UI should show a preview card of the product details with a prominent "Approve & Create" button. This interaction must be seamless on mobile (375px).

  ## Implementation Prompt
  **User Facing Outcome:** A user can type a request like "Add a new 'Chocolate Dream Cake' to my store" into the OHC Assistant. Instead of just replying with text, the Assistant understands the intent, drafts a product description, suggests a price based on similar items (if applicable), and presents a fully formed "Proposed Product" card in the chat interface. The user can review the card and click "Approve" to instantly create the product in the system.

  **Critical User Journey (CUJ):**
  1.  User logs in and opens the Assistant chat.
  2.  User inputs: "I want to start selling a new vegan chocolate cake."
  3.  The Assistant analyzes the request, realizes it requires creating a product, and uses tools/agents to generate a draft description and a suggested price (e.g., $45).
  4.  The Assistant replies in the chat with a summary and a structured "Proposed Action" card displaying the draft product details.
  5.  The user reviews the card and clicks "Approve."
  6.  The system creates the product in the backend and the Assistant confirms the action, perhaps suggesting a follow-up action like "Draft a promotional social media post."

  **Acceptance Criteria:**
  *   The Assistant can successfully interpret intents that require multi-step actions or data modification.
  *   Proposed actions are clearly presented to the user for approval via a structured UI component in the chat feed.
  *   Approval correctly triggers the backend mutation to create or update the relevant entity.
  *   The entire flow is fully functional and visually sound on a 375px mobile screen.

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
