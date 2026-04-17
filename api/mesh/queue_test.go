package mesh

import (
	"context"
	"testing"
	"github.com/stretchr/testify/assert"
)

func TestEnqueueMission_NilDB(t *testing.T) {
	ctx := context.Background()
	_, err := EnqueueMission(ctx, nil, "test", "P0", []byte("{}"))
	assert.Error(t, err)
}

func TestCompleteMission_NilDB(t *testing.T) {
	ctx := context.Background()
	err := CompleteMission(ctx, nil, "uuid", "agent")
	assert.Error(t, err)
}
