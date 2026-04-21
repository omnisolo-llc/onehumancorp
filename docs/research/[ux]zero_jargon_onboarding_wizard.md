# [UX] Zero-Jargon Onboarding Wizard

## Problem Statement
Fatima (food cart operator) and Leo (music tutor) find terms like "DNS," "SSL," "Payment Gateway," and "SEO" intimidating and confusing. If a user has to "configure" something, we've failed. Competitors still use technical onboarding flows that lead to high churn for non-technical users.

## Research Report
- **Competitor Audit**: Wix and GoDaddy still ask for "Domain Name" preferences during the first 30 seconds.
- **User Pain Point**: "I don't know what a 'Slug' is." - User feedback on Reddit.
- **OHC Advantage**: We can use LLM-driven "Business Advisory" agents to handle the technical mapping invisibly.

## Design Doc
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

### Feature: The "Just Decisions" Wizard
1.  **No Text Entry**: Wherever possible, use visual cards for "Vibe" selection.
2.  **Plain Language**: Instead of "SEO Meta Description," ask "How would you describe your business to a neighbor?"
3.  **Invisible Heavy Lifting**: The agent handles Stripe account linking, domain provisioning (via subdomains initially), and SEO mapping in the background.

### UI Flow
- **Step 1**: "What's your business name?"
- **Step 2**: "Show us what you sell (Upload photos or connect Instagram)."
- **Step 3**: "How do you want to get paid? (Tap for Stripe)."
- **Finish**: "Your business is live at [link]. Our agents are already looking for your first customer."

</div>

## Implementation Prompt
Rewrite the `BusinessSetupWizardScreen` to remove all technical jargon. Implement a `BusinessAdvisoryAgent` task that takes the plain-language inputs from the wizard and maps them to technical configurations (`tenant_settings`, `stripe_config`, `seo_metadata`) in the backend. All success states must use the `PulseAnimation` and premium glassmorphic cards.

## Priority
P0

## Estimated Scope
Medium
