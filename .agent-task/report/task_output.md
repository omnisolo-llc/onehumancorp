outcome: no_work
issue_title: "Integrate Cal.com for Unified Omnichannel Scheduling"
issue_description: |
  The requested feature to integrate Cal.com for scheduling already exists in the codebase and is actively supported.

  Evidence of implementation:
  1. The API client for Cal.com is fully implemented in `src/server/integrations/cal_com/client.rs` providing `get_free_busy`, `create_event`, and `get_booking_link` methods.
  2. The provider interface is implemented in `src/server/integrations/cal_com/provider.rs`.
  3. The Cal.com provider is registered in the integration catalog (`src/server/integrations/catalog.rs`) with id `cal_com`.
  4. The integration registry (`src/server/integrations/registry.rs`) fully supports Cal.com for credential storage, `get_free_busy`, `create_event` and `get_booking_link`.
  5. The UI natively supports connecting Cal.com in `src/ui/next/src/app/integrations/page.tsx` and it's covered in E2E tests (`src/e2e/tool_integrations.spec.ts`).

  Loaded skills during this trace:
  - `skills/using-superpowers/SKILL.md` (revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - `skills/brainstorming/SKILL.md` (revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - `skills/systematic-debugging/SKILL.md` (revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
