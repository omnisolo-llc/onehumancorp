outcome: no_work
issue_title: "Implement Custom Rust Omnichannel Chat System based on Chatwoot"
issue_description: |
  The current GitHub issue (#36816) asks to "Implement Custom Rust Omnichannel Chat System based on Chatwoot", which includes adding Meta Cloud API for WhatsApp, generic APIs, and WebSockets.

  According to the `RESEARCH.md` rules on missing SDKs and provider sandboxes, and the environment state, the necessary WhatsApp and Twilio integrations require external credentials to test.
  Running `env | grep -i 'whatsapp\|twilio\|meta'` confirms there are no sandbox credentials in the environment.

  The implementation is explicitly blocked as missing external credentials/sandboxes block the implementation and verification:
  "A missing SDK, provider sandbox, signing credential or owner interview is a specific outstanding verification dependency, not permission to report success."
  "missing external credentials, provider sandboxes, or SDKs (e.g., Meta Cloud API for WhatsApp, Twilio) constitute outstanding verification dependencies, not permission to report success. If a task requires unavailable external credentials to implement or test, and mocks are forbidden, you must return a blocked or no_work outcome rather than skipping tests, weakening assertions, or claiming success."

  Furthermore, the issue calls for Chatwoot concepts, but "Chatwoot has been permanently superseded by a native omnichannel chat design... Do not attempt to reintroduce Chatwoot or implement new third-party replacements for it without explicit authorization."
  There is a strict CI check `deploy/tests/no_chatwoot_residue_test.sh` that fails if the string 'Chatwoot' is found in any unapproved file.

  Therefore, no source code changes are performed, and this is reported as a 'no_work' outcome.

  Loaded skills:
  - superpowers:brainstorming from /tmp/superpowers/skills/brainstorming/SKILL.md (commit 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - superpowers:using-superpowers from /tmp/superpowers/skills/using-superpowers/SKILL.md (commit 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
