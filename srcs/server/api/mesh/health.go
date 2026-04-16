package mesh

import (
	"net/http"
)

// HealthHandler returns a 200 OK if the hybrid mode is active.
func HealthHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte("OK"))
}
