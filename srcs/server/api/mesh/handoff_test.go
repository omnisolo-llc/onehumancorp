package mesh

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridHandoffAdapter_BroadcastHandoff(t *testing.T) {
	memoryService := NewMemoryMeshService()
	adapter := NewHybridHandoffAdapter(memoryService)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Roles:           []string{"system"},
		OrganizationID: "org-test",
	})

	subChan, err := memoryService.Subscribe(ctx)
	if err != nil {
		t.Fatalf("failed to subscribe to memory service: %v", err)
	}

	missionID := "mission-123"
	err = adapter.BroadcastHandoff(ctx, missionID)
	if err != nil {
		t.Fatalf("failed to broadcast handoff: %v", err)
	}

	select {
	case msg := <-subChan:
		var payload HandoffPayload
		if err := json.Unmarshal([]byte(msg), &payload); err != nil {
			t.Fatalf("failed to unmarshal payload: %v", err)
		}
		if payload.MissionID != missionID {
			t.Errorf("expected mission ID %s, got %s", missionID, payload.MissionID)
		}
		if payload.Action != "handoff" {
			t.Errorf("expected action handoff, got %s", payload.Action)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timed out waiting for broadcasted handoff message")
	}
}

func TestHybridHandoffAdapter_SubscribeHandoffs(t *testing.T) {
	memoryService := NewMemoryMeshService()
	adapter := NewHybridHandoffAdapter(memoryService)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Roles:           []string{"system"},
		OrganizationID: "org-test",
	})

	subChan, err := adapter.SubscribeHandoffs(ctx)
	if err != nil {
		t.Fatalf("failed to subscribe to handoffs: %v", err)
	}

	missionID := "mission-456"
	expectedMsg := `{"missionID":"` + missionID + `","action":"handoff"}`

	// Broadcast manually via service
	err = memoryService.BroadcastIntent(ctx, expectedMsg)
	if err != nil {
		t.Fatalf("failed to broadcast test intent: %v", err)
	}

	select {
	case msg := <-subChan:
		if msg != expectedMsg {
			t.Errorf("expected msg %s, got %s", expectedMsg, msg)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timed out waiting for subscribed handoff message")
	}
}
