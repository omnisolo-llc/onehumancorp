issue_title: "🔗 Link: [blocked no-work finding: Issue #36208 Native Omnichannel Chat]"
issue_description: |
  The implementation of the Native Omnichannel Chat & AI Unified Inbox (GitHub Issue #36208) is blocked. The acceptance criteria explicitly require ZERO mocked data, real database seeds, and integration with Meta Graph API, WhatsApp Business API, and SMS Webhooks via Twilio. However, the required external provider sandboxes, signing credentials, and SDKs are unavailable in the current environment, which is an outstanding verification dependency. Furthermore, `src/server/api/tool_integrations.rs` explicitly fails closed with HTTP 501 for unverified connections per the remediation ledger (F06). Proceeding without valid provider access and real integration tests would violate the evidence discipline and zero mocked data requirement. No work can be performed until the required authorization and provider access are established.

  Resolves #36208

  # Executed test commands
  - `ls -la` (Truncated output)
  - `cat RESEARCH.md | head -n 50` (Truncated output)
  - `cat docs/research/native_migration_and_remediation.md` (Truncated output)
  - `cat AGENTS.md | head -n 50` (Truncated output)
  - `cat RESEARCH.md | grep -A 20 -i "Active business-capability map"` (Truncated output)
  - `cat docs/research/business_capability_and_usage_economics_audit.md | head -n 50` (Truncated output)
  - `git status && git branch -a` (Truncated output)
  - `git branch -a | grep -i "36208\|chat\|omni"` (Truncated output)
  - `git log --all --grep="36208"` (Passed)
  - `ls -l src/ui/next` (Truncated output)
  - `cat src/server/api/tool_integrations.rs | grep -A 20 -i "chatwoot\|whatsapp\|instagram"` (Truncated output)
  - `cat src/server/api/tool_integrations.rs` (Truncated output)
  - `grep -rn "chatwoot" src/` (Passed)
  - `git log --all --grep="chat"` (Truncated output)
  - `cat docs/research/native_migration_and_remediation.md | grep -i "chatwoot"` (Passed)
  - `grep -rn "chatwoot" docs/` (Truncated output)
  - `ls -la docs/research` (Truncated output)
  - `ls -l docs/superpowers/specs/` (Truncated output)
  - `cat docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md | grep -A 30 -i "chatwoot"` (Truncated output)
  - `ls -l docs/superpowers/plans/` (Truncated output)
  - `cat docs/superpowers/plans/2026-07-13-chatwoot-removal.md | grep -i "remove"` (Truncated output)
  - `cat deploy/tests/no_chatwoot_residue_test.sh` (Truncated output)
  - `git status` (Passed)
  - `cat docs/research/native_migration_and_remediation.md | grep -A 20 -i "M0"` (Truncated output)
  - `git status && git branch -a | grep -i "36208"` (Passed)
  - `cat .agent-task/report/task_output.md || true` (Passed)

  # Verified trace limitations
  - `ls -la`
  - `cat RESEARCH.md | head -n 50`
  - `cat docs/research/native_migration_and_remediation.md`
  - `cat AGENTS.md | head -n 50`
  - `cat RESEARCH.md | grep -A 20 -i "Active business-capability map"`
  - `cat docs/research/business_capability_and_usage_economics_audit.md | head -n 50`
  - `git status && git branch -a`
  - `git branch -a | grep -i "36208\|chat\|omni"`
  - `ls -l src/ui/next`
  - `cat src/server/api/tool_integrations.rs | grep -A 20 -i "chatwoot\|whatsapp\|instagram"`
  - `cat src/server/api/tool_integrations.rs`
  - `git log --all --grep="chat"`
  - `grep -rn "chatwoot" docs/`
  - `ls -la docs/research`
  - `ls -l docs/superpowers/specs/`
  - `cat docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md | grep -A 30 -i "chatwoot"`
  - `ls -l docs/superpowers/plans/`
  - `cat docs/superpowers/plans/2026-07-13-chatwoot-removal.md | grep -i "remove"`
  - `cat deploy/tests/no_chatwoot_residue_test.sh`
  - `cat docs/research/native_migration_and_remediation.md | grep -A 20 -i "M0"`
outcome: blocked
