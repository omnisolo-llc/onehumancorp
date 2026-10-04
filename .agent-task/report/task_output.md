outcome: no_work
issue_title: Add Twilio WhatsApp Integration for Work Intake & Notifications
issue_description: |
  The current codebase already has Twilio WhatsApp integration implemented.

  Codebase analysis shows that WhatsApp via Twilio is already configured and functional:
  - `src/server/integrations/twilio/client.rs`: Contains the `send_whatsapp` method to format and send WhatsApp messages.
  - `src/ui/next/src/app/integrations/page.tsx`: UI for saving Twilio API credentials (`accountSid`, `authToken`, `phoneNumber`) with `recordConfiguredConnection('whatsapp', result)`.
  - `src/server/api/twilio_webhook.rs`: Implements handlers to receive incoming WhatsApp messages (via Twilio webhook) and enqueue them for `message_triage` which shows up in the `WorkFeed` UI.
  - `src/server/domain/inbox.rs` and `src/server/orchestration/departments/customer_success_agent.rs` use the integration to send drafted replies securely via Twilio WhatsApp API.

  This request is therefore a "no-work" finding since the desired capability is already satisfied by the codebase architecture.

  Skill provenance: Loaded `skills/using-superpowers/SKILL.md` from upstream repository revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d.
