package api

import (
	"encoding/json"
	"log"
	"net/http"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		return true // allow all for simplicity
	},
}

type MeshHandler struct {
	transport orchestration.MeshTransport
}

func NewMeshHandler(t orchestration.MeshTransport) *MeshHandler {
	return &MeshHandler{transport: t}
}

func (h *MeshHandler) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		Topic   string                    `json:"topic"`
		Message orchestration.MeshMessage `json:"message"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if req.Topic == "" {
		http.Error(w, "topic is required", http.StatusBadRequest)
		return
	}

	err := h.transport.Publish(r.Context(), req.Topic, req.Message)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "ok"})
}

func (h *MeshHandler) HandleSubscribe(w http.ResponseWriter, r *http.Request) {
	topic := r.URL.Query().Get("topic")
	if topic == "" {
		http.Error(w, "topic is required", http.StatusBadRequest)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("websocket upgrade error: %v", err)
		return
	}
	defer conn.Close()

	sendMu := make(chan orchestration.MeshMessage, 100)

	cancel, err := h.transport.Subscribe(r.Context(), topic, func(msg orchestration.MeshMessage) {
		select {
		case sendMu <- msg:
		default:
			log.Printf("websocket buffer full, dropping message")
		}
	})
	if err != nil {
		conn.WriteMessage(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseInternalServerErr, err.Error()))
		return
	}
	defer cancel()

	// Writer pump
	go func() {
		for msg := range sendMu {
			conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
			if err := conn.WriteJSON(msg); err != nil {
				log.Printf("websocket write error: %v", err)
				return
			}
		}
	}()

	// Reader pump (for ping/pong and close detection)
	conn.SetReadLimit(512)
	conn.SetReadDeadline(time.Now().Add(60 * time.Second))
	conn.SetPongHandler(func(string) error { conn.SetReadDeadline(time.Now().Add(60 * time.Second)); return nil })

	for {
		_, _, err := conn.ReadMessage()
		if err != nil {
			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
				log.Printf("websocket unexpected close error: %v", err)
			}
			break
		}
	}
}
