package orchestration

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestOrchestrationMeshUI(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/", nil)
	w := httptest.NewRecorder()

	OrchestrationMeshUI(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusOK {
		t.Errorf("expected OK, got %v", res.Status)
	}

	if res.Header.Get("Content-Type") != "text/html" {
		t.Errorf("expected text/html")
	}

	body := w.Body.String()
	expectedStyle := "<style>body { backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; }</style>"

	if !strings.Contains(body, expectedStyle) {
		t.Errorf("UI missing expected style mandate")
	}
}
