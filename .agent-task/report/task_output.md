issue_title: "[architecture] Autonomous Legal & Compliance Protection Engine"
issue_description: |
  # Problem Statement

  For non-technical small business owners like **Fatima (food cart, 50, limited English)** and **Maya (baker, 28)**, navigating the legal complexities of running a business—Terms of Service, Privacy Policies, GDPR cookie consent, liability disclaimers, and deposit contracts—is an intimidating, expensive, and stressful barrier to entry.

  Existing platforms like Shopify or Wix expect users to either write their own policies from scratch, pay for expensive third-party plugins, or hire a lawyer. This leaves small merchants exposed to liability (e.g., food allergies, deposit disputes, GDPR violations). Furthermore, Fatima needs these protections accessible in multiple languages (Arabic + English) to serve her community without friction.

  **Pain Points:**
  - **Complexity & Legal Jargon:** Business owners do not know how to draft Terms of Service, Privacy Policies, or hazard disclaimers (e.g., allergen warnings for food, liability waivers for services).
  - **Language Barriers:** Multi-language compliance generation is either nonexistent or requires complex, paid add-ons.
  - **Static Documents:** Policies are generated once and never updated, even if the business starts selling in new jurisdictions or changes its product catalog.
  - **Disconnected Contracts:** Maya needs a simple contract for custom cake deposits, but standard platforms don't link product catalogs to dynamic contract generation.

  The opportunity is to build an **Autonomous Legal & Compliance Protection Engine** (The "Protector" Department) that lives inside OHC, natively generating, translating, and attaching appropriate legal documentation invisibly as the business owner operates.

  ---

  # Research Report

  Our dynamic research assessed industry leaders and the specific needs of micro-merchants regarding legal protection.

  ### Track 1: Market Mapping & Competitor Discovery

  1. **Shopify**: Provides basic boilerplate templates for Terms of Service and Privacy Policies, but requires manual editing and review. No proactive monitoring of products for liability. Multi-language requires paid apps like "Translate & Adapt".
  2. **Wix**: Similar to Shopify; provides generic templates. Cookie consent is a toggle, but detailed compliance requires manual configuration.
  3. **Squarespace**: Beautiful templates, but legal pages are just static text blocks the user must fill out.
  4. **Termly / Iubenda**: Standalone compliance tools that are too complex and expensive for a micro-merchant to integrate.

  ### Track 2: The "Protector" Advantage
  OHC's unique value is treating Legal & Compliance as an active, observing agent rather than a static template generator. Natively supporting multi-language (e.g., Fatima's Arabic) ensures global accessibility for underserved merchants.

  ---

  # Design Doc

  ### Architecture

  The Legal & Compliance Engine operates as an autonomous background process deeply integrated with the `Catalog`, `Tenant`, and `Orders` domains.

  - **Entity Model (PostgreSQL):**
    - `legal_policies`: `id`, `tenant_id`, `type` (TOS, Privacy, Refund), `content` (JSONB for multi-language), `version`, `status` (Draft, Active).
    - `product_disclaimers`: `id`, `tenant_id`, `product_id`, `disclaimer_type` (Allergen, Age Restriction, Hazard), `content`.
    - `contracts`: `id`, `tenant_id`, `order_id`, `status` (Pending Signature, Signed), `pdf_url`.

  - **AI Department ("The Protector"):**
    - Event-driven monitoring via the internal message bus.
    - **Triggers:**
      - `ProductCreated`: Agent analyzes the product title/description. If it detects "Peanut Butter Cake", it automatically generates an Allergen Disclaimer.
      - `TenantCreated`: Agent generates baseline TOS, Privacy, and Refund policies based on the business type (e.g., Bakery vs. Consulting).
      - `HighValueOrderPlaced`: Agent generates a custom deposit contract.
    - **Multi-Language Engine:** Policies are generated as abstract semantic representations and materialized on-the-fly via Gemini/GPT-4o into the tenant's primary language (e.g., Arabic) and the customer's browser language.

  - **Multi-Tenant Safety:** Strict row-level security (RLS) on all legal entities ensuring `tenant_id` isolation.

  ### System Diagram (Mermaid.js)

  ```mermaid
  graph TD
      A[Business Owner] -->|Adds Product: Peanut Cake| B(Catalog Service)
      B -->|Publish Event: ProductCreated| C{Message Bus}
      C -->|Consume Event| D[Legal & Compliance Agent]
      D -->|Analyze via LLM| E{Hazard Detected?}
      E -->|Yes: Allergen| F[Generate Disclaimer]
      E -->|No| G[End]
      F -->|Store| H[(PostgreSQL: product_disclaimers)]
      H -->|Translate| I[Multi-Language Materializer]
      I -->|Display| J[Customer Storefront]
  ```

  ### Mobile UX Flow (375px)

  **Scenario: Fatima adds a new food item.**
  1. Fatima adds "Spicy Peanut Chicken" to her menu via the mobile app.
  2. The "Protector" Agent surfaces a gentle, translucent glass notification card on her dashboard: *"I noticed this item contains peanuts. I've automatically added a standard allergen warning to protect your business. Want to review it?"*
  3. The card displays a snippet of the Arabic text and the English translation.
  4. Fatima taps "Approve" (One-tap action).
  5. The disclaimer is immediately visible to customers viewing that menu item.

  ### Key Design Decisions
  - **Invisible by Default:** The agent does not block the user. Disclaimers are generated as "Drafts" and can be auto-applied or require one-tap approval based on tenant settings.
  - **Plain Language:** The agent translates complex legal requirements into simple, actionable summaries for the owner (e.g., "This protects you if a customer cancels last minute" instead of "Cancellation indemnification clause").
  - **Zero-Config Multi-Language:** Rely on the LLM provider for high-quality, contextual translation of legal terms rather than maintaining static dictionaries.

  ---

  # Implementation Prompt

  **To the Implementer Agent:**

  Design and implement the core `Legal & Compliance` domain and the associated "Protector" AI Agent capabilities within the OHC backend.

  **Acceptance Criteria:**
  1.  **Data Models:** Create necessary database schemas (with RLS) for storing Tenant Policies, Product Disclaimers, and Contracts.
  2.  **Agent Logic:** Implement the "Protector" department agent that listens for product creation/modification events and uses the LLM provider to intelligently suggest disclaimers (e.g., allergens, hazards).
  3.  **Multi-Language Support:** Ensure the data structures and agent prompts support storing and retrieving these disclaimers in multiple languages (e.g., English and Arabic).
  4.  **API:** Expose gRPC/REST endpoints for the mobile client to fetch, review, and approve these generated legal documents.
  5.  **Tests:** 100% unit test coverage for the new domain logic and an E2E test simulating a business owner (like Fatima) adding a potentially hazardous product and approving the auto-generated disclaimer.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
