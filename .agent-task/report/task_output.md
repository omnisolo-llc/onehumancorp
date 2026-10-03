outcome: no_work
issue_title: "Native Rust Omnichannel Chat: Inbox, Conversations, and Messaging Pipeline"
issue_description: |
  The requested feature—building a native Rust omnichannel chat engine to replace Chatwoot, including DB migrations, `src/server/services/chat/repository`, `MessageService::send_message`, pubsub events, gRPC/REST endpoints, and mobile UX flow—is explicitly blocked.

  According to `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md`, the current focus is on "Correctness before expansion" and proving a reusable operating loop (lead → quote → booking → delivery → invoice → collection). The strategy explicitly mandates: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md." Furthermore, "Existing commerce, fulfillment and service modules preserved; earlier exclusive agency segment and fixed-price targets remain suspended." (F15).

  The issue claims a "mandatory directive to retire Chatwoot as an external third-party service" and that "OHC must own its complete omnichannel chat engine natively in Rust." However, `RESEARCH.md` explicitly states: "Integrate existing tools instead of rebuilding them... Reuse existing business modules... Defer new viral generators... agent marketplaces... unless an accepted issue shows they block the selected business outcome."

  There is no evidence in the active capability map or audit that replacing an existing chat service (if one is even actively used for this purpose) or building a comprehensive omnichannel chat from scratch is authorized. The `RESEARCH.md` document clearly states: "Research/report jobs: only .agent-task/report/task_output.md... A no-work result must use the supported runner outcome rather than a pretend feature issue." and "Terminal result contract (only when no implementation is possible or needed): write `.agent-task/report/task_output.md` as YAML with `outcome: blocked` or `outcome: no_work`, `issue_title`, and `issue_description` containing the exact prerequisite or verification evidence."

  Skill provenance: Loaded `superpowers` workflow at revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d, using `using-superpowers` skill.
