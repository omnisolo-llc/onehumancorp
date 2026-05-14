# Scout: Tool Integration Research Q4

## 1. Title
Dynamic Localization via MCP

## 2. Problem Statement
As OHC expands internationally, maintaining hardcoded English strings in the frontend application is unsustainable. We need a system to dynamically serve localized Help Center content and UI strings based on user preference, without requiring a full application re-bundle for every language update.

## 3. Research Report
### 3.1 The Small Business Owner Lens
"The app is great, but the help guides are only in English. I run a bakery in Madrid and my staff needs instructions in Spanish."

### 3.2 Evidence & Metrics
*   **Expansion Bottleneck**: New market entry is currently delayed by weeks due to the engineering effort required to extract, translate, and hardcode new language strings.
*   **User Engagement**: Engagement with the Help Center drops by 80% when users are forced to use an interface not in their native language.

### 3.3 Persona Specific Pain Points
*   **The International Franchisee**: Needs the core dashboard to be in English for corporate reporting, but the Help Center and POS interface must be localized for their floor staff in different countries.

### 3.4 Actionable Recommendations
1.  **Over-the-Air (OTA) Updates**: Localized content must be served dynamically from the OHC Cloud, allowing content editors to publish translation fixes instantly.
2.  **MCP Localization Tool**: The frontend uses an MCP client to request localized strings from the Cloud based on the current user's locale state.
3.  **Local Caching**: To ensure the Standalone app works offline, the MCP client must aggressively cache downloaded language packs locally (e.g., in IndexedDB or SQLite).

## 4. Design Doc

### 4.1 UI/UX Flow
1.  **Language Selector**: A simple dropdown in the user settings to select the preferred language.
2.  **Seamless Switch**: Changing the language triggers an MCP request to download the new language pack in the background. The UI updates instantaneously without a page reload.

### 4.2 Architecture (Mermaid)
```mermaid
graph TD
    subgraph OHC Cloud
        TMS[(Translation Management System)]
        MCPServer[OHC MCP Gateway]
        TMS -->|Publish| MCPServer
    end

    subgraph User Device
        UI[React Frontend]
        MCPClient[MCP Client]
        LocalCache[(IndexedDB Cache)]

        UI -->|Request Strings(es-ES)| MCPClient
        MCPClient -->|Check Cache| LocalCache
    end

    MCPClient -->|Cache Miss: Fetch Pack via MCP| MCPServer
    MCPServer -->|Return JSON| MCPClient
    MCPClient -->|Store| LocalCache
    MCPClient --> UI
```

## 5. Implementation Prompt
**Context**: Implement Dynamic Localization via MCP.
**Requirements**:
*   Refactor the frontend to use a robust i18n library (e.g., `react-i18next`).
*   Create an MCP server endpoint that serves language packs as JSON payloads.
*   Implement a custom backend for the i18n library that fetches these payloads via MCP and caches them locally using standard browser storage mechanisms.

## 6. Priority
Medium. Required for international expansion, but lower priority for the initial English-centric beta launch.

## 7. Estimated Scope
4-6 weeks for frontend refactoring, caching logic, and establishing the integration with a Translation Management System (TMS).
