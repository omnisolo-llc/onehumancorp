package mesh

import (
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
)

func TestLocalMesh(t *testing.T) {
	m := NewLocalMesh()
	ch, err := m.Subscribe("test-channel")
	assert.NoError(t, err)

	payload := []byte("hello world")
	err = m.Publish("test-channel", payload)
	assert.NoError(t, err)

	select {
	case msg := <-ch:
		assert.Equal(t, payload, msg)
	case <-time.After(1 * time.Second):
		t.Fatal("timed out waiting for message")
	}
}

func TestRedisMesh(t *testing.T) {
	mr, err := miniredis.Run()
	assert.NoError(t, err)
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	m := NewRedisMesh(client)
	ch, err := m.Subscribe("test-channel")
	assert.NoError(t, err)

	payload := []byte("hello redis")
	err = m.Publish("test-channel", payload)
	assert.NoError(t, err)

	select {
	case msg := <-ch:
		assert.Equal(t, payload, msg)
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for message")
	}
}

func TestTeammateMeshInterface(t *testing.T) {
	var _ TeammateMesh = (*LocalMesh)(nil)
	var _ TeammateMesh = (*RedisMesh)(nil)
}
