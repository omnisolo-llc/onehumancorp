package telemetry_test

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/src/server/telemetry"
	_ "modernc.org/sqlite"
)

type TestUserData struct {
	Email string `json:"email"`
}

func TestStandaloneTelemetryPIIDoesNotMutateOriginal(t *testing.T) {
	// Setup in-memory SQLite DB
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec("CREATE TABLE telemetry_buffer (id INTEGER PRIMARY KEY AUTOINCREMENT, metric_type TEXT, payload TEXT)")
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	// Initialize buffer func
	telemetry.InitStandaloneBuffer(db)

	ctx := context.Background()

	// In Standalone Mode, BufferMetricFunc unmarshals the JSON payload into a map[string]interface{},
	// then calls RedactInterfacePII. Because it unmarshals into a fresh map,
	// there is no in-place mutation of the original struct passed by the caller.
	originalMap := map[string]interface{}{
		"email": "test@example.com",
	}

	payloadBytes, _ := json.Marshal(originalMap)
	err = telemetry.BufferMetricFunc(ctx, "test_metric", string(payloadBytes))
	if err != nil {
		t.Fatalf("BufferMetricFunc failed: %v", err)
	}

	// Verify original is untouched
	if originalMap["email"] != "test@example.com" {
		t.Errorf("Expected original map to be unmodified, got: %v", originalMap["email"])
	}

	// Read from db
	var payload string
	err = db.QueryRow("SELECT payload FROM telemetry_buffer LIMIT 1").Scan(&payload)
	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}

	var storedData map[string]interface{}
	json.Unmarshal([]byte(payload), &storedData)

	if storedData["email"] != "[REDACTED_EMAIL]" {
		t.Errorf("Expected email to be redacted in DB, got: %v", storedData["email"])
	}
}

func TestRedactInterfacePIIPointerNotMutated(t *testing.T) {
	// Let's directly test RedactInterfacePII with a pointer
	original := &TestUserData{Email: "test@example.com"}

	// Redact it
	redacted := telemetry.RedactInterfacePII(original)

	// Original must not be mutated!
	if original.Email != "test@example.com" {
		t.Errorf("Expected original.Email to be 'test@example.com', got '%s'", original.Email)
	}

	// Redacted must be correct
	redactedMap, ok := redacted.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected redacted to be map[string]interface{}, got %T", redacted)
	}

	if redactedMap["email"] != "[REDACTED_EMAIL]" {
		t.Errorf("Expected redacted email to be '[REDACTED_EMAIL]', got '%v'", redactedMap["email"])
	}
}
