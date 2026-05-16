package interop

import (
	"context"
	"fmt"
	"sync"
	"testing"
	"time"

	interoppb "onehumancorp/srcs/server/pb/interop"
)

type MockBus struct {
	mu          sync.Mutex
	subscribers map[string][]func(Message)
}

func NewMockBus() *MockBus {
	return &MockBus{
		subscribers: make(map[string][]func(Message)),
	}
}

func (m *MockBus) Publish(ctx context.Context, msg Message) error {
	m.mu.Lock()
	subs := m.subscribers[msg.Topic]
	m.mu.Unlock()

	for _, sub := range subs {
		go sub(msg)
	}
	return nil
}

func (m *MockBus) Subscribe(ctx context.Context, topic string, handler func(Message)) (func(), error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.subscribers[topic] = append(m.subscribers[topic], handler)
	return func() {}, nil
}

type MockLock struct {
	mu    sync.Mutex
	locks map[string]string
}

func NewMockLock() *MockLock {
	return &MockLock{
		locks: make(map[string]string),
	}
}

func (m *MockLock) AcquireLock(ctx context.Context, resource string, owner string, ttlSeconds int) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if val, exists := m.locks[resource]; exists && val != owner {
		return false, fmt.Errorf("lock already held")
	}
	m.locks[resource] = owner
	return true, nil
}

func (m *MockLock) ReleaseLock(ctx context.Context, resource string, owner string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	if val, exists := m.locks[resource]; exists && val == owner {
		delete(m.locks, resource)
	}
	return nil
}

func TestHandoff(t *testing.T) {
	bus := NewMockBus()
	lock := NewMockLock()
	protocol := NewInteropProtocol(bus, lock, "node1")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	received := make(chan *interoppb.StateHandoff, 1)
	_, err := protocol.ListenForStateHandoff(ctx, func(handoff *interoppb.StateHandoff) {
		received <- handoff
	})
	if err != nil {
		t.Fatalf("failed to listen: %v", err)
	}

	err = protocol.Handoff(ctx, "mission1", "tenant1", []byte("state"))
	if err != nil {
		t.Fatalf("handoff failed: %v", err)
	}

	select {
	case handoff := <-received:
		if handoff.MissionId != "mission1" {
			t.Errorf("expected mission1, got %s", handoff.MissionId)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for handoff")
	}
}

func TestPing(t *testing.T) {
	bus := NewMockBus()
	lock := NewMockLock()
	protocol := NewInteropProtocol(bus, lock, "node1")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	_, err := protocol.ListenForPings(ctx)
	if err != nil {
		t.Fatalf("failed to listen for pings: %v", err)
	}

	ok, err := protocol.CheckHealth(ctx, 500)
	if err != nil {
		t.Fatalf("check health failed: %v", err)
	}
	if !ok {
		t.Fatal("expected health check to pass")
	}
}

func TestJobDispatch(t *testing.T) {
	bus := NewMockBus()
	lock := NewMockLock()
	protocol := NewInteropProtocol(bus, lock, "node1")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	received := make(chan *interoppb.JobDispatch, 1)
	_, err := protocol.ListenForJobs(ctx, "tenant1", func(job *interoppb.JobDispatch) {
		received <- job
	})
	if err != nil {
		t.Fatalf("failed to listen for jobs: %v", err)
	}

	ok, err := protocol.DispatchJob(ctx, "job1", "tenant1", "action1", []byte("payload"), 500)
	if err != nil {
		t.Fatalf("dispatch job failed: %v", err)
	}
	if !ok {
		t.Fatal("expected job dispatch to be acked")
	}

	select {
	case job := <-received:
		if job.JobId != "job1" {
			t.Errorf("expected job1, got %s", job.JobId)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for job")
	}
}

func TestJobStatus(t *testing.T) {
	bus := NewMockBus()
	lock := NewMockLock()
	protocol := NewInteropProtocol(bus, lock, "node1")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	received := make(chan *interoppb.JobStatusUpdate, 1)
	_, err := protocol.ListenForJobStatus(ctx, "job1", func(status *interoppb.JobStatusUpdate) {
		received <- status
	})
	if err != nil {
		t.Fatalf("failed to listen for job status: %v", err)
	}

	err = protocol.ReportJobStatus(ctx, "job1", "tenant1", "completed", []byte("details"))
	if err != nil {
		t.Fatalf("report job status failed: %v", err)
	}

	select {
	case status := <-received:
		if status.Status != "completed" {
			t.Errorf("expected completed, got %s", status.Status)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for status")
	}
}
