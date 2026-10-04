outcome: blocked
issue_title: "Integrate WhatsApp Cloud API for Unified Customer Messaging"
issue_description: |
  The task requires integrating WhatsApp Cloud API to handle 2-way messaging in OHC Work Triage. However, we found that:
  - src/e2e/whatsapp-cloud-api-integration.spec.ts explicitly includes a comment stating "This CI journey verifies the unavailable contract. Never start a real provider login if this environment unexpectedly has a configured SDK".
  - src/server/api/integrations_settings.rs defines a verification_unavailable() function. This function returns a 501 NOT_IMPLEMENTED response, accompanied by a comment stating "Registry construction is configuration, not provider verification. This route has no verified-connection receipt or durable credential flow yet."

  Attempting to implement a real integration and provider connection without the secure provider verification flows, credential flows, and provider access prerequisites would violate the safety guidelines and native-build audit findings. Therefore, this issue is blocked on providing the prerequisites and durable credential flow.

  Skills Loaded:
  - skills/using-superpowers/SKILL.md

  Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
