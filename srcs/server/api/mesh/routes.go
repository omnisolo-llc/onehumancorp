package mesh

import "net/http"

func RegisterRoutes(mux *http.ServeMux, meshService TeammateMeshService) {
	handler := NewWebSocketMeshHandler(meshService)
	mux.HandleFunc("/api/v1/mesh/stream", handler.HandleWebSocket)
}
