# [ai] 'Protector' Legal & Compliance Suite

**Priority:** P1
**Estimated Scope:** Medium

---

## Problem Statement
Small business owners live in fear of "doing something wrong" legally. For Fatima (food cart) or Carlos (handyman), the cost of a lawyer to draft a simple contract or privacy policy is prohibitive. Most "free" online generators produce generic, jargon-heavy documents that confuse customers and don't actually protect the specific nuances of a home-based bakery or a freelance repair service.

## Research Report

### Legal Barrier Comparison
```mermaid
bar-chart
    title "Cost of Legal Compliance"
    "Lawyer" : 2000
    "LegalZoom" : 400
    "Wix Generator" : 50
    "OHC Protector" : 0
```

### Competitor Audit
*   **LegalZoom/RocketLawyer:** Expensive, subscription-heavy, and intimidating for non-technical users.
*   **Shopify/Wix Generators:** Very basic. They provide a template but don't "know" your business.
*   **The "Ignore it" Method:** 60% of our target personas have NO terms of service or written contracts, leaving them vulnerable to chargebacks and liability.

### User Pain Points
1.  **Jargon Dread:** "Indemnification," "Severability" — these words alienate Maya and her customers.
2.  **Liability Gaps:** Fatima needs a "Halal certification" and "Allergy warning" disclaimer that a generic generator misses.
3.  **Contract Friction:** Carlos needs a way to get a "Work Order" signed on a phone screen without a 10-page PDF.

## Design Doc (High-Level)
### Entity Types
*   **PolicyArtifact**: Privacy Policy, Terms of Service, Refund Policy.
*   **ServiceAgreement**: A simplified "Work Order" for Carlos or "Custom Cake Contract" for Maya.
*   **ComplianceBadge**: Visual indicator for the storefront (e.g., "Verified Halal", "Insured Handyman").

### Mobile UX Flow (375px First)
1.  **Plain-Language Wizard:** Instead of a form, the "Protector" agent asks: "Do you offer refunds?" "What happens if you're late?"
2.  **Glassmorphic Policy View:** Policies are presented as clean, readable overlays, not walls of text.
3.  **1-Tap Signature:** For service agreements, a simple "I agree" button that records timestamp and IP, optimized for mobile thumb-reach.

### AI Integration Points
*   **Legal & Compliance Agent**: Drafts policies based on the specific business "Memory" (e.g., if the agent knows Maya sells cakes, it adds an allergy disclaimer automatically).
*   **Risk Scanner**: Scans the storefront and flags missing required policies for the user's jurisdiction (GDPR, CCPA).

## Implementation Prompt
Implement the 'Protector' Legal & Compliance Suite. This feature must allow a user to generate business-specific, plain-language legal documents (TOS, Privacy, Refund) and service contracts via an AI-guided interview. The output must be "Premium" in appearance, following OHC's Glassmorphism standards, and be highly readable on 375px screens. Focus on removing jargon and making the "Agreement" process as frictionless as a single tap for the end customer.

---
**Acceptance Criteria:**
*   AI-guided interview for policy generation.
*   Automatic inclusion of business-specific risk disclaimers (e.g., food allergies, service liability).
*   Mobile-optimized "Service Agreement" signature flow.
*   Automatic storefront placement of generated policies.
