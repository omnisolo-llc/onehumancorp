package db

import (
	"context"
	"os"
	"testing"
)

// NewTestProvider creates a new in-memory SQLite database provider for testing.
// NOTE: Must be added to srcs in BUILD.bazel if we want it exported to other tests.
func NewTestProvider(t *testing.T) Provider {
	os.Setenv("DATABASE_URL", "sqlite://:memory:")
	defer os.Unsetenv("DATABASE_URL")

	dbWrapper, err := New(context.Background())
	if err != nil {
		t.Fatalf("failed to create test db provider: %v", err)
	}

	provider := dbWrapper.Provider


	t.Cleanup(func() {
		dbWrapper.Close()
	})

	return provider
}
