# [Analytics] OHC Tool Integration Research Brief: Google Analytics 4 (GA4)

## Title
Actionable E-commerce Insights with Google Analytics 4

## Problem Statement
Small business owners need to understand how visitors interact with their OHC-hosted storefronts or booking pages. They need to know where traffic is coming from, which products/services are popular, and where customers are dropping off in the funnel. Without robust analytics, marketing efforts are essentially blind.

## Research Report
Google Analytics 4 (GA4) is the industry standard for web analytics, providing deep insights into user behavior and e-commerce performance.

**Evaluated Tool:**

1. **Google Analytics 4 (analytics.google.com)**
    *   **Focus:** Comprehensive web and app analytics.
    *   **Pros:** Free, ubiquitous, deeply integrated with ad ecosystems. Powerful e-commerce tracking capabilities.
    *   **Cons:** The learning curve can be steep. Privacy regulations require careful implementation of consent modes.

**Recommendation:**
Native integration with an industry standard analytics platform is essential. OHC must make it effortless for a business owner to simply provide their tracking identifier and instantly gain rich, e-commerce-specific tracking on their OHC properties.

## Design Doc
**Integration Approach: Native E-commerce Tracking**

1.  **Configuration:**
    *   Business owner enters their tracking identifier in OHC settings.
    *   OHC handles the injection of the required tracking script across all tenant-facing pages (storefront, checkout, booking).

2.  **Event Tracking (The Core Value):**
    *   OHC automatically pushes standard e-commerce events to the data layer:
        *   Product views
        *   Add to cart actions
        *   Checkout initiation
        *   Purchases (including transaction details, value, tax, shipping, and items)

3.  **Privacy & Consent:**
    *   Integrate the tracking implementation with OHC's cookie consent manager to ensure compliance with regional privacy laws.

## Implementation Prompt
**Objective:** Implement native e-commerce tracking for OHC storefronts.

**Acceptance Criteria:**
1.  Add a configuration field for the tracking identifier in the tenant settings.
2.  Implement a mechanism to dynamically inject the required tracking snippet into tenant-facing pages if the identifier is present.
3.  Implement robust data layer pushes for core e-commerce events (views, cart additions, checkouts, purchases), ensuring the payload structures strictly adhere to the recommended schema.
4.  Ensure tracking respects the user's cookie consent preferences.

## Priority
P1

## Estimated Scope
Medium
