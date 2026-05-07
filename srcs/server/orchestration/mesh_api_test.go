package orchestration

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestMeshAPI_HandleBroadcast(t *testing.T) {
	hub := NewLocalTeammateMesh()
	api := NewMeshAPI(hub)

	payload := []byte(`{"channel":"mesh:tasks","agent_id":"agent1","action":"completed","status":"success"}`)
	req, err := http.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer(payload))
	assert.NoError(t, err)

	rr := httptest.NewRecorder()

	received := make(chan []byte, 1)
	hub.Subscribe(context.Background(), "mesh:tasks", func(data []byte) {
		received <- data
	})

	api.HandleBroadcast(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)

	select {
	case data := <-received:
		expected := []byte(`{"Content":"{\"channel\":\"mesh:tasks\",\"agent_id\":\"agent1\",\"action\":\"completed\",\"status\":\"success\"}"}`)
		assert.JSONEq(t, string(expected), string(data))
	case <-time.After(1 * time.Second):
		t.Fatal("expected message not received")
	}
}
