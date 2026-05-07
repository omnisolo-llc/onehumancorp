package mesh

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"onehumancorp/srcs/server/orchestration"
)

type MockMeshHub struct {
	publishedData []byte
	publishedTopic string
}

func (m *MockMeshHub) Publish(ctx context.Context, channel string, data []byte) error {
	m.publishedTopic = channel
	m.publishedData = data
	return nil
}

func (m *MockMeshHub) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	return nil
}

var mockAgents []orchestration.Agent

func (m *MockMeshHub) AdvertiseCapabilities(ctx context.Context, agent orchestration.Agent) error {
	mockAgents = append(mockAgents, agent)
	return nil
}

func (m *MockMeshHub) DiscoverAgents(ctx context.Context, skill string) ([]orchestration.Agent, error) {
	return mockAgents, nil
}


func TestHandleBroadcast(t *testing.T) {
	mockMesh := &MockMeshHub{}
	handler := NewAPIHandler(mockMesh)

	body := []byte(`{
		"topic": "test_topic",
		"message": {
			"agent_id": "test_agent",
			"action": "test_action",
			"status": "ok",
			"payload": {"key": "value"},
			"msg_id": "test_msg_id"
		}
	}`)

	req, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewBuffer(body))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler.HandleBroadcast(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	expectedResponse := `{"success":true}` + "\n"
	if rr.Body.String() != expectedResponse {
		t.Errorf("handler returned unexpected body: got %v want %v", rr.Body.String(), expectedResponse)
	}

	if mockMesh.publishedTopic != "test_topic" {
		t.Errorf("Expected publish topic 'test_topic', got '%s'", mockMesh.publishedTopic)
	}

	var publishedMsg struct {
		AgentID string          `json:"agent_id"`
		Action  string          `json:"action"`
		Status  string          `json:"status"`
		Payload json.RawMessage `json:"payload"`
		MsgID   string          `json:"msg_id"`
	}
	err = json.Unmarshal(mockMesh.publishedData, &publishedMsg)
	if err != nil {
		t.Fatalf("Failed to unmarshal published data: %v", err)
	}

	if publishedMsg.AgentID != "test_agent" {
		t.Errorf("Expected AgentID 'test_agent', got '%s'", publishedMsg.AgentID)
	}
}

func TestHandleBroadcast_MissingFields(t *testing.T) {
	mockMesh := &MockMeshHub{}
	handler := NewAPIHandler(mockMesh)

	// Missing agent_id
	body := []byte(`{
		"topic": "test_topic",
		"message": {
			"action": "test_action",
			"status": "ok",
			"payload": {"key": "value"}
		}
	}`)

	req, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewBuffer(body))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler.HandleBroadcast(rr, req)

	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("handler returned wrong status code for missing agent_id: got %v want %v", status, http.StatusBadRequest)
	}
}

func TestHandleCapabilities_Advertise(t *testing.T) {
	mockMesh := &MockMeshHub{}
	handler := NewAPIHandler(mockMesh)
	mockAgents = nil

	body := []byte(`{
		"agent_id": "test_agent_cap",
		"role": "support",
		"skills": ["chat"]
	}`)

	req, err := http.NewRequest("POST", "/api/mesh/capabilities", bytes.NewBuffer(body))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler.HandleCapabilities(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	if len(mockAgents) != 1 || mockAgents[0].ID != "test_agent_cap" {
		t.Errorf("Expected agent to be advertised")
	}
}

func TestHandleCapabilities_Discover(t *testing.T) {
	mockMesh := &MockMeshHub{}
	handler := NewAPIHandler(mockMesh)
	mockAgents = []orchestration.Agent{{ID: "test_agent_cap", Skills: []string{"chat"}}}

	req, err := http.NewRequest("GET", "/api/mesh/capabilities?skill=chat", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler.HandleCapabilities(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}
