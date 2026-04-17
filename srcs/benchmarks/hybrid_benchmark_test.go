package benchmarks

import (
	"context"
	"fmt"
	"os"
	"testing"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func BenchmarkHybridSIPDB(b *testing.B) {
	// Setup SQLite
	sqliteDB, err := orchestration.NewSIPDB(":memory:")
	if err != nil {
		b.Fatalf("failed to init SQLite SIPDB: %v", err)
	}

	// Try to setup Postgres if DATABASE_URL is set, else skip Postgres tests
	dbURL := os.Getenv("DATABASE_URL")
	var pgProvider db.Provider
	var pgDB *orchestration.SIPDB
	if dbURL != "" {
		pool, err := pgxpool.New(context.Background(), dbURL)
		if err != nil {
			b.Fatalf("failed to connect to Postgres: %v", err)
		}
		defer pool.Close()
		pgProvider = db.NewPgProvider(pool)
		pgDB, err = orchestration.NewSIPDBWithProvider(pgProvider, "test-org")
		if err != nil {
			b.Fatalf("failed to init Postgres SIPDB: %v", err)
		}
	}

	b.Run("SQLite_UpdateMemory", func(b *testing.B) {
		ctx := context.Background()
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			err := sqliteDB.UpdateMemory(ctx, "test-agent", fmt.Sprintf("content-%d", i))
			if err != nil {
				b.Fatalf("UpdateMemory failed: %v", err)
			}
		}
	})

	b.Run("SQLite_SyncMissions", func(b *testing.B) {
		ctx := context.Background()
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			_, err := sqliteDB.SyncMissions(ctx, "remote")
			if err != nil {
				b.Fatalf("SyncMissions failed: %v", err)
			}
		}
	})

	if pgDB != nil {
		b.Run("Postgres_UpdateMemory", func(b *testing.B) {
			ctx := context.Background()
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				err := pgDB.UpdateMemory(ctx, "test-agent", fmt.Sprintf("content-%d", i))
				if err != nil {
					b.Fatalf("UpdateMemory failed: %v", err)
				}
			}
		})

		b.Run("Postgres_SyncMissions", func(b *testing.B) {
			ctx := context.Background()
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				_, err := pgDB.SyncMissions(ctx, "remote")
				if err != nil {
					b.Fatalf("SyncMissions failed: %v", err)
				}
			}
		})
	}
}
