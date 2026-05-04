# Issue Brief: Proactive Tax & Legal Guardrails

## Problem Statement
Small business owners live in fear of "invisible" legal and tax mistakes (GDPR violations, incorrect tax collection, expired licenses). Most platforms treat this as "user responsibility" or require hiring expensive consultants.

## Research Report
- **User Pain Point:** "I don't know if I'm supposed to collect sales tax for a customer in New Jersey."
- **Competitor Audit:** Shopify Tax is a paid add-on. Wix requires manual configuration of tax rules.
- **Opportunity:** "The Protector" (Legal & Compliance) and "The Accountant" (Finance) agents should proactively monitor every transaction and system state for compliance risks.

## Design Doc
### High-Level Architecture
- **Compliance Monitor:** A background agent that scans business activity against a database of local regulations.
- **Automatic Nexus Detection:** Detects when a business reaches a tax "nexus" in a new state and queues the necessary registration tasks.
- **Proactive Policy Generation:** Automatically updates the website's Terms of Service and Privacy Policy when new laws (like updated GDPR/CCPA) go into effect.

### Mobile UX Flow (375px)
- Notification: "Alert: You've reached the sales threshold for New Jersey. I've drafted your tax registration form and updated your checkout rules."
- 1-Tap Action: "Register & Apply Rules."

## Implementation Prompt
Implement the "Proactive Tax & Legal Guardrails" engine within "The Protector" department. This engine must monitor transaction history for tax nexus events and system state for legal compliance (e.g., missing privacy policy). It should provide the user with clear, non-technical "Fix" actions via the mobile dashboard.

## Priority
P1

## Estimated Scope
Medium
