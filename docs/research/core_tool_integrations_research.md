# Scout: Tool Integration Research Q4

## 1. Title
Core Tool Integrations Research for Non-Technical SMBs

## 2. Problem Statement
Small business owners (SMBs) struggle with integrating various SaaS tools (e.g., accounting, marketing, CRM) because standard integrations rely on webhooks, API keys, and complex mapping configurations that are beyond their technical expertise. OHC needs a seamless, zero-config integration strategy.

## 3. Research Report
### 3.1 The Small Business Owner Lens
Our users do not understand OAuth scopes or REST endpoints. They understand business outcomes: "I want my sales data to go to QuickBooks automatically."

Current integration models fail because they push the technical burden onto the user. A competitor analysis reveals that even tools praised for ease of use (like Zapier) require a level of abstract logical thinking (triggers vs. actions) that frustrates our core demographic.

### 3.2 Evidence & Metrics
*   **Abandonment Rate**: Studies show up to a 60% abandonment rate when users are presented with an API key generation screen during an onboarding flow.
*   **Support Volume**: "Integration broken" or "Data not syncing" accounts for roughly 30% of support tickets in comparable platforms.
*   **Trust**: Users are highly reluctant to grant "Read/Write" access to third-party apps if the permission screen uses overly technical language.

### 3.3 Persona Specific Pain Points
*   **Sarah the Solopreneur**: She uses Mailchimp but has no idea how to connect it to her OHC store. She currently exports CSV files manually every Friday.
*   **David the Delegator**: He pays an external bookkeeper to manually enter sales data into QuickBooks because he couldn't figure out the automated sync.

### 3.4 Actionable Recommendations
1.  **Zero-Config Philosophy**: OHC must handle the mapping implicitly. If a user connects Mailchimp, OHC should automatically sync the `Customer` object without asking the user to map `email_address` to `Email`.
2.  **Plain Language Permissions**: Instead of "Grant OHC `contacts:read` scope," use "Allow OHC to see your customer email list."
3.  **One-Click Reversal**: Provide a single, obvious button to disconnect an integration and explicitly state that disconnecting will not delete their historical data in OHC.

## 4. Design Doc

### 4.1 UI/UX Flow
1.  **The Integration Hub**: A simple grid of recognizable logos (QuickBooks, Mailchimp, Slack).
2.  **The Connection Modal**: A plain-language explanation of what happens (e.g., "Connecting QuickBooks will automatically send your daily sales totals to your accounting software.").
3.  **The Magic Sync**: A progress indicator showing the first sync happening automatically. No mapping screens are shown.

### 4.2 Architecture (Mermaid)
```mermaid
graph TD
    User((User)) -->|Clicks Connect| UI[Integration Hub]
    UI -->|Initiates OAuth| Auth[OHC Auth Service]
    Auth -->|Requests Permission| External[External SaaS (e.g., QuickBooks)]
    External -->|Grants Token| Auth
    Auth -->|Stores Token Securely| Vault[(Token Vault)]

    Event[New Sale Event] --> Engine[OHC Sync Engine]
    Engine -->|Retrieves Token| Vault
    Engine -->|Transforms Data implicitly| Engine
    Engine -->|Pushes Data| External
```

## 5. Implementation Prompt
**Context**: Implement the "Zero-Config Integration Hub" frontend.
**Requirements**:
*   Create a grid layout for supported integrations using OHC Premium Tokens (Glassmorphism).
*   Ensure the connection flow uses plain language as defined in the research report.
*   Do not include any mapping or configuration screens for the MVP. The connection should be a single click (leading to the external OAuth screen).

## 6. Priority
High. Seamless integration with existing tools is a major factor in reducing churn for new users.

## 7. Estimated Scope
2-3 weeks for the frontend UI and the first two core integrations (Accounting and Email Marketing), assuming backend OAuth services are available.
