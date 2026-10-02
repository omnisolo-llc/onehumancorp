outcome: no_work
issue_title: Native Rust Omnichannel Chat Engine
issue_description: |
  # Justification for Blocked / No Work

  The user asked to implement an omnichannel chat engine and unified inbox natively in Rust with WhatsApp, Instagram, and web channel support.

  However, per `RESEARCH.md`:

  > New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md.

  > Integrate existing tools instead of rebuilding them. Initial connector hypothesis is Google Workspace plus Stripe, subject to code/API/owner validation.

  > The core strategic directive in RESEARCH.md is to 'Integrate existing tools instead of rebuilding them.' Requests to build native replacements for external services (such as Chatwoot) without explicit authorization, evidence, and an expansion gate must be rejected with a no_work finding.

  Also, there is a specific restriction against implementing "unauthorized external channels (e.g. WhatsApp, Instagram)":

  > In the One Human Corp repository, integrating real external channels (e.g., WhatsApp, Instagram) strictly requires provider sandboxes and credentials to be available in the environment. If missing, report a blocked or no_work outcome.

  Since the environment lacks sandbox credentials for these external channels, and there is no authorization or evidence provided in `RESEARCH.md` to build a native omnichannel chat replacement or integrate these new channels, this task is blocked and results in a `no_work` outcome.

  Superpowers skill loaded: `skills/using-superpowers/SKILL.md` (revision `8ca22dba9a94f28898bbce59f2537ff4d87c747d`).
