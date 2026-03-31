# Core User Journey: B2B Agent Exchange

## 1. Overview
This document outlines the user journey for establishing and utilizing a Cross-Org Collaboration (B2B Agent Exchange) between two independent One Human Corp organizations.

## 2. Persona
- **Procurement Manager (Human Operator):** Desires to automate software or resource procurement from external vendors using AI agents.
- **Sales Director (Human Operator):** Wants their AI sales agents to automatically negotiate and close deals with incoming B2B buyer agents.

## 3. Scenario: Automated Procurement via Inter-Org Room
- **Pre-condition:** `acme.corp` (Buyer) wants to purchase cloud resources from `globex.com` (Vendor).
- **Action 1 (Setup):** Administrators from both organizations navigate to the "B2B Collaboration" settings in their OHC dashboards. They mutually exchange their OIDC JWKS URLs and define `TrustAgreement` objects. `acme.corp` whitelists the "Buyer Agent" role; `globex.com` whitelists the "Sales Agent" role.
- **System Response 1:** The `b2b-gateway` establishes trust.
- **Action 2 (Execution):** The `acme.corp` CEO instructs their Buyer Agent: "[Feature: b2b-collaboration] Procure 100 compute instances from globex.com."
- **System Response 2:** The Buyer Agent initiates a cross-org message through the Hub. The Hub detects the `TrustAgreement`, encapsulates the message, and tunnels it over mTLS to `globex.com`.
- **Action 3 (Negotiation):** The `globex.com` Sales Agent receives the message in a securely bridged "Inter-Org Collaboration Room" and responds with a quote.
- **System Response 3:** The negotiation proceeds autonomously within the established bounds. Upon reaching an agreement, the `acme.corp` Guardian Agent intercepts the final purchase contract for Human-in-the-Loop (HITL) approval due to the high financial value.
- **Action 4 (Approval):** The `acme.corp` CEO approves the transaction. The final signed contract event is emitted and logged independently in both organizations' audit trails.

## 4. Post-conditions
- Both organizations maintain completely isolated execution environments.
- Context window boundaries ensure that no proprietary internal data from `globex.com` is leaked into `acme.corp`'s long-term agent memory.
- The transaction is fully auditable via `events.jsonl` in both environments.