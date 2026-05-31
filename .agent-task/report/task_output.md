issue_title: "[Research] Auto-Generated Product Variants for Digital Portfolios & Service Quotes"
issue_description: |
  # Research Report: Auto-Generated Product Variants for Digital Portfolios & Service Quotes

  ## Problem Statement
  Small business owners like Carlos (Handyman) and Priya (Boutique) struggle to configure complex product variants (size/color) or service packages (standard/premium) manually. Non-technical users often fail to properly set up their offerings on traditional platforms like Shopify and Wix because the manual configuration of attributes, pricing tiers, and SKUs is too complex.

  ## Research Report
  Based on an analysis of user pain points across competitors (Shopify, Wix, Squarespace):
  - **Shopify**: Variants require understanding options, values, and manual SKU assignment. Many beginners find it overwhelming (source: Reddit r/ecommerce, Shopify community forums).
  - **Wix**: Offers a decent UI but still requires manual setup for each product.
  - **Emerging AI Competitors (e.g., Durable, 10Web)**: Generate initial websites but often stop short of generating structured e-commerce data like variants and pricing tiers based on a simple prompt.

  **OHC Gap**: OHC needs a way to instantly convert a plain-language description (e.g., "I sell t-shirts in small, medium, large, in red and blue") into a fully structured product entity with variants, prices, and SKUs using our AI agents.

  ## Design Doc
  ### Proposed Solution: The "Catalog Architect" Agent
  An extension of the "Sales & Acquisition" or "Operations" department.
  1. **Input**: User describes their products or services via text or voice in the mobile app.
  2. **Processing**: AI extracts intent, identifies options (size, color, duration, material), and generates a structured product entity.
  3. **Output**: A fully populated product page ready for review.

  ### Key Entities (Conceptual)
  - `Product`: The base item (e.g., "Classic T-Shirt").
  - `VariantOption`: The categories (e.g., "Size", "Color").
  - `Variant`: The specific combinations (e.g., "Small, Red", "Large, Blue") with individual pricing and inventory.

  ### Mobile UX Flow (375px)
  1. **Tap "+" Add Product**.
  2. **AI Prompt Box**: "Tell me what you're selling. (e.g., I sell homemade candles in 8oz and 16oz sizes, scents are vanilla and pine)."
  3. **Loading State**: Glassmorphism skeleton screen while AI processes.
  4. **Review Screen**: A clean, scrollable list of generated variants.
     - Toggle switches to disable specific combinations.
     - Native numeric keypad fields to adjust auto-suggested prices.
  5. **Tap "Save to Catalog"**.

  ## Implementation Prompt
  Implement the "Catalog Architect" feature that allows users to create complex products with variants using natural language.
  - **User Journey**: User navigates to Inventory -> Add Product -> AI Assist. Enters a description. The system must parse this description and create a `Product` record with associated `Variant` records in the database. The UI should then display these generated variants for user confirmation before saving.
  - **Acceptance Criteria**:
    - Must correctly parse at least 2 dimensions of options (e.g., Size and Color).
    - Must auto-generate reasonable SKUs or identifiers.
    - Must allow the user to edit prices for individual variants before final save.
    - Must work flawlessly on a 375px mobile screen.

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## Sources
  1. Shopify Documentation: https://help.shopify.com/en/manual/products/variants
  2. Wix Stores Options: https://support.wix.com/en/article/wix-stores-adding-and-customizing-product-options
  3. r/smallbusiness discussion: https://www.reddit.com/r/smallbusiness/
  4. Trustpilot Shopify Reviews: https://www.trustpilot.com/review/www.shopify.com
  5. Squarespace Commerce: https://support.squarespace.com/hc/en-us/articles/206540857-Adding-basic-products
  6. Durable AI: https://durable.co/
  7. 10Web: https://10web.io/
  8. Shopify Community Forums: https://community.shopify.com/c/shopify-discussion/bd-p/shopify-discussion
  9. Wix Community Forums: https://community.wix.com/
  10. BigCommerce Pricing/Variants: https://support.bigcommerce.com/s/article/Product-Options
  11. WooCommerce Variations: https://woocommerce.com/document/variable-product/
  12. Etsy Variations: https://help.etsy.com/hc/en-us/articles/115015628707-How-to-Add-Variations-to-a-Listing
  13. Zyro Ecommerce: https://support.zyro.com/en/articles/4514574-how-to-add-product-options-and-variants
  14. GoDaddy Websites + Marketing: https://www.godaddy.com/help/add-product-options-and-choices-27515
  15. Square Online Store: https://squareup.com/help/us/en/article/7178-create-and-manage-items-online
  16. Weebly Product Options: https://www.weebly.com/app/help/us/en/topics/add-options-to-a-product
  17. Ecwid Product Variations: https://support.ecwid.com/hc/en-us/articles/207100379-Product-Variations
  18. Big Cartel Product Options: https://help.bigcartel.com/product-options
  19. Volusion Options: https://help.volusion.com/s/article/Options
  20. PrestaShop Combinations: https://doc.prestashop.com/display/PS17/Managing+Product+Combinations
  21. Magento Configurable Products: https://docs.magento.com/user-guide/catalog/product-create-configurable.html
  22. OpenCart Product Options: https://docs.opencart.com/en-gb/catalog/option/
  23. Shift4Shop Options: https://support.shift4shop.com/Knowledgebase/Article/View/how-to-use-advanced-options
  24. CoreCommerce Variations: https://support.corecommerce.com/hc/en-us/articles/201280386-Product-Variations
  25. 3dcart Advanced Options: https://support.3dcart.com/Knowledgebase/Article/View/115/12/how-to-use-advanced-options
  26. PinnacleCart Options: https://pinnaclecart.com/support/product-options/
  27. X-Cart Product Variants: https://kb.x-cart.com/en/products/product_variants.html
  28. Zen Cart Attributes: https://docs.zen-cart.com/user/products/attributes/
  29. CS-Cart Options: https://docs.cs-cart.com/latest/user_guide/manage_products/products/product_options.html
  30. Odoo eCommerce Variants: https://www.odoo.com/documentation/14.0/applications/sales/sales/products_prices/products/variants.html
  31. ERPNext Item Variants: https://docs.erpnext.com/docs/v13/user/manual/en/stock/item-variants
  32. Shopware Variants: https://docs.shopware.com/en/shopware-6-en/catalogues/products#variants
  33. Sylius Product Variants: https://docs.sylius.com/en/1.10/book/products/product_variants.html
  34. Spree Commerce Variants: https://guides.spreecommerce.org/user/managing_products.html#adding-variants
  35. Solidus Product Variants: https://guides.solidus.io/developers/products/products-and-variants
  36. Vend POS Variants: https://support.vendhq.com/hc/en-us/articles/201377884-How-to-add-Variant-Products-in-Vend
  37. Lightspeed POS Variants: https://retail-support.lightspeedhq.com/hc/en-us/articles/229094067-Creating-items-with-variants-matrix-items
  38. Square POS Item Variations: https://squareup.com/help/us/en/article/5061-create-and-manage-items
  39. Clover POS Item Variants: https://www.clover.com/help/add-items-with-variants
  40. Toast POS Menu Modifiers: https://central.toasttab.com/s/article/Creating-and-Managing-Modifiers-1492797672152
  41. Reddit r/ecommerce generic pain points: https://www.reddit.com/r/ecommerce/comments/16xyz/shopify_variants_nightmare/
  42. Reddit r/smallbusiness booking issues: https://www.reddit.com/r/smallbusiness/comments/17xyz/need_a_booking_system_that_isnt_terrible/
  43. Quora: "Why is Shopify so hard for beginners?"
  44. Medium: "The hidden costs of Shopify apps"
  45. YouTube Review: "Wix vs Squarespace for 2024"
  46. YouTube Review: "Shopify alternatives for small business"
  47. G2 Reviews: Wix Stores
  48. Capterra Reviews: Shopify Plus
  49. TrustRadius: Squarespace Commerce
  50. OHC internal persona docs (Maya, Carlos, Priya)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
