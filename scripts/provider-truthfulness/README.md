# Provider acknowledgement regressions

Run `bash scripts/provider-truthfulness/run.sh` with the repository's Rust toolchain and cached locked dependencies. This small gate imports the complete production Twilio, Zoom and Daily clients and mounted WhatsApp settings handler files. It verifies registry versions/checksums against the repository lockfile and rejects changing source inputs during the run. It does not replace the full repository acceptance gates.

Twilio tests use actual TCP/HTTP exchanges on ephemeral loopback ports. The client sends its real search and provision requests to a private test-only fixture through its private endpoint field; the public constructor always selects the official HTTPS API. Responses exercise matching resource/account/number acknowledgements, missing or foreign receipts, malformed bodies, unfinished responses, transport loss, and explicit rejection. No live account, real credential or provider purchase is involved. The documented resource fields are in [Twilio's IncomingPhoneNumber API](https://www.twilio.com/docs/phone-numbers/api/incomingphonenumber-resource).

Zoom and Daily creation tests exercise the real request and receipt paths against
local HTTP fixtures through private methods. Public methods retain fixed official
HTTPS endpoints. Required receipt fields come from [Zoom's create-meeting response](https://developers.zoom.us/docs/api/meetings/#operation/meetingCreate)
(`join_url`) and [Daily's create-room response](https://docs.daily.co/reference/rest-api/rooms/create-room)
(`url`). A valid absolute HTTPS URL is returned unchanged; missing, duplicate,
wrong-type, empty or unsafe values cannot become fabricated meeting links.
Additive provider fields remain accepted. Requests have a 30-second overall,
5-second connect and 10-second read deadline, a 64 KiB streamed receipt limit,
and disabled retries and redirects. Tests cover exact/oversized/chunked bounds,
stalled/truncated receipts, post-consumption disconnects, non-success statuses,
and a single attempt. Unknown creation outcomes require reconciliation before
another call; neither provider response bodies nor credentials enter the errors.
Zoom's provider and registry forward errors once; the existing gRPC boundary
maps them to `Status::internal`. This change does not provide durable idempotency
or a reconciliation workflow across separate caller invocations. Daily remains a
separate crate; this test gate does not activate it in the root registry.

Cal.com is a separate, limited truthfulness fix. Its current v1 response schema
could not be verified from the official documentation available during review.
The client retains only its existing nonempty **string** `booking.id` contract;
numeric IDs and a v2 migration require verified provider documentation and new
fixtures. Local success tests prove preservation of that existing contract, not
live v1 compatibility. Missing, malformed, duplicate or unsupported receipts
return an unknown outcome instead of `mock_event_123`; creation has the same
deadline, retry, redirect and 64 KiB receipt limits described above. Availability
read/transport failures return errors without exposing the API key in its query
string. Booking-link lookup is unavailable until a verified account/team owner
and event-type mapping can resolve an actual URL; it rejects before network I/O
instead of inventing an `omnisolo-tenant` URL. No caller is activated or migrated.

The WhatsApp harness supplies only the handler's unrelated registry state and request DTO shape. Its registry is a negative effect sentinel: any attempt to configure an unverified connection is recorded and returns an error. It does not manufacture a successful connection. Both complete production handlers must return an unavailable response without invoking that effect. Actual provider verification and encrypted credential persistence are unavailable in this mounted path; `pending_verification` does not represent a queued verification job.

Separate UI tests check that an absent Meta SDK causes no credential or connect request, an unavailable server result cannot show a connected state, and voice provisioning needs an explicit valid acknowledgement. Acknowledged voice numbers are already persisted by the backend, so the UI does not issue a second settings mutation. Unknown provisioning outcomes remain visibly held in the current view; a durable server reconciliation hold is a separately tracked follow-on. No live deployment, Meta ceremony, real phone assignment or cross-reload safety is certified by this focused gate.
