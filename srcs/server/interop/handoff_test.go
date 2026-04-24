package interop

import (
	"context"
	"os"
	"testing"
	"github.com/redis/rueidis"
)

func TestMemoryHandoff_ExportImport(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	handoff, err := NewHandoffProtocol()
	if err != nil {
		t.Fatalf("Failed to create memory handoff: %v", err)
	}

	ctx := context.Background()
	sessionID := "test_session_1"

	state := &State{
		ID:    "state_1",
		Owner: "agent_A",
		Data: map[string]interface{}{
			"key1": "value1",
			"key2": 42.0,
		},
	}

	err = handoff.ExportState(ctx, sessionID, state)
	if err != nil {
		t.Fatalf("ExportState failed: %v", err)
	}

	importedState, err := handoff.ImportState(ctx, sessionID)
	if err != nil {
		t.Fatalf("ImportState failed: %v", err)
	}

	if importedState.ID != state.ID {
		t.Errorf("Expected state ID %s, got %s", state.ID, importedState.ID)
	}
	if importedState.Owner != state.Owner {
		t.Errorf("Expected owner %s, got %s", state.Owner, importedState.Owner)
	}
	if importedState.Data["key1"] != "value1" || importedState.Data["key2"] != 42.0 {
		t.Errorf("Expected data to match, got %v", importedState.Data)
	}
}

func TestMemoryHandoff_NilState(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	handoff, _ := NewHandoffProtocol()
	ctx := context.Background()

	err := handoff.ExportState(ctx, "session1", nil)
	if err == nil {
		t.Error("Expected error when exporting nil state")
	}
}

func TestMemoryHandoff_NotFound(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	handoff, _ := NewHandoffProtocol()
	ctx := context.Background()

	_, err := handoff.ImportState(ctx, "unknown_session")
	if err == nil {
		t.Error("Expected error when importing unknown session")
	}
}

func TestCloudHandoff_Fallback(t *testing.T) {
	os.Setenv("REDIS_URL", "invalid_url")
	os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("REDIS_URL")

	handoff, err := NewHandoffProtocol()
	if err != nil {
		t.Fatalf("Expected fallback to succeed, got error: %v", err)
	}

	if _, ok := handoff.(*MemoryHandoff); !ok {
		t.Errorf("Expected fallback to MemoryHandoff, got %T", handoff)
	}
}

func TestCloudHandoff_ExportImport(t *testing.T) {
	// Initialize a mock rueidis client
	mockClient, err := rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{"127.0.0.1:6379"}})
	if err != nil {
		t.Skip("Skipping cloud test because redis is not available locally")
	}
	defer mockClient.Close()

	err = mockClient.Do(context.Background(), mockClient.B().Ping().Build()).Error()
	if err != nil {
		t.Skip("Skipping cloud test because redis is not responding")
	}

	handoff := &CloudHandoff{client: mockClient}
	ctx := context.Background()
	sessionID := "test_cloud_session"

	state := &State{
		ID:    "state_cloud_1",
		Owner: "agent_C",
		Data: map[string]interface{}{
			"foo": "bar",
			"baz": 99.0,
		},
	}

	err = handoff.ExportState(ctx, sessionID, state)
	if err != nil {
		t.Fatalf("ExportState failed: %v", err)
	}

	importedState, err := handoff.ImportState(ctx, sessionID)
	if err != nil {
		t.Fatalf("ImportState failed: %v", err)
	}

	if importedState.ID != state.ID {
		t.Errorf("Expected state ID %s, got %s", state.ID, importedState.ID)
	}
	if importedState.Owner != state.Owner {
		t.Errorf("Expected owner %s, got %s", state.Owner, importedState.Owner)
	}
	if importedState.Data["foo"] != "bar" || importedState.Data["baz"] != 99.0 {
		t.Errorf("Expected data to match, got %v", importedState.Data)
	}

	// Test nil export
	if err := handoff.ExportState(ctx, sessionID, nil); err == nil {
		t.Errorf("Expected error when exporting nil state to cloud handoff")
	}

	// Test not found
	if _, err := handoff.ImportState(ctx, "non_existent_session"); err == nil {
		t.Errorf("Expected error when importing non-existent session from cloud handoff")
	}
}
