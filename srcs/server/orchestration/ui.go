package orchestration

import (
    "net/http"
)

// OrchestrationMeshUI injects the required styling for the Orchestration panel.
func OrchestrationMeshUI(w http.ResponseWriter, r *http.Request) {
    w.Header().Set("Content-Type", "text/html")
    html := `<!DOCTYPE html>
<html>
<head>
<style>body { backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; }</style>
</head>
<body>
    <div id="mesh-status">Mesh active</div>
</body>
</html>`
    w.Write([]byte(html))
}
