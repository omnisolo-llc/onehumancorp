import re

with open("srcs/server/auth/handlers.go", "r") as f:
    content = f.read()

powersync_handler = """	// Powersync endpoints
	keypair, _ := GeneratePowerSyncKeypair()
	mux.HandleFunc("/api/auth/powersync/jwks", PowerSyncJWKSHandler(keypair))
	mux.Handle("/api/auth/powersync/token", Middleware(store)(PowerSyncTokenHandler(store, keypair)))
"""

if "/api/auth/powersync/jwks" not in content:
    content = re.sub(r'func \(h \*Handlers\) Register\(mux \*http\.ServeMux\) \{', 'func (h *Handlers) Register(mux *http.ServeMux) {\n' + powersync_handler, content, count=1)

with open("srcs/server/auth/handlers.go", "w") as f:
    f.write(content)
