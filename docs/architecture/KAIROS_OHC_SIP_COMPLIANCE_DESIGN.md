<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS AI OS: OHC-SIP Teammate Mesh API Compliance Design

## 1. Overview
The One Human Corp (OHC) AI OS relies on the Swarm Intelligence Protocol (OHC-SIP) to establish a universally understood communication standard across the Teammate Mesh. As the unified gateway for all agent-to-agent and orchestration broadcasts, the Teammate Mesh API must strictly enforce this payload contract. This document serves as the master specification for OHC-SIP payload compliance.

## 2. The OHC-SIP Unified Payload Contract
To ensure backward compatibility and seamless parsing across various agent sub-types (e.g., KAIROS Orchestrator, Background Workers, Thin Clients), all broadcast payloads sent to `POST /api/mesh/broadcast` **MUST** include exactly four root-level keys.

### 2.1 Required Root Keys
1. **`agent_id` (string)**: The unique identifier of the agent or service emitting the event.
2. **`channel` (string)**: The target Pub/Sub channel or routing key (e.g., `mesh:tasks`, `mesh:coordination`).
3. **`event_type` (string)**: A precise string defining the nature of the event (e.g., `TASK_TRANSITION`, `AGENT_SPAWN`, `ERROR`).
4. **`data` (json.RawMessage)**: An arbitrary JSON object containing the context-specific payload.

### 2.2 Example Compliant Payload
```json
{
    "agent_id": "sub_agent_xyz123",
    "channel": "mesh:tasks",
    "event_type": "TASK_TRANSITION",
    "data": {
        "task_id": "uuid-1234",
        "previous_state": "PENDING",
        "new_state": "IN_PROGRESS"
    }
}
```

## 3. Validation Strategy
The Teammate Mesh unified gateway (`srcs/server/api/mesh/middleware.go`) serves as the gatekeeper.

- **Strict Enforcement**: The middleware must unmarshal incoming JSON and verify the explicit presence of `agent_id`, `channel`, `event_type`, and `data`.
- **Rejection Protocol**: Any payload missing one or more of these root keys must immediately be rejected with an `HTTP 400 Bad Request` and a standard error message specifying the missing OHC-SIP fields.
- **Data Encapsulation**: The `data` key acts as a wildcard envelope to support arbitrary event complexity without breaking the routing middleware.

## 4. Implementation Directives
The Implementer agent is tasked with updating the `meshPayload` struct and the `ValidationMiddleware` logic to reject deprecated keys (`action`, `status`) at the root level and enforce the strict OHC-SIP quad-key requirement.

---
*Authored by: Principal Product Architect & KAIROS Orchestrator (L7)*
*Identity: One Human Corp*

</div>
