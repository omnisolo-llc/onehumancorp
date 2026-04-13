package mesh

import (
	"encoding/json"
	"io"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// HandleBroadcast provides an HTTP endpoint for POST /api/mesh/broadcast
func HandleBroadcast(meshService TeammateMeshService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil || claims.OrganizationID == "" {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}

		body, err := io.ReadAll(r.Body)
		if err != nil {
			http.Error(w, "invalid request", http.StatusBadRequest)
			return
		}

		if err := meshService.BroadcastIntent(r.Context(), string(body)); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"ok"}`))
	}
}

// HandleListen provides an HTTP endpoint for GET /api/mesh/listen
func HandleListen(meshService TeammateMeshService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil || claims.OrganizationID == "" {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}

		ch, err := meshService.Subscribe(r.Context())
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)

		// We write a single message and return for simplistic polling,
		// or handle streaming depending on client needs. For now, read one.
		select {
		case msg, ok := <-ch:
			if !ok {
				http.Error(w, "channel closed", http.StatusInternalServerError)
				return
			}
			json.NewEncoder(w).Encode(map[string]string{"message": msg})
		case <-r.Context().Done():
			return
		}
	}
}
