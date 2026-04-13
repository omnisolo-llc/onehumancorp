package api

import (
	"log"
	"net/http"
	"context"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true
	},
}

func HandleKairosStream(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			log.Println("upgrade error:", err)
			return
		}
		defer conn.Close()

		ch := make(chan string, 100)
		subCtx, cancel := context.WithCancel(r.Context())
		defer cancel()

		// In a real implementation this would subscribe to Teammate Mesh Redis
		// Here we just send a heartbeat or read loop
		go func() {
		    for {
			    _, _, err := conn.ReadMessage()
			    if err != nil {
				    cancel()
				    break
			    }
			}
		}()

		for {
		    select {
		    case <-subCtx.Done():
		        return
		    case msg := <-ch:
		        conn.WriteMessage(websocket.TextMessage, []byte(msg))
		    }
		}
	}
}
