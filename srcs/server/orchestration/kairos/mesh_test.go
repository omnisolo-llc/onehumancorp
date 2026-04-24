package kairos

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
)

func TestMemoryMesh_PublishSubscribe(t *testing.T) {
	mesh := NewMemoryMesh()
	ctx := context.Background()
	ch, err := mesh.Subscribe(ctx, "test_channel")
	assert.NoError(t, err)

	msg := []byte("hello world")
	err = mesh.Publish(ctx, "test_channel", msg)
	assert.NoError(t, err)

	select {
	case received := <-ch:
		assert.Equal(t, msg, received)
	case <-time.After(time.Second):
		t.Fatal("timeout waiting for message")
	}
}

func TestRedisMesh_PublishSubscribe(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	mesh := NewRedisMesh(client)
	ctx := context.Background()
	ch, err := mesh.Subscribe(ctx, "test_channel")
	assert.NoError(t, err)

	// Wait a bit for the subscription to be established
	time.Sleep(100 * time.Millisecond)

	msg := []byte("hello world")
	err = mesh.Publish(ctx, "test_channel", msg)
	assert.NoError(t, err)

	select {
	case received := <-ch:
		assert.Equal(t, msg, received)
	case <-time.After(time.Second):
		t.Fatal("timeout waiting for message")
	}
}

func TestNewTeammateMesh(t *testing.T) {
	// Test with nil client
	mesh := NewTeammateMesh(nil)
	_, ok := mesh.(*MemoryMesh)
	assert.True(t, ok)

	// Test with redis client
	mr, _ := miniredis.Run()
	defer mr.Close()
	client := redis.NewClient(&redis.Options{Addr: mr.Addr()})
	mesh = NewTeammateMesh(client)
	_, ok = mesh.(*RedisMesh)
	assert.True(t, ok)
}

func TestLocalTeammateMesh_PublishSubscribe(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx := context.Background()

	taskCh, err := mesh.SubscribeTasks(ctx)
	assert.NoError(t, err)

	coordCh, err := mesh.SubscribeCoordination(ctx)
	assert.NoError(t, err)

	taskMsg := []byte("task message")
	err = mesh.PublishTask(ctx, taskMsg)
	assert.NoError(t, err)

	select {
	case received := <-taskCh:
		assert.Equal(t, taskMsg, received)
	case <-time.After(time.Second):
		t.Fatal("timeout waiting for task message")
	}

	coordMsg := []byte("coordination message")
	err = mesh.PublishCoordination(ctx, coordMsg)
	assert.NoError(t, err)

	select {
	case received := <-coordCh:
		assert.Equal(t, coordMsg, received)
	case <-time.After(time.Second):
		t.Fatal("timeout waiting for coordination message")
	}
}
