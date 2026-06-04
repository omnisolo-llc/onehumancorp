issue_title: "[Legal] Autonomous Food Safety & Allergen Compliance Engine"
issue_description: |
  # [Legal] Autonomous Food Safety & Allergen Compliance Engine

  ## Problem Statement
  Small food business owners like Maya (the home baker) and Fatima (the food cart operator) face immense risk and administrative burden when managing food allergens and safety compliance. Manually tracking, labeling, and communicating allergens (e.g., nuts, dairy, gluten) across menus, custom orders, and receipts is error-prone. A single oversight can lead to severe health consequences for customers and crushing legal liability for the business. These owners need an automated, foolproof system that seamlessly identifies allergens, applies standardized safety disclaimers, and protects both the customer and the business without requiring legal or food science expertise.

  ## Research Report
  - **Competitor Analysis:** Legacy platforms like Shopify and Wix require manual entry of allergen information as plain text descriptions or custom tags. There is no built-in ingredient parsing or proactive warning system. If Maya forgets to tag her new cake with "contains tree nuts," the platform will not catch it.
  - **The OHC Differentiator:** OHC treats compliance as an invisible infrastructure layer. When Fatima adds a new dish or Maya lists ingredients for a custom cake, the Legal & Compliance Agent ("The Protector") automatically parses the data, cross-references it with global allergen databases, and standardizes warning labels across the storefront, checkout, and printed receipts.
  - **Real-Time Customer Protection:** If a customer adds an order note stating "severe peanut allergy," the Operations Agent and Legal Agent coordinate to instantly evaluate the cart items. If a flagged item is detected, the checkout is paused, the customer receives a clear warning, and the business owner is notified.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Customer Order / Menu Update] --> B{Ingredient Parser}
      B -->|Extracts Ingredients| C[Legal & Compliance Agent]
      C -->|Cross-references DB| D[(Allergen & Safety Database)]
      D -->|Identifies Risks| C
      C --> E[Storefront UI]
      C --> F[Checkout Flow]
      C --> G[Operations / Kitchen Display]

      H[Customer Note: Allergy] --> I[Operations Agent]
      I --> C
      C -->|Cart Risk Analysis| J{Risk Detected?}
      J -->|Yes| K[Checkout Warning & Owner Alert]
      J -->|No| L[Proceed to Checkout]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  **Flow 1: Menu Item Creation (Maya's Perspective)**
  1. **Screen 1 (Item Setup):** Maya enters the title "Almond Joy Cake" and lists ingredients: "Flour, Sugar, Cocoa, Almonds, Coconut".
  2. **Screen 2 (Agent Proposal Feed):** A card appears from The Protector: "I detected Tree Nuts (Almonds, Coconut) in your new item. I have automatically applied the 'Contains Tree Nuts' badge and added the standard liability disclaimer. [Approve & Publish]"
  3. **Action:** Maya taps the large touch-friendly "Approve & Publish" button (minimum 44x44px).

  **Flow 2: Customer Checkout (Customer Perspective)**
  1. **Screen 1 (Cart):** Customer adds "Almond Joy Cake".
  2. **Screen 2 (Checkout Notes):** Customer types in the notes field: "Allergy: Peanuts".
  3. **Screen 3 (Risk Alert Overlay):** A frosted glass bottom-sheet slides up: "Warning: The Almond Joy Cake contains Tree Nuts and is prepared in a kitchen that handles Peanuts. Do you wish to proceed?" with clear "Acknowledge Risk" or "Remove Item" buttons.

  ### AI Agent Integration Points
  - **Legal & Compliance Agent ("The Protector"):** Core engine for parsing ingredients, applying badges, and managing liability disclaimers.
  - **Operations Agent ("The Manager"):** Flags allergy notes in the kitchen display or daily order printout for Fatima and Maya.
  - **Marketing Agent ("The Promoter"):** Ensures allergen badges are visible on promotional social posts or emails generated for the product.

  ### Key Design Decisions
  - **Proactive Parsing:** We rely on LLMs to understand unstructured ingredient lists and map them to standard allergen categories, removing the need for the owner to manually select checkboxes.
  - **Frictionless Approval:** The owner is presented with the parsed results as an "Approval Card" in their unified feed, keeping the UX extremely simple.
  - **Glassmorphism UI:** Alerts and badges use translucent glass styling with high-contrast text to ensure visibility without breaking the premium aesthetic.

  ## Implementation Prompt

  **Objective:** Implement the Autonomous Food Safety & Allergen Compliance Engine for food-based businesses.

  **User-Facing Outcome:** When a business owner (like Maya or Fatima) creates a product and lists ingredients, the system automatically detects allergens, applies visual warning badges to the storefront, and ensures disclaimers appear on checkout and receipts. Furthermore, customer allergy notes at checkout will proactively warn against conflicting cart items.

  **Critical User Journey (CUJ):**
  1. Log in to the OHC mobile app (375px viewport) as a food business owner.
  2. Navigate to create a new product and enter a title and a list of ingredients (e.g., "Peanut Butter Brownie").
  3. Observe the Legal Agent automatically generating a feed card proposing allergen badges (e.g., "Contains Peanuts, Dairy, Gluten").
  4. Tap "Approve & Publish".
  5. Navigate to the public storefront, add the item to the cart, and verify the allergen badges are clearly visible.
  6. Enter an allergy note during checkout and verify the risk warning is triggered if applicable.

  **Acceptance Criteria:**
  - The Legal Agent successfully parses natural language ingredient lists and identifies common top-9 allergens.
  - The UI displays clear, premium-styled (Glassmorphism) allergen badges on the product page and checkout.
  - The approval flow is handled via a single card in the Unified Agent Feed.
  - All layouts strictly adhere to 375px width constraints without horizontal scrolling and utilize >= 44x44px touch targets.
  - Zero mock data; use the real LLM provider interface for ingredient parsing.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
