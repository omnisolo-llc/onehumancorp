package api_test

import (
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/api"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleKairosStream(t *testing.T) {
    hub := orchestration.NewHub()
	s := httptest.NewServer(api.HandleKairosStream(hub))
	defer s.Close()

	u := "ws" + strings.TrimPrefix(s.URL, "http")
	ws, _, err := websocket.DefaultDialer.Dial(u, nil)
	if err != nil {
		t.Fatalf("%v", err)
	}
	defer ws.Close()
}
