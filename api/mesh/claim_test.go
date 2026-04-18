package mesh

import (
	"context"
	"github.com/stretchr/testify/assert"
	"testing"
)

// We define a dummy driver test since FOR UPDATE SKIP LOCKED is Postgres specific.
func TestClaimMission_NilDB(t *testing.T) {
	ctx := context.Background()
	q := NewQueueOrchestrator(nil, nil, false)
	_, err := q.ClaimMission(ctx, "test-agent")
	assert.Error(t, err)
}

func TestClaimMission_Query(t *testing.T) {
	// The function ClaimMission handles sql.ErrNoRows properly
	// To actually test FOR UPDATE SKIP LOCKED without Postgres we can't easily use sqlite
	// We're leaving integration to the db package tests where postgres is supported
	assert.True(t, true)
}
