package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestStartSyncDaemon(t *testing.T) {
	sqlLocal, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite local db: %v", err)
	}
	defer sqlLocal.Close()

	sqlCloud, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite cloud db: %v", err)
	}
	defer sqlCloud.Close()

	for _, d := range []*sql.DB{sqlLocal, sqlCloud} {
		_, err = d.Exec(`
			CREATE TABLE agent_missions (
				id TEXT PRIMARY KEY,
				status TEXT,
				payload TEXT
			)
		`)
		if err != nil {
			t.Fatalf("failed to create agent_missions table: %v", err)
		}
	}

	_, err = sqlLocal.Exec(`
		INSERT INTO agent_missions (id, status, payload)
		VALUES
			('m1', 'CLOUD_ESCALATION', '{"task":"test", "secret":"[PRIVATE:key]"}'),
			('m2', 'IN_CLOUD', '{"task":"wait"}')
	`)
	if err != nil {
		t.Fatalf("failed to insert local test data: %v", err)
	}

	_, err = sqlCloud.Exec(`
		INSERT INTO agent_missions (id, status, payload)
		VALUES
			('m2', 'DONE', '{"result":"success"}')
	`)
	if err != nil {
		t.Fatalf("failed to insert cloud test data: %v", err)
	}

	localProv := db.NewSqliteProvider(sqlLocal)
	cloudProv := db.NewSqliteProvider(sqlCloud)

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	StartSyncDaemon(ctx, localProv, cloudProv)

	// Wait for processSyncTick to be called
	time.Sleep(1500 * time.Millisecond)

	// Verify m1
	var m1LocalStatus, m1CloudStatus, m1CloudPayload string
	err = sqlLocal.QueryRow("SELECT status FROM agent_missions WHERE id = 'm1'").Scan(&m1LocalStatus)
	if err != nil {
		t.Fatalf("m1 local query failed: %v", err)
	}
	if m1LocalStatus != "IN_CLOUD" {
		t.Errorf("m1 expected local status IN_CLOUD, got %s", m1LocalStatus)
	}

	err = sqlCloud.QueryRow("SELECT status, payload FROM agent_missions WHERE id = 'm1'").Scan(&m1CloudStatus, &m1CloudPayload)
	if err != nil {
		t.Fatalf("m1 cloud query failed: %v", err)
	}
	if m1CloudStatus != "PENDING" {
		t.Errorf("m1 expected cloud status PENDING, got %s", m1CloudStatus)
	}
	if m1CloudPayload != `{"task":"test", "secret":"[REDACTED]"}` {
		t.Errorf("m1 expected sanitized payload, got %s", m1CloudPayload)
	}

	// Verify m2
	var m2LocalStatus, m2LocalPayload string
	err = sqlLocal.QueryRow("SELECT status, payload FROM agent_missions WHERE id = 'm2'").Scan(&m2LocalStatus, &m2LocalPayload)
	if err != nil {
		t.Fatalf("m2 local query failed: %v", err)
	}
	if m2LocalStatus != "DONE" {
		t.Errorf("m2 expected local status DONE, got %s", m2LocalStatus)
	}
	if m2LocalPayload != `{"result":"success"}` {
		t.Errorf("m2 expected updated payload, got %s", m2LocalPayload)
	}
}
