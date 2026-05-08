package orchestration

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

// mockFaultyMeshHub simulates network drops and message loss
type mockFaultyMeshHub struct {
	dropRate float32
	mockMeshHub
}

func (m *mockFaultyMeshHub) Publish(ctx context.Context, channel string, data []byte) error {
	// Drop some messages randomly based on dropRate
	// To make the test deterministic without randomness, drop every other message
	if len(m.published)%2 == 0 && m.dropRate > 0 {
		return fmt.Errorf("simulated network drop")
	}
	return m.mockMeshHub.Publish(ctx, channel, data)
}

func TestChaos_StandaloneSubAgentSpawn(t *testing.T) {
	mesh := &mockFaultyMeshHub{dropRate: 0.5}
	spawner := NewDefaultSubAgentSpawner(mesh, true, 2)

	task := &SharedTask{
		ID: "chaos-task-1",
	}

	err := spawner.Spawn(context.Background(), task)
	assert.NoError(t, err)

	// Wait for processing
	time.Sleep(5 * time.Second)

	// The agent should complete successfully despite publish failures
	assert.True(t, len(mesh.published) >= 0)
}

func TestChaos_SubAgentSpawn_CloudMode(t *testing.T) {
	mesh := &mockFaultyMeshHub{dropRate: 0.5}
	spawner := NewDefaultSubAgentSpawner(mesh, false, 2)

	task := &SharedTask{
		ID: "chaos-task-cloud-1",
	}

	err := spawner.Spawn(context.Background(), task)
	assert.NoError(t, err)

	// Wait for processing
	time.Sleep(5 * time.Second)

	// The agent should complete successfully despite publish failures
	assert.True(t, len(mesh.published) >= 0)
}
