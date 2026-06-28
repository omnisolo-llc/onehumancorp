issue_title: "Implement Multi-Currency and Localized Pricing Engine Architecture"
issue_description: |
  # Research Report: Multi-Currency and Localized Pricing Engine Architecture

  ## Track 1: Market Mapping & Competitor Discovery

  **Market Context:**
  For small business owners like Priya (Boutique Operator) and Maya (Home Baker), the ability to sell products and services in multiple regions and currencies is becoming increasingly important as they expand their reach beyond local markets. Traditional platforms like Shopify offer multi-currency support (Shopify Markets), but it often requires higher-tier plans or complex configurations that overwhelm non-technical owners.

  **Competitor Analysis:**
  - **Shopify**: Robust multi-currency and localized pricing features through Shopify Markets, but requires significant setup and often a premium plan for advanced features.
  - **Wix/Squarespace**: Basic currency conversion, but lack deep localization and automatic pricing adjustments based on region.
  - **Stripe**: Excellent backend multi-currency support, but requires custom integration or reliance on the platform's implementation.

  ## Track 2: OHC Gap & Pain Point Identification

  **The Gap:**
  OneHumanCorp (OHC) currently lacks a native, easy-to-use multi-currency and localized pricing engine. This forces owners to either stick to a single currency or manually manage multiple pricing tiers, which is error-prone and time-consuming.

  **Pain Points:**
  - **Complexity**: Owners don't want to deal with exchange rates and localization rules manually.
  - **Friction**: Customers in different regions see prices in foreign currencies, leading to cart abandonment.
  - **Lack of Automation**: There is no AI agent actively managing and optimizing localized pricing.

  ## Track 3: Deep Dive Architecture Design


  **Architecture Diagram (Mermaid.js)**
  ```mermaid
  erDiagram
      Tenant ||--o{ Product : has
      Tenant ||--o{ Region : operates_in
      Product ||--o{ LocalizedPrice : has
      Region ||--o{ LocalizedPrice : sets
      Currency ||--o{ Region : used_in

      Tenant {
          uuid id
          string name
          string base_currency
      }
      Product {
          uuid id
          uuid tenant_id
          float base_price
      }
      Region {
          uuid id
          uuid tenant_id
          string name
          uuid default_currency_id
      }
      Currency {
          uuid id
          string code
          float exchange_rate_to_usd
          datetime last_updated
      }
      LocalizedPrice {
          uuid id
          uuid product_id
          uuid region_id
          float price
      }
  ```

  **Sequence Diagram (Mermaid.js)**
  ```mermaid
  sequenceDiagram
      actor Customer
      participant Storefront UI
      participant Sales Agent
      participant Finance Agent
      participant Currency API (Redis Cache)

      Customer->>Storefront UI: Visits site from Germany
      Storefront UI->>Sales Agent: Request localized prices (Region: EU)
      Sales Agent->>Currency API (Redis Cache): Get current EUR rate
      Currency API (Redis Cache)-->>Sales Agent: Return EUR rate (e.g. 0.92)
      Sales Agent->>Finance Agent: Request formatted price calculation
      Finance Agent-->>Sales Agent: Return €19.99 (rounded)
      Sales Agent-->>Storefront UI: Display €19.99
      Storefront UI-->>Customer: Shows localized price
  ```

  **Architecture Design:**
  - **Data Model (PostgreSQL)**:
    - Introduce `Currency` and `Region` entities.
    - Update `Product` and `Service` entities to support a `base_price` and a `localized_prices` JSONB column (or a separate `PriceBook` table) to store region-specific pricing.
  - **Exchange Rate Service**: Integrate with a reliable exchange rate API (e.g., Open Exchange Rates or Fixer.io) and cache rates in Redis for performance and resilience.
  - **AI Agent Coordination**:
    - **Finance Agent**: Automatically updates exchange rates daily and suggests localized pricing adjustments based on market trends and competitor analysis.
    - **Sales Agent**: Dynamically presents localized prices to customers based on their IP address or selected region.
  - **Mobile-First Implementation**:
    - The owner dashboard must clearly display the base currency while allowing easy, tap-friendly management of localized pricing on a 375px viewport.

  **Mobile UX Flow (375px)**
  1.  **Customer View**: Automatically detects region and displays localized pricing with a clear currency symbol.
  2.  **Owner View**: A simple toggle to "Enable Multi-Currency". The Finance Agent handles the rest, showing a summary of localized sales and exchange rate impacts.

  ## Track 4: Implementation Prompt

  **Feature Name**: Autonomous Multi-Currency and Localized Pricing Engine
  **Target Persona**: Priya (Boutique Operator) expanding online sales globally.
  **Outcome**: Priya can enable multi-currency sales with a single tap. The Finance Agent automatically manages exchange rates and localized pricing, ensuring a seamless experience for international customers and accurate financial reporting in her base currency.

  **Critical User Journey (CUJ)**:
  1.  Priya goes to the "Settings" > "Pricing & Currency" section in the OHC mobile app.
  2.  She toggles on "Enable International Sales".
  3.  The Finance Agent automatically sets up localized pricing for key regions (e.g., EUR, GBP, CAD) based on current exchange rates and rounds them to attractive numbers (e.g., €19.99 instead of €19.43).
  4.  An international customer visits Priya's online store and sees prices in their local currency.
  5.  Upon purchase, the transaction is processed via Stripe in the local currency, and the OHC dashboard displays the revenue in Priya's base currency, clearly indicating the exchange rate applied.

  **Next Actions for Engineering**:
  1.  Implement the database schema changes for multi-currency support (`base_price`, localized pricing).
  2.  Develop the backend service to fetch and cache exchange rates.
  3.  Extend the Finance Agent to handle automatic localized price generation and rounding.
  4.  Update the storefront UI to dynamically display localized prices and the owner dashboard to manage them.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
