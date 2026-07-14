# OIDC authority transport and cache contract

OIDC issuer discovery and JWKS endpoints must use HTTPS. The server rejects
redirects, HTTPS-to-HTTP JWKS downgrade, private or special-purpose addresses,
non-success responses, oversized bodies, oversized key sets, and ambiguous key
IDs. DNS, connection, request, total-fetch, and single-flight wait times are
bounded.

Local development may use HTTP only when both `OHC_OIDC_ALLOW_HTTP=true` and
`OHC_ALLOW_LOCAL_IPS=true` are set. Even then, HTTP targets must resolve to a
loopback, private, or unique-local address; these settings cannot
enable public cleartext authorities. Never set either exception in a shared or
production environment.

Successful key sets are cached for five minutes and shared without copying.
Outages are negatively cached for 30 seconds. An unknown key ID in an already
cached set triggers one bounded per-issuer refresh so normal signing-key
rotation is recognized; further unknown-key refreshes are suppressed for 30
seconds to prevent attacker-controlled key IDs from causing discovery traffic.
