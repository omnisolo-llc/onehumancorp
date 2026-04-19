package hybrid_discovery

import (
	"context"
	"database/sql"
	"database/sql/driver"
	"os"
	"testing"

	_ "modernc.org/sqlite"
)

func TestDiscoveryProxy_SQLite(t *testing.T) {
	// Create a temporary SQLite database
	dbFile := "test_discovery.db"
	defer os.Remove(dbFile)

	db, err := sql.Open("sqlite", dbFile)
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}
	defer db.Close()

	proxy := NewDiscoveryProxy(db, "switchboard.local")

	if !proxy.isSQLite() {
		t.Errorf("Expected isSQLite() to return true")
	}

	ctx := context.Background()

	// Test SearchTools
	tools, err := proxy.SearchTools(ctx, "calculator")
	if err != nil {
		t.Fatalf("SearchTools failed: %v", err)
	}

	if len(tools) == 0 {
		t.Errorf("Expected to find calculator tool")
	} else if tools[0].Name != "local-calculator" {
		t.Errorf("Expected 'local-calculator', got '%s'", tools[0].Name)
	}

	// Test RequestToolSVID
	svid, err := proxy.RequestToolSVID(ctx, "local-calculator")
	if err != nil {
		t.Fatalf("RequestToolSVID failed: %v", err)
	}

	if svid.ID != "spiffe://local.standalone/tool/local-calculator" {
		t.Errorf("Unexpected SVID ID: %s", svid.ID)
	}
}

// Dummy driver definition to test false condition
type dummyDriver struct{}
func (d dummyDriver) Open(name string) (driver.Conn, error) { return nil, nil }

func TestDiscoveryProxy_Postgres(t *testing.T) {
	// We don't need a real Postgres connection, just a non-SQLite driver
	// Let's create a nil DB proxy which will route to Switchboard
	proxy := NewDiscoveryProxy(nil, "switchboard.cloud.internal")

	if proxy.isSQLite() {
		t.Errorf("Expected isSQLite() to return false for nil db")
	}

	ctx := context.Background()

	// Test SearchTools
	tools, err := proxy.SearchTools(ctx, "calculator")
	if err != nil {
		t.Fatalf("SearchTools failed: %v", err)
	}

	if len(tools) == 0 {
		t.Errorf("Expected to find calculator tool")
	} else if tools[0].Name != "cloud-calculator" {
		t.Errorf("Expected 'cloud-calculator', got '%s'", tools[0].Name)
	}

	// Test RequestToolSVID
	svid, err := proxy.RequestToolSVID(ctx, "cloud-calculator")
	if err != nil {
		t.Fatalf("RequestToolSVID failed: %v", err)
	}

	if svid.ID != "spiffe://cloud.internal/tool/cloud-calculator" {
		t.Errorf("Unexpected SVID ID: %s", svid.ID)
	}
}
