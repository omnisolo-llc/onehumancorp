package tasks

import "net/http"

func RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/tasks/queue", QueueHandler)
}
