outcome: no_work
issue_title: Native Rust Omnichannel Chat Integration - Chatwoot Migration
issue_description: |
  # Native Rust Omnichannel Chat Integration - Chatwoot Migration

  ## No-work Finding
  This task requested building a native Rust microservice inside the OHC monorepo to replicate Chatwoot (omnichannel customer support, webhooks, WebSockets, Flutter UI, etc). This request is rejected based on the explicit strategic directives in `RESEARCH.md`.

  According to `RESEARCH.md` (Revision: 2026-09-18-usage-audit), the core directive is:
  > **Integrate existing tools instead of rebuilding them.**

  Furthermore, the "What to defer" section states:
  > Defer new viral generators... agent marketplaces, additional harness adapters, visual workflow builders, simultaneous HR/payroll/MRP coverage, generic UI restyling and unsupported global compliance claims unless an accepted issue shows they block the selected business outcome. Retain useful existing implementations; avoid deletion or a platform rewrite just to fit this document.
  > Prefer popular, actively maintained libraries for standard infrastructure and established providers for payments, delivery, messaging, banking and specialized obligations. Build OmniSolo's differentiated layer: persistent business context, specialist coordination, reliable cross-tool execution, verification and recovery, and a simple owner experience.

  The request explicitly mentions:
  > OHC Strategy: We will not integrate with Chatwoot as a third-party service. Instead, we will build a native Rust microservice/crate inside the OHC monorepo that replicates these core capabilities.

  This is a direct violation of the directive to integrate existing tools and prefer established providers for messaging. Building a native omnichannel chat engine replicating an entire existing open-source product (Chatwoot) is not within the authorized scope of OmniSolo's differentiated layer.

  Additionally, the request mentions developing "Flutter UI components". This violates the established technology stack (Next.js/React and Rust/Tauri). `RESEARCH.md` mandates tracing actual loaded frontend code and explicitly says: "do not dismiss Next as legacy or initiate a Go/Flutter rewrite."

  Therefore, no implementation work will be performed for this issue.

  ## Outstanding Blockers / Dependencies
  - Authorization / Strategy Alignment: Building native replacements for external services requires explicit authorization, evidence, and an expansion gate in `RESEARCH.md`.
  - Technology Stack Alignment: Any frontend work must be done in the existing Next.js/React architecture, not Flutter/Dart.
