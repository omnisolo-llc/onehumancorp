# Integration registry configuration boundary

`IntegrationsRegistry::connect` validates a supported provider and its required
input fields before changing any instance, credential record, or provider client.
An invalid reconfiguration leaves the previous local configuration intact.
Catalog entries with no implemented registry connection path are rejected.

A successful result has status `configured`. It means credentials and a client
configuration exist in the current process. It is not a verified account,
persisted connection, permission check, or successful provider action.
`test_connection` reports that provider verification is unavailable rather than
returning success without making a check. The integration UI keeps configured
responses visibly unverified and offers a Review action without automatically
repeating a connection request or navigating to a connected workflow.

## Explicit Razorpay credentials

The existing `ConnectIntegrationRequest` protobuf fields 1–7 keep their numbers.
Fields 8 (`api_key`) and 9 (`api_secret`) carry the explicit Razorpay key/secret
pair. Both are required. The generic `api_token` and `bot_token` fields are never
treated as an implicit pair. Credentials are not included in the returned
`IntegrationInstance`, errors, or status text.

Other providers retain their existing field mapping. Telegram, Discord, Slack,
and the WhatsApp alias remain recognized. A configured NATS transport retains
its existing background connection attempt; configured status does not certify
that the attempt succeeded.

NATS response/provider metadata omits URL userinfo while retaining the nonsecret
endpoint. The original URL remains the connection input; this does not implement
NATS authentication. Registry handshake logs/errors suppress credential-bearing
connector details without suppressing concurrent application logs. These are
library/legacy-service serialization guarantees; the legacy IntegrationService
is not mounted by the current server, and the separate HTTP integration endpoint
does not accept `base_url`.

## Verification and remaining implementation gaps

The focused std-only tests execute the actual production input validator. Source
contracts check forwarding into every real registry constructor, validation before
mutation, wire field numbers, and the configured status. UI tests cover configured
responses and reloads without a false Connected badge or a retry loop. These tests
make no live provider or payment calls. Full registry tests also cover rejected
configuration, unchanged prior state, and explicit Razorpay storage; running the
whole server test target remains required to verify those tests.

This repair does not establish production readiness for the registry as a whole:

- Credentials and instances are process-local; encrypted durable storage and
  verified connection receipts are not supplied by this registry.
- Disconnect still needs a complete client-revocation lifecycle, including pending
  background transport initialization and in-flight actions.
- The legacy PR/issue methods retain in-memory records rather than provider
  acknowledgements; their service implementations are not proof of a mounted,
  externally completed workflow.
- The legacy chat enqueue method returns its local message before background
  delivery finishes; that response is not a delivery receipt.
- Razorpay checkout semantics and payment reconciliation require separate end-to-end
  verification. Correct credential forwarding alone does not verify a checkout.

The mounted WhatsApp settings handlers separately report provider verification as
unavailable. Registry configuration tests do not turn those handlers into verified
connection workflows, or certify a new HTTP/gRPC mount.
