package api

import (
	"context"
	"errors"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"onehumancorp/srcs/server/orchestration"

	"github.com/gorilla/websocket"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type errorTransport struct {
	orchestration.MeshTransport
}

func (e *errorTransport) Publish(ctx context.Context, channel string, data []byte) error {
	return errors.New("publish error")
}

func (e *errorTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	return errors.New("subscribe error")
}

func TestMeshHandler_HandleBroadcast(t *testing.T) {
	transport := orchestration.NewMemoryMeshTransport()
	handler := NewMeshHandler(transport)

	reqBody := `{"agent_id":"test-agent","channel":"test","event_type":"TASK","data":{"foo":"bar"}}`
	req := httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", strings.NewReader(reqBody))
	rr := httptest.NewRecorder()

	handler.HandleBroadcast(rr, req)
	assert.Equal(t, http.StatusOK, rr.Code)
	assert.JSONEq(t, `{"status":"ok"}`, rr.Body.String())

	reqBody = `{"channel":"test","data":{"foo":"bar"}}`
	req = httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", strings.NewReader(reqBody))
	rr = httptest.NewRecorder()
	handler.HandleBroadcast(rr, req)
	assert.Equal(t, http.StatusBadRequest, rr.Code)

	reqBody = `{"channel":"test", "data": {`
	req = httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", strings.NewReader(reqBody))
	rr = httptest.NewRecorder()
	handler.HandleBroadcast(rr, req)
	assert.Equal(t, http.StatusBadRequest, rr.Code)

	req = httptest.NewRequest(http.MethodGet, "/api/v1/mesh/broadcast", nil)
	rr = httptest.NewRecorder()
	handler.HandleBroadcast(rr, req)
	assert.Equal(t, http.StatusMethodNotAllowed, rr.Code)

	errHandler := NewMeshHandler(&errorTransport{})
	reqBody = `{"agent_id":"test-agent","channel":"test","event_type":"TASK","data":{"foo":"bar"}}`
	req = httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", strings.NewReader(reqBody))
	rr = httptest.NewRecorder()
	errHandler.HandleBroadcast(rr, req)
	assert.Equal(t, http.StatusInternalServerError, rr.Code)
}

func TestMeshHandler_HandleSubscribe(t *testing.T) {
	transport := orchestration.NewMemoryMeshTransport()
	handler := NewMeshHandler(transport)
	server := httptest.NewServer(http.HandlerFunc(handler.HandleSubscribe))
	defer server.Close()

	url := "ws" + strings.TrimPrefix(server.URL, "http") + "?channel=test-channel"
	conn, _, err := websocket.DefaultDialer.Dial(url, nil)
	require.NoError(t, err)
	defer conn.Close()

	message := []byte(`{"hello":"world"}`)
	err = transport.Publish(context.Background(), "test-channel", message)
	require.NoError(t, err)

	_, p, err := conn.ReadMessage()
	require.NoError(t, err)
	assert.Equal(t, message, p)

	urlNoChan := "ws" + strings.TrimPrefix(server.URL, "http")
	_, resp, err := websocket.DefaultDialer.Dial(urlNoChan, nil)
	assert.Error(t, err)
	assert.Equal(t, http.StatusBadRequest, resp.StatusCode)
}
