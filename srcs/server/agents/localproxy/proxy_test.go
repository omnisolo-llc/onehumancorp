package localproxy

import (
	"context"
	"database/sql"
	"strings"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"onehumancorp/srcs/server/agents/sandbox"
)

func TestLocalExecutionProxy_ExecuteTerminal(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}
	defer db.Close()

	sm, err := sandbox.NewSandboxManager()
	if err != nil {
		t.Fatalf("failed to create sandbox manager: %v", err)
	}
	defer sm.Cleanup()

	proxy, err := NewLocalExecutionProxy(db, sm)
	if err != nil {
		t.Fatalf("failed to create proxy: %v", err)
	}

	ctx := context.Background()

	// Test 1: Simple echo
	stdout, stderr, err := proxy.ExecuteTerminal(ctx, `echo "hello proxy"`)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !strings.Contains(stdout, "hello proxy") {
		t.Errorf("expected 'hello proxy' in stdout, got %q", stdout)
	}
	if stderr != "" {
		t.Errorf("expected empty stderr, got %q", stderr)
	}

	// Verify database sync for Test 1
	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM execution_logs").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 execution log, got %d", count)
	}

	var savedCommand, savedStdout string
	err = db.QueryRow("SELECT command, stdout FROM execution_logs LIMIT 1").Scan(&savedCommand, &savedStdout)
	if err != nil {
		t.Fatalf("failed to query log: %v", err)
	}
	if savedCommand != `echo "hello proxy"` {
		t.Errorf("expected command `echo \"hello proxy\"`, got %q", savedCommand)
	}
	if savedStdout != stdout {
		t.Errorf("expected saved stdout to match exact output")
	}

	// Test 2: Error command
	stdout, stderr, err = proxy.ExecuteTerminal(ctx, `ls /directory_that_does_not_exist_in_ohc_proxy`)
	if err == nil {
		t.Errorf("expected error executing invalid command, got nil")
	}
	if !strings.Contains(stderr, "No such file or directory") {
		t.Errorf("expected error message in stderr, got %q", stderr)
	}

	// Verify DB synced the failed command
	err = db.QueryRow("SELECT COUNT(*) FROM execution_logs").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 2 {
		t.Errorf("expected 2 execution logs, got %d", count)
	}
}
