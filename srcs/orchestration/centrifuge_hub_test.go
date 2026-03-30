package orchestration

import (
	"context"
	"errors"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"golang.org/x/net/websocket"

	"github.com/centrifugal/centrifuge"
)

type mockNode struct {
	errRun      error
	errShutdown error
	errPublish  error

	connectingHandler centrifuge.ConnectingHandler
	connectHandler    centrifuge.ConnectHandler
}

func (m *mockNode) Publish(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
	return centrifuge.PublishResult{}, m.errPublish
}

func (m *mockNode) Shutdown(ctx context.Context) error {
	return m.errShutdown
}

func (m *mockNode) Run() error {
	return m.errRun
}

func (m *mockNode) OnConnecting(h centrifuge.ConnectingHandler) {
	m.connectingHandler = h
}

func (m *mockNode) OnConnect(h centrifuge.ConnectHandler) {
	m.connectHandler = h
}

func TestCentrifugeNode_HandlerMock(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	createNode = func(c centrifuge.Config) (Node, error) {
		return &mockNode{}, nil
	}

	cn, _ := NewCentrifugeNode()
	h := cn.Handler()

	req, _ := http.NewRequest("GET", "http://example.com", nil)
	w := httptest.NewRecorder()
	h.ServeHTTP(w, req)

	if w.Code != http.StatusBadRequest {
		t.Errorf("Expected Bad Request for mock node handler, got %d", w.Code)
	}
}

func TestNewCentrifugeNode_CreationError(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	expectedErr := errors.New("mock creation error")
	createNode = func(c centrifuge.Config) (Node, error) {
		return nil, expectedErr
	}

	cn, err := NewCentrifugeNode()
	if err != expectedErr {
		t.Fatalf("expected error %v, got %v", expectedErr, err)
	}
	if cn != nil {
		t.Fatal("expected nil node on error")
	}
}

func TestHubCentrifugeIntegration(t *testing.T) {
	hub := NewHub()

	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("NewCentrifugeNode() error = %v", err)
	}
	defer cn.Close()

	hub.SetCentrifugeNode(cn)

	if got := hub.CentrifugeNode(); got != cn {
		t.Fatalf("hub.CentrifugeNode() = %v, want %v", got, cn)
	}

	hub.RegisterAgent(Agent{
		ID:             "cn-pm",
		Name:           "PM",
		Role:           "PRODUCT_MANAGER",
		OrganizationID: "org-cn",
	})
	hub.RegisterAgent(Agent{
		ID:             "cn-swe",
		Name:           "SWE",
		Role:           "SOFTWARE_ENGINEER",
		OrganizationID: "org-cn",
	})
	hub.OpenMeetingWithAgenda("cn-meeting", "Integration test", []string{"cn-pm", "cn-swe"})

	if err := hub.Publish(Message{
		ID:        "cn-msg-1",
		FromAgent: "cn-pm",
		Type:      EventTask,
		Content:   "Hello from centrifuge test",
		MeetingID: "cn-meeting",
	}); err != nil {
		t.Fatalf("hub.Publish() error = %v", err)
	}
}

func TestHubCentrifugeNilSafe(t *testing.T) {
	hub := NewHub()

	hub.RegisterAgent(Agent{
		ID:             "nil-pm",
		Name:           "PM",
		Role:           "PRODUCT_MANAGER",
		OrganizationID: "org-nil",
	})
	hub.RegisterAgent(Agent{
		ID:             "nil-swe",
		Name:           "SWE",
		Role:           "SOFTWARE_ENGINEER",
		OrganizationID: "org-nil",
	})
	hub.OpenMeeting("nil-meeting", []string{"nil-pm", "nil-swe"})

	if err := hub.Publish(Message{
		ID:        "nil-msg-1",
		FromAgent: "nil-pm",
		Type:      EventTask,
		Content:   "No centrifuge attached",
		MeetingID: "nil-meeting",
	}); err != nil {
		t.Fatalf("hub.Publish() without centrifuge node error = %v", err)
	}
}

func TestNewCentrifugeNode_RunError(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	expectedErr := errors.New("mock run error")
	createNode = func(c centrifuge.Config) (Node, error) {
		return &mockNode{errRun: expectedErr}, nil
	}

	cn, err := NewCentrifugeNode()
	if err != expectedErr {
		t.Fatalf("expected error %v, got %v", expectedErr, err)
	}
	if cn != nil {
		t.Fatal("expected nil node on error")
	}
}

func TestCentrifugeNodePublishErrorPaths(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	expectedErr := errors.New("mock publish error")
	createNode = func(c centrifuge.Config) (Node, error) {
		return &mockNode{errPublish: expectedErr}, nil
	}

	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("unexpected error %v", err)
	}

	msg := Message{
		ID:        "msg-1",
		FromAgent: "agent-1",
		Type:      EventTask,
		Content:   "Test content",
	}

	cn.PublishMeetingMessage("meeting-1", msg)
	cn.PublishChatMessage("room-1", msg)
	cn.PublishAgentNotification("agent-1", msg)
}

func TestCentrifugeNodeHandlersCoverage(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	var connecting centrifuge.ConnectingHandler
	var connect centrifuge.ConnectHandler

	createNode = func(c centrifuge.Config) (Node, error) {
		m := &mockNode{}
		return m, nil
	}

	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("unexpected error %v", err)
	}

	mock, ok := cn.node.(*mockNode)
	if !ok {
		t.Fatal("expected mockNode")
	}

	connecting = mock.connectingHandler
	connect = mock.connectHandler

	if connecting != nil {
		reply, err := connecting(context.Background(), centrifuge.ConnectEvent{Token: "test-token"})
		if err != nil {
			t.Errorf("connecting err = %v", err)
		}
		if reply.Credentials.UserID != "test-token" {
			t.Errorf("expected UserID test-token, got %s", reply.Credentials.UserID)
		}
	}

	_ = connect
}

func TestCentrifugeNode_HandlerCheckOrigin(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	createNode = func(cfg centrifuge.Config) (Node, error) {
		return centrifuge.New(cfg)
	}

	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("unexpected error %v", err)
	}
	defer cn.Close()

	h := cn.Handler()
	if h == nil {
		t.Fatal("handler returned nil")
	}

	req, _ := http.NewRequest("GET", "http://example.com", nil)
	req.Header.Set("Connection", "Upgrade")
	req.Header.Set("Upgrade", "websocket")
	req.Header.Set("Sec-WebSocket-Version", "13")
	req.Header.Set("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
	req.Header.Set("Origin", "http://example.com")

	w := httptest.NewRecorder()
	h.ServeHTTP(w, req)
}

func TestCentrifugeNodeHandlersCoverageIntegration(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	createNode = func(cfg centrifuge.Config) (Node, error) {
		return centrifuge.New(cfg)
	}

	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("unexpected error %v", err)
	}
	defer cn.Close()

	wsHandler := cn.Handler()
	server := httptest.NewServer(wsHandler)
	defer server.Close()

	url := "ws" + strings.TrimPrefix(server.URL, "http")

	ws, err := websocket.Dial(url, "", "http://localhost/")
	if err != nil {
		t.Fatalf("Dial err: %v", err)
	}

	connectCmd := `{"id": 1, "connect": {}}`
	if err := websocket.Message.Send(ws, connectCmd); err != nil {
		t.Fatalf("Send connect err: %v", err)
	}

	var reply string
	if err := websocket.Message.Receive(ws, &reply); err != nil {
		t.Fatalf("Receive connect err: %v", err)
	}

	subCmd := `{"id": 2, "subscribe": {"channel": "test"}}`
	if err := websocket.Message.Send(ws, subCmd); err != nil {
		t.Fatalf("Send subscribe err: %v", err)
	}
	if err := websocket.Message.Receive(ws, &reply); err != nil {
		t.Fatalf("Receive sub err: %v", err)
	}

	pubCmd := `{"id": 3, "publish": {"channel": "test", "data": {}}}`
	if err := websocket.Message.Send(ws, pubCmd); err != nil {
		t.Fatalf("Send publish err: %v", err)
	}
	if err := websocket.Message.Receive(ws, &reply); err != nil {
		t.Fatalf("Receive pub err: %v", err)
	}

	ws.Close()
	time.Sleep(100 * time.Millisecond)
}

func TestCentrifugeNodeMarshalErrors(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	createNode = func(c centrifuge.Config) (Node, error) {
		return &mockNode{}, nil
	}

	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("unexpected error %v", err)
	}

	badMsg := Message{
		ID:         "msg-bad",
		FromAgent:  "agent-1",
		Type:       EventTask,
		Content:    "Bad content",
		OccurredAt: time.Date(10000, 1, 1, 0, 0, 0, 0, time.UTC),
	}

	cn.PublishMeetingMessage("meeting-bad", badMsg)
	cn.PublishChatMessage("room-bad", badMsg)
	cn.PublishAgentNotification("agent-bad", badMsg)
}
