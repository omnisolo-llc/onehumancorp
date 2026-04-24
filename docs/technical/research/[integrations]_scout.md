<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title: [integrations] Scout: Resource Scout & Tool Integrator

## Problem Statement
The OHC swarm requires a specialized agent, "Scout," dedicated to finding external resources, APIs, and tools, and seamlessly integrating them into the swarm's available capabilities. Currently, the discovery and integration of new tools are manual or loosely coupled, preventing the swarm from autonomously expanding its skill set in response to novel tasks.

## Research Report
Market analysis shows that leading agentic frameworks are moving towards autonomous tool discovery and integration. An agent capable of "scouting" the web for APIs, parsing their specifications (e.g., OpenAPI), and registering them dynamically with the orchestration engine provides a significant "Unfair Advantage." Scout will act as the bridge between the external tool ecosystem and OHC's internal Dynamic Tool Discovery MCP.

## Design Doc
**Architecture:**
- Create a new sub-agent persona/definition for `Scout`.
- Scout will utilize Web Surfing/Scraping capabilities to find resources.
- Scout will leverage API Schema Parsing (OpenAPI/Swagger) to understand tool capabilities.
- Scout will interface with the `Dynamic Tool Discovery` MCP (Switchboard) to register new tools dynamically.
- Scout must operate securely within the Agent Harness, respecting all network proxy rules and multi-tenant guardrails.

**API / Integration Points:**
- Interactions with `src/server/lib/integrations/hybrid_discovery/DiscoveryProxy` for tool registration.
- Usage of secure scraping workers for web traversal.

**Security:**
- All dynamically discovered tools must undergo safety checks (Agentic Guardrails) before being made available to the broader swarm.
- Scout must authenticate via SPIFFE/SPIRE for all internal MCP interactions.

## Implementation Prompt
"Implement the Scout sub-agent definition and its core tool integration pipeline.
1. Define the Scout persona and its base prompts, emphasizing its role in finding and safely registering new tools.
2. Implement an integration pipeline that allows Scout to take an OpenAPI spec URL, parse it, and register the parsed endpoints as new MCP tools via the Dynamic Tool Discovery system.
3. Ensure the registration process includes a validation step against OHC's safety guardrails.
4. Write E2E tests simulating Scout finding a dummy API, parsing it, and successfully registering a new tool that another agent can then use.
5. Update necessary `BUILD.bazel` files."

## Priority
P2

## Estimated Scope
Large
</div>
