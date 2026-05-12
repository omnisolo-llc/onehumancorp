# Title: Auto-CRM Recovery Agent

## Problem Statement
Small businesses lose significant revenue to abandoned carts. While platforms like Shopify offer cart recovery features, they often require manual configuration of email templates, delays, and triggers, which many non-technical owners fail to set up optimally.

## Research Report
- **Industry Data:** The average cart abandonment rate across e-commerce is nearly 70%. SMBs without dedicated marketing teams often leave this revenue on the table because setting up automated flows feels too complex.
- **Value Proposition:** A zero-configuration agent that automatically detects abandoned carts and sends personalized, timing-optimized follow-up messages would instantly increase a user's bottom line without any setup effort.

## Design Doc
- **Core Entity Types:** Shopping Cart, Customer Interaction, Follow-up Message.
- **Key Relationships:** The agent monitors Shopping Carts for abandonment, generates a Follow-up Message, and logs the Customer Interaction.
- **Mobile UX Flow (375px first):**
    1. A single toggle during onboarding: "Enable Auto-Cart Recovery".
    2. The agent works silently in the background.
    3. The dashboard displays a simple metric: "Revenue recovered by AI this week: $X".

## Implementation Prompt
- **User-Facing Outcome:** Without any configuration of email templates or timing rules, an agent automatically follows up with customers who left items in their cart, using personalized messaging to close the sale.
- **Critical User Journey (CUJ):**
    1. User enables the Auto-CRM Recovery Agent.
    2. A customer adds items to their cart but leaves the site.
    3. The agent determines the optimal time to send a follow-up email/SMS based on the customer's profile.
    4. The agent generates a personalized message and sends it.
    5. The user sees recovered revenue on their dashboard.
- **Acceptance Criteria:**
    - Zero-configuration setup (just an on/off toggle).
    - Agent automatically generates context-aware messages (e.g., offering a small discount if the cart value is high).
    - Clear reporting on recovered revenue.

## Priority
P1

## Estimated Scope
Medium
