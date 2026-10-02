outcome: no_work
issue_title: Native Omnichannel Chat & Messaging Engine (Rust)
issue_description: |
  The task asks to implement a native omnichannel chat and messaging engine based on an audit of Chatwoot, including database schemas, gRPC APIs, and a Flutter UI for the widget.

  According to the OneHumanCorp operating contract (revision 2026-09-18-usage-audit), this task is blocked and should result in a `no_work` outcome.

  The constraints state:
  1. "implementing or testing new external channel integrations (e.g., WhatsApp, Instagram) strictly requires provider sandboxes and credentials to be available in the environment. If these are missing, it constitutes an outstanding verification dependency, and you must report a blocked or no_work outcome." No provider sandboxes or credentials are provided for WhatsApp, Instagram, or Email channels.
  2. The prompt specifically asks for a "Flutter UI view" and "Flutter web component". The OneHumanCorp constraints explicitly state: "If an assigned issue requests implementation in an unaligned language or framework (e.g., Flutter/Dart or Golang instead of Next.js/React or Rust), you must ignore the requested stack and implement using the repository's actual, established technologies." However, since there are no API credentials to even test the backend routing, this task cannot be implemented regardless.
  3. The requested task asks to build a new major unverified feature based on public research/Chatwoot audit instead of an agreed upon 90-day pilot scope. "New epics need an explicit evidence-backed decision... A missing SDK, provider sandbox, signing credential or owner interview is a specific outstanding verification dependency, not permission to report success."

  Required evidence/approvals before this feature can be built:
  1. Access to an explicitly labeled official provider sandbox or contract double for WhatsApp, Instagram, and Email.
  2. Concrete evidence (interviews/measured demand) from the 10 pilot partners (Days 15-45) proving the need for this specific inbox capability natively rather than an integration, with an explicitly approved scope decision.

  Until these dependencies are provided, this feature cannot be implemented.
