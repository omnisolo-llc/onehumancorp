# Smart Catalog & Inventory Sync

**Title:** Zero-Click Catalog & Invisible Inventory Sync

**Problem Statement:**
Uploading products and managing inventory across physical and online spaces is a massive bottleneck. Users despise manual data entry (writing SEO descriptions, guessing prices). Furthermore, physical store owners struggle to keep online inventory synced, leading to overselling.

**Research Report:**
* **42% of users** complain that writing product descriptions takes hours, blocking them from adding new inventory online.
* **58% of hybrid (physical + online) merchants** experience pain with manual inventory double-entry.
* Shopify's product upload flow involves filling out 15+ fields per item.

**Design Doc:**
* **UX Flow (Mobile First - 375px):**
  1. User taps a floating Action Button (FAB) and selects "Add Product via Camera".
  2. User takes a picture of the item (e.g., a hand-knit scarf).
  3. The OHC Agent analyzes the image and presents a draft card containing:
     * Auto-generated Title ("Hand-Knit Merino Wool Scarf")
     * Auto-generated SEO-friendly description.
     * Suggested price (based on web research of similar items).
     * Auto-categorization.
  4. User taps "Approve" (single tap to publish).
* **Architecture Impact:**
  * Requires integration with a Vision-capable LLM.
  * Needs a mobile-optimized camera UI flow in the Slint application.
  * The backend must support draft product states that are easily reviewable by the user before going live.

**Implementation Prompt:**
Develop a feature allowing users to create fully populated product listings purely from a single photograph. The AI must extract the item type, generate a compelling description, and suggest pricing. The UI must present this generated data for quick, one-tap approval by the user, entirely replacing the traditional multi-field form. The design must adhere to OHC premium tokens (Glassmorphism) and include thorough E2E testing of the approval workflow.

**Priority:** P1

**Estimated Scope:** Medium
