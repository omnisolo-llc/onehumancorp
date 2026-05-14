# Scout: Tool Integration Research Q4

## 1. Title
Hybrid PubSub Model Context Protocol (MCP) Integration

## 2. Problem Statement
As OHC expands its AI capabilities, we need a robust mechanism for our cloud-based AI agents to communicate with on-premise or local tools (like local inventory databases or edge POS systems) securely, without requiring small business owners to configure firewalls or port forwarding.

## 3. Research Report
### 3.1 The Small Business Owner Lens
Our users do not have IT departments. Asking them to "open port 8080" or "configure a reverse proxy" is a guaranteed path to churn. They need the cloud AI to simply "talk" to their store computer as easily as they talk to a human employee.

### 3.2 Evidence & Metrics
*   **Setup Failure Rate**: Historical data shows that any feature requiring network configuration by an SMB has a >90% failure rate.
*   **Security Anxiety**: SMBs are increasingly aware of cybersecurity threats but lack the skills to secure open ports.

### 3.3 Persona Specific Pain Points
*   **David the Delegator**: He has an old Windows PC in the back office that runs his custom inventory software. He wants the OHC AI Agent to be able to check stock levels on that PC, but he has no idea how to connect it to the internet safely.

### 3.4 Actionable Recommendations
1.  **Outbound-Only Connections**: The local agent must connect *out* to the OHC cloud. The cloud must never attempt an inbound connection to the local network.
2.  **Zero-Touch Deployment**: The local agent should be a single executable that the user double-clicks. It should automatically negotiate a secure tunnel (e.g., via WebSockets or NATS) back to the OHC cloud.
3.  **MCP as the Standard**: We should adopt the Model Context Protocol (MCP) to standardize how the cloud AI discovers and interacts with these local tools over the secure tunnel.

## 4. Design Doc

### 4.1 UI/UX Flow
1.  **Download**: User clicks "Connect Local Computer" in the OHC dashboard and downloads a single executable file.
2.  **Run**: User runs the file on their store PC. A simple window says "Connected securely to OHC."
3.  **Discovery**: In the OHC cloud dashboard, the local computer automatically appears as an available "Resource" for the AI Agent.

### 4.2 Architecture (Mermaid)
```mermaid
graph TD
    subgraph SMB Local Network
        LocalTool[(Local Inventory DB)]
        LocalAgent[OHC Local Agent (MCP Client)]
        LocalTool <--> LocalAgent
    end

    subgraph OHC Cloud
        PubSub[NATS / WebSocket PubSub]
        CloudAI[OHC AI Agent]
        CloudAgent[MCP Server Proxy]

        CloudAI <--> CloudAgent
        CloudAgent <--> PubSub
    end

    LocalAgent -->|Outbound Secure Tunnel| PubSub
```

## 5. Implementation Prompt
**Context**: Develop the architecture for the Hybrid PubSub MCP integration.
**Requirements**:
*   Design a protocol where a local executable establishes a persistent, outbound secure WebSocket or NATS connection to the OHC cloud.
*   Implement a proxy layer in the cloud that allows the Cloud AI Agent to issue MCP commands over this tunnel to the local agent.
*   Ensure all data is encrypted end-to-end and the user is never prompted for network configuration.

## 6. Priority
Medium. Critical for advanced retail use cases but not required for digital-only SMBs.

## 7. Estimated Scope
4-6 weeks for the backend tunnel architecture, local agent MVP, and cloud proxy implementation.
