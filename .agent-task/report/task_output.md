issue_title: "Architecture: Invisible Multilingual Omnichannel Localization Mesh"
issue_description: |
  **Title**: Architecture: Invisible Multilingual Omnichannel Localization Mesh

  **Problem Statement**:
  Small business owners like Fatima (food cart operator, limited English) operate in diverse neighborhoods and struggle to serve customers who speak different languages. Existing platforms (Shopify, Wix) require manual translation apps, duplicate catalogs, and complex configurations. Non-technical users cannot manage i18n JSON files, hreflang tags, or RTL (Right-to-Left) layouts. When a customer messages in a foreign language, friction causes lost sales.

  **Research Report**:
  - **Competitor Analysis**:
    - *Shopify*: Requires paid apps (e.g., Langify) or complex Markets configuration. Translating a storefront requires manual entry or expensive credits.
    - *Wix*: Multilingual features duplicate pages or require manual text replacement. The UI is too clunky for non-technical users.
    - *Squarespace*: Extremely limited native support; often requires workarounds.
  - **Data & Findings**: 50% of urban local businesses serve demographics speaking at least two different primary languages. Seamless language switching increases local conversion rates by up to 35%.
  - **The OHC Opportunity**: OHC must abstract internationalization completely. Using AI, the platform should auto-detect the buyer's language, dynamically translate the catalog, handle RTL natively in the Glassmorphism design system, and auto-translate 2-way chat (Customer Success AI Agent) invisibly. Fatima sees Arabic; her customer sees English or Spanish.

  **Design Doc**:
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    graph TD;
        Buyer[Buyer Device] -->|Request Storefront| EdgeCDN[Edge Caching & CDN];
        EdgeCDN -->|Lang Detect via Header/IP| Translator[Edge Translation Proxy Worker];
        Translator -->|Fetch Base Content| DB[(Tenant DB: Default Locale)];
        DB --> Translator;
        Translator -->|AI Embeddings/Cache| Cache[(Redis Cache: Translations)];
        Translator -->|Return Localized| EdgeCDN;

        Chat[Omnichannel Chat] --> CS_Agent[Customer Success Agent];
        CS_Agent -->|Detect Intent & Lang| LLM[Gemini Pro Translation & Reply];
        LLM -->|Reply in Buyer Lang| Chat;
        LLM -->|Save in Owner Lang| Inbox[Owner Unified Inbox];
    ```
  - **UI/UX Mobile Flow (375px)**:
    - *Buyer View*: Automatic localization based on device locale. Floating subtle globe icon for manual language override. Automatic RTL flip for Arabic/Hebrew handled by layout constraints.
    - *Owner View (Fatima)*: The entire app, including generated invoices, daily briefs, and KDS orders, is rendered in her selected language (Arabic). When a Spanish customer pre-orders, notes ("Sin cebolla") are auto-translated to Arabic ("بدون بصل") in her KDS app.
  - **AI Integration Points**:
    - The "Customer Success Agent" intercepts incoming DMs, translates them for the owner, and formulates AI drafts in the customer's native language.
    - The "Marketing & Advertising Agent" automatically generates SEO meta tags for all dynamically translated languages to capture local search traffic.

  **Implementation Prompt**:
  Design and implement the `LocalizationMesh` service in Go, and the Flutter `I18nProxy` provider.
  1. The Go backend must intercept content requests, detect `Accept-Language`, and serve AI-translated strings from a Redis cache (falling back to Gemini Pro for new strings).
  2. Ensure row-level security maintains tenant isolation while caching translated strings.
  3. In Flutter, implement an `AutoRTLBuilder` that dynamically reverses flex directions and text alignments when RTL languages are detected.
  4. Add E2E Playwright tests simulating a Spanish buyer interacting with an Arabic owner's food cart, verifying translation caching and correct RTL rendering on the 375px viewport.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
