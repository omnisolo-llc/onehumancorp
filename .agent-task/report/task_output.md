issue_title: "[Architecture] Autonomous Globalization & Localization Mesh"
issue_description: |
  # [Architecture] Autonomous Globalization & Localization Mesh

  ## Problem Statement
  Small business owners—whether they are teaching music (Leo) or selling digital templates globally—face extreme friction when dealing with international customers.
  Non-technical users struggle with configuring complex exchange rates, manually translating product listings into different languages, dealing with localized tax rules (VAT/GST), and integrating multiple localized payment providers (like iDEAL in the Netherlands or Alipay in Asia). This creates a massive barrier to international revenue. They are currently forced to either restrict their business locally or juggle dozens of third-party apps, many of which only function effectively on a desktop computer.

  ## Research Report
  We evaluated the current state of small business globalization on leading platforms:

  | Feature / Domain | Shopify | Wix | OHC (Current) | OHC (Target) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Translation** | Requires "Translate & Adapt" app or paid 3rd party apps | Basic manual string translation | None | **Zero-touch AI Translation** (catalog, UI, and messages auto-translated based on buyer's locale) |
  | **Multi-Currency** | Shopify Markets (complex setup, requires Shopify Payments) | Supported, but requires manual configuration | None | **Invisible Multi-Currency** (Auto-conversion with transparent fee structure) |
  | **Localized Payments** | Dozens of toggles and configuration pages | Needs to be configured per region | Stripe only | **Auto-Routing Payments** (Presents locally relevant payment methods automatically) |
  | **Tax Compliance** | TaxJar / Manual rules | Manual rules | None | **Autonomous Tax Engine** (AI calculates and collects VAT/GST implicitly based on buyer location) |
  | **Mobile Mgmt** | Difficult to manage international settings on mobile | Very limited | Native Mobile First | **100% Mobile Configuration** via AI conversational interface |

  ### The Competitive Gap
  *   **Shopify** solves globalization through "Shopify Markets", an enterprise-grade configuration dashboard that overwhelms small merchants. Setting up translation and multi-currency often requires a developer or multiple paid apps.
  *   **Wix** allows manual translation but lacks robust, seamless multi-currency checkout without advanced setup.
  *   **OHC Opportunity:** Implement an "Autonomous Globalization Mesh." The merchant never manually translates anything or configures a tax rule. The OHC platform automatically determines the buyer's location, translates the storefront and checkout dynamically via Edge AI, calculates accurate local taxes, and presents the locally preferred payment methods.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      BuyerDevice[Buyer Mobile Browser] -->|Edge Request| EdgeCache[Edge CDN / Cache Layer]
      EdgeCache -->|Location / Headers| Routing[Globalization Routing Engine]

      subgraph KAIROS Orchestration
          Routing --> LocalizationAgent[Localization AI Agent]
          LocalizationAgent --> Translation[Real-time Translation Service]
          LocalizationAgent --> Currency[Live FX & Pricing Engine]
          LocalizationAgent --> Tax[Tax Compliance & VAT Engine]
      end

      Translation --> Storefront(Storefront UI)
      Translation --> Catalog[(Product Catalog DB)]
      Currency --> Ledger[Multi-tenant Ledger]
      Tax --> Checkout[Payment & Checkout Service]
      Checkout --> Gateway[Global Payment Gateway Abstraction]

      Gateway --> BuyerDevice

      MerchantApp[OHC Merchant App] -->|Plain English Request| OperationsAgent[Operations Dept AI]
      OperationsAgent --> LocalizationAgent
  ```

  ### Mobile UX Flow (375px Viewport)
  **For the Merchant (Activation):**
  1. **Home Feed:** Merchant receives an AI Insight Card: *"You've had 5 inquiries from Mexico this week. Tap to enable Spanish localization and MXN currency."*
  2. **Action Sheet:** A single button: `[Enable Global Reach]`. No complex settings.
  3. **Confirmation:** A success toast with a translucent glass background: *"Your store is now optimized for Mexico. Pricing and taxes will be handled automatically."*
  4. **Advanced Settings (Hidden):** Under a gear icon, merchants can view explicit exchange rate margins and override specific AI translations, wrapped in a simple uni-fi style card UI.

  **For the Buyer (Experience):**
  1. **Auto-Detection:** The storefront detects the buyer is in France.
  2. **Seamless View:** Product descriptions (originally English) appear in natural French. Prices are in EUR (automatically adjusted for exchange rate + configured margin).
  3. **Checkout:** The checkout automatically includes necessary VAT calculation and offers `Cartes Bancaires` or `Apple Pay` as the primary payment method.

  ### AI Integration Points
  *   **Localization AI Agent:** Works invisibly at the edge to translate catalog descriptions and UI strings into the buyer's locale in real-time, caching the results.
  *   **Operations Dept AI:** Proactively monitors traffic and inquiries, suggesting globalization features to the merchant only when relevant.
  *   **Customer Support AI:** Translates incoming foreign-language inquiries (e.g., via Instagram DM) into the merchant's native language, and translates the merchant's reply back to the buyer's language.

  ### Key Architectural Decisions
  *   **Edge-First Translation:** Translations must be resolved at the edge (CDN layer) to ensure sub-200ms latency for global buyers, maintaining a high-performance storefront.
  *   **Zero-Trust Isolation:** Multi-currency ledgers and localized tax data must strictly enforce `organization_id` isolation within the database layer.
  *   **No Manual String Tables:** We will not use traditional `.po` or `i18n.json` files that merchants must edit. The AI is the source of truth for localization, mapping back to a single canonical (native language) source string.
  *   **Transparent Abstraction:** The platform must hide the complexity of FX rates and local payment gateways behind a unified API, presenting the merchant with simplified "Net Revenue in their native currency."

  ## Implementation Prompt
  **To the Implementer Agent:**
  Design and implement the core `Globalization Routing Engine` and the necessary database schema extensions to support the Autonomous Globalization & Localization Mesh.

  **User Facing Outcome (CUJ):**
  A merchant can seamlessly receive orders from international buyers. When an international buyer accesses the storefront, they see localized currency, translated text, and local payment methods without the merchant ever having configured translation files or exchange rates manually.

  **Acceptance Criteria:**
  1.  **Data Model:** Define the database schema (e.g., Postgres) to support a canonical product catalog that can be dynamically related to cached AI translations and localized pricing rules, ensuring strict multi-tenant isolation via `organization_id`.
  2.  **API Layer:** Create the necessary backend API endpoints (Rust) that accept a buyer's locale/currency preferences and return the fully localized product and checkout details.
  3.  **Agent Integration:** Define the interface for the Localization AI Agent to asynchronously translate catalog items when a new locale is detected or requested.
  4.  **Performance:** The architecture must account for edge-caching so that subsequent requests for the same locale/product are served without invoking the LLM.
  5.  **Mobile First:** Ensure all APIs are designed to support a rapid, offline-capable mobile experience for the merchant app.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_estimated_scope: Large
