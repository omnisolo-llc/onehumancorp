package hybrid_sync

import (
	"context"
	"database/sql"
	"encoding/json"
	"strings"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func setupMockCloudDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open mock cloud db: %v", err)
	}

	createTableQuery := `
		CREATE TABLE agent_missions (
			mission_id TEXT PRIMARY KEY,
			organization_id TEXT,
			status TEXT,
			payload TEXT
		)
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create agent_missions table: %v", err)
	}

	return db
}

func setupMockLocalDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open mock local db: %v", err)
	}

	createTableQuery := `
		CREATE TABLE agent_missions (
			mission_id TEXT PRIMARY KEY,
			organization_id TEXT,
			status TEXT,
			payload TEXT
		)
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create agent_missions table: %v", err)
	}

	return db
}

func TestSyncLocalToCloud(t *testing.T) {
	localDB := setupMockLocalDB(t)
	defer localDB.Close()

	cloudDB := setupMockCloudDB(t)
	defer cloudDB.Close()

	localMission := &AgentMission{
		MissionID:      "123e4567-e89b-12d3-a456-426614174000",
		OrganizationID: "org-1",
		Status:         "PENDING",
		Payload: map[string]interface{}{
			"rag_context": "sensitive ssn 123-45-6789 data with email test@example.com",
			"other":       "normal text",
		},
	}

	payloadBytes, _ := json.Marshal(localMission.Payload)
	_, err := localDB.Exec(`
		INSERT INTO agent_missions (mission_id, organization_id, status, payload)
		VALUES (?, ?, ?, ?)
	`, localMission.MissionID, localMission.OrganizationID, localMission.Status, string(payloadBytes))
	if err != nil {
		t.Fatalf("Failed to insert into local DB: %v", err)
	}

	sync := NewDefaultMissionSynchronizer(cloudDB)

	err = sync.SyncLocalToCloud(context.Background(), localMission)
	if err != nil {
		t.Fatalf("SyncLocalToCloud failed: %v", err)
	}

	var syncedMissionID, syncedOrgID, syncedStatus, syncedPayload string
	err = cloudDB.QueryRow(`
		SELECT mission_id, organization_id, status, payload
		FROM agent_missions
		WHERE mission_id = ?
	`, localMission.MissionID).Scan(&syncedMissionID, &syncedOrgID, &syncedStatus, &syncedPayload)
	if err != nil {
		t.Fatalf("Failed to query cloud DB: %v", err)
	}

	if syncedMissionID != localMission.MissionID {
		t.Errorf("Expected mission ID %s, got %s", localMission.MissionID, syncedMissionID)
	}
	if syncedOrgID != localMission.OrganizationID {
		t.Errorf("Expected organization ID %s, got %s", localMission.OrganizationID, syncedOrgID)
	}
	if syncedStatus != localMission.Status {
		t.Errorf("Expected status %s, got %s", localMission.Status, syncedStatus)
	}

	var payloadMap map[string]interface{}
	err = json.Unmarshal([]byte(syncedPayload), &payloadMap)
	if err != nil {
		t.Fatalf("Failed to unmarshal payload: %v", err)
	}

	ragContext, ok := payloadMap["rag_context"].(string)
	if !ok {
		t.Errorf("Expected scrubbed payload, but rag_context missing")
	} else {
		if strings.Contains(ragContext, "test@example.com") {
			t.Errorf("Expected payload to have scrubbed email, got %s", ragContext)
		}
		if strings.Contains(ragContext, "123-45-6789") {
			t.Errorf("Expected payload to have scrubbed SSN, got %s", ragContext)
		}
	}
}
