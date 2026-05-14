# Scout: Tool Integration Research Q4

## 1. Title
Legacy Point of Sale (POS) Integration via Model Context Protocol (MCP)

## 2. Problem Statement
A significant portion of established SMBs (especially in retail and hospitality) rely on older, on-premise POS systems (e.g., NCR Aloha, legacy Micros). These systems lack modern APIs. We need a way to bridge these legacy, closed-ecosystem databases with the OHC Cloud using MCP.

## 3. Research Report
### 3.1 The Small Business Owner Lens
"I've used the same cash register software for 10 years. I can't afford to rip it out, but I want my new online OHC store to know what I sold in the restaurant today."

### 3.2 Evidence & Metrics
*   **Market Share**: Legacy on-premise POS systems still account for over 40% of the market share in restaurants older than 5 years.
*   **Integration Cost**: Traditional integration with these systems requires expensive, custom middleware or VPN setups, costing thousands of dollars per location.

### 3.3 Persona Specific Pain Points
*   **The Second-Generation Restaurateur**: Took over a family diner. Wants to offer online ordering via OHC, but the kitchen relies on an old POS system that only prints to a specific local serial printer.

### 3.4 Actionable Recommendations
1.  **The "Sidecar" Agent**: Deploy a lightweight OHC Standalone binary on the same local network (or same machine) as the legacy POS server.
2.  **Database Polling via MCP**: Create specialized MCP "Tool Plugins" within the Sidecar agent that know how to execute read-only SQL queries directly against the legacy system's underlying database (often old versions of SQL Server, Sybase, or even flat files).
3.  **One-Way Sync**: For safety and compliance, initial integrations should be strictly read-only (pulling sales data up to OHC), avoiding the immense risk of writing back to a brittle legacy system.

## 4. Design Doc

### 4.1 UI/UX Flow
1.  **Configuration**: In the OHC Cloud, the user selects their legacy POS type.
2.  **Local Setup**: The user downloads the OHC Sidecar and is prompted to provide local database credentials (e.g., ODBC connection string).
3.  **Operation**: The Cloud AI Agent can now answer questions like "How many burgers did we sell on the floor today?" by querying the legacy DB via the Sidecar.

### 4.2 Architecture (Mermaid)
```mermaid
graph TD
    CloudAI[OHC Cloud AI] -->|MCP Query| CloudGateway[OHC MCP Gateway]

    CloudGateway -->|Secure Tunnel| Sidecar[OHC Local Sidecar Agent]

    subgraph Legacy Environment
        Sidecar -->|Load Plugin| Plugin NCR[NCR Aloha Plugin]
        Plugin NCR -->|ODBC/Direct SQL| LegacyDB[(Legacy POS Database)]
    end

    LegacyDB -->|Result Set| Plugin NCR
    Plugin NCR -->|Format as MCP Response| Sidecar
    Sidecar -->|Tunnel| CloudGateway
```

## 5. Implementation Prompt
**Context**: Develop the architecture for Legacy POS Database polling via MCP.
**Requirements**:
*   Implement a plugin architecture within the Rust Standalone binary that can dynamically load database drivers (e.g., ODBC).
*   Create an MCP Server wrapper that translates natural language requests (or structured cloud requests) into the specific SQL dialects required by target legacy systems.
*   Ensure rigorous read-only enforcement at the connection level to prevent catastrophic local data corruption.

## 6. Priority
Medium-Low. High value for specific mature verticals, but technically fraught and hard to scale generically.

## 7. Estimated Scope
12+ weeks. Requires deep reverse-engineering of legacy database schemas and building secure, generic database translation layers.
