package api

import (
	"fmt"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// HandleKairosStream handles the WebSocket or SSE stream for KAIROS Analytics.
func HandleKairosStream(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		flusher, ok := w.(http.Flusher)
		if !ok {
			http.Error(w, "Streaming unsupported", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "text/event-stream")
		w.Header().Set("Cache-Control", "no-cache")
		w.Header().Set("Connection", "keep-alive")
		w.Header().Set("Access-Control-Allow-Origin", "*")

		ctx := r.Context()
		ticker := time.NewTicker(15 * time.Second)
		defer ticker.Stop()

		var subChan <-chan struct{}
		var unsubscribe func()

		if hub != nil {
			subChan, unsubscribe = hub.Subscribe("system")
			defer unsubscribe()
		}

		w.WriteHeader(http.StatusOK)
		flusher.Flush()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				fmt.Fprintf(w, ": heartbeat\n\n")
				flusher.Flush()
			case _, ok := <-subChan:
				if !ok {
					return
				}
				messages := hub.Inbox("system")
				for _, msg := range messages {
					eventStr := fmt.Sprintf(`{"event":"TaskBroadcast","status":"INFO","type":"%s"}`, msg.Type)
					if msg.Type == "mesh:tasks" || msg.Type == "mesh:coordination" {
						eventStr = msg.Content
					}
					fmt.Fprintf(w, "data: %s\n\n", eventStr)
				}
                // Send immediately if we process anything
				flusher.Flush()
			}
		}
	}
}
