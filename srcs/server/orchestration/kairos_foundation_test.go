package orchestration

import (
    "context"
    "os"
    "testing"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestKairosFoundation(t *testing.T) {
    ctx := context.Background()
    os.Setenv("DATABASE_URL", "sqlite://:memory:")
    defer os.Unsetenv("DATABASE_URL")

    d, err := db.New(ctx)
    if err != nil {
        t.Fatalf("Failed to initialize db: %v", err)
    }
    err = d.RunMigrations(ctx)
    if err != nil {
        t.Fatalf("Failed to run migrations: %v", err)
    }
}
