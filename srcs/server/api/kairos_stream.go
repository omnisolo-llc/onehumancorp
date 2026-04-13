package api

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow all for now
	},
}

type KairosStreamHandler struct {
	TeammateMesh orchestration.TeammateMesh
}

func NewKairosStreamHandler(tm orchestration.TeammateMesh) *KairosStreamHandler {
	return &KairosStreamHandler{TeammateMesh: tm}
}

type wsMessage struct {
	Type    string \`json:"type"\`
	Payload string \`json:"payload"\`
}

func (h *KairosStreamHandler) ServeWS(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("upgrade error:", err)
		return
	}

	msgChan := make(chan wsMessage, 100)

	// Single writer goroutine
	go func() {
		defer conn.Close()
		for msg := range msgChan {
			bytes, _ := json.Marshal(msg)
			if err := conn.WriteMessage(websocket.TextMessage, bytes); err != nil {
				log.Println("write error:", err)
				return
			}
		}
	}()

	// Subscribe to mesh channels
	subTasks, err := h.TeammateMesh.Subscribe(r.Context(), "mesh:tasks", func(msg []byte) {
		select {
		case msgChan <- wsMessage{Type: "mesh:tasks", Payload: string(msg)}:
		default:
		}
	})
	if err != nil {
		log.Println("subscribe error:", err)
		close(msgChan)
		return
	}
	defer subTasks.Close()

	subCoord, err := h.TeammateMesh.Subscribe(r.Context(), "mesh:coordination", func(msg []byte) {
		select {
		case msgChan <- wsMessage{Type: "mesh:coordination", Payload: string(msg)}:
		default:
		}
	})
	if err != nil {
		log.Println("subscribe error:", err)
		close(msgChan)
		return
	}
	defer subCoord.Close()

	subAutoDream, err := h.TeammateMesh.Subscribe(r.Context(), "autodream", func(msg []byte) {
		select {
		case msgChan <- wsMessage{Type: "autodream", Payload: string(msg)}:
		default:
		}
	})
	if err != nil {
		log.Println("subscribe error:", err)
		close(msgChan)
		return
	}
	defer subAutoDream.Close()

	// Wait for client to disconnect
	for {
		if _, _, err := conn.ReadMessage(); err != nil {
			break
		}
	}
	close(msgChan)
}
