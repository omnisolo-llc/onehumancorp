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
			task_id TEXT,
			dependencies TEXT,
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
			task_id TEXT,
			dependencies TEXT,
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
		TaskID:         "task-123",
		Dependencies:   []string{"dep1", "dep2"},
		Status:         "PENDING",
		Payload: map[string]interface{}{
			"rag_context": "sensitive ssn 123-45-6789 data with email test@example.com",
			"nested": map[string]interface{}{
				"hidden_ssn": "another 987-65-4321 inside nested",
			},
		},
	}

	payloadBytes, _ := json.Marshal(localMission.Payload)
	depsBytes, _ := json.Marshal(localMission.Dependencies)
	_, err := localDB.Exec(`
		INSERT INTO agent_missions (mission_id, organization_id, task_id, dependencies, status, payload)
		VALUES (?, ?, ?, ?, ?, ?)
	`, localMission.MissionID, localMission.OrganizationID, localMission.TaskID, string(depsBytes), localMission.Status, string(payloadBytes))
	if err != nil {
		t.Fatalf("Failed to insert into local DB: %v", err)
	}

	sync := NewDefaultMissionSynchronizer(cloudDB)

	err = sync.SyncLocalToCloud(context.Background(), localMission)
	if err != nil {
		t.Fatalf("SyncLocalToCloud failed: %v", err)
	}

	var syncedMissionID, syncedOrgID, syncedTaskID, syncedDeps, syncedStatus, syncedPayload string
	err = cloudDB.QueryRow(`
		SELECT mission_id, organization_id, task_id, dependencies, status, payload
		FROM agent_missions
		WHERE mission_id = ?
	`, localMission.MissionID).Scan(&syncedMissionID, &syncedOrgID, &syncedTaskID, &syncedDeps, &syncedStatus, &syncedPayload)
	if err != nil {
		t.Fatalf("Failed to query cloud DB: %v", err)
	}

	if syncedMissionID != localMission.MissionID {
		t.Errorf("Expected mission ID %s, got %s", localMission.MissionID, syncedMissionID)
	}
	if syncedOrgID != localMission.OrganizationID {
		t.Errorf("Expected organization ID %s, got %s", localMission.OrganizationID, syncedOrgID)
	}
	if syncedTaskID != localMission.TaskID {
		t.Errorf("Expected task ID %s, got %s", localMission.TaskID, syncedTaskID)
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

	nestedMap, ok := payloadMap["nested"].(map[string]interface{})
	if !ok {
		t.Errorf("Expected scrubbed payload, but nested missing")
	} else {
		hiddenSsn, ok := nestedMap["hidden_ssn"].(string)
		if !ok || strings.Contains(hiddenSsn, "987-65-4321") {
			t.Errorf("Expected payload to have scrubbed nested SSN, got %s", hiddenSsn)
		}
	}
}
