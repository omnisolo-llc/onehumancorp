package services

import (
	"database/sql"
	"fmt"
	"strings"
	"sync"
	"testing"

	"github.com/onehumancorp/mono/apps/api/db"
	_ "modernc.org/sqlite"
)

func setupSecurityDB(t *testing.T, isSQLite bool) (*sql.DB, string) {
	// For standalone isolation testing, simulate missing _pragma=busy_timeout
	// and verify our driver usage.
	var dsn string
	if isSQLite {
		dsn = fmt.Sprintf("file:%s?mode=memory&cache=shared&_pragma=busy_timeout(15000)", t.Name())
	} else {
		t.Skip("Postgres requires external service")
	}

	conn, err := sql.Open("sqlite", dsn)
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	if err := db.CreateSchema(conn, isSQLite); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	return conn, dsn
}

func TestTaskQueueService_Security_StandaloneSQLiteContention(t *testing.T) {
	conn, _ := setupSecurityDB(t, true)
	defer conn.Close()

	svc := NewTaskQueueService(conn, true)

	// Simulate high-concurrency burst
	concurrency := 100
	var wg sync.WaitGroup
	errs := make(chan error, concurrency*2)

	// Pre-fill queue
	for i := 0; i < concurrency; i++ {
		err := svc.PushTask(fmt.Sprintf("task-%d", i), "High Throughput Payload", nil)
		if err != nil {
			t.Fatalf("failed to pre-fill task: %v", err)
		}
	}

	// Hammer the queue with parallel claims
	wg.Add(concurrency)
	for i := 0; i < concurrency; i++ {
		go func(agentID string) {
			defer wg.Done()
			_, err := svc.ClaimTask(agentID)
			if err != nil {
				errs <- err
			}
		}(fmt.Sprintf("agent-%d", i))
	}

	wg.Wait()
	close(errs)

	for err := range errs {
		if strings.Contains(err.Error(), "database is locked") || strings.Contains(err.Error(), "SQLITE_BUSY") {
			t.Fatalf("Security Regression: Standalone SQLite Lock Contention detected: %v", err)
		} else {
			t.Errorf("Unexpected error during parallel execution: %v", err)
		}
	}
}
