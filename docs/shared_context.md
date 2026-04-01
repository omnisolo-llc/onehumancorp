<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Hybrid Contextual Insights: Shared Context & Best Practices

## System Architecture

One Human Corp (OHC) relies on a Cloud-Native Hybrid Architecture. All new feature additions must seamlessly integrate into the K8s-based orchestration layer. The primary interface for internal routing is the `Hub`.

**Hybrid Contextual Insights (Hub Messaging):**
> *When implementing complex multi-agent workflows, prefer asynchronous message passing via `Publish` rather than synchronous RPC calls. This prevents deadlock loops and allows LangGraph to accurately checkpoint intermediate states.*

## VRAM Quota Management

Enterprise adoption is hindered by unpredictable LLM costs and runaway compute.

**Hybrid Contextual Insights (Resource Quotas):**
> *Any feature that dynamically spawns new agent instances (like `DelegateSubTask`) MUST explicitly check the Hub's VRAM quota (hard limit: 10 active agents). Write locks must be used during check-and-spawn operations to prevent Time-of-Check to Time-of-Use (TOCTOU) quota bypasses.*

## Modular Capability Mesh

The system transitions from static blueprints to a dynamic Capability Plugin Mesh.

**Hybrid Contextual Insights (Plugin Mesh):**
> *When building tools, expose them as standardized `CapabilityManifests` rather than hardcoding bespoke API clients. The MCP Gateway ensures secure, mTLS-gated (SPIFFE/SPIRE) access for all agents automatically.*

## Aesthetic UI Tokens

All interfaces must follow the "Premium Branding" guidelines.

**Hybrid Contextual Insights (Design System):**
> *Never use raw hex colors or flat backgrounds in the Next.js UI. Always implement Glassmorphism tokens (`backdrop-filter: blur(15px)`, `background: rgba(255, 255, 255, 0.05)`) to maintain OHC's high-density, ghostly surface aesthetic.*


</div>
