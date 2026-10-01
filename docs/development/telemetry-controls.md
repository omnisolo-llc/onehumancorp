# Product telemetry control boundaries

The current product collector is instance-wide. It is not a per-tenant opt-out
implementation. Tenant ADMIN/OPERATOR roles do not establish platform authority.
The hosted settings endpoint therefore exposes collection state read-only and
rejects writes until an explicit platform-wide control is implemented.

In established standalone mode, an authenticated existing ADMIN may update the
stored preference when operator configuration does not force collection and
persistent settings storage is available. This does not grant new roles or
change environment/operator configuration.

`GET /api/v1/settings/telemetry` returns:

- `product_telemetry_enabled`: stored preference, retained for compatibility
- `preference_enabled`: the same stored preference with an explicit name
- `effective_enabled`: configured collection OR the actual runtime flag
- `operator_enforced`: configuration currently forces collection on
- `can_change`: this caller can change the collector through this endpoint
- `change_block_reason`: `hosted_global_control_unavailable`, `admin_required`,
  `operator_enforced`, `persistent_storage_unavailable`, or null

The UI must display effective collection separately from the saved preference.
A false preference does not mean collection is disabled when operator
configuration forces it. Missing/unknown permission fields do not grant write
permission. In particular, hosted read-only state does not promise tenant-level
control that the current collector cannot enforce.

`POST` accepts exactly a boolean `product_telemetry_enabled`. Success is emitted
only after the candidate settings file is atomically saved and memory/runtime
state is published under the same write lock. The response includes `success`
and the same state fields. Authentication/authority errors are 401/403,
operator-enforced state is 409, missing persistent storage is 503, and a failed
save is 500 with `success:false`. Invalid payloads are rejected rather than
silently interpreted as disabling telemetry. Failed saves preserve the prior
committed preference and effective runtime flag.

The missing hosted platform control and absence of per-tenant collection
preferences are product capability gaps. They must not be filled by inventing an
operator role or promoting a tenant administrator.

## Verification boundary

`settings.rs` and `api/telemetry_settings_test.rs` contain real temporary-file,
concurrent writer/reader and mounted HTTP tests. They cover denied hosted tenant
admins, standalone role separation, forced configuration, malformed requests and
persistence failures. Tests change only isolated test files and test-process
flags. They do not prove delivery to a telemetry backend, operate production
settings, or substitute for the full application acceptance gates.
