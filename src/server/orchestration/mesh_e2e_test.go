package orchestration

import (
	"github.com/onehumancorp/mono/src/server/auth"

	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

func TestMeshAPI_E2E(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory&cache=shared")
	ctx, cancel := context.WithTimeout(context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"}), 5*time.Second)
	defer cancel()

	provider := db.NewTestProvider(t)
	defer provider.Close()

	_, err := provider.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload JSON,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	meshTransport := NewMemoryMeshTransport(provider)
	api := NewMeshAPI(meshTransport)
	mux := http.NewServeMux()
	api.RegisterRoutes(mux)

	server := httptest.NewServer(mux)
	defer server.Close()

	channel := "ohc.mesh.agent.e2e_test"
	syncReq, err := http.NewRequestWithContext(ctx, http.MethodGet, server.URL+"/api/mesh/sync?channel="+channel, nil)
	if err != nil {
		t.Fatalf("Failed to create sync request: %v", err)
	}

	client := server.Client()

	syncCh := make(chan string, 1)
	errCh := make(chan error, 1)
	go func() {
		resp, err := client.Do(syncReq)
		if err != nil {
			errCh <- err
			return
		}
		defer resp.Body.Close()

		if resp.StatusCode != http.StatusOK {
			errCh <- err
			return
		}

		buf := make([]byte, 1024)
		for {
			n, err := resp.Body.Read(buf)
			if err != nil {
				return
			}
			msg := string(buf[:n])
			if strings.Contains(msg, "TEST_ACTION") {
				syncCh <- msg
				return
			}
		}
	}()

	time.Sleep(100 * time.Millisecond)

	payload := map[string]interface{}{
		"channel": channel,
		"action":  "TEST_ACTION",
		"status":  "COMPLETED",
	}
	body, _ := json.Marshal(payload)
	broadcastReq, err := http.NewRequestWithContext(ctx, http.MethodPost, server.URL+"/api/mesh/broadcast", bytes.NewBuffer(body))
	if err != nil {
		t.Fatalf("Failed to create broadcast request: %v", err)
	}
	broadcastReq.Header.Set("Content-Type", "application/json")

	resp, err := client.Do(broadcastReq)
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Fatalf("Expected status 200 for broadcast, got %d", resp.StatusCode)
	}

	select {
	case msg := <-syncCh:
		if !strings.Contains(msg, "TEST_ACTION") {
			t.Errorf("Expected SSE message to contain 'TEST_ACTION', got: %s", msg)
		}
	case err := <-errCh:
		t.Fatalf("Sync error: %v", err)
	case <-time.After(2 * time.Second):
		t.Fatal("Timeout waiting for SSE message")
	}
}
