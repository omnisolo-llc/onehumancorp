package orchestration

import (
	"encoding/json"
	"net/http"
	"strings"
)

func RegisterDepartmentHTTPHandlers(mux *http.ServeMux, hub *Hub) {
	mux.HandleFunc("/api/v1/draft-actions", func(w http.ResponseWriter, r *http.Request) {
		if hub.DeptManager == nil {
			http.Error(w, "DepartmentManager not initialized", http.StatusInternalServerError)
			return
		}

		if r.Method == http.MethodGet {
			actions := hub.DeptManager.GetDraftActions()
			w.Header().Set("Content-Type", "application/json")
			json.NewEncoder(w).Encode(actions)
			return
		}

		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	})

	mux.HandleFunc("/api/v1/draft-actions/", func(w http.ResponseWriter, r *http.Request) {
		if hub.DeptManager == nil {
			http.Error(w, "DepartmentManager not initialized", http.StatusInternalServerError)
			return
		}

		// Expected format: /api/v1/draft-actions/{id}/{action}
		path := strings.TrimPrefix(r.URL.Path, "/api/v1/draft-actions/")
		parts := strings.Split(path, "/")
		if len(parts) != 2 {
			http.Error(w, "Invalid path format", http.StatusBadRequest)
			return
		}
		id := parts[0]
		action := parts[1]

		if r.Method == http.MethodPost {
			var newStatus string
			if action == "approve" {
				newStatus = "approved"
			} else if action == "reject" {
				newStatus = "rejected"
			} else {
				http.Error(w, "Invalid action. Use 'approve' or 'reject'.", http.StatusBadRequest)
				return
			}

			err := hub.DeptManager.UpdateDraftActionStatus(id, newStatus)
			if err != nil {
				http.Error(w, err.Error(), http.StatusNotFound)
				return
			}
			w.WriteHeader(http.StatusOK)
			return
		}

		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	})

	mux.HandleFunc("/api/v1/events/trigger", func(w http.ResponseWriter, r *http.Request) {
		if hub.DeptManager == nil {
			http.Error(w, "DepartmentManager not initialized", http.StatusInternalServerError)
			return
		}

		if r.Method == http.MethodPost {
			var event DepartmentEvent
			if err := json.NewDecoder(r.Body).Decode(&event); err != nil {
				http.Error(w, "Invalid JSON body", http.StatusBadRequest)
				return
			}

			hub.DispatchDepartmentEvent(r.Context(), event)
			w.WriteHeader(http.StatusOK)
			return
		}

		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	})
}
