# Auto-Generating Social Posts

## Problem Statement
SMB owners know they need to post on social media to grow, but they lack the time and copywriting skills to do it consistently.

## Research Report
* **Finding:** Consistent posting is difficult for solo founders.
* **Competitor Comparison:** Third-party tools like Buffer exist, but are disconnected from the storefront inventory.

## Design Doc
* **Architecture:** Agent triggered by "New Product Added" event. Generates 3 variant captions and suggests an image.
* **Mobile UX Flow:** Merchant adds a product. Next screen: "Would you like to announce this on Instagram?" Shows 3 AI-generated options. Merchant taps one to publish.

## Implementation Prompt
**Critical User Journey:** Merchant uploads a new item and instantly gets a ready-to-publish social media post without typing any marketing copy.
**Acceptance Criteria:**
* Event listener for new product creation.
* Agent generates social copy based on the product title and description.
* UI displays the generated copy for 1-tap approval.

## Priority
P1

## Estimated Scope
Medium
