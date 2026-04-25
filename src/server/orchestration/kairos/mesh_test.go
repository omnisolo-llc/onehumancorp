package kairos

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/onehumancorp/mono/src/server/telemetry"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
)

func TestLocalTeammateMesh(t *testing.T) {
	telemetry.InitTelemetry()

	mesh := NewLocalTeammateMesh()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	taskChan, err := mesh.SubscribeTasks(ctx)
	assert.NoError(t, err)

	coordChan, err := mesh.SubscribeCoordination(ctx)
	assert.NoError(t, err)

	err = mesh.PublishTask(ctx, []byte("task-1"))
	assert.NoError(t, err)

	select {
	case msg := <-taskChan:
		assert.Equal(t, "task-1", string(msg))
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for task message")
	}

	err = mesh.PublishCoordination(ctx, []byte("coord-1"))
	assert.NoError(t, err)

	select {
	case msg := <-coordChan:
		assert.Equal(t, "coord-1", string(msg))
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for coordination message")
	}
}

func TestRedisTeammateMesh(t *testing.T) {
	telemetry.InitTelemetry()

	mr, err := miniredis.Run()
	assert.NoError(t, err)
	defer mr.Close()

	client := redis.NewClient(&redis.Options{Addr: mr.Addr()})

	mesh := NewTeammateMesh(client)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	subChan, err := mesh.Subscribe(ctx, "mesh:test")
	assert.NoError(t, err)

	// Wait for subscription to establish
	time.Sleep(100 * time.Millisecond)

	err = mesh.Publish(ctx, "mesh:test", []byte("hello-redis"))
	assert.NoError(t, err)

	select {
	case msg := <-subChan:
		assert.Equal(t, "hello-redis", string(msg))
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for redis message")
	}
}
