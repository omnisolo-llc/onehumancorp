# Issue Brief: Autonomous Global Localization for SMBs

## Problem Statement
Small business owners like Fatima (Halal Food Cart) or Maya (Baker) often have customers from diverse linguistic backgrounds or want to expand into new markets but are limited by language barriers. Competitors like Shopify and Wix offer "translation apps" that are either expensive, manual, or produce "robotic" results that don't capture the brand's vibe.

## Research Report
- **Market Gap:** 50% of small businesses in major metro areas serve multi-lingual communities.
- **Competitor State:** Shopify Magic supports 8 languages but primarily for product descriptions, not full storefront "vibe" localization. Most solutions require a $29/mo app.
- **OHC Advantage:** With the "Department: Marketing & Advertising" (The Promoter), OHC can offer **Autonomous Localization**. Instead of just translating text, the agent adapts the cultural context, currency, and local delivery expectations automatically based on the visitor's location.

| Feature | Legacy Translation | OHC Autonomous Localization |
| :--- | :--- | :--- |
| **Effort** | High (Manual review) | Zero (Background) |
| **Context** | Literal (Word-for-word) | Cultural (Vibe-aware) |
| **Cost** | Extra App Fee ($20+) | Built-in |

## Design Doc
### High-Level Architecture
- **Localization Agent:** Part of "The Promoter" department. Monitors storefront traffic and visitor locales.
- **Vibe-Preserving Translation:** Uses LLMs to translate storefront content while maintaining the brand's "OHC Premium" tone (e.g., friendly, professional, or minimalist).
- **Dynamic Asset Swapping:** Automatically swaps images or icons that might be culturally specific (e.g., changing a "Dollar" icon to "Euro" or a "Truck" to a "Bike" for local delivery context).

### Mobile UX Flow (375px First)
- **Settings Toggle:** A single "Global Mode" switch in the Dashboard.
- **Review Feed:** A notification saying "The Promoter localized your menu into Arabic for 15 visitors today. [Review Changes]".
- **One-Tap Approval:** Owners can quickly scan the localized version on their phone and approve.

### Implementation Prompt
Implement a localization engine within "The Promoter" agent department. This engine should use Gemini/GPT-4 to provide context-aware, "vibe-preserving" translations of the storefront. It should also include a "Culture Sync" feature that adjusts currency formats and local delivery terminology based on detected user geography.

## Priority
P1

## Estimated Scope
Medium
