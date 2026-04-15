import sys

def patch_file(filepath, old_content, new_content):
    with open(filepath, 'r') as f:
        content = f.read()

    if old_content in content:
        content = content.replace(old_content, new_content)
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Patched {filepath}")
    else:
        print(f"Could not find old content in {filepath}")


new_content = """	// Validate received payload
	if len(receivedPayloads) != 2 {
		t.Fatalf("expected 2 missions to be synced, got %d", len(receivedPayloads))
	}

	var m1Payload, m4Payload *SyncDaemonPayload
	for i := range receivedPayloads {
		if receivedPayloads[i].ID == "m1" {
			m1Payload = &receivedPayloads[i]
		}
		if receivedPayloads[i].ID == "m4" {
			m4Payload = &receivedPayloads[i]
		}
	}

	if m1Payload == nil || m4Payload == nil {
		t.Fatalf("expected m1 and m4 to be synced, got payloads: %v", receivedPayloads)
	}

	if m1Payload.Status != "PENDING" {
		t.Errorf("expected m1 status PENDING, got %s", m1Payload.Status)
	}

	if m4Payload.Status != "BURSTING" {
		t.Errorf("expected m4 status BURSTING, got %s", m4Payload.Status)
	}

	// Verify sanitization
	expectedPayload := `{"details":" email is [REDACTED_EMAIL]","task":"test-mission"}`
	if m1Payload.Payload != expectedPayload {
		t.Errorf("expected sanitized payload %q, got %q", expectedPayload, m1Payload.Payload)
	}

	expectedM4Payload := `{"task":"burst-mission"}`
	if m4Payload.Payload != expectedM4Payload {
		t.Errorf("expected sanitized payload %q, got %q", expectedM4Payload, m4Payload.Payload)
	}

	// Validate db status updated
	var synced bool
	err = sqlDB.QueryRow("SELECT synced_to_cloud FROM agent_missions WHERE id = 'm1'").Scan(&synced)
	if err != nil {
		t.Fatalf("failed to query m1 synced status: %v", err)
	}
	if !synced {
		t.Error("expected m1 to be synced_to_cloud = true")
	}

	var m4Synced bool
	err = sqlDB.QueryRow("SELECT synced_to_cloud FROM agent_missions WHERE id = 'm4'").Scan(&m4Synced)
	if err != nil {
		t.Fatalf("failed to query m4 synced status: %v", err)
	}
	if !m4Synced {
		t.Error("expected m4 to be synced_to_cloud = true")
	}"""

old_content = """	// Validate received payload
	if len(receivedPayloads) != 2 {
		t.Fatalf("expected 1 mission to be synced, got %d", len(receivedPayloads))
	}
	if receivedPayloads[0].ID != "m1" && receivedPayloads[1].ID != "m1" {
		t.Errorf("expected payload ID m1, got %s", receivedPayloads[0].ID)
	}
	if receivedPayloads[0].Status != "PENDING" {
		t.Errorf("expected status PENDING, got %s", receivedPayloads[0].Status)
	}

	// Verify sanitization
	expectedPayload := `{"details":" email is [REDACTED_EMAIL]","task":"test-mission"}`
	if receivedPayloads[0].Payload != expectedPayload {
		t.Errorf("expected sanitized payload %q, got %q", expectedPayload, receivedPayloads[0].Payload)
	}

	// Validate db status updated
	var synced bool
	err = sqlDB.QueryRow("SELECT synced_to_cloud FROM agent_missions WHERE id = 'm1'").Scan(&synced)
	if err != nil {
		t.Fatalf("failed to query m1 synced status: %v", err)
	}
	if !synced {
		t.Error("expected m1 to be synced_to_cloud = true")
	}"""


patch_file("srcs/server/orchestration/sync_daemon_test.go", old_content, new_content)
