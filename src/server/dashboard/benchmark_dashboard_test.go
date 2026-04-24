package dashboard

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/domain"
	"github.com/onehumancorp/mono/src/server/billing"
	"github.com/onehumancorp/mono/src/server/orchestration"
)

// We define minimal mock implementations to measure the performance improvement
// of the parallelized snapshotLocked method vs the sequential version.

type mockHubBench struct {
	orchestration.Hub
}

func (m *mockHubBench) Agents() []orchestration.Agent {
	time.Sleep(10 * time.Millisecond) // Simulated DB latency
	return []orchestration.Agent{}
}

func (m *mockHubBench) Meetings() []orchestration.MeetingRoom {
	time.Sleep(10 * time.Millisecond) // Simulated DB latency
	return []orchestration.MeetingRoom{}
}

type mockTaskManagerBench struct {
	orchestration.TaskManager
}

func (m *mockTaskManagerBench) PeekTasks(ctx context.Context, limit int) ([]*orchestration.SharedTask, error) {
	time.Sleep(20 * time.Millisecond) // Simulated DB latency
	return []*orchestration.SharedTask{}, nil
}

func (m *mockHubBench) TaskManager() orchestration.TaskManager {
	return &mockTaskManagerBench{}
}

type mockTrackerBench struct {
	billing.Tracker
}

func (m *mockTrackerBench) Summary(orgID string) billing.Summary {
	time.Sleep(30 * time.Millisecond) // Simulated DB latency
	return billing.Summary{}
}

func BenchmarkDashboardSnapshot_Parallel(b *testing.B) {
	s := &Server{
		org:     domain.Organization{ID: "test-org"},
		hub:     &mockHubBench{},
		tracker: &mockTrackerBench{},
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		s.snapshotLocked() // Now uses parallel execution
	}
}
