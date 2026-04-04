package dashboard

import (
	"encoding/json"
	"net/http"
)

func (s *Server) handleMeshDirect(w http.ResponseWriter, r *http.Request) {
	s.hub.HandleMeshDirect(w, r)
}

func (s *Server) handleMeshMailbox(w http.ResponseWriter, r *http.Request) {
	s.hub.HandleMeshMailbox(w, r)
}
