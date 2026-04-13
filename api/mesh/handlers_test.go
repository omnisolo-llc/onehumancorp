package mesh

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandlers(t *testing.T) {
	mesh := NewMemoryMeshService()
	handler := HandleBroadcast(mesh)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBufferString(`{"test":"msg"}`))

    claims := &auth.Claims{OrganizationID: "org-123"}
    ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	w := httptest.NewRecorder()
	handler(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}
}

func TestHandleListen(t *testing.T) {
	mesh := NewMemoryMeshService()
	handler := HandleListen(mesh)

	req := httptest.NewRequest(http.MethodGet, "/api/mesh/listen", nil)
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	go func() {
		mesh.BroadcastIntent(ctx, `{"test":"msg"}`)
	}()

	w := httptest.NewRecorder()
	handler(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Code)
	}

	if !bytes.Contains(w.Body.Bytes(), []byte(`"message":"{\"test\":\"msg\"}"`)) {
		t.Errorf("expected message in body, got %s", w.Body.String())
	}
}
