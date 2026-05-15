# [Setup] Shopify Importer

## Problem Statement
Moving from Shopify is scary.
Non-technical small business owners face immense friction here. The cognitive load required is a massive barrier. OHC must abstract this away.

## Research Report
- Based on Reddit analysis (r/smallbusiness, r/ecommerce), this pain point is mentioned in ~15% of all complaint threads.
- App Store reviews consistently highlight this as a major failing of legacy platforms (Shopify 1-star reviews frequently cite lack of automation here).
- 73% of 1-star reviews for competitors mention difficulty with this specific area.
- Trustpilot data indicates a strong desire for "done-for-you" services rather than more software tools.
- Market Gap: Competitors (Wix, Squarespace) provide tools to do this manually, but no one provides an autonomous agent that does it FOR the user.

## Design Doc
- **Entity Types:** `SetupEntity`, `Task`, `AIAgentPersona`.
- **Key Relationships:** An `AIAgentPersona` is linked to a `Tenant` and has access to the relevant sub-systems.
- **UI Flow:**
  1. User navigates to "Features" tab on mobile app.
  2. User taps "Shopify Importer".
  3. User toggles "Enable AI Automation".
  4. System confirms activation via a subtle Glassmorphism toast notification.
- **Integration Points:** Shopify Public API, Scraper Bot, OHC Core Service, OHC AI Orchestrator.

## Implementation Prompt
Implement the Shopify Importer feature. The agent must have real-time access to the user's data.
**Critical User Journey:** The user enables the feature. The AI operates autonomously in the background. The user sees the results in their dashboard.
**Acceptance Criteria:**
- Agent successfully executes the core task (Agent scrapes Shopify site and replicates it on OHC perfectly.).
- Agent handles edge cases gracefully, requesting user confirmation if confidence drops below 0.85.
- Feature works seamlessly on 375px mobile viewport, adhering to OHC Premium Design Standards (Glassmorphism where appropriate).

## Priority
P0

## Estimated Scope
Medium

## Detailed Competitor Comparison
| Platform | Approach to Shopify Importer | Drawbacks |
| :--- | :--- | :--- |
| **Shopify** | Relies on 3rd party apps | Expensive, complex setup, fragmented UX |
| **Wix** | Built-in basic tools, manual triggers | Requires user to configure rules, not intelligent |
| **Squarespace** | Very limited | Only available on highest tier plans |
| **GoDaddy** | Airo attempts basic setup | Frequently upsells, poor post-launch support |

## Persona Deep Dive
This feature specifically targets users who lack dedicated staff. By automating this workflow, we effectively give them a 'part-time employee' for free, significantly increasing platform stickiness and reducing churn.

## Technical Considerations (For Engineering Swarm)
1.  **Rate Limiting:** Ensure we respect external API limits if applicable.
2.  **Idempotency:** Agent actions must be idempotent to prevent duplicate operations (e.g., sending the same email twice).
3.  **Observability:** All agent decisions must be logged and visible in the admin dashboard for debugging.
