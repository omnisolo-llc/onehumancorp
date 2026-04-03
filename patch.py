import re

with open("srcs/server/orchestration/sync_daemon.go", "r") as f:
    content = f.read()

# Replace RedactPIIMap
old_sanitize = """		var parsedPayload map[string]interface{}
		if err := json.Unmarshal([]byte(payloadData), &parsedPayload); err == nil {
			parsedPayload = telemetry.RedactPIIMap(parsedPayload)
			if redactedBytes, err := json.Marshal(parsedPayload); err == nil {
				payloadData = string(redactedBytes)
			}
		} else {
			payloadData = telemetry.RedactPII(payloadData)
		}"""

new_sanitize = """		var sanitizeRecursively func(data interface{}) interface{}
		sanitizeRecursively = func(data interface{}) interface{} {
			switch v := data.(type) {
			case string:
				return telemetry.RedactPII(v)
			case map[string]interface{}:
				for key, val := range v {
					v[key] = sanitizeRecursively(val)
				}
				return v
			case []interface{}:
				for i, val := range v {
					v[i] = sanitizeRecursively(val)
				}
				return v
			default:
				return v
			}
		}

		var parsedPayload map[string]interface{}
		if err := json.Unmarshal([]byte(payloadData), &parsedPayload); err == nil {
			parsedIface := sanitizeRecursively(parsedPayload)
			if redactedBytes, err := json.Marshal(parsedIface); err == nil {
				payloadData = string(redactedBytes)
			}
		} else {
			payloadData = telemetry.RedactPII(payloadData)
		}"""
content = content.replace(old_sanitize, new_sanitize)

# Replace UPDATE loop
old_update = """	// Mark as synced
	for _, id := range ids {
		_, err := d.dbWrapper.Exec(ctx, "UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1", id)
		if err != nil {
			slog.Error("sync_daemon: failed to update agent_missions status", "id", id, "error", err)
		}
	}"""
new_update = """	// Mark as synced
	if len(ids) > 0 {
		idList := ""
		for i, id := range ids {
			if i > 0 {
				idList += ","
			}
			idList += fmt.Sprintf("'%s'", id)
		}
		query := fmt.Sprintf("UPDATE agent_missions SET synced_to_cloud = true WHERE id IN (%s)", idList)
		_, err := d.dbWrapper.Exec(ctx, query)
		if err != nil {
			slog.Error("sync_daemon: failed to update agent_missions status in bulk", "error", err)
		}
	}"""
content = content.replace(old_update, new_update)

# Replace Auth
old_auth = """	if authHeader := os.Getenv("OHC_CLOUD_API_KEY"); authHeader != "" {
		req.Header.Set("Authorization", "Bearer "+authHeader)
	} else if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}"""
new_auth = """	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}"""
content = content.replace(old_auth, new_auth)

with open("srcs/server/orchestration/sync_daemon.go", "w") as f:
    f.write(content)
