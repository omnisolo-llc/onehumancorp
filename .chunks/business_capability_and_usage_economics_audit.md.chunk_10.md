 brokering.

**Anthropic:** [Claude Code legal/auth guidance](https://code.claude.com/docs/en/legal-and-compliance) permits hosting the **unmodified** Claude Code binary under stated conditions, including users authenticating and paying directly. It prohibits a third-party Claude.ai login/token relay and end-user usage resale/intermediation in that arrangement. An OHC-owned Agent SDK/API application needs API/cloud authentication. These are distinct architectures; confirm the proposed deployment against the full terms rather than treating consumer credentials as BYOK API keys.

**Google:** [Gemini CLI quotas](https://geminicli.com/docs/resources/quota-and-pricing/) distinguishes eligible Google AI/Code Assist login allowances from API-key and Vertex billing. A generic Workspace subscription is not automatically a Gemini API entitlement. [Gemini API billing](https://ai.google.dev/gemini-api/docs/billing) ties API keys to projects and Cloud Billing; token classes and cache storage can have separate costs. Verify current plan eligibility and embedding terms, and never silently switch an exhausted subscription session to a paid OHC API key.

This is product due diligence, not a legal opinion. No supported native-client subscription mode was demonstrated end-to-end in OHC. The existing Codex adapter and API/OAuth credential types are reuse assets, not that demonstration.

## 7. Cost accounting before a price card

The current source does not supply a measured deployment cost, representative workload distribution or reconciled provider invoice. Do not replace the former invented monthly budgets with invented per-token or per-compute prices.

A proposed internal cost model is:

```text
OHC serving cost = OHC-funded model/tool usage
                 + allocated CPU, memory and GPU resources
                 + databases, storage, queues, backups and logs
                 + browser/runtime capacity and network egress
                 + paid external tools and communication
