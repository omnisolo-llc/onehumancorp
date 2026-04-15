package mesh

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric/noop"
)

func TestMain(m *testing.M) {
	provider := noop.NewMeterProvider()
	otel.SetMeterProvider(provider)
	m.Run()
}

func TestMemoryMeshService(t *testing.T) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	svc := NewMemoryMeshService()

	sub, err := svc.Subscribe(ctx)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = svc.BroadcastIntent(ctx, "hello")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	select {
	case msg := <-sub:
		if msg != "hello" {
			t.Errorf("expected 'hello', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Error("timeout waiting for message")
	}
}

func TestAuthErrors(t *testing.T) {
	ctx := context.Background() // No claims
	svc := NewMemoryMeshService()

	err := svc.BroadcastIntent(ctx, "hello")
	if err == nil {
		t.Error("expected unauthorized error")
	}

	_, err = svc.Subscribe(ctx)
	if err == nil {
		t.Error("expected unauthorized error")
	}
}

func TestMeshHandlerBroadcast(t *testing.T) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	reqBody := []byte(`{"intent":"hello handler"}`)
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer(reqBody))
	req = req.WithContext(ctx)
	w := httptest.NewRecorder()

	handler.Broadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
	}
}
