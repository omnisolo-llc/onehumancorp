# [STRATEGY] Market Sizing & Go-to-Market Strategy

## Problem Statement
OHC needs a focused entry point into the massive $1T+ small business software market to avoid competing directly with Shopify on features before achieving scale.

## Research Report
*   **Total Addressable Market (TAM):** ~33.3M small businesses in the US (SBA, 2023). Over 80% are non-employer firms.
*   **Beachhead Market:** Service-based solopreneurs (e.g., tutors, handymen, independent consultants). This segment is poorly served by e-commerce-first tools.
*   **Expansion Vectors:**
    1.  Geographic: Latin America (high mobile penetration, low trust in traditional SaaS).
    2.  Vertical: Mobile food vendors (food trucks, popup stands).

## Design Doc
*   **Target Persona:** Carlos (handyman, 42). Relies on word-of-mouth. Needs a simple booking and quoting tool, not a complex storefront.
*   **Platform Focus:** Prioritize service booking, simplified CRM, and automated invoicing over complex inventory management.

## Implementation Prompt
Focus next quarter's engineering efforts on refining the `booking.rs` and `billing` services. Develop simple, automated workflows for quote generation, approval, and payment collection that can be managed entirely via SMS or a simplified mobile app interface.

## Priority
P1

## Estimated Scope
Large
