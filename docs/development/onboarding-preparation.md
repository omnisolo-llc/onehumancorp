# Durable local onboarding preparation

The mounted authenticated onboarding API now separates reviewed local preparation
from the explicit local launch marker. Neither response proves a public site,
provider checkout, policy generation, SEO delivery or downstream automation completed.

## Preparation and recovery

`POST /api/v1/onboarding/start` keeps the existing request fields. All reviewed
`initial_products` and variants are validated and saved synchronously in one
PostgreSQL transaction, together with tenant details, idle preparation-owned
agents and the protected receipt in the existing `onboarding_state` table.
The unsupported `onboarding_generate_catalog` job is no longer enqueued.
Legacy first-product and existing persona-default catalog definitions remain.

Money uses strict decimal minor units, with at most two fractional digits and a
maximum of 1,000,000,000 cents. Product prices cannot be negative. Variant
adjustments can be negative but cannot make the resulting price negative. Invalid,
nonfinite, exponent, overflow and fractional-cent values are rejected.

Successful start responses retain `success`, `message`, `organization_id` and
`user_id`, and add `status`, `preparation_id`, `product_ids` and `preparation`.
The protected preparation projection contains:

- `preparation_id`, `status`, `organization_id`, `user_id`
- Stable `primary_product_id` and the original snake_case `reviewed_request`
- `catalog`: reviewed entries with `product_id`, `item_type`, `name`, `description`,
  decimal-string `price`, and variants with `variant_id`, `name`, and decimal-string
  `price_modifier`
- `notification_status`

`GET /api/v1/onboarding/state` returns this preparation at the top level alongside
sanitized wizard state. It reads the receipt and wizard state in the same database
transaction. Browser-supplied draft JSON cannot create or modify the receipt. Draft chat history
may contain at most 20 user/assistant role/content pairs, 4,000 characters per
message and 12,000 total; images, identity and authority keys are stripped, and
invalid histories return 400 rather than falsely reporting a saved conversation.
The equivalent legacy `chat_history` and bounded snake_case business draft fields
also survive reload. Legacy `capabilities` stores only draft/schedule/inventory
booleans as draft preferences; it grants no execution or launch authority.

Exact repeated or concurrent starts return the recorded preparation without
creating duplicate rows or overwriting later edits. After an unknown response,
read authenticated state before deciding whether another explicit action is needed.
For zero-click preparation, the original prompt/image identity is checked before
calling intake again; already committed preparation can be recovered without a
new provider call. Concurrent first-time intake requests can still both incur
intake work before either commits; callers must not blindly retry that POST.

## Explicit pre-launch revision

A changed request must include `replaces_preparation_id`. Existing catalog entries
carry their `product_id`; existing variants carry `variant_id`. New entries omit
IDs. Matching never uses array position or fuzzy names. The primary product ID
remains stable across reordering.

The original authenticated principal must still have active ADMIN/owner authority.
Every prior prepared product and variant must remain present, and stored snapshots
must still match under row locks. Tenant name/subdomain changes made elsewhere also
require reconciliation. Destructive removals are rejected; use the catalog editor
for supported later changes. Existing product type, stock, and unrelated metadata
are preserved. A launched preparation cannot be revised by replaying onboarding.

The latest reviewed agent selection controls activation. Existing operator-managed
agents are not replaced or activated by this workflow. Preparation-owned agent
changes made elsewhere require reconciliation before automatic activation.

## Launch and notification boundary

`POST /api/v1/onboarding/launch` requires JSON `{ "preparation_id": "..." }`.
It verifies the same tenant and principal, current authority and unchanged prepared
catalog, then commits the local launched marker, activation of eligible agents,
subscriptions and the existing welcome/report setup rows. Exact launched replays
return the original identity before examining subsequently changed catalog rows.

Activation/product/storefront/policy notifications are deferred until this explicit
launch. Their envelopes are durable in the receipt. A transaction claims envelopes
before a single publication attempt; an ambiguous attempt is not automatically
republished because consumers have not been proven idempotent. The existing Hub
broadcast API does not confirm receipt or completion, so `delivery_unconfirmed`
remains truthful even when its method returns `Ok`. A stopped process can leave
`pending` or `delivery_unconfirmed` work requiring reconciliation. This repair
adds no claim that a durable notification worker or public deployment exists.

## Errors and verification

Errors use `success:false`, an `error` reason and `reconciliation_required` where
applicable. Changed identity/snapshot or unsupported revision returns 409; invalid
input 400; authority failure 401/403. Transaction errors never produce success.
All COMMIT errors conservatively require reconciliation, including lost replies.

The portable focused harness is `scripts/onboarding-durability/run.sh`. It requires
an owned isolated PostgreSQL database and checks exact production helper/API
source, real bearer validation, real transaction rollback and source/lock identity.
Provider intake, environment provisioning and downstream publication are not run.
Full workspace acceptance and application E2E remain separate gates.
