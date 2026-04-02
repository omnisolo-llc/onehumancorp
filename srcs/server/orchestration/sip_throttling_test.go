package orchestration

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func createSIPTestDB(t *testing.T, dbName string) db.Provider {
    os.Setenv("DATABASE_URL", "sqlite://file:"+dbName+"?mode=memory&cache=shared")
	ctx := context.Background()
	dbProv, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}

    _, err = dbProv.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS agent_missions (
            id TEXT PRIMARY KEY,
            status TEXT NOT NULL,
            payload TEXT,
            assigned_agent_id TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    `)
    if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

    return dbProv
}

func TestSIPThrottling_Cloud(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	provider := createSIPTestDB(t, "sipth1")
	defer provider.Close()

    sipdb, err := NewSIPDBWithProvider(provider)
	if err != nil {
		t.Fatalf("failed to init SIPDB: %v", err)
	}

	if err := sipdb.UpsertMission(context.Background(), "mission-123", "PENDING", "{}", false); err != nil {
		t.Fatalf("UpsertMission failed: %v", err)
	}
}

func TestSIPThrottling_Standalone(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	provider := createSIPTestDB(t, "sipth2")
	defer provider.Close()

    sipdb, err := NewSIPDBWithProvider(provider)
	if err != nil {
		t.Fatalf("failed to init SIPDB: %v", err)
	}

	if err := sipdb.UpsertMission(context.Background(), "mission-124", "PENDING", "{}", false); err != nil {
		t.Fatalf("UpsertMission failed: %v", err)
	}
}
