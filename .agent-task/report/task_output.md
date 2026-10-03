outcome: blocked
issue_title: "Implement Native Rust Omnichannel Chat & Agentic Triage (Chatwoot Replacement)"
issue_description: "The requested implementation for Native Rust Omnichannel Chat introduces new channels (Instagram DM, WhatsApp, etc.), which requires explicit evidence and an expansion gate in RESEARCH.md according to the OHC operating contract. I ran `grep -i \"expansion\" RESEARCH.md` which confirmed 'Additional channels... Require retained-customer need, measurable value and explicit strategy approval'. No such expansion gate for omnichannel chat was found in RESEARCH.md. Additionally, the issue requests building a Flutter PWA, but the project's frontend architecture has explicitly migrated away from Flutter/Dart to Next.js inside a Tauri shell. As per system instructions ('if the issue describes implementing using golang/dart, but the current repository is actually implemented using rust/nodejs, you must implement using rust/nodejs instead'), this would require adapting to the Next.js stack. However, the core backend requirement of introducing new channels remains blocked by the missing explicit expansion strategy in RESEARCH.md, rendering the entire implementation blocked."
issue_priority: "P1"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
