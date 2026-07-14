# HTTP authentication rate-limit deployment contract

The Rust authentication transport keeps bounded, privacy-preserving login
buckets in process memory. Standalone deployments need no additional setting.

Cloud/multitenant startup fails closed unless
`OHC_AUTH_RATE_LIMIT_DEPLOYMENT` is exactly one of:

- `single-instance`: the backend is constrained to one replica.
- `upstream-bounded`: a trusted gateway enforces equivalent source and account
  login bounds before traffic can reach any backend replica.

An `upstream-bounded` gateway must apply both of these fixed-window limits to
every bounded, syntactically valid login request that reaches authentication,
including successful requests:

- At most 5 attempts per source in 300 seconds.
- At most 20 attempts per normalized account in 300 seconds.

The source is the direct peer IP unless that peer exactly matches an address in
`OHC_AUTH_TRUSTED_PROXY_IPS`. Only for those peers, the source is the single,
unambiguous IP in either `Forwarded: for=<ip>` or `X-Forwarded-For: <ip>`; the
gateway must reject multiple, combined, or malformed forwarding values. The
account key is `lowercase(trim(organization_id))`, a separator, then
`lowercase(trim(username_or_email))`. The two limits are independent, their
window begins with the first counted request, and a rejection must include the
remaining whole-second `Retry-After`. Gateway logs and counters must avoid raw
identifiers; use a keyed digest or an equivalently privacy-preserving key.

The application indexes expiry and recency so limiter operations are
logarithmic. Each source and account map holds at most 4,096 entries; if a map
is full, an unseen key fails closed until the earliest entry expires instead of
evicting an active limit. At most 16 password hashes may run concurrently;
excess work fails fast with a generic no-store `503`.

Do not set `upstream-bounded` merely because a load balancer exists. The
gateway policy must be deployed and monitored. Multi-replica cloud deployments
without that upstream policy are unsupported because process-local counters do
not form a distributed security boundary.
