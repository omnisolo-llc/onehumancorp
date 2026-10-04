# NATS public metadata contract

This source-bound gate exercises the real registry's validation, IntegrationInstance construction, insertion and list methods, followed by the actual legacy service response bodies. It serializes those responses with the repository's generated protobuf types and Serde. The focused registry carrier retains only the instance map; unrelated provider initialization is outside this serialization test. The actual NATS client/provider modules and catalog are imported directly for separate provider metadata and loopback handshake cases.

The legacy IntegrationService is not registered in the current server. The mounted HTTP integration API is a different implementation and does not accept base_url. Passing this gate proves a reusable registry/service serialization boundary, not exposure through a currently mounted HTTP route or a complete provider connection.

Only public NATS URL userinfo is removed. Scheme, host, port, path, query and fragment remain available; ordinary URL configurations are unchanged. The original connection URL still goes to the connector. This does not add authentication support, reject normal endpoint configuration or certify NATS/JetStream authentication. Query-string secrets and post-connect/reconnect logging are not certified by this contract. Query strings are not treated as authentication settings here.

The owned loopback NATS endpoint rejects a real CONNECT/PING handshake with a synthetic canary. The configured URL, returned provider error and DEBUG logs must not disclose that canary; concurrent unrelated security logging must remain visible. No real credentials, live service, account grant or network setting is used or changed.

The manifest deliberately shares the verified mesh gate's dependency-feature graph, with OpenTelemetry for the actual integration client. This avoids building a separate narrowed feature variant. Prepare root-locked dependencies with fetch.sh, then run the offline test gate. A full server build, complete application test suite and live mounted API/provider verification remain separate requirements.
