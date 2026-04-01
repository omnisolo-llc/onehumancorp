package orchestration

import (
	"context"
	"os"
	"testing"
	"time"
)

func TestHub_StandaloneMode_Throttling(t *testing.T) {
	// Force standalone mode
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	hub := NewHub()
	if hub.throttleSem == nil {
		t.Fatalf("expected throttleSem to be initialized in Standalone Mode")
	}
	if cap(hub.throttleSem) != 5 {
		t.Fatalf("expected throttle capacity to be 5, got %d", cap(hub.throttleSem))
	}
}

func TestHub_CloudMode_NoThrottling(t *testing.T) {
	// Force cloud mode
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	hub := NewHub()
	if hub.throttleSem != nil {
		t.Fatalf("expected throttleSem to be nil in Cloud Mode, got non-nil")
	}
}

func TestDelegateTask_WithThrottling(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	hub := NewHub()
	db, err := NewSIPDB("file:dummy_throttle.db?mode=memory&cache=shared")
	if err != nil {
		t.Fatalf("failed to create sip db: %v", err)
	}
	hub.SetSIPDB(db)

	hub.RegisterAgent(Agent{
		ID:             "router-1",
		Name:           "Router",
		Role:           "ROUTER",
		OrganizationID: "org-1",
	})
	hub.RegisterAgent(Agent{
		ID:             "worker-1",
		Name:           "Worker",
		Role:           "WORKER",
		OrganizationID: "org-1",
	})
	hub.OpenMeeting("m-throttle", []string{"router-1", "worker-1"})

	// Since capacity is 5, dispatching 10 should not block the main thread, but they will be throttled in the background
	for i := 0; i < 10; i++ {
		msg := Message{ID: "msg-" + string(rune(i)), Content: "throttle test", MeetingID: "m-throttle"}
		err = hub.DelegateTask("router-1", "worker-1", msg)
		if err != nil {
			t.Fatalf("DelegateTask failed: %v", err)
		}
	}

	// Wait enough time for background goroutines to finish
	time.Sleep(200 * time.Millisecond)

	missions, _ := db.GetPendingMissions(context.Background(), "WORKER")
	if len(missions) != 10 {
		t.Fatalf("expected 10 missions, got %d", len(missions))
	}
}
