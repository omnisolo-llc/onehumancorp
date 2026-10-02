outcome: no_work
issue_title: "Architecting OHC-Native Omnichannel Chat and Real-Time Routing Engine"
issue_description: |
  According to RESEARCH.md, the core strategic directive is to integrate existing tools instead of rebuilding them. The issue description explicitly requests implementing an OHC-Native Omnichannel Inbox to replace existing third-party customer support software (like Chatwoot, Zendesk). This contradicts the core directive: "Integrate existing tools instead of rebuilding them."

  Furthermore, the issue requires a large implementation effort (backend + UI) for a new chat engine which violates the directive: "Evaluate managed API, customer API-key/cloud-account billing, provider-permitted native-client subscription access and local inference separately. ... Evidence comes before another concrete product plan."

  And: "No deferred feature is commissioned merely because an old prompt says mandatory or a generic TODO exists."

  Finally, the issue instructions states: "The exclusive digital-service first market is suspended, not validated demand. Evaluate owner needs across plausible service, retail and physical-operation workflows already represented in the code." which clearly states that there shouldn't be large feature implementations that haven't been validated yet.

  Skill Provenance: Loaded superpowers workflow (8ca22dba9a94f28898bbce59f2537ff4d87c747d) skills using-superpowers, brainstorming.
  Executed test commands:
  cat RESEARCH.md
  cat docs/research/business_capability_and_usage_economics_audit.md
  ls -la .agent-task/
  ls -la .agent-task/report
  cat .agent-task/report/task_output.md
  cat RESEARCH.md | head -n 50
  git log --oneline -n 5 && git status
