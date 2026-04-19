# Universal Agent Harness Transport Bridge

<style>
.premium-card {
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 24px;
  font-family: 'Outfit', 'Inter', sans-serif;
  color: #E2E8F0;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  margin-bottom: 24px;
}
.premium-title {
  font-family: 'Outfit', sans-serif;
  font-weight: 600;
  color: #FFFFFF;
  margin-top: 0;
}
</style>

<div class="premium-card">
<h2 class="premium-title">Overview</h2>
The Universal Agent Harness Transport Bridge is a core architectural component of the OHC-HA (Hybrid Architecture). It solves the rigid environment lock-in problem observed in competing systems (like Claude Code's SdkControlTransport) by dynamically wiring the transport layer based on the execution mode (Cloud vs. Standalone).
</div>

<div class="premium-card">
<h2 class="premium-title">Architecture</h2>
The orchestrator interacts with sub-agents via a virtual MCP (Model Context Protocol) interface. The bridge abstracts the underlying communication medium:

- <strong>Standalone Mode:</strong> Uses `InProcessTransport` (wrapping OS-level stdio) for zero-latency local execution.
- <strong>Cloud Mode:</strong> Uses `RedisPubSubTransport` for horizontally scalable, multi-tenant execution across Kubernetes pods.

```mermaid
graph TD
    A[Agent Orchestrator] -->|Virtual MCP Interface| B(Universal Transport Bridge)
    B -->|Cloud Mode| C[RedisPubSubTransport]
    B -->|Standalone Mode| D[InProcessTransport]
    C --> E[(Redis Cluster)]
    D --> F[Local Process OS Stdio]
```
</div>
