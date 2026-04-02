package orchestration

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestMeshServer(t *testing.T) {
	ms := NewMeshServer()

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		ms.HandleSubscribe(w, r, "room-1")
	}))
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")

	// Client 1
	dialer := websocket.Dialer{}
	conn1, _, err := dialer.Dial(wsURL, nil)
	require.NoError(t, err)
	defer conn1.Close()

	// Client 2
	conn2, _, err := dialer.Dial(wsURL, nil)
	require.NoError(t, err)
	defer conn2.Close()

	// Give clients time to connect
	time.Sleep(50 * time.Millisecond)

	// Send message from client 1
	msg := MeshMessage{
		SenderID: "agent-1",
		Role:     "SWE",
		Content:  "Hello Room",
	}
	err = conn1.WriteJSON(msg)
	require.NoError(t, err)

	// Client 2 should receive it
	var recvMsg MeshMessage
	err = conn2.ReadJSON(&recvMsg)
	require.NoError(t, err)

	assert.Equal(t, "agent-1", recvMsg.SenderID)
	assert.Equal(t, "SWE", recvMsg.Role)
	assert.Equal(t, "Hello Room", recvMsg.Content)
	assert.NotEmpty(t, recvMsg.Timestamp)
}
