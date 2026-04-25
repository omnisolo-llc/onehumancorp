package api

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/lib/analytics"
	"github.com/onehumancorp/mono/src/server/services/growth"
)

func setupTestDependencies(t *testing.T) (*growth.InviteTracker, *growth.ViralLoopTracker) {
	ctx := context.Background()
	t.Setenv("DATABASE_URL", "sqlite://:memory:")

	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to connect to memory db: %v", err)
	}

	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	it := growth.NewInviteTracker(database)

	analyticsTracker := analytics.NewTracker()
	vt := growth.NewViralLoopTracker(analyticsTracker)

	return it, vt
}

func TestGrowthHandler_HandleInvite(t *testing.T) {
	it, vt := setupTestDependencies(t)
	handler := NewGrowthHandler(it, vt)

	t.Run("Valid request", func(t *testing.T) {
		reqBody := InviteRequest{
			TeamID:    "team1",
			InviterID: "user1",
			InviteeID: "user2",
		}
		bodyBytes, _ := json.Marshal(reqBody)
		req := httptest.NewRequest(http.MethodPost, "/api/v1/invites", bytes.NewBuffer(bodyBytes))
		w := httptest.NewRecorder()

		handler.HandleInvite(w, req)

		if w.Code != http.StatusCreated {
			t.Errorf("Expected status %d, got %d", http.StatusCreated, w.Code)
		}
	})

	t.Run("Invalid method", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/api/v1/invites", nil)
		w := httptest.NewRecorder()

		handler.HandleInvite(w, req)

		if w.Code != http.StatusMethodNotAllowed {
			t.Errorf("Expected status %d, got %d", http.StatusMethodNotAllowed, w.Code)
		}
	})

	t.Run("Missing fields", func(t *testing.T) {
		reqBody := InviteRequest{
			TeamID: "team1",
		}
		bodyBytes, _ := json.Marshal(reqBody)
		req := httptest.NewRequest(http.MethodPost, "/api/v1/invites", bytes.NewBuffer(bodyBytes))
		w := httptest.NewRecorder()

		handler.HandleInvite(w, req)

		if w.Code != http.StatusBadRequest {
			t.Errorf("Expected status %d, got %d", http.StatusBadRequest, w.Code)
		}
	})

	t.Run("Invalid body", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/api/v1/invites", bytes.NewBuffer([]byte("invalid json")))
		w := httptest.NewRecorder()

		handler.HandleInvite(w, req)

		if w.Code != http.StatusBadRequest {
			t.Errorf("Expected status %d, got %d", http.StatusBadRequest, w.Code)
		}
	})

	t.Run("RecordInvite Error", func(t *testing.T) {
		// Create a handler with a broken DB connection to simulate error
		ctx := context.Background()
		t.Setenv("DATABASE_URL", "sqlite://:memory:")
		database, err := db.New(ctx)
		if err != nil {
			t.Fatalf("failed to connect to memory db: %v", err)
		}

		// Run migrations but then close the DB so inserts fail
		if err := database.RunMigrations(ctx); err != nil {
			// Ignore error for sqlite syntax due to existing bug
		}

		brokenIt := growth.NewInviteTracker(database)
		database.Close()

		analyticsTracker := analytics.NewTracker()
		vt := growth.NewViralLoopTracker(analyticsTracker)

		brokenHandler := NewGrowthHandler(brokenIt, vt)

		reqBody := InviteRequest{
			TeamID:    "team1",
			InviterID: "user1",
			InviteeID: "user2",
		}
		bodyBytes, _ := json.Marshal(reqBody)
		req := httptest.NewRequest(http.MethodPost, "/api/v1/invites", bytes.NewBuffer(bodyBytes))
		w := httptest.NewRecorder()

		brokenHandler.HandleInvite(w, req)

		if w.Code != http.StatusInternalServerError {
			t.Errorf("Expected status %d, got %d", http.StatusInternalServerError, w.Code)
		}
	})
}

func TestGrowthHandler_HandleAcceptInvite(t *testing.T) {
	it, vt := setupTestDependencies(t)
	handler := NewGrowthHandler(it, vt)

	t.Run("Valid request", func(t *testing.T) {
		reqBody := AcceptInviteRequest{
			InviteeID: "user2",
		}
		bodyBytes, _ := json.Marshal(reqBody)
		req := httptest.NewRequest(http.MethodPost, "/api/v1/invites/accept", bytes.NewBuffer(bodyBytes))
		w := httptest.NewRecorder()

		handler.HandleAcceptInvite(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("Expected status %d, got %d", http.StatusOK, w.Code)
		}
	})

	t.Run("Invalid method", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/api/v1/invites/accept", nil)
		w := httptest.NewRecorder()

		handler.HandleAcceptInvite(w, req)

		if w.Code != http.StatusMethodNotAllowed {
			t.Errorf("Expected status %d, got %d", http.StatusMethodNotAllowed, w.Code)
		}
	})

	t.Run("Missing fields", func(t *testing.T) {
		reqBody := AcceptInviteRequest{}
		bodyBytes, _ := json.Marshal(reqBody)
		req := httptest.NewRequest(http.MethodPost, "/api/v1/invites/accept", bytes.NewBuffer(bodyBytes))
		w := httptest.NewRecorder()

		handler.HandleAcceptInvite(w, req)

		if w.Code != http.StatusBadRequest {
			t.Errorf("Expected status %d, got %d", http.StatusBadRequest, w.Code)
		}
	})

	t.Run("Invalid body", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/api/v1/invites/accept", bytes.NewBuffer([]byte("invalid json")))
		w := httptest.NewRecorder()

		handler.HandleAcceptInvite(w, req)

		if w.Code != http.StatusBadRequest {
			t.Errorf("Expected status %d, got %d", http.StatusBadRequest, w.Code)
		}
	})

	t.Run("RecordInvite Error", func(t *testing.T) {
		// Create a handler with a broken DB connection to simulate error
		ctx := context.Background()
		t.Setenv("DATABASE_URL", "sqlite://:memory:")
		database, err := db.New(ctx)
		if err != nil {
			t.Fatalf("failed to connect to memory db: %v", err)
		}

		// Run migrations but then close the DB so inserts fail
		if err := database.RunMigrations(ctx); err != nil {
			// Ignore error for sqlite syntax due to existing bug
		}

		brokenIt := growth.NewInviteTracker(database)
		database.Close()

		analyticsTracker := analytics.NewTracker()
		vt := growth.NewViralLoopTracker(analyticsTracker)

		brokenHandler := NewGrowthHandler(brokenIt, vt)

		reqBody := InviteRequest{
			TeamID:    "team1",
			InviterID: "user1",
			InviteeID: "user2",
		}
		bodyBytes, _ := json.Marshal(reqBody)
		req := httptest.NewRequest(http.MethodPost, "/api/v1/invites", bytes.NewBuffer(bodyBytes))
		w := httptest.NewRecorder()

		brokenHandler.HandleInvite(w, req)

		if w.Code != http.StatusInternalServerError {
			t.Errorf("Expected status %d, got %d", http.StatusInternalServerError, w.Code)
		}
	})
}
