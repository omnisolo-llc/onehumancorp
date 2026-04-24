# OHC Hybrid Agentic OS - Privacy & Compliance Audit Report

## 1. Hybrid Privacy Audit: Cloud vs. Standalone
Data handling in the OneHumanCorp (OHC) "Hybrid Agentic OS" requires distinct treatment of user information depending on the runtime environment to respect privacy-by-design.

### Cloud-Native Data Handling
In the cloud, multi-tenancy ensures strict isolation using row-level security (`ENABLE ROW LEVEL SECURITY`) bound by a `tenant_id` column. However, because services share the same computing and logging substrate, telemetry payloads from background task execution (such as `RecordSubAgentExecution` or `RecordAutoDreamSyncError`) pose a significant risk of leaking Personally Identifiable Information (PII) if unredacted. The primary mechanism guarding against this is the `BufferMetricFunc` combined with JSON serialization that passes data through `RedactInterfacePII()`.

### Standalone (Local Sovereignty) Data Handling
In Standalone mode (identified by `OHC_STANDALONE=true` and `OHC_MULTITENANT=false`), the local database is isolated per installation (e.g., SQLite). The system utilizes an in-memory or purely local fallback mechanism where telemetry is accumulated in the local SQLite table `local_telemetry_buffer`. The critical distinction is that Standalone operations *never* exfiltrate data automatically unless explicit consent via `OHC_TELEMETRY_ENABLED=true` is provided. If opted out, the local buffering daemon's `BufferMetricFunc` is explicitly nulled out, effectively preventing any tracking data from being recorded locally or pushed.

## 2. Compliance Guardrails
To prevent accidental data exfiltration in multi-tenant environments, the following technical controls are verified to be active and operational:

*   **`RedactInterfacePII` Enforcer:** The system implements a comprehensive scanner (`global_pii_linter_test.go` and `buffer_pii_linter_test.go`) that crawls the entire `srcs/server/telemetry` AST structure. It enforces a strict rule: any code that utilizes `json.Marshal` on payload maps *must* encapsulate the data with `RedactInterfacePII` or `RedactPII`.
*   **Redaction Targets:** The redaction logic properly obfuscates Emails, Phone Numbers, SSNs, Credit Cards, OpenAI Keys, Anthropic Keys, and AWS Access/Secret Keys using a suite of Regex patterns and targeted string replacement methods.
*   **PII-Redacting Logger Handler:** The server utilizes a custom slog handler wrapper (`PIIRedactingHandler` in `logger.go`) which dynamically scrubs incoming log messages and structured attributes. This prevents developers from accidentally outputting unstructured PII data to stdout or remote logging solutions.
*   **Standard Logger Prevention Linter:** To completely mitigate the risk of developers bypassing `PIIRedactingHandler` by writing to standard Go `log.Print` or `fmt.Print` (which have no concept of multi-tenant masking), a new global policy-as-code lint rule `TestStdLogLinter` was created in `ast_pii_linter_test.go`. This automated check parses the AST across `srcs/server` and prevents the usage of the legacy unredacted loggers, ensuring all multi-tenant backend logging safely flows through the central `slog` sink.

## 3. Local Sovereignty Audit
A deep review of the Standalone wrapper demonstrates robust privacy controls preserving user data sovereignty:

1.  **Opt-Out by Default Strategy:** Telemetry initialization enforces an opt-out policy when `OHC_MULTITENANT=false`. If `OHC_TELEMETRY_ENABLED` is missing or false, the entire background metric recording pipeline (`BufferMetricFunc`) is disabled and evaluates to a no-op handler.
2.  **No Unconsented Exfiltration:** The `SyncDaemon` architecture strictly checks for local sync allowances before forwarding any buffers to the cloud. When telemetry sync is enabled (e.g., via the `InitStandaloneBuffer` sequence), metrics are scrubbed locally of any PII using `RedactInterfacePII` prior to even being inserted into `local_telemetry_buffer`. Consequently, the synchronization daemon only sends fully anonymized metadata (e.g., cost metrics, API latencies) to the central server.
3.  **Local Memory Backend Isolation:** Memory for AI Agents relies on the `VectorRepository` powered by a local SQLite dot-product extension. This prevents contextual prompt embeddings from being transmitted to the central cloud for long-term storage or RAG processing, keeping users' proprietary knowledge strictly within their execution sandbox.

## Conclusion
The OHC codebase successfully enforces its ethics-first commitment. Strict multi-tenant row-level isolation and thorough PII redaction protect cloud users, while a transparent, opt-in telemetry system and completely localized database instances protect Standalone desktop users against non-consented data harvesting.
