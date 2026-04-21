package kairos

import (
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
)

func TestMemoryMesh_PublishSubscribe(t *testing.T) {
	mesh := NewMemoryMesh()
	ch, err := mesh.Subscribe("test_channel")
	assert.NoError(t, err)

	msg := []byte("hello world")
	err = mesh.Publish("test_channel", msg)
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
	ch, err := mesh.Subscribe("test_channel")
	assert.NoError(t, err)

	// Wait a bit for the subscription to be established
	time.Sleep(100 * time.Millisecond)

	msg := []byte("hello world")
	err = mesh.Publish("test_channel", msg)
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
