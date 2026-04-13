package mesh

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandlePublish(t *testing.T) {
	pubsub := NewMemoryPubSub()
	handler := HandlePublish(pubsub)

	req := httptest.NewRequest(http.MethodPost, "/", bytes.NewBufferString(`{"topic":"test", "message":{"test":"true"}}`))
	claims := &auth.Claims{Subject: "test-agent"}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Result().StatusCode != http.StatusOK {
		t.Errorf("Expected 200 OK, got %d", w.Result().StatusCode)
	}
}
