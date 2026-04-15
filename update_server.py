import sys

with open('srcs/server/dashboard/server.go', 'r') as f:
    content = f.read()

# Replace the body of handleMeshV2Broadcast
old_body = """func (s *Server) handleMeshV2Broadcast(w http.ResponseWriter, r *http.Request) {
	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordMeshBroadcast(r.Context(), mode)

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}"""

new_body = """func (s *Server) handleMeshV2Broadcast(w http.ResponseWriter, r *http.Request) {
	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordMeshBroadcast(r.Context(), mode)

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	// We create a temporary LocalMeshBroker here for now as requested.
	// In reality this should be passed in via dependencies.
	import_mesh := true
	_ = import_mesh

	broker := orchestrationmesh.NewLocalMeshBroker()
	handler := orchestrationmesh.NewHTTPHandler(broker)
	handler.ServeHTTP(w, r)
	return"""

content = content.replace(old_body, new_body)

import_to_add = '\t"github.com/onehumancorp/mono/srcs/server/auth"\n\torchestrationmesh "github.com/onehumancorp/mono/srcs/server/orchestration/mesh"'
content = content.replace('\t"github.com/onehumancorp/mono/srcs/server/auth"', import_to_add)

with open('srcs/server/dashboard/server.go', 'w') as f:
    f.write(content)
