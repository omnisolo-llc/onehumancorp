# Research Output: SaaS Business Journey Architecture

## Overview
I have systematically documented the "Business Journey Architecture" by mapping the end-to-end SaaS lifecycle for the five core user personas. Recognizing that the OHC platform must successfully acquire, onboard, and monetize non-technical users, these issue briefs focus explicitly on the merchant's progression through the SaaS funnel (Acquisition, Onboarding, Activation, Retention, Revenue Upgrade, and Referral), rather than just their interactions with end-consumers.

## Generated Issue Briefs
Five detailed architectural issue briefs have been revised and created in the `docs/research/` directory. Each brief outlines the problem statement, SaaS landscape research, a Mermaid.js architectural sequence diagram detailing the platform lifecycle, key design decisions, and an Implementation Prompt for downstream implementer agents.

1.  **`docs/research/[architecture]_journey_maya.md`**: Focuses on Maya (Home Baker). Details a conversational AI onboarding flow that defers complex setup, driving activation via instant storefront generation and triggering revenue upgrades when product limits are reached.
2.  **`docs/research/[architecture]_journey_carlos.md`**: Focuses on Carlos (Handyman). Maps a voice-first onboarding flow suitable for a user in a truck, leveraging AI action quotas (e.g., automated quoting) as the primary monetization lever to drive Starter tier upgrades.
3.  **`docs/research/[architecture]_journey_priya.md`**: Focuses on Priya (Boutique Owner). Outlines a vision-AI batch ingestion process for complex inventories to overcome onboarding friction, with Pro tier upgrades triggered by advanced feature requests like custom domain SSL provisioning.
4.  **`docs/research/[architecture]_journey_leo.md`**: Focuses on Leo (Music Tutor). Details the transition from legacy platforms using shadow-syncing, utilizing AI primarily as a proactive churn-prevention engine to secure creator MRR, and gating digital storage to drive upgrades.
5.  **`docs/research/[architecture]_journey_fatima.md`**: Focuses on Fatima (Food Cart Operator). Maps an out-of-band conversational onboarding process via WhatsApp and OCR, focusing the mobile app strictly on operational alerts, and driving upgrades purely based on calculated transaction fee ROI.

## Strategic Impact
By reframing the architecture around the SaaS lifecycle of the merchant, we ensure that implementer agents build features that actively drive platform growth and monetization. The design decisions emphasize deferred complexity, alternative onboarding vectors (voice, vision, external chat), and usage-based upgrade triggers that align platform revenue with the user's derived value.

## Next Steps
- Implementer agents must review these lifecycle maps and begin constructing the underlying billing, telemetry, and onboarding engines required to support these flows.
- The platform telemetry system must be updated to track the defined "Activation" events for each persona to measure the success of these new onboarding architectures.
