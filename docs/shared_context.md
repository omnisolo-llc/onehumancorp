<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 24px; border-radius: 12px; font-family: 'Outfit', sans-serif; color: #E0E0E0;">

# OHC Shared Context & Developer Insights

This document synthesizes current technical debt and actionable engineering items across the `srcs/` codebase to maintain harmony with platform specifications.

## Security & Static Analysis

**IronClaw CLI Scanner Context:**
- **Context:** The security analysis engine in `srcs/cmd/ironclaw` has existing security vulnerabilities that require fixing.
- **Insight:** The `TODO: fix security` markers indicate an incomplete implementation of the scanning logic or an internal issue regarding parsing insecure tokens. The codebase contains a hardcoded vulnerability scan test (`password = "secret"`) and unhandled internal security constraints.
- **Actionable:** Prioritize resolving these findings to achieve >95% security module coverage.

## Core Capabilities Mesh

**Capability Plugins Registry:**
- **Context:** Agents will discover skills dynamically via the MCP Gateway instead of static Skill Blueprints.
- **Insight:** Documentation surrounding dynamic plugin loading in the Swarm Memory database needs expansion. Expect a new `capability_plugins` table in `ohc.db` to govern this ecosystem.

</div>
