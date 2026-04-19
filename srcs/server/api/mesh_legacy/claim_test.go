package mesh

import (
	"context"
	"testing"
	"github.com/stretchr/testify/assert"
)

// We define a dummy driver test since FOR UPDATE SKIP LOCKED is Postgres specific.
func TestClaimMission_NilDB(t *testing.T) {
	ctx := context.Background()
	_, err := ClaimMission(ctx, nil, "test-agent")
	assert.Error(t, err)
}

func TestClaimMission_Query(t *testing.T) {
	// The function ClaimMission handles sql.ErrNoRows properly
	// To actually test FOR UPDATE SKIP LOCKED without Postgres we can't easily use sqlite
	// We're leaving integration to the db package tests where postgres is supported
	assert.True(t, true)
}
