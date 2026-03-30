import re

with open('srcs/orchestration/service_test.go', 'r') as f:
    content = f.read()

new_test = """func TestHub_DelegateMissionWithSIPDB(t *testing.T) {
	hub := NewHub()

	dbPath := filepath.Join(t.TempDir(), "chaos2.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("failed to create sip db: %v", err)
	}
	hub.SetSIPDB(db)

	hub.RegisterAgent(Agent{
		ID:             "swe-1",
		Name:           "SWE",
		Role:           "SOFTWARE_ENGINEER",
		OrganizationID: "org-1",
	})
	hub.RegisterAgent(Agent{
		ID:             "pm-1",
		Name:           "PM",
		Role:           "PRODUCT_MANAGER",
		OrganizationID: "org-1",
	})
	hub.OpenMeeting("m-1", []string{"swe-1", "pm-1"})

	msg := Message{ID: "msg-1", Content: "do work", MeetingID: "m-1"}
	err = hub.DelegateTask("pm-1", "swe-1", msg)
	if err != nil {
		t.Fatalf("expected nil err, got %v", err)
	}

	// Let the goroutine run
	time.Sleep(150 * time.Millisecond) // enough time for sipdb

	missions, _ := db.GetPendingMissions(context.Background(), "SOFTWARE_ENGINEER")
	if len(missions) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions))
	}
}"""

content = re.sub(r'func TestHub_DelegateMissionWithSIPDB\(t \*testing\.T\) \{.*?\n\}', new_test, content, flags=re.DOTALL)

with open('srcs/orchestration/service_test.go', 'w') as f:
    f.write(content)
