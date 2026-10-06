# OpenRouter chat: implementation and acceptance gap

The maintained Next application has no `/chat` page. The existing authenticated
`POST /api/v1/chat` handler answers from Help Center articles, and
`POST /api/v1/agents/chat` records a department action for review. Neither dispatches
to OpenRouter. Assistant task creation persists a planning record; its current
model choices are `Auto` and `Agent`. `ProviderType::Openrouter` and the protobuf
enum alone do not implement a provider binding. The connection vault does not
support OpenRouter verification or encrypted storage.

`src/e2e/test_openrouter.spec.ts` therefore verifies the real authenticated
configuration rejection, unchanged stored connection identities, and actual
assistant model choices. It does not claim a successful model response. The
former test targeted a missing page and `.message.assistant` markup that no
maintained component renders; replacing that invalid case is not provider
completion.

A configured-response acceptance test remains required when a real mounted
OpenRouter adapter is implemented:

- Use the authenticated owner/tenant boundary and a verified, encrypted tenant
  connection. Reject missing, foreign-tenant and revoked credentials.
- Exercise the production adapter against an owned loopback HTTP server through
  a test-only endpoint seam. Assert the exact request model and message, a
  nonempty acknowledged assistant response and its displayed text. Do not use
  Playwright response substitution or a hard-coded assistant reply.
- Verify the UI does not call another provider or invent running/completed work
  for unconfigured, rejected, malformed, interrupted or unknown outcomes.
- Enforce the tenant's budget and cancellation before dispatch; verify duplicate
  submission and response-loss handling without unapproved provider retries.
- Run the resulting browser path with the real application and database.
  Local fixture evidence must be labelled separately from an explicitly
  authorized provider-sandbox test. No live provider call is authorized by CI.

WhatsApp embedded signup and verification are likewise not completed by an honest
501 readiness response. The current settings adapters must not store submitted
credentials or present a verified connection until that separate flow exists.
