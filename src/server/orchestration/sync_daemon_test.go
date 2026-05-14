package orchestration

import (
	"context"
	"database/sql"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func ClearSemaphore() {
	for {
		select {
		case <-throttleSemaphore:
		default:
			return
		}
	}
}

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open test database: %v", err)
	}

	createTableQuery := `
	CREATE TABLE agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL,
		payload BLOB,
		synced_to_cloud BOOLEAN DEFAULT FALSE,
		sync_error TEXT,
		last_synced_at TIMESTAMP
	);
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_Success(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-2', 'BURSTING', '{"key": "value2"}', FALSE);
	`
	_, err := db.Exec(insertDataQuery)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/v1/sync" {
			t.Errorf("Expected path '/api/v1/sync', got %s", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("Expected POST method, got %s", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		if !strings.Contains(string(body), "value") {
			t.Errorf("Expected body to contain 'value', got %s", string(body))
		}

		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	daemon := NewHybridMCPRAGDaemon(db, server.URL)

	err = daemon.SyncPendingMissions(context.Background())
	if err != nil {
		t.Fatalf("SyncPendingMissions failed: %v", err)
	}

	rows, err := db.Query("SELECT id, synced_to_cloud, sync_error FROM agent_missions")
	if err != nil {
		t.Fatalf("Failed to query database after sync: %v", err)
	}
	defer rows.Close()

	for rows.Next() {
		var id string
		var synced bool
		var syncError sql.NullString
		if err := rows.Scan(&id, &synced, &syncError); err != nil {
			t.Fatalf("Failed to scan row: %v", err)
		}

		if !synced {
			t.Errorf("Mission %s should be synced", id)
		}
		if syncError.Valid {
			t.Errorf("Mission %s should not have an error, got %v", id, syncError.String)
		}
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_HTTPError(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	_, err := db.Exec("INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-err', 'CLOUD_ESCALATION', '{}', FALSE)")
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer server.Close()

	daemon := NewHybridMCPRAGDaemon(db, server.URL)

	err = daemon.SyncPendingMissions(context.Background())
	if err != nil {
		t.Fatalf("SyncPendingMissions failed: %v", err)
	}

	var synced bool
	var syncError sql.NullString
	err = db.QueryRow("SELECT synced_to_cloud, sync_error FROM agent_missions WHERE id = 'mission-err'").Scan(&synced, &syncError)
	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}

	if synced {
		t.Errorf("Mission should not be synced")
	}
	if !syncError.Valid || !strings.Contains(syncError.String, "cloud API returned HTTP 500") {
		t.Errorf("Expected HTTP 500 error, got %v", syncError.String)
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_Cooldown(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	// Insert test data with recent errors
	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud, sync_error, last_synced_at) VALUES
	('mission-error-1', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE, 'API Timeout', datetime('now', '-1 minutes')),
	('mission-error-2', 'CLOUD_ESCALATION', '{"key": "value2"}', FALSE, 'HTTP 500', datetime('now', '-6 minutes'));
	`
	_, err := db.Exec(insertDataQuery)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	daemon := NewHybridMCPRAGDaemon(db, server.URL)

	err = daemon.SyncPendingMissions(context.Background())
	if err != nil {
		t.Fatalf("SyncPendingMissions failed: %v", err)
	}

	rows, err := db.Query("SELECT id, synced_to_cloud FROM agent_missions")
	if err != nil {
		t.Fatalf("Failed to query database after sync: %v", err)
	}
	defer rows.Close()

	syncedMap := make(map[string]bool)
	for rows.Next() {
		var id string
		var synced bool
		if err := rows.Scan(&id, &synced); err != nil {
			t.Fatalf("Failed to scan row: %v", err)
		}
		syncedMap[id] = synced
	}

	if syncedMap["mission-error-1"] != false {
		t.Errorf("Expected mission-error-1 to NOT be synced due to cooldown")
	}
	if syncedMap["mission-error-2"] != true {
		t.Errorf("Expected mission-error-2 to be synced after cooldown expired")
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_TableDriven(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	tests := []struct {
		name           string
		missions       []string // SQL insert
		statusCode     int
		expectedSync   map[string]bool
		expectedError  map[string]string
	}{
		{
			name: "Scenario 1 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-1', 'CLOUD_ESCALATION', '{\"data\":1}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-1": false},
			expectedError: map[string]string{"mission-td-1": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 2 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-2', 'CLOUD_ESCALATION', '{\"data\":2}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-2": true},
			expectedError: map[string]string{"mission-td-2": ""},
		},
		{
			name: "Scenario 3 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-3', 'CLOUD_ESCALATION', '{\"data\":3}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-3": false},
			expectedError: map[string]string{"mission-td-3": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 4 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-4', 'CLOUD_ESCALATION', '{\"data\":4}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-4": true},
			expectedError: map[string]string{"mission-td-4": ""},
		},
		{
			name: "Scenario 5 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-5', 'CLOUD_ESCALATION', '{\"data\":5}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-5": false},
			expectedError: map[string]string{"mission-td-5": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 6 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-6', 'CLOUD_ESCALATION', '{\"data\":6}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-6": true},
			expectedError: map[string]string{"mission-td-6": ""},
		},
		{
			name: "Scenario 7 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-7', 'CLOUD_ESCALATION', '{\"data\":7}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-7": false},
			expectedError: map[string]string{"mission-td-7": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 8 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-8', 'CLOUD_ESCALATION', '{\"data\":8}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-8": true},
			expectedError: map[string]string{"mission-td-8": ""},
		},
		{
			name: "Scenario 9 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-9', 'CLOUD_ESCALATION', '{\"data\":9}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-9": false},
			expectedError: map[string]string{"mission-td-9": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 10 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-10', 'CLOUD_ESCALATION', '{\"data\":10}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-10": true},
			expectedError: map[string]string{"mission-td-10": ""},
		},
		{
			name: "Scenario 11 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-11', 'CLOUD_ESCALATION', '{\"data\":11}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-11": false},
			expectedError: map[string]string{"mission-td-11": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 12 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-12', 'CLOUD_ESCALATION', '{\"data\":12}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-12": true},
			expectedError: map[string]string{"mission-td-12": ""},
		},
		{
			name: "Scenario 13 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-13', 'CLOUD_ESCALATION', '{\"data\":13}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-13": false},
			expectedError: map[string]string{"mission-td-13": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 14 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-14', 'CLOUD_ESCALATION', '{\"data\":14}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-14": true},
			expectedError: map[string]string{"mission-td-14": ""},
		},
		{
			name: "Scenario 15 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-15', 'CLOUD_ESCALATION', '{\"data\":15}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-15": false},
			expectedError: map[string]string{"mission-td-15": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 16 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-16', 'CLOUD_ESCALATION', '{\"data\":16}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-16": true},
			expectedError: map[string]string{"mission-td-16": ""},
		},
		{
			name: "Scenario 17 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-17', 'CLOUD_ESCALATION', '{\"data\":17}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-17": false},
			expectedError: map[string]string{"mission-td-17": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 18 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-18', 'CLOUD_ESCALATION', '{\"data\":18}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-18": true},
			expectedError: map[string]string{"mission-td-18": ""},
		},
		{
			name: "Scenario 19 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-19', 'CLOUD_ESCALATION', '{\"data\":19}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-19": false},
			expectedError: map[string]string{"mission-td-19": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 20 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-20', 'CLOUD_ESCALATION', '{\"data\":20}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-20": true},
			expectedError: map[string]string{"mission-td-20": ""},
		},
		{
			name: "Scenario 21 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-21', 'CLOUD_ESCALATION', '{\"data\":21}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-21": false},
			expectedError: map[string]string{"mission-td-21": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 22 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-22', 'CLOUD_ESCALATION', '{\"data\":22}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-22": true},
			expectedError: map[string]string{"mission-td-22": ""},
		},
		{
			name: "Scenario 23 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-23', 'CLOUD_ESCALATION', '{\"data\":23}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-23": false},
			expectedError: map[string]string{"mission-td-23": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 24 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-24', 'CLOUD_ESCALATION', '{\"data\":24}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-24": true},
			expectedError: map[string]string{"mission-td-24": ""},
		},
		{
			name: "Scenario 25 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-25', 'CLOUD_ESCALATION', '{\"data\":25}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-25": false},
			expectedError: map[string]string{"mission-td-25": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 26 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-26', 'CLOUD_ESCALATION', '{\"data\":26}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-26": true},
			expectedError: map[string]string{"mission-td-26": ""},
		},
		{
			name: "Scenario 27 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-27', 'CLOUD_ESCALATION', '{\"data\":27}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-27": false},
			expectedError: map[string]string{"mission-td-27": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 28 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-28', 'CLOUD_ESCALATION', '{\"data\":28}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-28": true},
			expectedError: map[string]string{"mission-td-28": ""},
		},
		{
			name: "Scenario 29 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-29', 'CLOUD_ESCALATION', '{\"data\":29}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-29": false},
			expectedError: map[string]string{"mission-td-29": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 30 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-30', 'CLOUD_ESCALATION', '{\"data\":30}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-30": true},
			expectedError: map[string]string{"mission-td-30": ""},
		},
		{
			name: "Scenario 31 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-31', 'CLOUD_ESCALATION', '{\"data\":31}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-31": false},
			expectedError: map[string]string{"mission-td-31": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 32 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-32', 'CLOUD_ESCALATION', '{\"data\":32}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-32": true},
			expectedError: map[string]string{"mission-td-32": ""},
		},
		{
			name: "Scenario 33 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-33', 'CLOUD_ESCALATION', '{\"data\":33}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-33": false},
			expectedError: map[string]string{"mission-td-33": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 34 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-34', 'CLOUD_ESCALATION', '{\"data\":34}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-34": true},
			expectedError: map[string]string{"mission-td-34": ""},
		},
		{
			name: "Scenario 35 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-35', 'CLOUD_ESCALATION', '{\"data\":35}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-35": false},
			expectedError: map[string]string{"mission-td-35": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 36 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-36', 'CLOUD_ESCALATION', '{\"data\":36}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-36": true},
			expectedError: map[string]string{"mission-td-36": ""},
		},
		{
			name: "Scenario 37 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-37', 'CLOUD_ESCALATION', '{\"data\":37}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-37": false},
			expectedError: map[string]string{"mission-td-37": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 38 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-38', 'CLOUD_ESCALATION', '{\"data\":38}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-38": true},
			expectedError: map[string]string{"mission-td-38": ""},
		},
		{
			name: "Scenario 39 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-39', 'CLOUD_ESCALATION', '{\"data\":39}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-39": false},
			expectedError: map[string]string{"mission-td-39": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 40 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-40', 'CLOUD_ESCALATION', '{\"data\":40}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-40": true},
			expectedError: map[string]string{"mission-td-40": ""},
		},
		{
			name: "Scenario 41 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-41', 'CLOUD_ESCALATION', '{\"data\":41}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-41": false},
			expectedError: map[string]string{"mission-td-41": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 42 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-42', 'CLOUD_ESCALATION', '{\"data\":42}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-42": true},
			expectedError: map[string]string{"mission-td-42": ""},
		},
		{
			name: "Scenario 43 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-43', 'CLOUD_ESCALATION', '{\"data\":43}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-43": false},
			expectedError: map[string]string{"mission-td-43": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 44 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-44', 'CLOUD_ESCALATION', '{\"data\":44}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-44": true},
			expectedError: map[string]string{"mission-td-44": ""},
		},
		{
			name: "Scenario 45 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-45', 'CLOUD_ESCALATION', '{\"data\":45}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-45": false},
			expectedError: map[string]string{"mission-td-45": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 46 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-46', 'CLOUD_ESCALATION', '{\"data\":46}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-46": true},
			expectedError: map[string]string{"mission-td-46": ""},
		},
		{
			name: "Scenario 47 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-47', 'CLOUD_ESCALATION', '{\"data\":47}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-47": false},
			expectedError: map[string]string{"mission-td-47": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 48 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-48', 'CLOUD_ESCALATION', '{\"data\":48}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-48": true},
			expectedError: map[string]string{"mission-td-48": ""},
		},
		{
			name: "Scenario 49 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-49', 'CLOUD_ESCALATION', '{\"data\":49}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-49": false},
			expectedError: map[string]string{"mission-td-49": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 50 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-50', 'CLOUD_ESCALATION', '{\"data\":50}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-50": true},
			expectedError: map[string]string{"mission-td-50": ""},
		},
		{
			name: "Scenario 51 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-51', 'CLOUD_ESCALATION', '{\"data\":51}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-51": false},
			expectedError: map[string]string{"mission-td-51": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 52 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-52', 'CLOUD_ESCALATION', '{\"data\":52}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-52": true},
			expectedError: map[string]string{"mission-td-52": ""},
		},
		{
			name: "Scenario 53 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-53', 'CLOUD_ESCALATION', '{\"data\":53}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-53": false},
			expectedError: map[string]string{"mission-td-53": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 54 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-54', 'CLOUD_ESCALATION', '{\"data\":54}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-54": true},
			expectedError: map[string]string{"mission-td-54": ""},
		},
		{
			name: "Scenario 55 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-55', 'CLOUD_ESCALATION', '{\"data\":55}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-55": false},
			expectedError: map[string]string{"mission-td-55": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 56 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-56', 'CLOUD_ESCALATION', '{\"data\":56}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-56": true},
			expectedError: map[string]string{"mission-td-56": ""},
		},
		{
			name: "Scenario 57 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-57', 'CLOUD_ESCALATION', '{\"data\":57}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-57": false},
			expectedError: map[string]string{"mission-td-57": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 58 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-58', 'CLOUD_ESCALATION', '{\"data\":58}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-58": true},
			expectedError: map[string]string{"mission-td-58": ""},
		},
		{
			name: "Scenario 59 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-59', 'CLOUD_ESCALATION', '{\"data\":59}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-59": false},
			expectedError: map[string]string{"mission-td-59": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 60 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-60', 'CLOUD_ESCALATION', '{\"data\":60}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-60": true},
			expectedError: map[string]string{"mission-td-60": ""},
		},
		{
			name: "Scenario 61 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-61', 'CLOUD_ESCALATION', '{\"data\":61}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-61": false},
			expectedError: map[string]string{"mission-td-61": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 62 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-62', 'CLOUD_ESCALATION', '{\"data\":62}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-62": true},
			expectedError: map[string]string{"mission-td-62": ""},
		},
		{
			name: "Scenario 63 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-63', 'CLOUD_ESCALATION', '{\"data\":63}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-63": false},
			expectedError: map[string]string{"mission-td-63": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 64 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-64', 'CLOUD_ESCALATION', '{\"data\":64}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-64": true},
			expectedError: map[string]string{"mission-td-64": ""},
		},
		{
			name: "Scenario 65 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-65', 'CLOUD_ESCALATION', '{\"data\":65}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-65": false},
			expectedError: map[string]string{"mission-td-65": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 66 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-66', 'CLOUD_ESCALATION', '{\"data\":66}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-66": true},
			expectedError: map[string]string{"mission-td-66": ""},
		},
		{
			name: "Scenario 67 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-67', 'CLOUD_ESCALATION', '{\"data\":67}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-67": false},
			expectedError: map[string]string{"mission-td-67": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 68 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-68', 'CLOUD_ESCALATION', '{\"data\":68}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-68": true},
			expectedError: map[string]string{"mission-td-68": ""},
		},
		{
			name: "Scenario 69 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-69', 'CLOUD_ESCALATION', '{\"data\":69}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-69": false},
			expectedError: map[string]string{"mission-td-69": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 70 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-70', 'CLOUD_ESCALATION', '{\"data\":70}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-70": true},
			expectedError: map[string]string{"mission-td-70": ""},
		},
		{
			name: "Scenario 71 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-71', 'CLOUD_ESCALATION', '{\"data\":71}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-71": false},
			expectedError: map[string]string{"mission-td-71": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 72 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-72', 'CLOUD_ESCALATION', '{\"data\":72}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-72": true},
			expectedError: map[string]string{"mission-td-72": ""},
		},
		{
			name: "Scenario 73 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-73', 'CLOUD_ESCALATION', '{\"data\":73}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-73": false},
			expectedError: map[string]string{"mission-td-73": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 74 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-74', 'CLOUD_ESCALATION', '{\"data\":74}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-74": true},
			expectedError: map[string]string{"mission-td-74": ""},
		},
		{
			name: "Scenario 75 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-75', 'CLOUD_ESCALATION', '{\"data\":75}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-75": false},
			expectedError: map[string]string{"mission-td-75": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 76 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-76', 'CLOUD_ESCALATION', '{\"data\":76}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-76": true},
			expectedError: map[string]string{"mission-td-76": ""},
		},
		{
			name: "Scenario 77 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-77', 'CLOUD_ESCALATION', '{\"data\":77}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-77": false},
			expectedError: map[string]string{"mission-td-77": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 78 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-78', 'CLOUD_ESCALATION', '{\"data\":78}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-78": true},
			expectedError: map[string]string{"mission-td-78": ""},
		},
		{
			name: "Scenario 79 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-79', 'CLOUD_ESCALATION', '{\"data\":79}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-79": false},
			expectedError: map[string]string{"mission-td-79": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 80 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-80', 'CLOUD_ESCALATION', '{\"data\":80}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-80": true},
			expectedError: map[string]string{"mission-td-80": ""},
		},
		{
			name: "Scenario 81 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-81', 'CLOUD_ESCALATION', '{\"data\":81}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-81": false},
			expectedError: map[string]string{"mission-td-81": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 82 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-82', 'CLOUD_ESCALATION', '{\"data\":82}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-82": true},
			expectedError: map[string]string{"mission-td-82": ""},
		},
		{
			name: "Scenario 83 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-83', 'CLOUD_ESCALATION', '{\"data\":83}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-83": false},
			expectedError: map[string]string{"mission-td-83": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 84 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-84', 'CLOUD_ESCALATION', '{\"data\":84}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-84": true},
			expectedError: map[string]string{"mission-td-84": ""},
		},
		{
			name: "Scenario 85 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-85', 'CLOUD_ESCALATION', '{\"data\":85}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-85": false},
			expectedError: map[string]string{"mission-td-85": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 86 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-86', 'CLOUD_ESCALATION', '{\"data\":86}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-86": true},
			expectedError: map[string]string{"mission-td-86": ""},
		},
		{
			name: "Scenario 87 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-87', 'CLOUD_ESCALATION', '{\"data\":87}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-87": false},
			expectedError: map[string]string{"mission-td-87": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 88 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-88', 'CLOUD_ESCALATION', '{\"data\":88}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-88": true},
			expectedError: map[string]string{"mission-td-88": ""},
		},
		{
			name: "Scenario 89 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-89', 'CLOUD_ESCALATION', '{\"data\":89}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-89": false},
			expectedError: map[string]string{"mission-td-89": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 90 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-90', 'CLOUD_ESCALATION', '{\"data\":90}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-90": true},
			expectedError: map[string]string{"mission-td-90": ""},
		},
		{
			name: "Scenario 91 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-91', 'CLOUD_ESCALATION', '{\"data\":91}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-91": false},
			expectedError: map[string]string{"mission-td-91": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 92 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-92', 'CLOUD_ESCALATION', '{\"data\":92}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-92": true},
			expectedError: map[string]string{"mission-td-92": ""},
		},
		{
			name: "Scenario 93 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-93', 'CLOUD_ESCALATION', '{\"data\":93}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-93": false},
			expectedError: map[string]string{"mission-td-93": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 94 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-94', 'CLOUD_ESCALATION', '{\"data\":94}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-94": true},
			expectedError: map[string]string{"mission-td-94": ""},
		},
		{
			name: "Scenario 95 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-95', 'CLOUD_ESCALATION', '{\"data\":95}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-95": false},
			expectedError: map[string]string{"mission-td-95": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 96 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-96', 'CLOUD_ESCALATION', '{\"data\":96}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-96": true},
			expectedError: map[string]string{"mission-td-96": ""},
		},
		{
			name: "Scenario 97 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-97', 'CLOUD_ESCALATION', '{\"data\":97}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-97": false},
			expectedError: map[string]string{"mission-td-97": "cloud API returned HTTP 500"},
		},
		{
			name: "Scenario 98 - Status 200",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-98', 'CLOUD_ESCALATION', '{\"data\":98}', FALSE)",
			},
			statusCode: 200,
			expectedSync: map[string]bool{"mission-td-98": true},
			expectedError: map[string]string{"mission-td-98": ""},
		},
		{
			name: "Scenario 99 - Status 500",
			missions: []string{
				"INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-td-99', 'CLOUD_ESCALATION', '{\"data\":99}', FALSE)",
			},
			statusCode: 500,
			expectedSync: map[string]bool{"mission-td-99": false},
			expectedError: map[string]string{"mission-td-99": "cloud API returned HTTP 500"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			db := setupTestDB(t)
			defer db.Close()

			for _, q := range tt.missions {
				if _, err := db.Exec(q); err != nil {
					t.Fatalf("Failed to insert test data: %v", err)
				}
			}

			server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(tt.statusCode)
			}))
			defer server.Close()

			daemon := NewHybridMCPRAGDaemon(db, server.URL)
			daemon.mode = tt.name

			_ = daemon.SyncPendingMissions(context.Background())

			for id, expectedSynced := range tt.expectedSync {
				var synced bool
				var syncError sql.NullString
				err := db.QueryRow("SELECT synced_to_cloud, sync_error FROM agent_missions WHERE id = ?", id).Scan(&synced, &syncError)
				if err != nil {
					t.Fatalf("Failed to query db for %s: %v", id, err)
				}

				if synced != expectedSynced {
					t.Errorf("Mission %s: expected synced=%v, got %v", id, expectedSynced, synced)
				}

				expectedErrStr := tt.expectedError[id]
				if expectedErrStr != "" {
					if !syncError.Valid || !strings.Contains(syncError.String, expectedErrStr) {
						t.Errorf("Mission %s: expected error containing %q, got %v", id, expectedErrStr, syncError.String)
					}
				} else {
					if syncError.Valid && syncError.String != "" {
						t.Errorf("Mission %s: expected no error, got %v", id, syncError.String)
					}
				}
			}
		})
	}
}
