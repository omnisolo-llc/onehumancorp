issue_title: "Implement Autonomous Universal Product Variant & Inventory Matrix Engine"
issue_description: |
  **Title**: Implement Autonomous Universal Product Variant & Inventory Matrix Engine

  **Problem Statement**:
  For owners like Priya (boutique operator), selling items with variants (like size and color) is a critical capability. Currently, managing distinct sizes, colors, or material types often forces small business owners to create entirely separate product listings or maintain complex external spreadsheets. When inventory runs out for a specific size (e.g., Medium Red Shirt), the owner needs the system to autonomously update availability across online storefronts, POS, and conversational channels (like Instagram DMs) without manual intervention. The lack of a unified variant matrix creates stockouts, customer frustration, and requires the owner to act as a manual data entry clerk.

  **Research Report**:
  - **Market Context**: Shopify handles variants natively with a hard limit (typically 100 variants and 3 options per product) which can sometimes restrict merchants, but covers 95% of use cases. Wix and Squarespace offer similar variant matrices but often require complex, multi-step inventory syncs. GoDaddy provides a simplified variant system that is easy but inflexible.
  - **The OHC Opportunity**: Instead of exposing a complex nested database schema to Priya, OHC can use AI to parse natural language product descriptions ("I just got 10 red and 5 blue silk blouses in sizes S, M, L") and autonomously generate the variant matrix. The Operations Assistant tracks the inventory per variant, while the Customer Assistant knows exactly what's in stock when replying to customer inquiries.

  **Design Doc**:
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    erDiagram
        PRODUCT ||--o{ PRODUCT_OPTION : has
        PRODUCT ||--o{ PRODUCT_VARIANT : contains
        PRODUCT_OPTION ||--o{ OPTION_VALUE : defines
        PRODUCT_VARIANT }o--o{ OPTION_VALUE : maps_to
        PRODUCT_VARIANT ||--|| INVENTORY_LEVEL : tracks

        PRODUCT {
            uuid id
            string base_name
            string description
        }
        PRODUCT_OPTION {
            uuid id
            string name "e.g., Color, Size"
        }
        OPTION_VALUE {
            uuid id
            string value "e.g., Red, Medium"
        }
        PRODUCT_VARIANT {
            uuid id
            string sku
            decimal price_override
        }
        INVENTORY_LEVEL {
            int available_quantity
            int reserved_quantity
        }
    ```

  - **UI Wireframes (375px first)**:
    - **Screen 1 (AI Product Intake)**: A clean chat-like input or voice memo button. Priya types/says: "Add a new summer dress in floral and solid blue, sizes small to large, $45 each."
    - **Screen 2 (Variant Confirmation Card)**: A glassmorphic card displaying the generated matrix. "Summer Dress: 2 colors, 3 sizes. Total 6 variants." A single "Confirm & Publish" button.
    - **Screen 3 (Inventory Toggles)**: A simple list view showing variants with pill-shaped inventory toggles (In Stock / Low / Sold Out) that Priya can tap with one thumb.

  - **Mobile UX Flow**:
    1. Owner taps "+" on the Offers tab.
    2. Owner inputs natural language description of the product and its variants.
    3. System extracts the options (e.g., Color, Size) and generates the matrix.
    4. Owner reviews the generated cards (translucent glass styling, clear hierarchy).
    5. Owner publishes the offer, making it immediately available across web, POS, and AI chat.

  - **AI Agent Integration Points**:
    - **Operations Assistant**: Automatically creates the matrix and sets up inventory tracking. Triggers low-stock alerts when a variant drops below a threshold.
    - **Customer Assistant**: Accesses the variant matrix during chat. If a customer asks "Do you have the floral dress in Medium?", the agent checks `INVENTORY_LEVEL` for that specific variant and replies accurately, even generating a checkout link for the variant.

  - **Key Design Decisions**:
    - *Matrix vs Flat List*: We use a relational matrix structure to ensure accurate inventory tracking per variant combination, but we hide this complexity from the user via the AI intake.
    - *Optimistic UI*: Inventory toggles update instantly on the device and sync in the background to ensure fast operation even on slow mobile networks.
    - *Tenant Isolation*: All variant and option records must enforce row-level security using `tenant_id`.

  **Implementation Prompt**:
  "As an implementer, your task is to build the Universal Product Variant & Inventory Matrix Engine. The goal is to allow owners to add products with variations (e.g., size, color) using natural language, which the system then converts into a structured variant matrix. You need to implement the backend models, multi-tenant isolation, the Operations Assistant capability to generate this matrix from text, and the mobile-first (375px) Flutter/Tauri UI screens for the owner to review and manage these variants. The UI must use translucent glass styling and UniFi-style card layouts. Ensure the Customer Assistant can query variant availability. Write full Playwright E2E tests validating that an owner can create a variant product and a customer can query its specific availability."

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
