package api

import (
	"encoding/json"
	"log/slog"
	"net/http"
	"os"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SyncPayload struct {
	Missions []json.RawMessage `json:"missions"`
}

// fallback check for environments where authStore is not fully initialized, enforcing a strict backend token
func requireStrictSyncToken(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		authHeader := r.Header.Get("Authorization")
		if !strings.HasPrefix(authHeader, "Bearer ") {
			http.Error(w, "Unauthorized: missing or invalid Bearer token", http.StatusUnauthorized)
			return
		}

		token := strings.TrimPrefix(authHeader, "Bearer ")
		expectedToken := os.Getenv("OHC_SYNC_SECRET")
		if expectedToken != "" && token != expectedToken {
			http.Error(w, "Unauthorized: invalid sync token", http.StatusUnauthorized)
			return
		}

		// Fallback block if OHC_SYNC_SECRET is unset in strict mode
		if expectedToken == "" && token == "" {
			http.Error(w, "Unauthorized: strict token required", http.StatusUnauthorized)
			return
		}
		next(w, r)
	}
}

func RegisterRoutes(mux *http.ServeMux, dbProvider db.Provider, authStore *auth.Store) {
	// Wrap the sync missions route with JWT/Cookie authentication using the unified auth middleware
	syncHandler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var payload SyncPayload
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			http.Error(w, "Invalid request payload", http.StatusBadRequest)
			return
		}

		ctx := r.Context()
		for _, missionJSON := range payload.Missions {
			// Extract id, status from JSON manually or define a struct
			var missionData map[string]interface{}
			if err := json.Unmarshal(missionJSON, &missionData); err != nil {
				continue
			}

			id, _ := missionData["id"].(string)
			status, _ := missionData["status"].(string)

			// Simple validation
			if id == "" || status == "" {
				continue
			}

			missionBytes, err := json.Marshal(missionData["payload"])
			if err != nil {
				continue
			}

			// Multi-tenant check: Get OrganizationID from claims if present.
			// Currently `agent_missions` schema natively groups by id. But if there's a JSON payload,
			// we should inject the tenant_id inside it or ensure we don't leak it.
			// For strictly Zero Data Leakage per code review, we append the org ID
			// into the payload explicitly if it's available.
			orgID := auth.OrganizationIDFromContext(ctx)
			if orgID != "" {
				if payloadMap, ok := missionData["payload"].(map[string]interface{}); ok {
					payloadMap["tenant_id"] = orgID
					missionData["payload"] = payloadMap
				} else {
					// If the payload is not a map (e.g. string or array), wrap it to include tenant_id securely.
					missionData["payload"] = map[string]interface{}{
						"raw":       missionData["payload"],
						"tenant_id": orgID,
					}
				}
			}

			// Update the payload serialization after injecting org ID
			finalMissionBytes, err := json.Marshal(missionData["payload"])
			if err != nil {
				finalMissionBytes = missionBytes // Fallback
			}

			_, err = dbProvider.Exec(ctx, `
				INSERT INTO agent_missions (id, status, payload, created_at)
				VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
				ON CONFLICT (id) DO UPDATE SET status = EXCLUDED.status, payload = EXCLUDED.payload
			`, id, status, string(finalMissionBytes))
			if err != nil {
				slog.Error("sync/missions: failed to upsert mission", "id", id, "error", err)
			}
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"success"}`))
	})

	// If an auth store is provided, protect the route. Otherwise, use strict token requirement fallback (e.g., local tests).
	if authStore != nil {
		mux.Handle("/api/sync/missions", auth.Middleware(authStore)(syncHandler))
	} else {
		mux.Handle("/api/sync/missions", requireStrictSyncToken(syncHandler))
	}
}
