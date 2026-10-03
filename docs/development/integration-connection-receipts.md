# Integration connection feedback

The integration cards must not infer a connection from prompt text, blank credentials, a pending HTTP response, or a contradictory success/error body. Unsupported generic card flows use their existing unavailable message and collect no credential through a browser prompt. The separate verified business connection component is unchanged.

Twilio credentials and channel selection are required in every runtime, including the production UI. A submitted connection is shown as connected only after HTTP 200 and an unambiguous usable connection receipt. Existing flat and nested receipt shapes remain supported; contradictory outer or nested status/error fields are rejected. Integration-list errors also cannot populate connected cards.

Focused tests render the actual component with isolated HTTP and platform boundaries, including a production NODE_ENV case. Existing genuine receipt success coverage remains. The browser journeys assert the configured-unavailable state and absence of provider requests rather than the former fabricated success. No live credentials, connection grant, provider payment, or message is part of verification.

This is a feedback-correctness repair. It does not implement unsupported provider verification, credential storage, cross-view session lifetime handling, cancellation, or unknown-outcome reconciliation. Full browser execution and supported provider acceptance remain separate release gates.
