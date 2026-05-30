issue_title: "[Feature] Autonomous Omnilingual Localization Engine"
issue_description: |
  ## Problem Statement
  The global small business owner base is not exclusively English-speaking. Users like Fatima (food cart, limited English) require tools that adapt to their language naturally. Additionally, businesses in multicultural hubs often serve customers who speak diverse languages. Currently, OneHumanCorp (OHC) platform components lack deep, dynamic localization. A business owner shouldn't have to manually create multiple website versions or translate every AI-generated message. They need an automated system that serves content—from storefront text to AI customer service replies—in the native language of both the business owner (for management) and the end-user (for purchasing).

  ## Research Report
  ### Market Needs
  - **Multi-language Markets:** In the US alone, over 67 million people speak a language other than English at home.
  - **Competitors:** Platforms like Shopify and Wix offer multi-language plugins, but they often require tedious manual translation or integration of third-party apps with recurring fees. Squarespace offers basic localization but no dynamic agent translation. GoDaddy's multi-language support is rigid.
  - **Fatima Persona:** Needs an interface in Arabic to manage her business effectively while still catering to an English-speaking customer base for pre-orders.

  ### The OHC Solution
  By utilizing the embedded Large Language Model (LLM) agents, OHC can dynamically translate and localize content without creating fragmented data copies. The system will detect the end-user's browser language and dynamically serve AI-translated content, while the business owner interacts with the management dashboard and AI agents entirely in their preferred language.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Device
          App[OHC App UI - Arabic]
      end

      subgraph OHC Edge Gateway
          Router[Localization Router]
          CustomerBrowser[Customer Browser - Detect Locale]
      end

      subgraph Core Platform
          AgentMesh[AI Agent Mesh]
          DB[(Multi-tenant Database)]
      end

      App -->|Requests Data| Router
      Router -->|Checks Preferences| DB
      DB --> AgentMesh
      AgentMesh -->|Translates on-the-fly| Router
      Router -->|Serves Arabic UI| App

      CustomerBrowser -->|Requests Storefront| Router
      Router -->|Fetches Base Data| DB
      DB --> AgentMesh
      AgentMesh -->|Translates to Customer Locale| Router
      Router -->|Serves Localized Storefront| CustomerBrowser
  ```

  ### UI Wireframes & Mobile UX Flow
  - **Language Selection:** Upon first login, the user selects their preferred management language. A persistent setting is available in the profile card.
  - **Storefront Display:** The owner can preview how their storefront looks to customers in different locales via a simple dropdown in the mobile builder.
  - **Seamless Omnichannel Communication:** If an English customer DMs an Arabic-speaking owner on Instagram, the AI Agent acts as an invisible translator, displaying the message to the owner in Arabic and drafting the reply back in English.
  - **RTL Support:** The UI dynamically adjusts layout constraints to right-to-left (RTL) for languages like Arabic, ensuring touch targets (≥44x44px) and cards mirror properly on a 375px mobile screen.

  ### AI Agent Integration Points
  - **Agent Mesh (Translation Interceptor):** A middleware layer in the agent mesh detects language discrepancies between the stored tenant content and the requesting user/owner, dynamically invoking the LLM to translate strings on the fly.

  ### Key Design Decisions
  - **Dynamic AI Translation vs. Static Storage:** Instead of duplicating database rows for every supported language (which scales poorly), the AI Agent Mesh will cache on-the-fly translations for the storefront, reducing database bloat.
  - **Dashboard Localization:** The OHC management dashboard UI text will be driven by standardized localization files, but dynamic content (like AI Advisor reports or customer message drafts) will be translated into the owner's preferred language by the LLM.

  ## Implementation Prompt
  Design and implement the Autonomous Omnilingual Localization Engine.
  - **Requirement 1:** Modify the core data models and API responses to support dynamic language detection.
  - **Requirement 2:** Integrate the LLM pipeline to perform high-fidelity translations of dynamic content (like product descriptions and AI Advisor reports) based on the tenant's preferred language.
  - **Requirement 3:** Ensure the UI gracefully handles right-to-left (RTL) languages like Arabic (for the Fatima persona).
  - **Acceptance Criteria:** A user can set their tenant language to Arabic, see the dashboard in Arabic, while their storefront successfully serves English to a user with an `en-US` browser locale. Translations of dynamically generated content must be cached to ensure sub-500ms response times on the edge.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []