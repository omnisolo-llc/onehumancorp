package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestMeshHandler_Broadcast_Success(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to create miniredis: %v", err)
	}
	defer mr.Close()

	redisClient := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	handler := NewMeshHandler(redisClient)

	event := MeshEvent{
		AgentID:   "worker-1",
		Channel:   "mesh:tasks",
		EventType: "TASK_TRANSITION",
	}
	event.Data.TaskID = "uuid-1234"
	event.Data.PreviousState = "QUEUED"
	event.Data.NewState = "IN_PROGRESS"

	body, err := json.Marshal(event)
	if err != nil {
		t.Fatalf("Failed to marshal event: %v", err)
	}

	req, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewReader(body))
	if err != nil {
		t.Fatalf("Failed to create request: %v", err)
	}

	rr := httptest.NewRecorder()
	handler.Broadcast(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}

func TestMeshHandler_Broadcast_BadRequest(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to create miniredis: %v", err)
	}
	defer mr.Close()

	redisClient := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	handler := NewMeshHandler(redisClient)

	// Send invalid JSON
	req, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewReader([]byte("{invalid json}")))
	if err != nil {
		t.Fatalf("Failed to create request: %v", err)
	}

	rr := httptest.NewRecorder()
	handler.Broadcast(rr, req)

	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusBadRequest)
	}
}

func TestMeshHandler_Broadcast_RedisError(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to create miniredis: %v", err)
	}
	addr := mr.Addr()
	mr.Close()

	redisClient := redis.NewClient(&redis.Options{
		Addr: addr,
	})

	handler := NewMeshHandler(redisClient)

	event := MeshEvent{
		AgentID:   "worker-1",
		Channel:   "mesh:tasks",
		EventType: "TASK_TRANSITION",
	}
	event.Data.TaskID = "uuid-1234"
	event.Data.PreviousState = "QUEUED"
	event.Data.NewState = "IN_PROGRESS"

	body, err := json.Marshal(event)
	if err != nil {
		t.Fatalf("Failed to marshal event: %v", err)
	}

	req, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewReader(body))
	if err != nil {
		t.Fatalf("Failed to create request: %v", err)
	}

	rr := httptest.NewRecorder()
	handler.Broadcast(rr, req)

	if status := rr.Code; status != http.StatusInternalServerError {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusInternalServerError)
	}
}
