# Harness local services

Local services run in a separate container with the worker's network namespace. The native worker has no mounts for service configuration, SQLite, blob storage, or tool roots. Each worker and its daemon use the same control token (at least 32 ASCII characters). Worker gRPC clients must send that token as `authorization: Bearer <token>` metadata. Native harnesses receive different credentials scoped to their current attempt.

For Compose, provision `<root>/<harness>/configuration.json` and `<root>/<harness>/data`, then set `OMNISOLO_LOCAL_SERVICE_ROOT` to the absolute root and `OMNISOLO_<HARNESS>_LOCAL_SERVICE_CONTROL_TOKEN` to the worker's token (`OPEN_INTERPRETER` uses an underscore). Start a selected worker and its daemon with:

```sh
docker compose -f deploy/docker-compose.yml -f deploy/docker-compose.local-services.yml up -d harness-codex-local-services
```

The selected daemon depends on its worker. Unselected tokens may remain unset; selecting an unconfigured daemon fails closed. With `--profile harness`, configure all 12 worker tokens and data directories.

For Helm, add `localServices` to the selected `harnessWorkers.workers` entry:

```yaml
localServices:
  image: omnisolo/harness-local-services:local
  configMap: admitted-service-config
  dataClaim: selected-service-data
  controlSecretName: worker-control
  controlSecretKey: token
```

The ConfigMap contains `configuration.json`; the PVC is mounted only in the daemon at `/services`. Configuration must explicitly admit tenant, project, workspace and session UUIDs before startup. Database schema and selected storage/tool roots must already exist. Example:

```json
{"version":1,"scopes":[{"tenant_id":"tenant","project_id":"project","workspace_id":"workspace","session_id":"aaaaaaaa-aaaa-4aaa-aaaa-aaaaaaaaaaaa","task_id":null,"attempt_id":null}],"sqlite_url":"sqlite:///services/memory.db","blob_root":"/services/blobs","tool_root":"/services/tools","allowed_tools":["Read"],"browser":true}
```

Configure the daemon image tag appropriate to the deployment. Provision the existing `agent_memory` FTS5 schema and per-project tool directories before mounting data. Worker bootstrap reads admitted scopes and backend identity from the authenticated daemon; native request payloads cannot grant new scopes.

Select exactly one memory configuration. `sqlite_url` reuses the existing agent SQLite database. Alternatively, set `memory` to one of these service-owned configurations:

```json
{"backend":"json","root":"/services/selected-json"}
{"backend":"anthropic","root":"/services/selected-anthropic"}
{"backend":"redis","url":"redis://selected-host/0","namespace":"selected-memory"}
{"backend":"vector","url":"postgres://selected-host/database","tenant_id":"tenant","agent_id":"agent"}
```

Vector storage requires the existing consolidated-memory schema and an explicit `embedding` configuration containing `base_url`, `model`, and `api_key_env`. Supply that environment credential only to the daemon. All admitted scopes must match the selected vector tenant. Storage selection is never inferred from a native request or replaced by a fallback database.

For configuration containing credentials, use a Kubernetes Secret with `localServices.configSecret` instead of `configMap`, or a protected Compose configuration file. The Secret also contains `configuration.json`. Mounts remain daemon-only. `integration_read` returns metadata for an allowed read-only tool; `integration_invoke` executes it with explicit arguments. Worker startup retries a starting daemon for a bounded interval and rejects invalid control credentials immediately.

Additional daemon environment entries, such as an embedding key sourced with `secretKeyRef`, go under Helm `localServices.env`. For Compose, add them to the daemon service's environment in a private override file. Do not add backend credentials to the native worker environment.

The pinned Codex image sets `OMNISOLO_HARNESS_EXTERNAL_SANDBOX=1` because the
worker container supplies isolation and cannot create a nested Bubblewrap
namespace. The worker selects Codex's `externalSandbox` turn policy with network
access for its scoped loopback services. Host workers default to Codex's read-only
sandbox; session metadata cannot select the container policy.
