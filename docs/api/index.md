<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# API

This section groups API-facing playbooks and reference material.

## Primary References

- [OHC Interactive API Playbook](./playbook.md) - Core REST endpoints and integration patterns

## gRPC Services

The server implements gRPC services defined in `src/proto/`. See proto definitions for:
- `HubService` - Authentication, growth, B2B, ops, MCP, integration, chat
- `AgentService` - Task execution, sub-agent dispatch
- `ModelService` - LLM provider management
- `BillingService` - Token usage tracking, cost summaries

</div>