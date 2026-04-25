package mesh

import (
	"context"
	"fmt"
	"github.com/gorilla/websocket"
    "encoding/json"
    "net/http"
	"io/ioutil"
	meshpb "github.com/onehumancorp/mono/src/proto/ohc/mesh"
	"google.golang.org/protobuf/proto"
)

type HTTPHandler struct {
    Broker MeshBroker
}

func NewHTTPHandler(broker MeshBroker) *HTTPHandler {
    return &HTTPHandler{Broker: broker}
}

func (h *HTTPHandler) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
    if r.Method != http.MethodPost {
        http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
        return
    }

    r.Body = http.MaxBytesReader(w, r.Body, 1024*1024)

    var payload struct {
        Channel   string          `json:"channel"`
        EventType string          `json:"event_type"`
        Data      json.RawMessage `json:"data"`
    }

    if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
        http.Error(w, "invalid request", http.StatusBadRequest)
        return
    }

    if payload.Channel == "" {
        http.Error(w, "invalid channel", http.StatusBadRequest)
        return
    }

    rawMsg, err := json.Marshal(payload)
    if err != nil {
        http.Error(w, "internal server error", http.StatusInternalServerError)
        return
    }

    if err := h.Broker.Broadcast(r.Context(), payload.Channel, rawMsg); err != nil {
        http.Error(w, "failed to broadcast", http.StatusInternalServerError)
        return
    }

    w.WriteHeader(http.StatusOK)
    _, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (h *HTTPHandler) HandleBroadcastV2(w http.ResponseWriter, r *http.Request) {
    if r.Method != http.MethodPost {
        http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
        return
    }

    if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

    r.Body = http.MaxBytesReader(w, r.Body, 1024*1024)

    bodyBytes, err := ioutil.ReadAll(r.Body)
    if err != nil {
        http.Error(w, "failed to read body", http.StatusBadRequest)
        return
    }

    var payload meshpb.MeshEvent
    if err := proto.Unmarshal(bodyBytes, &payload); err != nil {
        http.Error(w, "invalid request", http.StatusBadRequest)
        return
    }

    if payload.Channel == "" {
        http.Error(w, "invalid channel", http.StatusBadRequest)
        return
    }

    if err := h.Broker.Broadcast(r.Context(), payload.Channel, bodyBytes); err != nil {
        http.Error(w, "failed to broadcast", http.StatusInternalServerError)
        return
    }

    w.WriteHeader(http.StatusOK)
    _, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (h *HTTPHandler) HandleSubscribeV2(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

    if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	channel := r.URL.Query().Get("channel")
	if channel == "" {
		http.Error(w, "missing channel", http.StatusBadRequest)
		return
	}

	defaultUpgrader := websocket.Upgrader{}
	conn, err := defaultUpgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
	defer conn.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

    msgChan := make(chan []byte, 100)
	sub, err := h.Broker.Subscribe(ctx, channel, func(msg []byte) {
        select {
        case msgChan <- msg:
        case <-ctx.Done():
        }
    })
	if err != nil {
		conn.WriteMessage(websocket.BinaryMessage, []byte(fmt.Sprintf("Failed to subscribe: %v", err)))
		return
	}
    defer sub.Close()

	clientDone := make(chan struct{})

	go func() {
		defer close(clientDone)
		for {
			if _, _, err := conn.ReadMessage(); err != nil {
				break
			}
		}
	}()

	for {
		select {
		case <-clientDone:
			return
		case <-r.Context().Done():
		    return
		case msg := <-msgChan:
			if err := conn.WriteMessage(websocket.BinaryMessage, msg); err != nil {
				return
			}
		}
	}
}
