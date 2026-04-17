package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestServer_EscalateMissions(t *testing.T) {
	ctx := context.Background()

	tmpDir, err := os.MkdirTemp("", "escalate_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	dbPath := filepath.Join(tmpDir, "escalate.db")
	dbInstance, err := db.NewProvider("sqlite", dbPath, false)
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	org := &db.Organization{ID: "org-1", Name: "Test Org"}
	hub := orchestration.NewHub(dbInstance, orchestration.Config{
		Organization: org,
	})

	srv := &Server{
		hub: hub,
		org: org,
	}

	t.Run("HandleMissionEscalate", func(t *testing.T) {
		reqBody := map[string]interface{}{
			"local_id": "loc-1",
			"payload": map[string]interface{}{
				"role": "test",
				"task": "do task",
			},
		}
		jsonData, _ := json.Marshal(reqBody)

		req := httptest.NewRequest(http.MethodPost, "/api/v1/missions/escalate", bytes.NewBuffer(jsonData))
		req.Header.Set("Content-Type", "application/json")

		// Mock auth
		ctx = auth.ContextWithClaims(req.Context(), &auth.Claims{
			OrganizationID: "org-1",
		})
		req = req.WithContext(ctx)

		rr := httptest.NewRecorder()
		srv.handleMissionEscalate(rr, req)

		if rr.Code != http.StatusAccepted {
			t.Errorf("expected status %d, got %d", http.StatusAccepted, rr.Code)
		}

		var resp map[string]string
		json.Unmarshal(rr.Body.Bytes(), &resp)

		if resp["status"] != "ACCEPTED" {
			t.Errorf("expected status ACCEPTED, got %s", resp["status"])
		}
		if resp["cloud_id"] == "" {
			t.Errorf("expected cloud_id, got empty")
		}

		// Verify mission was added to SIPDB
		status, _ := hub.SIPDB().GetMissionStatus(ctx, resp["cloud_id"])
		if status != "PENDING" {
			t.Errorf("expected mission status PENDING, got %s", status)
		}
	})

	t.Run("HandleMissionStatus", func(t *testing.T) {
		// Pre-populate mission
		hub.SIPDB().UpsertMission(ctx, "c-1", "DONE", "{}", false)

		req := httptest.NewRequest(http.MethodGet, "/api/v1/missions/c-1/status", nil)
		ctx = auth.ContextWithClaims(req.Context(), &auth.Claims{
			OrganizationID: "org-1",
		})
		req = req.WithContext(ctx)

		rr := httptest.NewRecorder()
		srv.handleMissionStatus(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("expected status %d, got %d", http.StatusOK, rr.Code)
		}

		var resp map[string]string
		json.Unmarshal(rr.Body.Bytes(), &resp)

		if resp["status"] != "DONE" {
			t.Errorf("expected status DONE, got %s", resp["status"])
		}
	})
}
