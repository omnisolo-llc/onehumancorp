package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

type mockDbProvider struct {
	db.Provider
	isSqlite bool
}

func (m *mockDbProvider) IsSQLite() bool {
	return m.isSqlite
}

type mockRueidisClient struct {
	rueidis.Client
}

func TestLocalTeammateMesh(t *testing.T) {
	mesh := NewLocalTeammateMesh()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	channel := "test_channel"
	msgPayload := []byte("hello")

	sub, err := mesh.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	err = mesh.Publish(ctx, channel, msgPayload)
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	select {
	case msg := <-sub:
		if string(msg) != string(msgPayload) {
			t.Errorf("Expected %s, got %s", msgPayload, msg)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout")
	}

	err = mesh.Unsubscribe(ctx, channel)
	if err != nil {
		t.Fatalf("Failed to unsubscribe: %v", err)
	}
}

func TestNewTeammateMesh(t *testing.T) {
	t.Run("SQLite uses LocalTeammateMesh", func(t *testing.T) {
		provider := &mockDbProvider{isSqlite: true}
		mesh := NewTeammateMesh(provider, &mockRueidisClient{})

		if _, ok := mesh.(*LocalTeammateMesh); !ok {
			t.Errorf("Expected LocalTeammateMesh, got %T", mesh)
		}
	})

	t.Run("Redis client uses RedisTeammateMesh", func(t *testing.T) {
		provider := &mockDbProvider{isSqlite: false}
		mesh := NewTeammateMesh(provider, &mockRueidisClient{})

		if _, ok := mesh.(*RedisTeammateMesh); !ok {
			t.Errorf("Expected RedisTeammateMesh, got %T", mesh)
		}
	})
}
