import re

with open("srcs/server/orchestration/sip.go", "r") as f:
    content = f.read()

# Replace BufferMetric to sanitize before insert
old_buffer_metric = """func (s *SIPDB) BufferMetric(ctx context.Context, metricType string, payload string) error {
	return withSipRetry(ctx, func() error {
		_, err := s.db.Exec(ctx,
			"INSERT INTO telemetry_buffer (metric_type, payload, created_at, organization_id) VALUES ($1, $2, CURRENT_TIMESTAMP, $3)",
			metricType, payload, s.orgID,
		)
		return err
	})
}"""

new_buffer_metric = """func (s *SIPDB) BufferMetric(ctx context.Context, metricType string, payload string) error {
	// Sanitize payload before storing in the buffer
	var obj interface{}
	if err := json.Unmarshal([]byte(payload), &obj); err == nil {
		sanitizedObj := SanitizePayloadMap(obj)
		if b, err := json.Marshal(sanitizedObj); err == nil {
			payload = string(b)
		}
	} else {
		payload, _ = SanitizePayload(payload)
	}

	return withSipRetry(ctx, func() error {
		_, err := s.db.Exec(ctx,
			"INSERT INTO telemetry_buffer (metric_type, payload, created_at, organization_id) VALUES ($1, $2, CURRENT_TIMESTAMP, $3)",
			metricType, payload, s.orgID,
		)
		return err
	})
}"""

content = content.replace(old_buffer_metric, new_buffer_metric)

with open("srcs/server/orchestration/sip.go", "w") as f:
    f.write(content)

print("Patched BufferMetric in sip.go")
